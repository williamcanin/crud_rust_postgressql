use super::traits::DatabaseFields;
use crate::database::model::ConnectionData;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::collections::HashMap;
use tokio_postgres::{self as postgres, types::Type, Row};

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

  pub async fn insert<T: DatabaseFields>(
    &mut self,
    db_fields: &T,
    schema: &str,
    tbl_name: &str,
  ) -> Result<i32, postgres::Error> {
    let fields = db_fields.fields().join(", ");
    let placeholders: Vec<String> = (1..=db_fields.fields().len())
      .map(|i| format!("${}", i))
      .collect();
    let query = format!(
      "INSERT INTO {schema}.{tbl_name} ({}) VALUES ({}) RETURNING id",
      fields,
      placeholders.join(", ")
    );

    let row: postgres::Row = self.client.query_one(&query, &db_fields.values()).await?;
    let id: i32 = row.get(0);

    Ok(id)
  }

  pub async fn update<T: DatabaseFields>(
    &mut self,
    id: i32,
    db_fields: &T,
    schema: &str,
    tbl_name: &str,
  ) -> Result<(), postgres::Error> {
    let fields = db_fields.fields();
    let assignments: Vec<String> = fields
      .iter()
      .enumerate()
      .map(|(i, field)| format!("{} = ${}", field, i + 1))
      .collect();
    let query = format!(
      "UPDATE {schema}.{tbl_name} SET {} WHERE id = ${}",
      assignments.join(", "),
      fields.len() + 1
    );

    let mut values = db_fields.values();
    values.push(&id);

    self.client.execute(&query, &values).await?;

    Ok(())
  }

  fn trim_field(&self, row: &Row, field: &str) -> String {
    row.get::<_, String>(field).trim().to_string()
  }

  fn ignored_fields(&self) -> Vec<&'static str> {
    vec!["id"]
  }

  pub async fn read(
    &mut self,
    id: i32,
    schema: &str,
    tbl_name: &str,
  ) -> Result<HashMap<String, String>, postgres::Error> {
    let query = format!("SELECT * FROM {}.{} WHERE id = $1", schema, tbl_name);
    let row = self.client.query_one(&query, &[&id]).await?;

    let mut result = HashMap::new();

    for col in row.columns() {
      let col_name = col.name();
      if !self.ignored_fields().contains(&col_name) {
        let value: String = match *col.type_() {
          Type::INT4 | Type::INT8 => row.get::<_, i32>(col_name).to_string(),
          Type::FLOAT4 | Type::FLOAT8 => row.get::<_, f64>(col_name).to_string(),
          Type::BOOL => row.get::<_, bool>(col_name).to_string(),
          Type::TEXT | Type::VARCHAR | Type::BPCHAR => self.trim_field(&row, col_name),
          Type::TIMESTAMP => {
            let timestamp: NaiveDateTime = row.get(col_name);
            let datetime: DateTime<Utc> = Utc.from_utc_datetime(&timestamp);
            datetime.to_string()
          }
          _ => {
            let value: Option<String> = row.get(col_name);
            value.unwrap_or_else(|| "NULL".to_string())
          }
        };
        result.insert(col_name.to_string(), value);
      }
    }

    Ok(result)
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

  pub async fn to_close(self) {
    drop(self.client);
  }
}
