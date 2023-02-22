use mongodb::{Client, options::{ClientOptions, FindOptions, UpdateOptions}};
use bson::{doc, oid::ObjectId};
use serde::{Serialize, Deserialize};

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

    // Insert a document
    let user = User {
        id: ObjectId::new(),
        name: "Sam Parmar".to_string(),
        email: "sameer.parmar@minddeft.net".to_string(),
        age: 22,
    };

    collection.insert_one(user, None).await.unwrap();

    // Find documents
    let filter = doc! { "name": "John Doe" };
    let find_options = FindOptions::builder().build();
    let cursor = collection.find(filter, find_options).await.unwrap();
    let mut users = vec![];

    for result in cursor {
        match result {
            Ok(document) => {
                let user: User = bson::from_bson(bson::Bson::Document(document)).unwrap();
                users.push(user);
            },
            Err(error) => println!("Error: {:?}", error),
        }
    }

    println!("{:?}", users);

    // Update a document
    let filter = doc! { "name": "John Doe" };
    let update = doc! { "$set": { "age": 31 } };
    let update_options = UpdateOptions::builder().build();

    collection.update_one(filter, update, update_options).await.unwrap();

    // Find updated documents
    let filter = doc! { "name": "John Doe" };
    let find_options = FindOptions::builder().build();
    let cursor = collection.find(filter, find_options).await.unwrap();
    let mut users = vec![];

    for result in cursor {
        match result {
            Ok(document) => {
                let user: User = bson::from_bson(bson::Bson::Document(document)).unwrap();
                users.push(user);
            },
            Err(error) => println!("Error: {:?}", error),
        }
    }

    println!("{:?}", users);
    
    // Delete a document
    let filter = doc! { "name": "John Doe" };
    collection.delete_one(filter, None).await.unwrap();
}
