// src/grocery_item.rs

// GroceryItem struct represents a grocery item with a name and quantity
// Technically, this is not necessary, but it's a good practice to encapsulate the data
// and may be used later for more complex functionality.
// It was good practice to map a c++ class to a rust struct

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct GroceryItem {
    name: String,
    quantity: i32,
}

impl GroceryItem {
    /// Constructor: start quantity at 0.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            quantity: 0,
        }
    }

    /// Constructor with explicit quantity.
    pub fn with_quantity(name: &str, quantity: i32) -> Self {
        Self {
            name: name.to_string(),
            quantity,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn quantity(&self) -> i32 {
        self.quantity
    }

    pub fn add_item(&mut self) {
        self.quantity += 1;
    }

    pub fn remove_item(&mut self) {
        if self.quantity > 0 {
            self.quantity -= 1;
        }
    }
}
