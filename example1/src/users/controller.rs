use super::model::UserFields;
use crate::database::{controller::Database, model::ConnectionData};
use postgres::types::Type;
use std::collections::HashMap;
use tokio_postgres::{self as postgres, Row};

pub struct Users {
  conn: Database,
}

impl Users {
  pub async fn new(data: &ConnectionData) -> Result<Self, postgres::Error> {
    let conn = Database::connect(data).await?;
    Ok(Self { conn })
  }

  pub async fn insert(
    &mut self,
    db_fields: &UserFields,
    schema: &str,
    tbl_name: &str,
  ) -> Result<i32, postgres::Error> {
    let client = &self.conn.client;
    let fields = db_fields.fields().join(", ");
    let placeholders: Vec<String> = (1..=db_fields.fields().len())
      .map(|i| format!("${}", i))
      .collect();
    let query = format!(
      "INSERT INTO {schema}.{tbl_name} ({}) VALUES ({}) RETURNING id",
      fields,
      placeholders.join(", ")
    );

    let row: postgres::Row = client.query_one(&query, &db_fields.values()).await?;
    let id: i32 = row.get(0);

    Ok(id)
  }

  pub async fn update(
    &mut self,
    id: i32,
    db_fields: &UserFields,
    schema: &str,
    tbl_name: &str,
  ) -> Result<(), postgres::Error> {
    let client = &self.conn.client;
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

    client.execute(&query, &values).await?;

    Ok(())
  }

  fn trim(&self, row: &Row, field: &str) -> String {
    row.get::<_, String>(field).trim().to_string()
  }

  pub async fn read(
    &mut self,
    id: i32,
    schema: &str,
    tbl_name: &str,
  ) -> Result<HashMap<String, String>, postgres::Error> {
    let client = &self.conn.client;
    let query = format!("SELECT * FROM {}.{} WHERE id = $1", schema, tbl_name);
    let row = client.query_one(&query, &[&id]).await?;

    let mut result = HashMap::new();

    for col in row.columns() {
      let col_name = col.name();
      if col_name != "id" {
        let value: String = match *col.type_() {
          Type::INT4 | Type::INT8 => row.get::<_, i32>(col_name).to_string(),
          Type::FLOAT4 | Type::FLOAT8 => row.get::<_, f64>(col_name).to_string(),
          Type::BOOL => row.get::<_, bool>(col_name).to_string(),
          Type::TEXT | Type::VARCHAR | Type::BPCHAR => self.trim(&row, col_name),
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

  // pub async fn read(
  //     &mut self,
  //     id: i32,
  //     schema: &str,
  //     tbl_name: &str,
  // ) -> Result<UserFields, postgres::Error> {
  //     let client = &self.conn.client;
  //     let query = format!("SELECT * FROM {schema}.{tbl_name} WHERE id = $1");
  //     let row = client.query_one(&query, &[&id]).await?;

  //     Ok(UserFields {
  //         name: self.trim(&row, "name"),
  //         email: self.trim(&row, "email"),
  //         age: row.get("age"),
  //     })
  // }

  pub async fn close_connection(self) {
    self.conn.to_close().await;
  }
}
