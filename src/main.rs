use async_std::stream::StreamExt;
use bson::{doc, oid::ObjectId};
use mongodb::{
    options::{ClientOptions, FindOptions, UpdateOptions},
    Client,
};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    email: String,
    age: i32,
}

fn varify_name(name: &str) -> bool {
    let mut is_valid = true;

    if name.is_empty() {
        is_valid = false;
        println!(" !!This field cannot be empty!!");
    }
    if name.chars().next() == Some('_') {
        is_valid = false;
        println!("\n !!Name must not start with an underscore!!");
    }
    is_valid
}

#[tokio::main]
async fn main() {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("mydatabase");
    let collection = db.collection("users");

    loop {
        println!("\nEnter your Choice: \n👉 Press 1 for Insert. \n👉 Press 2 for Read. \n👉 Press 3 for Update. \n👉 Press 4 for Delete. \n👉 Press 5 for Exit.");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().to_string();

        match choice.as_str() {
            "1" => {
                // Insert a document

                loop {
                    println!("\n- 🤨 Enter first name only:");
                    let mut input_name = String::new();
                    io::stdin().read_line(&mut input_name).unwrap();
                    let name = input_name.trim().to_string();

                    if varify_name(&name) {
                        // collection.insert_one(doc!{ "name" : name }, None).await.unwrap();
                    } else {
                        continue;
                    }

                    println!("\n- 📧 Enter email address:");
                    let mut email = String::new();
                    io::stdin().read_line(&mut email).unwrap();
                    let email = email.trim().to_string();

                    println!("\n- 🎂 Enter your age:");
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
                    println!("\nHello, {}!!👋", name);
                    break;
                }
            }
            "2" => {
                // Find documents
                println!("\n🔎 Enter name which you want to find:");
                let mut input_name = String::new();
                io::stdin().read_line(&mut input_name).unwrap();
                let name = input_name.trim().to_string();

                let filter = doc! { "name": name };
                let find_options = FindOptions::builder().build();
                let mut cursor = collection.find(filter, find_options).await.unwrap();
                // let mut users = vec![];

                cursor.next().await.into_iter().for_each(|result| {
                    match result {
                        Ok(document) => {
                            println!("\nName: {:?}\nEmail: {:?}\nAge: {:?}",document.name,document.email,document.age);
                        },
                        Err(e) => println!("Error: {:?}", e),
                    }
                });
            }
            "3" => {
                // Update a document
                println!("\n🛠🤦 Enter name which you want to update:");
                let mut input_name = String::new();
                io::stdin().read_line(&mut input_name).unwrap();
                let name = input_name.trim().to_string();

                let filter = doc! { "name": &name };

                println!("\n🆕 Enter new name:");
                let mut new_input_name = String::new();
                io::stdin().read_line(&mut new_input_name).unwrap();
                let new_name = new_input_name.trim().to_string();

                println!("\n🆕 Enter new email address:");
                let mut input_email = String::new();
                io::stdin().read_line(&mut input_email).unwrap();
                let email = input_email.trim().to_string();

                println!("\n🆕 Enter new age:");
                let mut age = String::new();
                io::stdin().read_line(&mut age).unwrap();
                let age:i32 = age.trim().parse().unwrap();

                let update = doc! { "$set": { "name": new_name, "email": email, "age": age} };
                let update_options = UpdateOptions::builder().build();

                collection.update_one(filter, update, update_options).await.unwrap();

                // Find updated documents
                let filter = doc! { "name": &name };
                let find_options = FindOptions::builder().build();
                let mut cursor = collection.find(filter, find_options).await.unwrap();
                

            cursor.next().await.into_iter().for_each(|result| {
                match result {
                    Ok(document) => {
                        println!(" {:?} \n ✌ Record updated successfully!!",document);
                        // println!("\nUpdate successfully!!")
                    },
                    Err(error) => println!("Error: {:?}", error),
                }
            });
                
            }
            "4" => {
                println!("\n⛔ Enter name which you want to delete:");
                let mut input_name = String::new();
                io::stdin().read_line(&mut input_name).unwrap();
                let name = input_name.trim().to_string();

                let filter = doc! { "name": name };
                collection.delete_one(filter, None).await.unwrap();
                println!("\n Record deleted!💁‍♀️");
            }
            "5" => {
                println!("\n Thank you!!😇\n Have a good day!!🤗\n");
                break;
            }

            _ => {
                println!("\n0Invalid input😵🥴");
            }
        };
    }
}
