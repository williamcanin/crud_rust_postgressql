#[derive(Debug)]
pub struct UserFields {
  pub name: String,
  pub email: String,
  pub age: i32,
  pub driver_license: bool,
}

impl UserFields {
  pub fn fields(&self) -> Vec<&str> {
    vec!["name", "email", "age", "driver_license"]
  }

  pub fn values(&self) -> Vec<&(dyn tokio_postgres::types::ToSql + Sync)> {
    vec![&self.name, &self.email, &self.age, &self.driver_license]
  }
}
