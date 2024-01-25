use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use std::io;
use std::collections::HashMap;
mod inventory;
use inventory::InventoryItem;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/items", get(add))
        .route("/items", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    let mut inventory = InventoryItem::new();

    let item_key: &mut usize = &mut 0;

    let inventory_items: &mut HashMap<usize, InventoryItem> = &mut HashMap::new();

    loop {
        println!("====================== MAIN MENU ======================");
        println!("Type add to add an entry to the collection");
        println!("Type view to view the entire collection");
        println!("Type delete to delete an entry in the collection");
        println!("Type edit to edit an entry in the collection");
        println!("Type exit to stop the program");
        println!("=======================================================");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");
        input = input.trim().to_string();

        if input == "add" {
            println!("Type what you would like to add");

            let mut item_input = String::new();
            io::stdin()
                .read_line(&mut item_input)
                .expect("Invalid input");
            item_input = item_input.trim().to_string();

            println!("Type the quantity of the item");

            let mut quantity_input = String::new();

            io::stdin()
                .read_line(&mut quantity_input)
                .expect("Invalid input");
            quantity_input = quantity_input.trim().to_string();

            let quantity_input: i32 = quantity_input.parse::<i32>().unwrap();

            println!("Type how you would like to tag this item");

            let mut tag_input = String::new();

            io::stdin()
                .read_line(&mut tag_input)
                .expect("Invalid input");
            tag_input = tag_input.trim().to_string();

            inventory.add(item_input, quantity_input, tag_input, item_key, inventory_items);
        }

        if input == "view" {
            inventory.print_contents(inventory_items);
        }

        else if input == "delete" {
            println!("Type the name of the item in the inventory that you would like to delete");

            let mut item_delete_choice = String::new();

            io::stdin()
                .read_line(&mut item_delete_choice)
                .expect("Invalid input");
            item_delete_choice = item_delete_choice.trim().to_string();

            inventory.delete(item_delete_choice, inventory_items);
        }

        else if input == "edit" {
            println!("Type the name of the entry you would like to edit");

            let mut edit_input = String::new();
            io::stdin()
                .read_line(&mut edit_input)
                .expect("Invalid input");
            edit_input = edit_input.trim().to_string();

            let mut key_match = 0;

            for (_key, item) in inventory_items.iter_mut() {
                if item.name == edit_input {
                    key_match = item.key;
                }
            }

            if key_match == 0 {
                println!("That name is not recognized");
            }

            else if key_match != 0 {
                println!("Type the updated name of the entry");

                let mut edit_input_name = String::new();
                io::stdin()
                    .read_line(&mut edit_input_name)
                    .expect("Invalid input");
                edit_input_name = edit_input_name.trim().to_string();

                println!("Type the updated quantity of the entry");

                let mut edit_input_quantity = String::new();
                io::stdin()
                    .read_line(&mut edit_input_quantity)
                    .expect("Invalid input");
                edit_input_quantity = edit_input_quantity.trim().to_string();
                
                let edit_input_quantity: i32 = edit_input_quantity.parse::<i32>().unwrap();

                println!("Type the updated tag of the entry");

                let mut edit_input_tag = String::new();
                io::stdin()
                    .read_line(&mut edit_input_tag)
                    .expect("Invalid input");
                edit_input_tag = edit_input_tag.trim().to_string();

                let edit_replacement = InventoryItem { name: edit_input_name, quantity: edit_input_quantity, tag: edit_input_tag, key: key_match };

                inventory.edit(edit_replacement, inventory_items);
            }
        }

        else if input == "exit" {
            break;
        }
    }
}