#![cfg(feature = "_rdfox")]

use {
    crate::{
        prefixes::Prefixes,
        rdfox::{GraphConnection, Parameters, Transaction},
        FactDomain,
        Statement,
    },
    ekg_namespace::{consts::DEFAULT_GRAPH_RDFOX, Class},
    indoc::formatdoc,
    std::{ops::Deref, sync::Arc},
};

/// Some simple queries about a [`Class`](Class)
#[derive(Debug, Clone)]
pub struct ClassReport<'a>(pub &'a Class);

impl<'a> std::fmt::Display for ClassReport<'a> {
    /// TODO: Generate a decent looking set of class metrics
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}

impl<'a> ClassReport<'a> {
    pub fn number_of_individuals(&self, tx: &Arc<Transaction>) -> Result<usize, ekg_error::Error> {
        let default_graph = DEFAULT_GRAPH_RDFOX.deref().as_display_iri();
        let prefixes = Prefixes::builder().declare(&self.0.namespace).build()?;
        let sparql = formatdoc! {r##"
            SELECT DISTINCT ?thing
            WHERE {{
                {{
                    GRAPH ?graph {{
                        ?thing a {self}
                    }}
                }} UNION {{
                        ?thing a {self}
                    BIND({default_graph} AS ?graph)
                }}
            }}
            "##
        };
        tracing::debug!(target: "sparql", "\n{sparql}");
        let params = Parameters::builder().fact_domain(FactDomain::ALL).build()?;
        let count_result = Statement::new(&prefixes, sparql.into())?
            .cursor(&tx.connection, &params)?
            .count(tx);
        #[allow(clippy::let_and_return)]
        count_result
    }

    pub fn number_of_individuals_in_graph(
        &self,
        tx: &Arc<Transaction>,
        graph_connection: &GraphConnection,
    ) -> Result<usize, ekg_error::Error> {
        let graph = graph_connection.graph.as_display_iri();
        let prefixes = Prefixes::builder().declare(&self.0.namespace).build()?;
        let sparql = formatdoc! {r##"
            SELECT DISTINCT ?thing
            WHERE {{
                GRAPH {graph} {{
                    ?thing a {self}
                }}
            }}
            "##
        };
        tracing::debug!(target: "sparql", "\n{sparql}");
        let params = Parameters::builder().fact_domain(FactDomain::ALL).build()?;
        let count_result = Statement::new(&prefixes, sparql.into())?
            .cursor(&graph_connection.data_store_connection, &params)?
            .count(tx);
        #[allow(clippy::let_and_return)]
        count_result
    }
}
