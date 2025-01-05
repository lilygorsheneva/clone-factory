use std::{io, fs};
use std::collections::HashMap;
use serde_derive::Deserialize;
use toml;

// Functions responsible for loading essential data from config files.

// The appearance of an item. Ratatui-specific
#[derive(Deserialize)]
pub struct AppearanceDefiniton {
    pub name: String,
    pub glyph: String,
    pub color: String,
}

// An item.
#[derive(Deserialize)]
pub struct ItemDefiniton {
    pub name: String,
    pub description: String,
    pub on_use: Option<String>,
}

// A crafting recipe.
#[derive(Deserialize)]
pub struct RecipeDefiniton {
    pub ingredients: Vec<String>,
    pub product: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Data {
    pub appearances: HashMap<String, AppearanceDefiniton>,
    pub items: HashMap<String, ItemDefiniton>,
    pub recipes: HashMap<String, RecipeDefiniton>,
}

fn read() -> Data {
    let path = "src/data.toml";
    let s = fs::read_to_string(path).unwrap();
    toml::from_str(&s).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let data = read();
        assert_eq!(data.items["raw_crystal"].name, "Raw Crystal")
    }
}
