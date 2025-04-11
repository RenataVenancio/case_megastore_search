use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    pub id: u32,
    pub name: String,
    pub site: String,
    pub location: String,
}