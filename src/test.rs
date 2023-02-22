use mongodb::{bson::{doc, oid::ObjectId}, options::ClientOptions, Client};
use std::io::stdin;
use futures_util::stream::TryStreamExt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct User {
    #[serde(rename = "_id")]
    id: Option<ObjectId>,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    let database = client.database("test");
    let collection = database.collection::<User>("users");

    println!("Enter command: create, read, update or delete");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let command = input.trim();

    match command {
        "create" => {
            println!("Enter name:");
            let mut name = String::new();
            stdin().read_line(&mut name).unwrap();
            let name = name.trim();

            println!("Enter email:");
            let mut email = String::new();
            stdin().read_line(&mut email).unwrap();
            let email = email.trim();

            let user = User {
                id: None,
                name: name.to_string(),
                email: email.to_string(),
            };

            let result = collection.insert_one(user, None).await?;
            println!("Inserted id {}", result.inserted_id);
        },
        "read" => {
            let filter = doc! {};
            let mut cursor = collection.find(filter, None).await?;

            while let Some(result) = cursor.try_next().await? {
                println!("{:?}", result);
            }
        },
        "update" => {
            println!("Enter id:");
            let mut id = String::new();
            stdin().read_line(&mut id).unwrap();
            let id = id.trim();

            let filter = doc! {"_id": ObjectId::with_string(id)?};

            println!("Enter name:");
            let mut name = String::new();
            stdin().read_line(&mut name).unwrap();
            let name = name.trim();

            println!("Enter email:");
            let mut email = String::new();
            stdin().read_line(&mut email).unwrap();
            let email = email.trim();

            let update = doc! {"$set": {"name": name, "email": email}};

            let result = collection.update_one(filter, update, None).await?;
            println!("Updated {} document(s)", result.modified_count);
        },
        "delete" => {
            println!("Enter id:");
            let mut id = String::new();
            stdin().read_line(&mut id).unwrap();
            let id = id.trim();

            let filter = doc! {"_id": ObjectId::with_string(id)?};

            let result = collection.delete_one(filter, None).await?;
            println!("Deleted {} document(s)", result.deleted_count);
        },
        _ => println!("Invalid command"),
    };

    Ok(())
}
