// src/main.rs

mod grocery_item;
mod inventory;
mod db;

use crate::db::Db;
use crate::inventory::{print_all_frequencies, print_histogram, retrieve_inventory, search_item, Inventory};
use std::io::{self, Write};

// Print the menu
fn print_menu() {
    println!("1. Search for an item");
    println!("2. Display all frequencies");
    println!("3. Print histogram");
    println!("4. Exit");
}

// Load the inventory from the file
//Print the menu
// Read the user's choice
// Execute the choice
// Repeat until the user chooses to exit
// Exit the program
fn main() {
    // Try to load the inventory from the file.
    let inventory: Inventory = match retrieve_inventory() {
        Ok(inv) => inv,
        Err(e) => {
            eprintln!("Error loading inventory: {}", e);
            return; // exit main()
        }
    };

    let db = match Db::new("inventory.db") {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error opening database: {}", e);
            return;
        }
    };

    if let Err(e) = db.sync_inventory(&inventory) {
        eprintln!("Error syncing inventory to database: {}", e);
        return;
    }

    loop {
        print_menu();
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input.");
            continue;
        }
        
        let input = input.trim();
        let choice: i32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                print!("Enter item to search: ");
                io::stdout().flush().unwrap();

                let mut search_name = String::new();
                if io::stdin().read_line(&mut search_name).is_err() {
                    println!("Failed to read item name.");
                    continue;
                }

                let search_name = search_name.trim();
                search_item(&inventory, search_name);
            }
            2 => {
                print_all_frequencies(&inventory);
            }
            3 => {
                print_histogram(&inventory);
            }
            4 => {
                println!("Good Bye!");
                break;
            }
            _ => {
                println!("Invalid input!");
            }
        }
    }
}

