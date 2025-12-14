// src/inventory.rs

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

/// Item name key maps to value quantity
pub type Inventory = BTreeMap<String, i32>;

/// Reads "inventory.txt", counts frequencies, writes "frequency.dat",
/// and returns the in-memory map.
pub fn retrieve_inventory() -> io::Result<Inventory> {
    // Open the input file explicitly by name
    let file = File::open("inventory.txt")?;
    let reader = BufReader::new(file);

    let mut inventory: Inventory = BTreeMap::new();

    // Each line is an item name. Names may appear multiple times in the file.
    for line_result in reader.lines() {
        let line = line_result?; // ? will propagate I/O error if any
        let name = line.trim().to_string();

        if name.is_empty() {
            continue;
        }

        // If the name isn't in the map, insert 0; then add 1
        let count = inventory.entry(name).or_insert(0);
        *count += 1;
    }

    // Write out the frequencies to frequency.dat.
    let mut out_file = File::create("frequency.dat")?;
    for (name, quantity) in &inventory { //Reference borrowed from inventory as read-only
        writeln!(out_file, "{} {}", name, quantity)?;
    }

    Ok(inventory)
}

/// Print item name + frequency for all items.
pub fn print_all_frequencies(inventory: &Inventory) { //Reference borrowed from inventory as read-only
    for (name, quantity) in inventory {
        println!("{name} {quantity}");
    }
}

/// Print histogram with asterisks.
pub fn print_histogram(inventory: &Inventory) { //Reference borrowed from inventory as read-only
    for (name, quantity) in inventory {
        let stars: String = std::iter::repeat('*')
            .take(*quantity as usize)
            .collect();
        println!("{name} {stars}");
    }
}

/// Search for one item by name.
pub fn search_item(inventory: &Inventory, search: &str) { //Reference borrowed from inventory as read-only
    if let Some(qty) = inventory.get(search) {
        println!("{search} {qty}");
    } else {
        println!("{search} not found.");
    }
}
