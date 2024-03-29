use {
    crate::{statement::Statement, ParsedStatement},
    ekg_error::Error,
    ekg_util::{env::mandatory_env_var, log::LOG_TARGET_SPARQL},
    mime::APPLICATION_WWW_FORM_URLENCODED,
};

/// Simple SPARQL client for sending SPARQL queries (or update statements) to a
/// SPARQL endpoint.
#[derive(Clone)]
pub struct SPARQLClient {
    pub(crate) client: hyper::client::Client<
        hyper_rustls::HttpsConnector<hyper::client::HttpConnector>,
        hyper::Body,
    >,
    pub(crate) query_endpoint:  iri_string::types::IriReferenceString,
    pub(crate) update_endpoint: iri_string::types::IriReferenceString,
}

impl SPARQLClient {
    pub async fn from_env() -> Result<Self, Error> {
        let query_endpoint = mandatory_env_var("EKG_SPARQL_QUERY_ENDPOINT", None)?;
        let update_endpoint = mandatory_env_var("EKG_SPARQL_UPDATE_ENDPOINT", None)?;

        Self::new(
            &iri_string::types::IriReferenceString::try_from(query_endpoint.as_str())?,
            Some(&iri_string::types::IriReferenceString::try_from(
                update_endpoint.as_str(),
            )?),
        )
        .await
    }

    pub async fn new(
        query_endpoint: &iri_string::types::IriReferenceStr,
        update_endpoint: Option<&iri_string::types::IriReferenceStr>,
    ) -> Result<Self, Error> {
        let tls_connector = ekg_util::tls_connector::create().await?;

        // Build the hyper client from the HTTPS connector.
        let builder = hyper::client::Client::builder();
        let http_client = builder.build(tls_connector);

        Ok(Self {
            client:          http_client,
            query_endpoint:  query_endpoint.to_owned(),
            update_endpoint: if let Some(update_endpoint) = update_endpoint {
                update_endpoint.to_owned()
            } else {
                query_endpoint.to_owned()
            },
        })
    }

    /// Convert a SPARQL statement into a hyper::Body, properly encoded.
    fn statement_as_body(parsed_statement: &ParsedStatement) -> Result<hyper::Body, Error> {
        let operation = if parsed_statement.statement_type.is_update_statement() {
            "update"
        } else {
            "query"
        };
        let meal = &[(operation, parsed_statement.statement.as_str())];
        let body_str = serde_urlencoded::to_string(meal)?;
        Ok(hyper::Body::from(body_str))
    }

    async fn build_request(
        &self,
        statement: &Statement,
    ) -> Result<hyper::Request<hyper::Body>, ekg_error::Error> {
        let parsed_statement = ParsedStatement::parse(statement, None)?;
        let iri = if parsed_statement.statement_type.is_query_statement() {
            &self.query_endpoint
        } else {
            &self.update_endpoint
        };
        let hyper_uri = hyper::Uri::try_from(iri.as_str())?;
        let accept_header = parsed_statement
            .statement_type
            .default_statement_response_mime_type();

        tracing::debug!(target: LOG_TARGET_SPARQL, "SPARQL endpoint: {:}", iri);
        let request = hyper::http::request::Builder::new()
            .method(hyper::http::method::Method::POST)
            .uri(hyper_uri)
            .header(
                hyper::http::header::CONTENT_TYPE,
                APPLICATION_WWW_FORM_URLENCODED.as_ref(),
            )
            .header(
                hyper::http::header::ACCEPT,
                accept_header,
            )
            // See https://docs.aws.amazon.com/neptune/latest/userguide/access-graph-sparql-http-trailing-headers.html
            .header(hyper::http::header::TE, "trailers, deflate, gzip")
            .body(Self::statement_as_body(&parsed_statement)?)?;
        tracing::info!("request: {:?}", request);
        Ok(request)
    }

    pub async fn execute(&self, statement: &Statement) -> Result<(), Error> {
        tracing::debug!(target: LOG_TARGET_SPARQL, "Execute SPARQL statement:\n{}", statement);

        let req = self.build_request(statement).await?;
        match self.client.request(req).await {
            Ok(response) => {
                let status_code = response.status();
                let (parts, body) = response.into_parts();
                if !status_code.is_success() {
                    tracing::info!(
                        "response: status={:} headers={:?}",
                        parts.status.as_str(),
                        parts.headers
                    );
                }
                // TODO: limit the amount of memory used here
                #[allow(deprecated)]
                let body_bytes = hyper::body::to_bytes(body).await?;
                let v: serde_json::Value = serde_json::from_slice::<serde_json::Value>(&body_bytes)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                tracing::info!("response3: {:?}", serde_json::to_string(&v));
                Ok(())
            },
            Err(error) => {
                tracing::error!("error: {:?}", error);
                Err(Error::from(error))
            },
        }
    }
}
