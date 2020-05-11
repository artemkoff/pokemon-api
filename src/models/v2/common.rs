use super::resource::NamedResource;
use serde::{Deserialize, Serialize};

pub type Integer = i32;
pub type Id = Integer;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Name {
    /// The localized name for an API resource in a specific language.
    pub name: String,

    /// The language this name is in.
    pub language: NamedResource,
}
