mod database;
mod options;
mod tests;
mod users;
use crate::database::controller::Database;
use options::connection_data;
use users::model::UserFields;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let data = connection_data();

  match Database::connect(&data).await {
    Ok(mut db) => {
      let tbl_name = "users";
      let mut get_id: i32 = 0;
      let new_user = UserFields {
        name: String::from("John"),
        email: String::from("john@example.com"),
        age: 35,
      };
      match db.insert(&new_user, &data.schema, tbl_name).await {
        Ok(id) => {
          println!("User inserted with ID: {}", id);
          get_id = id;
        }
        Err(e) => eprintln!("Error inserting user: {}", e),
      };

      match db.read(get_id, &data.schema, tbl_name).await {
        Ok(data) => {
          println!("User added read: {:?}", data);
        }
        Err(e) => eprintln!("Error reading user: {}", e),
      };

      let updated_user = UserFields {
        name: String::from("William"),
        email: String::from("william@example.com"),
        age: 36,
      };

      match db
        .update(get_id, &updated_user, &data.schema, tbl_name)
        .await
      {
        Ok(_) => println!("User updated ID: {get_id}"),
        Err(e) => eprintln!("Error inserting user: {}", e),
      };

      match db.read(get_id, &data.schema, tbl_name).await {
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
