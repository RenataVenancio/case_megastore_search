


use serde::Deserialize;
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub api_key: String,
}

// pub fn load_config() -> Config {
//     let data = fs::read_to_string("config.json").expect("Erro ao ler config.json");
//     serde_json::from_str(&data).expect("Erro ao converter config.json")
// }

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok(); // Carrega variáveis do arquivo `.env`

        let api_url = env::var("API_URL").expect("API_URL não encontrada no .env");
        let api_key = env::var("API_KEY").expect("API_KEY não encontrada no .env");

        Config { api_url, api_key }
    }
}