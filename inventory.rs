use std::collections::HashMap;
extern crate serde;
extern crate serde_json;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryItem {
    pub name: String,
    pub quantity: i32,
    pub tag: String,
    pub key: usize,
}

impl InventoryItem {
    pub fn new() -> InventoryItem {
        InventoryItem {
            name: String::new(),
            quantity: 0,
            tag: String::new(),
            key: 0,
        }
    }

    fn increment_key(x: &mut usize) {
        *x += 1;
    }

    pub fn add(
        &mut self,
        item_input: String,
        quantity_input: i32,
        tag_input: String,
        item_key: &mut usize,
        inventory_items: &mut HashMap<usize, InventoryItem>,
    ) {
        Self::increment_key(item_key);

        let item_input = InventoryItem {
            name: item_input,
            quantity: quantity_input,
            tag: tag_input,
            key: *item_key,
        };

        inventory_items.insert(*item_key, item_input);
    }

    pub fn print_contents(&mut self, inventory_items: &mut HashMap<usize, InventoryItem>) {
        for (_key, item) in inventory_items.iter() {
            println!(
                "{}. Name: {}, Quantity: {}, Tag: {}",
                item.key, item.name, item.quantity, item.tag
            );
        }
    }

    pub fn delete(
        &mut self,
        item_delete_choice: String,
        inventory_items: &mut HashMap<usize, InventoryItem>,
    ) {
        let mut remove = vec![];

        for (key, item) in inventory_items.iter() {
            if item.name == item_delete_choice {
                remove.push(*key);
            }
        }

        for key in remove.iter().rev() {
            inventory_items.remove(key);
        }
    }

    pub fn edit(
        &mut self,
        edit_replacement: InventoryItem,
        inventory_items: &mut HashMap<usize, InventoryItem>,
    ) {
        for (key, item) in inventory_items.iter_mut() {
            if *key == edit_replacement.key {
                *item = edit_replacement.clone();
            }
        }
    }
}