use crate::database::model::ConnectionData;
use tokio_postgres::{self as postgres};

pub struct Database {
  pub client: postgres::Client,
}

impl Database {
  pub async fn connect(data: &ConnectionData) -> Result<Self, postgres::Error> {
    let (client, connection) = postgres::connect(&data.db_url, postgres::NoTls).await?;
    tokio::spawn(async move {
      if let Err(e) = connection.await {
        eprintln!("connection error: {}", e);
      }
    });

    Ok(Self { client })
  }

  pub async fn to_close(self) {
    drop(self.client);
  }

  pub async fn delete(
    &mut self,
    schema: &str,
    tbl_name: &str,
    id: i32,
  ) -> Result<bool, postgres::Error> {
    let query = format!("DELETE FROM {schema}.{tbl_name} WHERE id = $1;");
    match self.client.query(&query, &[&id]).await {
      Ok(_) => Ok(true),
      Err(e) => Err(e),
    }
  }
}
