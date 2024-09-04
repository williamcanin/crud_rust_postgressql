use super::model::UserFields;
use crate::database::traits::DatabaseFields;
use tokio_postgres::types::ToSql;

impl DatabaseFields for UserFields {
  fn fields(&self) -> Vec<&str> {
    self.fields()
  }

  fn values(&self) -> Vec<&(dyn ToSql + Sync)> {
    self.values()
  }
}
