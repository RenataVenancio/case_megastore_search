use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub link: String,
    pub category: String,
    pub store: String,
    pub quantity: u32,
    pub price: f64,
}