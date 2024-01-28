#![deny(unused_crate_dependencies)]

use thiserror::Error;

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum Error {
    #[allow(dead_code)]
    #[error("Unknown Error")]
    Unknown,

    #[allow(dead_code)]
    #[error("Not Implemented")]
    NotImplemented,

    #[error("Cannot start a new transaction")]
    CannotStartNewTransaction,

    #[allow(dead_code)]
    #[error("Invalid Input")]
    InvalidInput,

    #[allow(dead_code)]
    #[error("No Input Records")]
    NoInputRecords,

    #[allow(dead_code)]
    #[error("Timeout")]
    Timeout,

    #[allow(dead_code)]
    #[error("Mandatory Environment Variable {0} is empty")]
    EnvironmentVariableEmpty(String),

    #[allow(dead_code)]
    #[error("Mandatory Environment Variable {0} missing")]
    MandatoryEnvironmentVariableMissing(String),

    #[allow(dead_code)]
    #[error("Mandatory Environment Variable {0} is not a valid IRI")]
    MandatoryEnvironmentVariableIsNotIRI(String),

    #[allow(dead_code)]
    #[error("Service error {0}")]
    ServiceError(String),

    #[allow(dead_code)]
    #[error("No event")]
    NoEvent,

    #[allow(dead_code)]
    #[error("No subject")]
    NoSubject,

    #[allow(dead_code)]
    #[error("No predicate")]
    NoPredicate,

    #[allow(dead_code)]
    #[error(
        "Detected an unknown story input parameter [{param}] for story [{story_key}], expected \
         parameters are: {expected_params:?}"
    )]
    DetectedUnknownStoryInputParameter {
        story_key:       String,
        param:           String,
        expected_params: Vec<String>,
    },

    #[error("No base IRI specified")]
    NoBaseIRISpecified,

    #[error("No identifier namespace specified")]
    NoIdentifierNamespaceSpecified,

    #[error("Incorrect base IRI: {iri}")]
    IncorrectBaseIRI { iri: String },

    #[allow(dead_code)]
    #[error("Parse Error")]
    Parse,

    #[cfg(feature = "uuid")]
    #[error(transparent)]
    UuidParseError(#[from] uuid::Error),

    #[error("Could not fetch context and objects")]
    MissingContext,

    #[error("Could not generate metadata")]
    CouldNotGenerateMetadata,

    #[error("Could not find root project")]
    CouldNotFindRootProject,

    #[error("Could not rewrite IRI [{iri}]")]
    CouldNotRewriteIRI { iri: String },

    #[allow(dead_code)]
    #[error("Invalid Docker Image ID")]
    InvalidDockerImageId,

    #[error("Could not lock a resource: {msg}")]
    CouldNotLock { msg: String },

    #[error("Missing Identifier Base IRI")]
    MissingIdentifierBaseIRI,

    #[error("Could not create the story service client")]
    CouldNotCreateClient,

    #[error("Could not connect to the database server")]
    CouldNotConnectToServer,

    #[error("There's no context")]
    NoContextProvided,

    #[error("Invalid Story Service IRI")]
    InvalidClientIri,

    #[allow(dead_code)]
    #[error("Persona {persona_key} does not exist")]
    PersonaDoesNotExist { persona_key: String },

    #[allow(dead_code)]
    #[error("Story {use_case_key}/{story_key} does not exist")]
    StoryDoesNotExist { story_key: String, use_case_key: String },

    #[allow(dead_code)]
    #[error("UseCase {use_case_key} does not exist")]
    UseCaseDoesNotExist { use_case_key: String },

    #[cfg(feature = "serde")]
    #[error("JSON Parsing Error")]
    JSONParseError(serde_path_to_error::Error<serde_json::Error>),

    #[cfg(feature = "serde")]
    #[error("Serde Error")]
    SerdeError(#[from] serde_json::Error),

    #[error("Invalid Story IRI")]
    InvalidStoryIri,

    #[error("Could not get story results")]
    CouldNotGetStoryResults,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    /// Encountered a syntax error in a SPARQL statement
    #[cfg(not(target_arch = "wasm32"))]
    #[error("Encountered SPARQL error \"{source:}\" in\n{statement:}")]
    SPARQLStatementError {
        #[source]
        source:    spargebra::ParseError,
        statement: String,
    },

    #[error(transparent)]
    FormatError(#[from] core::fmt::Error),

    /// Represents all other cases of `ignore::Error`
    /// (see <https://docs.rs/ignore/latest/ignore/enum.Error.html>)
    #[cfg(feature = "fs")]
    #[error(transparent)]
    WalkError(#[from] ignore::Error),

    #[cfg(feature = "iref")]
    #[error(transparent)]
    IriErrorString(#[from] iref::IriError<String>),

    #[cfg(feature = "iri-string")]
    #[error(transparent)]
    IriStringParseError(#[from] iri_string::validate::Error),

    #[error(transparent)]
    CApiError(#[from] std::ffi::NulError),

    #[cfg(all(feature = "rdftk-support", not(target_arch = "wasm32")))]
    #[error(transparent)]
    RDFTkError(#[from] rdftk_core::error::Error),

    #[cfg(all(feature = "rdftk-support", not(target_arch = "wasm32")))]
    #[error(transparent)]
    RDFTkIRIError(#[from] rdftk_iri::error::Error),

    #[cfg(feature = "rdf-store")]
    #[error(transparent)]
    RDFStoreError(#[from] rdf_store_rs::RDFStoreError),

    #[error("Could not open database: {source:}")]
    CouldNotOpenDatabase { source: Box<Error> },

    #[cfg(all(not(target_arch = "wasm32"), feature = "cli"))]
    #[error(transparent)]
    ExcelWriterError(#[from] xlsxwriter::XlsxError),
    // TODO: Turn this into feature for
    // conditional compilation
    #[cfg(all(not(target_arch = "wasm32"), feature = "no-wasm"))]
    #[error(transparent)]
    EnvFilterParseError(#[from] tracing_subscriber::filter::ParseError),

    #[cfg(all(not(target_arch = "wasm32"), feature = "no-wasm"))]
    #[error(transparent)]
    TracingSubscriberError(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[cfg(all(not(target_arch = "wasm32"), feature = "no-wasm"))]
    #[error(transparent)]
    TracingSubscriberTryInitError(#[from] tracing_subscriber::util::TryInitError),

    #[cfg(all(
        not(target_arch = "wasm32"),
        feature = "no-wasm",
        feature = "color-eyre"
    ))]
    #[error(transparent)]
    ColorEyreError(#[from] color_eyre::eyre::ErrReport),

    #[cfg(all(not(target_arch = "wasm32"), feature = "no-wasm", feature = "salvo"))]
    #[error(transparent)]
    InvalidHeaderValue(#[from] salvo::http::header::InvalidHeaderValue),

    #[cfg(all(feature = "salvo", not(target_arch = "wasm32")))]
    #[error(transparent)]
    SalvoError(#[from] salvo::Error),

    #[cfg(all(feature = "salvo", not(target_arch = "wasm32")))]
    #[error(transparent)]
    ToStrError(#[from] salvo_core::http::header::ToStrError),

    #[cfg(feature = "tokio")]
    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),

    #[cfg(feature = "tauri")]
    #[error(transparent)]
    TauriError(#[from] tauri::Error),

    #[cfg(feature = "gix")]
    #[error(transparent)]
    GitDiscoverUpwardsError2(#[from] gix_discover::upwards::Error),

    #[cfg(feature = "reqwest")]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[cfg(feature = "reqwest")]
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[cfg(feature = "reqwest")]
    #[error(transparent)]
    HttpError(#[from] http::Error),

    #[cfg(all(feature = "reqwest", not(target_arch = "wasm32")))]
    #[error(transparent)]
    StreamBodyError(#[from] reqwest_streams::error::StreamBodyError),

    #[cfg(feature = "iri-string")]
    #[error(transparent)]
    IriStringCreationError(#[from] iri_string::types::CreationError<String>),

    #[cfg(feature = "iri")]
    #[error("Encountered IRI error \"{error:}\" in\n{iri:}")]
    IrefError { error: ekg_error::Error, iri: String },

    #[cfg(not(any(target_arch = "wasm32", feature = "wasm-support")))]
    #[error(transparent)]
    FromEnvError(#[from] tracing_subscriber::filter::FromEnvError),

    #[cfg(feature = "aws-lambda-runtime")]
    #[error(transparent)]
    LambdaError(#[from] lambda_runtime::Error),

    #[error(transparent)]
    HyperError(#[from] hyper::Error),

    #[error(transparent)]
    HttpError(#[from] hyper::http::Error),

    #[error(transparent)]
    InvalidUri(#[from] hyper::http::uri::InvalidUri),

    #[error(transparent)]
    InvalidUri2(#[from] fluent_uri::ParseError),

    #[error(transparent)]
    SerdeUrlEncodingError(#[from] serde_urlencoded::ser::Error),

    #[error("Unknown data type {data_type_id}")]
    UnknownDataType { data_type_id: u8 },
    #[error("Unknown value [{value}] for data type {data_type_xsd_iri:?}")]
    UnknownValueForDataType {
        data_type_xsd_iri: String,
        value:             String,
    },
    #[error("Unknown XSD data type {data_type_iri}")]
    UnknownXsdDataType { data_type_iri: String },
    #[error("Unknown literal value in N-Triples format: {value}")]
    UnknownNTriplesValue { value: String },
}

unsafe impl Send for Error {}

// #[cfg(feature = "iref")]
// impl From<(ekg_error::Error, String)> for Error {
//     fn from(value: (ekg_error::Error, String)) -> Self {
//         Error::IrefError { error: value.0, iri: value.1 }
//     }
// }

#[cfg(all(feature = "salvo", not(target_arch = "wasm32")))]
#[salvo::async_trait]
impl salvo::Writer for Error {
    async fn write(
        mut self,
        _req: &mut salvo::http::Request,
        _depot: &mut salvo::Depot,
        res: &mut salvo::http::Response,
    ) {
        res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
        res.render("custom error");
    }
}

/// Map an [`rdf_store_rs::RDFStoreError`] to a DataRoad Error
#[cfg(feature = "rdf-store")]
pub fn error_from_rdfox(rdf_store_error: rdf_store_rs::RDFStoreError) -> Error {
    tracing::error!("Got error: {rdf_store_error:?}");
    Error::RDFStoreError(rdf_store_error)
}
