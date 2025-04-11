use clap::Parser;

/// CLI do buscador da MegaStore
#[derive(Parser, Debug)]
#[command(name = "case_megastore_search")]
#[command(author = "Renata Alves")]
#[command(version = "1.0")]
#[command(about = "Busca produtos em JSON", long_about = None)]
pub struct Args {
    
    /// Nome parcial do produto
    #[arg(long)]
    pub name: Option<String>,

    /// Nome da loja
    #[arg(long)]
    pub store: Option<String>,

    /// Cidade onde a loja está localizada
    #[arg(long)]
    pub location: Option<String>,

    /// Categoria do produto (ex: "Eletrônicos")
    #[arg(long)]
    pub category: Option<String>,

    /// Preço mínimo
    #[arg(long)]
    pub min_price: Option<f64>,

    /// Preço máximo
    #[arg(long)]
    pub max_price: Option<f64>,

    /// Campo de ordenação: price ou name
    #[arg(long)]
    pub sort: Option<String>,

    /// Exibir apenas produtos disponíveis em estoque
    #[arg(long)]
    pub available_only: bool,

    /// Exportar resultados para CSV ou JSON (ex: --export csv ou --export json)
    #[arg(long)]
    pub export: Option<String>,

    

}