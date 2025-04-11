mod cli;
mod config;
mod models;
mod services;

use cli::Args;
use clap::Parser;
use colored::*;
use config::Config;
use services::export::{export_to_csv, export_to_json};
use services::filters::aplicar_filtros;
use services::utils::{load_products_from_file, load_stores_from_file};

fn main() {
    // Etapa 1: Carrega configurações
    let config = Config::from_env();
    let args = Args::parse();
    println!("Configuração carregada com sucesso: {:?}", config);

    // Etapa 2: Carrega os dados mockados
    let products_path = "data/products.json";
    let stores_path = "data/stores.json";

    let mut produtos = match load_products_from_file(products_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Erro ao carregar produtos: {}", e);
            return;
        }
    };

    let lojas = match load_stores_from_file(stores_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Erro ao carregar lojas: {}", e);
            return;
        }
    };

    // Etapa 3: Aplicar filtros (centralizados no módulo filters.rs)
    produtos = aplicar_filtros(
        produtos,
        args.name.clone(),
        args.store.clone(),
        args.location.clone(),
        args.category.clone(),
        args.min_price,
        args.max_price,
        args.available_only,
        &lojas,
    );

    // Etapa 4: Ordenação
    if let Some(sort_by) = &args.sort {
        match sort_by.as_str() {
            "price" => produtos.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap()),
            "name" => produtos.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
            _ => eprintln!("Campo de ordenação inválido. Use 'price' ou 'name'"),
        }
    }

    // Etapa 5: Exibir resultados
    if produtos.is_empty() {
        println!("{}", "Nenhum produto encontrado com os filtros fornecidos.".red().bold());
    } else {
        for produto in &produtos {
            let id = format!("{}", produto.id).dimmed();
            let name = produto.name.blue().bold();
            let description = produto.description.normal();
            let category = produto.category.cyan();
            let store = produto.store.yellow();
            let price = format!("R${:.2}", produto.price).green();
            let quantity = if produto.quantity > 0 {
                format!("{} unidades", produto.quantity).normal()
            } else {
                "Esgotado".red().bold()
            };
            let link = produto.link.underline();

            println!(
                "\n{} - {}\nDescrição: {}\nCategoria: {}\nLoja: {}\nPreço: {}\nEstoque: {}\nLink: {}\n",
                id, name, description, category, store, price, quantity, link
            );
        }
    }

    // Etapa 6: Exportar resultados
    if let Some(format) = args.export.as_deref() {
        let resultado = match format {
            "csv" => export_to_csv(&produtos, "resultado.csv"),
            "json" => export_to_json(&produtos, "resultado.json"),
            _ => {
                eprintln!("Formato de exportação inválido. Use 'csv' ou 'json'.");
                return;
            }
        };

        match resultado {
            Ok(_) => println!("✅ Resultados exportados com sucesso para 'resultado.{}'", format),
            Err(e) => eprintln!("❌ Erro ao exportar: {}", e),
        }
    }
}
