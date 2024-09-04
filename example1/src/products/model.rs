use chrono::NaiveDate;

#[derive(Debug)]
pub struct ProductsFields {
  pub name: String,
  pub price: f64,
  pub supplier: String,
  pub code: i32,
  pub expiration_date: NaiveDate,
}

impl ProductsFields {
  pub fn fields(&self) -> Vec<&str> {
    vec!["name", "price", "supplier", "code", "expiration_date"]
  }

  pub fn values(&self) -> Vec<&(dyn tokio_postgres::types::ToSql + Sync)> {
    vec![
      &self.name,
      &self.price,
      &self.supplier,
      &self.code,
      &self.expiration_date,
    ]
  }
}
