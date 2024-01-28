use {
    core::fmt::{Display, Formatter},
    ekg_namespace::{DEFAULT_GRAPH_RDFOX, LOG_TARGET_SPARQL},
    indoc::formatdoc,
    std::{borrow::Cow, ops::Deref},
};

#[cfg(test)]
mod tests;

/// SPARQL Statement
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Statement {
    pub(crate) prefixes: crate::Prefixes,
    pub(crate) text:     String,
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // writeln!(f, "SPARQL Statement:")?;
        for (number, line) in self.text.lines().enumerate() {
            writeln!(f, "{:0>4}: {line}", number + 1)?;
        }
        Ok(())
    }
}

impl Statement {
    pub fn new<'a>(
        prefixes: &'a crate::Prefixes,
        statement: Cow<str>,
    ) -> Result<Self, ekg_error::Error> {
        let s = Self {
            prefixes: prefixes.clone(),
            text:     format!("{}\n{}", &prefixes.to_string(), statement.trim()),
        };
        tracing::trace!(target: LOG_TARGET_SPARQL, "{:}", s);
        Ok(s)
    }

    pub fn as_str(&self) -> &str { self.text.as_str() }

    pub fn no_comments(&self) -> String { no_comments(self.text.as_str()) }

    /// Return a Statement that can be used to export all data in
    /// `application/nquads` format
    pub fn nquads_query(prefixes: &crate::Prefixes) -> Result<Statement, ekg_error::Error> {
        let default_graph = DEFAULT_GRAPH_RDFOX.deref().as_display_iri();
        let statement = Statement::new(
            prefixes,
            formatdoc!(
                r##"
                SELECT ?S ?P ?O ?G
                WHERE {{
                    {{
                        GRAPH ?G {{ ?S ?P ?O }}
                    }} UNION {{
                        ?S ?P ?P .
                        BIND({default_graph} AS ?G)
                    }}
                }}
            "##
            )
            .into(),
        )?;
        Ok(statement)
    }
}

pub fn no_comments(string: &str) -> String {
    use std::fmt::Write;

    let re = fancy_regex::Regex::new(r"(.*)(?!#>)#.*$").unwrap();

    let do_line = |line: &str| -> (bool, String) {
        let caps = re.captures(line);
        if let Ok(Some(caps)) = caps {
            let mat = caps.get(1).unwrap();
            (
                true,
                line[mat.start()..mat.end()].trim_end().to_string(),
            )
        } else {
            (false, line.trim_end().to_string())
        }
    };

    let mut output = String::new();
    for line in string.lines() {
        let mut line = line.to_string();
        loop {
            let (again, result) = do_line(line.as_str());
            if again {
                // Repeat the call to do_line again to make sure that all #-comments are removed
                // (there could be multiple on one line)
                line = result;
            } else {
                writeln!(&mut output, "{result}").unwrap();
                break
            }
        }
    }
    output
}
