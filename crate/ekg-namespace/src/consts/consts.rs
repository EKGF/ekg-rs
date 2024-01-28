//! static/const strings

use const_format::concatcp;

/// The part in the URL that identifies the URL as being an "EKG/IRI" or an "EKG
/// Identifier", it is assumed to sit between two slashes somewhere in the
/// beginning of the URL path.
pub const DEFAULT_ID_PATH: &str = "id";
/// The part in the URL that identifies the URL as being a special "EKG/IRI" or
/// an "EKG Identifier" meant to identify so-called "named graphs".
/// It is assumed to sit between two slashes somewhere in the beginning of the
/// URL path.
pub const DEFAULT_GRAPH_PATH: &str = "graph";
/// The part in the URL that identifies the URL as being a special "EKG/IRI" or
/// an "EKG Identifier" meant to identify so-called "ontologies" or "axioms"
/// within Ontologies.
/// It is assumed to sit between two slashes somewhere in the beginning of the
/// URL path.
pub const DEFAULT_ONTOLOGY_PATH: &str = "ontology";

/// The placeholder "base IRI" is a special value that we use in git-based
/// metadata, avoiding the need to put anyone's company name in there which
/// drastically lowers the reusability of a given piece of metadata (usyally in
/// the form of Turtle RDF or SPARQL). This placeholder will be replaced with
/// your configured base IRI at load time, for instance when you execute the
/// Dataroaa CLI as `ekg load`.
pub const PLACEHOLDER_BASE_IRI: &str = "https://placeholder.kg";
pub const DEFAULT_BASE_IRI: &str = PLACEHOLDER_BASE_IRI;
// deprecated
/// The default address and port where the DataRoad Studio or DataRoad Server
/// host your DataRoad Access Point (DAP).
pub const DEFAULT_LOCAL_BASE_IRI: &str = "http://127.0.0.1:7878";
/// Only use [`PLACEHOLDER_ID_BASE_IRI`] in git-based RDF files
pub const PLACEHOLDER_ID_BASE_IRI: &str =
    concatcp!(PLACEHOLDER_BASE_IRI, "/", DEFAULT_ID_PATH, "/");
/// Only use [`PLACEHOLDER_GRAPH_BASE_IRI`] in git-based RDF files
pub const PLACEHOLDER_GRAPH_BASE_IRI: &str =
    concatcp!(PLACEHOLDER_BASE_IRI, "/", DEFAULT_GRAPH_PATH, "/");
/// Only use [`PLACEHOLDER_ONTOLOGY_BASE_IRI`] in git-based RDF files
pub const PLACEHOLDER_ONTOLOGY_BASE_IRI: &str = concatcp!(
    PLACEHOLDER_BASE_IRI,
    "/",
    DEFAULT_ONTOLOGY_PATH,
    "/"
);
