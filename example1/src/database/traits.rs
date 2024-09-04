use tokio_postgres::types::ToSql;

pub trait DatabaseFields {
  fn fields(&self) -> Vec<&str>;
  fn values(&self) -> Vec<&(dyn ToSql + Sync)>;
}
