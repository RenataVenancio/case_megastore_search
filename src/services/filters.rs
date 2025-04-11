use crate::models::{Product, Store};

/// Aplica todos os filtros definidos pelos argumentos da CLI sobre a lista de produtos.
pub fn aplicar_filtros(
    mut produtos: Vec<Product>,
    name: Option<String>,
    store: Option<String>,
    location: Option<String>,
    category: Option<String>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    available_only: bool,
    lojas: &[Store],
) -> Vec<Product> {
    // Filtro por nome parcial do produto (ex: "notebook" encontra "Notebook Slim")
    if let Some(name) = name {
        produtos = produtos
            .into_iter()
            .filter(|p| p.name.to_lowercase().contains(&name.to_lowercase()))
            .collect();
    }

    // Filtro por nome exato da loja (case-insensitive)
    if let Some(store) = store {
        produtos = produtos
            .into_iter()
            .filter(|p| p.store.to_lowercase() == store.to_lowercase())
            .collect();
    }

    // Filtro por localização (cidade) da loja
    // Busca as lojas com localização correspondente e filtra produtos dessas lojas
    if let Some(location) = location {
        let lojas_filtradas: Vec<String> = lojas
            .iter()
            .filter(|l| l.location.to_lowercase() == location.to_lowercase())
            .map(|l| l.name.clone())
            .collect();

        produtos = produtos
            .into_iter()
            .filter(|p| lojas_filtradas.contains(&p.store))
            .collect();
    }

    // Filtro por categoria exata (case-insensitive)
    if let Some(category) = category {
        produtos = produtos
            .into_iter()
            .filter(|p| p.category.to_lowercase() == category.to_lowercase())
            .collect();
    }

    // Filtro por preço mínimo
    if let Some(min) = min_price {
        produtos = produtos
            .into_iter()
            .filter(|p| p.price >= min)
            .collect();
    }

    // Filtro por preço máximo
    if let Some(max) = max_price {
        produtos = produtos
            .into_iter()
            .filter(|p| p.price <= max)
            .collect();
    }

    // Filtro para exibir apenas produtos com quantidade > 0 (em estoque)
    if available_only {
        produtos = produtos
            .into_iter()
            .filter(|p| p.quantity > 0)
            .collect();
    }

    produtos
}
