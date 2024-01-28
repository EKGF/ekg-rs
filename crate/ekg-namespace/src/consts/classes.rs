#![allow(missing_docs)]
#![allow(clippy::wildcard_imports)]

use {
    crate::{
        consts::{local_names::*, namespaces::*},
        Class,
    },
    lazy_static::lazy_static,
};

// Classes
#[rustfmt::skip]
lazy_static! {
    pub static ref CLASS_CONCEPT_CONCEPT: Class = Class::declare(PREFIX_CONCEPT.clone(), LN_CONCEPT);
    pub static ref CLASS_DATASET_DATASET: Class = Class::declare(PREFIX_DATASET.clone(), LN_DATASET);
    pub static ref CLASS_DCAT_DATASET: Class = Class::declare(PREFIX_DCAT.clone(), LN_DATASET);
    pub static ref CLASS_EKGMM_CAPABILITY: Class = Class::declare(PREFIX_EKGMM.clone(), LN_CAPABILITY);
    pub static ref CLASS_PERSONA_PERSONA: Class = Class::declare(PREFIX_PERSONA.clone(), LN_PERSONA);
    pub static ref CLASS_SBE_SCENARIO: Class = Class::declare(PREFIX_SBE.clone(), LN_SCENARIO);
    pub static ref CLASS_SBE_STATE: Class = Class::declare(PREFIX_SBE.clone(), LN_STATE);
    pub static ref CLASS_SBE_STORY_REQUEST: Class = Class::declare(PREFIX_SBE.clone(), LN_STORY_REQUEST);
    pub static ref CLASS_STORY_MANDATORY_PARAMETER: Class = Class::declare(PREFIX_STORY.clone(), LN_MANDATORY_PARAMETER);
    pub static ref CLASS_STORY_OPTIONAL_PARAMETER: Class = Class::declare(PREFIX_STORY.clone(), LN_OPTIONAL_PARAMETER);
    pub static ref CLASS_STORY_OPTIONAL_STORY_OUTPUT: Class = Class::declare(PREFIX_STORY.clone(), LN_OPTIONAL_STORY_OUTPUT);
    pub static ref CLASS_STORY_IMPL_SPARQL: Class = Class::declare(PREFIX_STORY_IMPL_SPARQL.clone(), LN_STORY_SPARQL_IMPLEMENTATION);
    pub static ref CLASS_STORY_IMPL_SPARQL_ASK: Class = Class::declare(PREFIX_STORY_IMPL_SPARQL.clone(), LN_ASK_UC);
    pub static ref CLASS_STORY_IMPL_SPARQL_CONSTRUCT: Class = Class::declare(PREFIX_STORY_IMPL_SPARQL.clone(), LN_CONSTRUCT_UC);
    pub static ref CLASS_STORY_IMPL_SPARQL_DELETE: Class = Class::declare(PREFIX_STORY_IMPL_SPARQL.clone(), LN_DELETE_UC);
    pub static ref CLASS_STORY_IMPL_SPARQL_DESCRIBE: Class = Class::declare(PREFIX_STORY_IMPL_SPARQL.clone(), LN_DESCRIBE_UC);
    pub static ref CLASS_STORY_IMPL_SPARQL_UPDATE: Class = Class::declare(PREFIX_STORY_IMPL_SPARQL.clone(), LN_UPDATE_UC);
    pub static ref CLASS_STORY_IMPL_SPARQL_SELECT: Class = Class::declare(PREFIX_STORY_IMPL_SPARQL.clone(), LN_SELECT_UC);
    pub static ref CLASS_STORY_IMPL_SPARQL_SPARQL10: Class = Class::declare(PREFIX_STORY_IMPL_SPARQL.clone(), LN_SPARQL10_UC);
    pub static ref CLASS_STORY_IMPL_SPARQL_SPARQL11: Class = Class::declare(PREFIX_STORY_IMPL_SPARQL.clone(), LN_SPARQL11_UC);
    pub static ref CLASS_STORY_STORY: Class = Class::declare(PREFIX_STORY.clone(), LN_STORY);
    pub static ref CLASS_STORY_STORY_INPUT: Class = Class::declare(PREFIX_STORY.clone(), LN_STORY_INPUT);
    pub static ref CLASS_STORY_STORY_OUTPUT: Class = Class::declare(PREFIX_STORY.clone(), LN_STORY_OUTPUT);
    pub static ref CLASS_STORY_TRANSFORMATION_RULE: Class = Class::declare(PREFIX_STORY.clone(), LN_TRANSFORMATION_RULE);
    pub static ref CLASS_USE_CASE_USE_CASE: Class = Class::declare(PREFIX_USE_CASE.clone(), LN_USE_CASE);
}
