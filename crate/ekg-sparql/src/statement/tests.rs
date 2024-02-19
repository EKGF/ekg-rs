#![cfg(all(test, not(target_family = "wasm")))]

#[test_log::test]
fn test_no_comments() {
    let sparql = indoc::formatdoc! {r##"
            PREFIX abc: <https://whatever.org#> # focus on this and the next line
            PREFIX owl: <http://www.w3.org/2002/07/owl#>
            SELECT DISTINCT ?thing
            WHERE {{
                {{ # some comment
                    GRAPH ?graph {{ # more # and more
                        ?thing a Whatever#
                    }}
                }} UNION {{
                    ?thing a Whatever .# abc
                                       # def
                    BIND(graph:Graph AS ?graph)
                }}
            }}
            "##
    };
    let expected = indoc::formatdoc! {r##"
            PREFIX abc: <https://whatever.org#>
            PREFIX owl: <http://www.w3.org/2002/07/owl#>
            SELECT DISTINCT ?thing
            WHERE {{
                {{
                    GRAPH ?graph {{
                        ?thing a Whatever
                    }}
                }} UNION {{
                    ?thing a Whatever .

                    BIND(graph:Graph AS ?graph)
                }}
            }}
            "##
    };
    let actual = crate::no_comments(sparql.as_str());
    assert_eq!(actual.as_str(), expected.as_str());
}

#[test_log::test]
fn test_embedded_param() {
    let sparql = indoc::formatdoc! {r##"
            PREFIX abc: <https://whatever.org#>
            PREFIX owl: <http://www.w3.org/2002/07/owl#>
            # rdfox-query-validation: standard-compliant
            # unrecognized: some other comment
            SELECT DISTINCT ?thing
            # some other comment with a colon: in there
            WHERE {{
                ?thing ?p ?o .
            }}
            "##
    };
    let statement = crate::Statement::new(
        crate::Prefixes::builder().build().unwrap(),
        std::borrow::Cow::Borrowed(sparql.as_str()),
    );
    assert!(statement.is_ok());
    let statement = statement.unwrap();

    assert_eq!(
        statement.params.get(crate::RDFOX_QUERY_VALIDATION),
        Some(&crate::RDFOX_QUERY_VALIDATION_STANDARD_COMPLIANT)
    );
}
