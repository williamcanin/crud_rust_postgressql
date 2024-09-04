#[derive(Debug)]
pub struct UserFields {
  pub name: String,
  pub email: String,
  pub age: i32,
}

impl UserFields {
  pub fn fields(&self) -> Vec<&str> {
    vec!["name", "email", "age"]
  }

  pub fn values(&self) -> Vec<&(dyn tokio_postgres::types::ToSql + Sync)> {
    vec![&self.name, &self.email, &self.age]
  }
}
