mod database;
mod options;
mod products;
mod tests;
mod users;
mod utils;
use crate::database::controller::Database;
use options::connection_data;
use products::model::ProductsFields;
use users::model::UserFields;
use utils::set_date;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let data = connection_data();

  match Database::connect(&data).await {
    Ok(mut db) => {
      let mut id: i32 = 0;

      let tbl_name_users = "users";
      let tbl_name_products = "products";

      let new_user = UserFields {
        name: String::from("John"),
        email: String::from("john@example.com"),
        age: 17,
        driver_license: false,
      };

      match db.insert(&new_user, &data.schema, tbl_name_users).await {
        Ok(id_recovered) => {
          println!("User inserted with ID: {}", id);
          id = id_recovered;
        }
        Err(e) => eprintln!("Error inserting user: {}", e),
      };

      match db.read(id, &data.schema, tbl_name_users).await {
        Ok(data) => {
          println!("User added read: {:?}", data);
        }
        Err(e) => eprintln!("Error reading user: {}", e),
      };

      let updated_user = UserFields {
        name: String::from("William"),
        email: String::from("william@example.com"),
        age: 36,
        driver_license: true,
      };

      match db
        .update(id, &updated_user, &data.schema, tbl_name_users)
        .await
      {
        Ok(_) => println!("User updated ID: {id}"),
        Err(e) => eprintln!("Error inserting user: {}", e),
      };

      match db.read(id, &data.schema, tbl_name_users).await {
        Ok(data) => {
          println!("Updated user read: {:?}", data);
        }
        Err(e) => eprintln!("Error reading user: {}", e),
      };

      let new_product = ProductsFields {
        name: String::from("iPhone 15"),
        price: 1500.45,
        supplier: String::from("Apple"),
        code: 1457,
        expiration_date: set_date(2024, 05, 21),
      };

      match db
        .insert(&new_product, &data.schema, tbl_name_products)
        .await
      {
        Ok(id_recovered) => {
          println!("Product inserted with ID: {}", id);
          id = id_recovered;
        }
        Err(e) => eprintln!("Error inserting user: {}", e),
      };

      match db.read(id, &data.schema, tbl_name_products).await {
        Ok(data) => {
          println!("Updated user read: {:?}", data);
        }
        Err(e) => eprintln!("Error reading user: {}", e),
      };
    }
    Err(e) => eprintln!("Error connection database: {}", e),
  };

  Ok(())
}
