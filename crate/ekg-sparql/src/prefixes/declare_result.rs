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
