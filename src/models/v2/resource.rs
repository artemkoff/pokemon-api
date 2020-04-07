use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Resource {
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NamedResource {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResourceList {
    pub count: usize,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Resource>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NamedResourceList {
    pub count: usize,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<NamedResource>,
}