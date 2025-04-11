use std::fs;
use std::path::Path;
use serde_json::from_str;
use crate::models::{Product, Store};

pub fn load_products_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Product>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let products: Vec<Product> = from_str(&data)?;
    Ok(products)
}

pub fn load_stores_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Store>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let stores: Vec<Store> = from_str(&data)?;
    Ok(stores)
}
