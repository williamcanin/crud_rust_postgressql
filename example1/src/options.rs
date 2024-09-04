use crate::database::model::ConnectionData;
use dotenv::dotenv;
use std::env;

// Dados de conexão remota.
pub fn connection_data() -> ConnectionData {
  // Carrega as variáveis do arquivo .env
  dotenv().ok();
  ConnectionData {
    db_url: env::var("DB_URL").unwrap(),
    schema: env::var("SCHEMA").unwrap(),
  }
}
