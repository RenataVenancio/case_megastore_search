use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Filters {
    pub category: Option<String>,
    pub keyword: Option<String>,
    pub store: Option<String>,
}