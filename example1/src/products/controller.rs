use super::model::ProductsFields;
use crate::database::traits::DatabaseFields;
use tokio_postgres::types::ToSql;

impl DatabaseFields for ProductsFields {
  fn fields(&self) -> Vec<&str> {
    self.fields()
  }

  fn values(&self) -> Vec<&(dyn ToSql + Sync)> {
    self.values()
  }
}
