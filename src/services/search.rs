use crate::models::{Product, Store};

/// Função principal que executa a busca de produtos com base no termo fornecido.
/// Retorna uma lista de produtos cujo nome contém o termo (case-insensitive).
pub fn search_products(term: &str, products: &[Product], _stores: &[Store]) -> Vec<Product> {
    products
        .iter()
        .filter(|product| product.name.to_lowercase().contains(&term.to_lowercase()))
        .cloned()
        .collect()
}
