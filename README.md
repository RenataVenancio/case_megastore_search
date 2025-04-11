## 🛒 MegaStore Search — Sistema de Busca Otimizado (Rust)

Este projeto implementa um sistema de busca para o catálogo da MegaStore, utilizando a linguagem Rust com suporte a filtros, ordenação, exportação e estilização em terminal.

---

#### 🚀 Funcionalidades

✅ Busca produtos com base em:

- 🔍 Nome parcial do produto (`--name`)
- 🏬 Loja (`--store`) ou 📍 localização da loja (`--location`)
- 🧠 Categoria (`--category`)
- 💰 Faixa de preço (`--min-price`, `--max-price`)
- 📦 Disponibilidade (`--available-only`)
- 🔃 Ordenação (`--sort price` ou `--sort name`)
- 🧾 Exportação (`--export csv` ou `--export json`)

---

#### 📂 Estrutura

case_megastore_search  
├── src/ # Código-fonte principal  
│ ├── models/ # Models de produto e loja  
│ ├── services/ # Filtros, exportação e utilitários  
│ ├── cli.rs # Interface de linha de comando (clap)  
│ ├── config.rs # Carregamento do .env  
│ └── main.rs # Ponto de entrada da aplicação  
├── data/ # Arquivos mock (JSON)  
│ ├── products.json  
│ └── stores.json  
├── tests/ # Testes de integração  
├── .env # Configurações (API_KEY etc.)  
├── Cargo.toml  
└── README.md  


---

#### ⚙️ Como executar

### 1. Clone o projeto

git clone https://github.com/seu-usuario/case_megastore_search.git
cd case_megastore_search

### 2. Instale as dependências

Certifique-se de ter o Rust instalado. Se não tive:  
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  

### 3. Crie o arquivo .env

API_URL=https://api.megastore.com
API_KEY=123456

### 4. Rode a aplicação com filtros

cargo run -- --name notebook --store "Tech Center" --available-only --sort price

### 5. Exporte os resultados (opcional)

cargo run -- --category "Eletrônicos" --export csv

cargo run -- --location "São Paulo" --category "Informática" --min-price 1000 --max-price 3000 --sort name --export json

#### 🧪 Executar testes

cargo test  

Você verá:  

running 4 tests  
test test_filtrar_por_disponibilidade ... ok  
test test_filtrar_por_categoria ... ok  
test test_filtrar_por_nome_parcial ... ok  
test test_filtrar_por_preco_maximo ... ok  
  
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s  

#### 📦 Tecnologias

* Rust
* clap – Parser de argumentos CLI
* serde – Serialização de JSON
* colored – Estilização de terminal
* csv – Exportação para arquivos CSV
* dotenvy – Variáveis de ambiente

#### 📘 Exemplo de uso

### Buscar notebooks até R$3000
cargo run -- --name notebook --max-price 3000

### Mostrar apenas produtos com estoque disponíveis
cargo run -- --name cafeteira --available-only

### Filtrar por loja e ordenar por preço
cargo run -- --store "InfoShop" --max-price 1500 --sort price

### Exportar todos os eletrônicos disponíveis para CSV
cargo run -- --category "Eletrônicos" --available-only --export csv

#### ✍️ Autor(a)
Renata Alves
Disciplina: Data Structures Strategy and Implementation
Professora: [Nome da Professora]

#### 📄 Licença

MIT License
