#![cfg(test)]

use crate::{Prefixes, SPARQLClient, Statement};

#[test_log::test(tokio::test)]
async fn test_sparql_client() -> Result<(), ekg_error::Error> {
    let sparql_client = SPARQLClient::new("https://dbpedia.org/sparql".try_into()?, None).await?;

    let statement = Statement::new(
        &Prefixes::empty()?,
        "SELECT * WHERE { ?s ?p ?o } LIMIT 10".to_string().into(),
    )?;
    sparql_client.execute(&statement).await?;

    // assert!(result.is_object());
    //
    // let head = result["head"].as_object().unwrap();
    //
    // let vars = head["vars"]
    //     .as_array()
    //     .unwrap()
    //     .into_iter()
    //     .map(|v| v.as_str().unwrap())
    //     .collect::<Vec<_>>()
    //     .join(",");
    //
    // assert_eq!(vars, "s,p,o");

    Ok(())
}
