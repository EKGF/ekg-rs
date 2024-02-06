#[repr(u32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[allow(non_camel_case_types)]
/// The result of declaring a prefix.
/// Note that the values are defined in the RDFox C API.
pub enum PrefixesDeclareResult {
    PREFIXES_INVALID_PREFIX_NAME = 0,
    PREFIXES_NO_CHANGE           = 1,
    PREFIXES_REPLACED_EXISTING   = 2,
    PREFIXES_DECLARED_NEW        = 3,
}

#[cfg(feature = "_rdfox")]
impl From<rdfox_sys::CPrefixes_DeclareResult> for PrefixesDeclareResult {
    fn from(value: rdfox_sys::CPrefixes_DeclareResult) -> Self {
        match value {
            rdfox_sys::CPrefixes_DeclareResult::PREFIXES_INVALID_PREFIX_NAME => {
                PrefixesDeclareResult::PREFIXES_INVALID_PREFIX_NAME
            },
            rdfox_sys::CPrefixes_DeclareResult::PREFIXES_NO_CHANGE => {
                PrefixesDeclareResult::PREFIXES_NO_CHANGE
            },
            rdfox_sys::CPrefixes_DeclareResult::PREFIXES_REPLACED_EXISTING => {
                PrefixesDeclareResult::PREFIXES_REPLACED_EXISTING
            },
            rdfox_sys::CPrefixes_DeclareResult::PREFIXES_DECLARED_NEW => {
                PrefixesDeclareResult::PREFIXES_DECLARED_NEW
            },
        }
    }
}
