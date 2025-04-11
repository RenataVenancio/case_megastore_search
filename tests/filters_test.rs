use case_megastore_search::models::{Product, Store};
use case_megastore_search::services::filters::aplicar_filtros;

fn mock_produtos() -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Notebook Gamer".into(),
            description: "Rápido e potente".into(),
            link: "https://loja.com/p1".into(),
            category: "Informática".into(),
            store: "Tech Center".into(),
            price: 2999.99,
            quantity: 10,
        },
        Product {
            id: 2,
            name: "Cafeteira Elétrica".into(),
            description: "Faz café excelente".into(),
            link: "https://loja.com/p2".into(),
            category: "Eletrodomésticos".into(),
            store: "Casa & Vídeo".into(),
            price: 199.90,
            quantity: 0,
        },
    ]
}

fn mock_lojas() -> Vec<Store> {
    vec![
        Store {
            id: 1,
            name: "Tech Center".into(),
            location: "São Paulo".into(),
            site: "https://techcenter.com".into(),
        },
        Store {
            id: 2,
            name: "Casa & Vídeo".into(),
            location: "Rio de Janeiro".into(),
            site: "https://casaevideo.com".into(),
        },
    ]
}

#[test]
fn test_filtrar_por_nome_parcial() {
    let produtos = mock_produtos();
    let lojas = mock_lojas();

    let filtrados = aplicar_filtros(
        produtos,
        Some("note".to_string()),
        None,
        None,
        None,
        None,
        None,
        false,
        &lojas,
    );

    assert_eq!(filtrados.len(), 1);
    assert_eq!(filtrados[0].name, "Notebook Gamer");
}

#[test]
fn test_filtrar_por_categoria() {
    let produtos = mock_produtos();
    let lojas = mock_lojas();

    let filtrados = aplicar_filtros(
        produtos,
        None,
        None,
        None,
        Some("Eletrodomésticos".to_string()),
        None,
        None,
        false,
        &lojas,
    );

    assert_eq!(filtrados.len(), 1);
    assert_eq!(filtrados[0].category, "Eletrodomésticos");
}

#[test]
fn test_filtrar_por_disponibilidade() {
    let produtos = mock_produtos();
    let lojas = mock_lojas();

    let filtrados = aplicar_filtros(
        produtos,
        None,
        None,
        None,
        None,
        None,
        None,
        true, // somente disponíveis
        &lojas,
    );

    assert_eq!(filtrados.len(), 1);
    assert!(filtrados[0].quantity > 0);
}

#[test]
fn test_filtrar_por_preco_maximo() {
    let produtos = mock_produtos();
    let lojas = mock_lojas();

    let filtrados = aplicar_filtros(
        produtos,
        None,
        None,
        None,
        None,
        None,
        Some(200.0), // max_price
        false,
        &lojas,
    );

    assert_eq!(filtrados.len(), 1);
    assert!(filtrados[0].price <= 200.0);
}
