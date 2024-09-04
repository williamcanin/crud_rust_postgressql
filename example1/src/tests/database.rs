use tokio;

#[tokio::test]
async fn connection() -> Result<(), Box<dyn std::error::Error>> {
  let data = crate::options::connection_data();
  match crate::database::controller::Database::connect(&data).await {
    Ok(client) => {
      // Caso tenha conexão, feche a mesma.
      println!("Connection successful!");
      client.to_close().await;
    }
    Err(e) => {
      panic!("Failed to create PostgresSQl client: {}", e);
    }
  };
  Ok(())
}
