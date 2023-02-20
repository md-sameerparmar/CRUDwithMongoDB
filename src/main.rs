use mongodb::{Client, options::{ClientOptions, FindOptions, UpdateOptions}};
use bson::{doc, oid::ObjectId};
use serde::{Serialize, Deserialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    email: String,
    age: i32,
}


#[tokio::main]
async fn main() {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("mydatabase");
    let collection = db.collection("users");

    // let val = "test";
    // Insert a document
    // let mut users: Vec<User> = Vec::new();
    
    println!("\n- Enter first name only:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    if name.is_empty() {
        println!("- First, Enter your name:");
    }


    println!("\n- Enter Email:");
    let mut email = String::new();
    io::stdin().read_line(&mut email).unwrap();
    let email = email.trim().to_string();

    println!("\n- Enter Age:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).unwrap();
    let age = age.trim().to_string();

    let user = User {
        id: ObjectId::new(),
        name: name.to_string(),
        email: email.to_string(),
        age: age.parse().unwrap(),
    };

    collection.insert_one(user, None).await.unwrap();

    // Find documents
    // let filter = doc! { "name": "John Doe" };
    // let find_options = FindOptions::builder().build();
    // let cursor = collection.find(filter, find_options).await.unwrap();
    // let mut users = vec![];

    // for result in cursor {
    //     match result {
    //         Ok(document) => {
    //             let user: User = bson::from_bson(bson::Bson::Document(document)).unwrap();
    //             users.push(user);
    //         },
    //         Err(error) => println!("Error: {:?}", error),
    //     }
    // }

    // println!("{:?}", users);
}
