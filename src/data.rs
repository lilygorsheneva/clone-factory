use crate::action::{self};
use ratatui::style::Color;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::{fs, io};
use toml;

// Functions responsible for loading essential data from config files.

// The appearance of an item. Ratatui-specific
#[derive(Deserialize)]
pub struct AppearanceDefiniton {
    pub glyph: String,
    pub glyph_n: Option<String>,
    pub glyph_s: Option<String>,
    pub glyph_e: Option<String>,
    pub glyph_w: Option<String>,
    pub color: String,
}

// An item.
#[derive(Deserialize)]
pub struct ItemDefiniton {
    pub name: String,
    pub glyph: String,
    pub color: String,
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

#[derive(Default, Deserialize)]
pub struct Data {
    pub actor_appearances: HashMap<String, AppearanceDefiniton>,
    pub building_appearances: HashMap<String, AppearanceDefiniton>,

    pub items: HashMap<String, ItemDefiniton>,
    pub recipes: HashMap<String, RecipeDefiniton>,

    #[serde(skip_deserializing)]
    pub functions: HashMap<String, Box<action::ItemUseFn>>,

    #[serde(skip_deserializing)]
    pub color_map: HashMap<String, Color>
}

pub fn get_config() -> Data {
    let mut data = Data::read();
    data.bind_functions();
    data.bind_colors();
    data
}

// Currently same as get_config
pub fn get_test_config() -> Data {
    let mut data = Data::read();
    data.bind_functions();
    data.bind_colors();
    data
}

impl Data {
    fn read() -> Data {
        let path = "src/data.toml";
        let s = fs::read_to_string(path).unwrap();
        toml::from_str(&s).unwrap()
    }

    fn bind_functions(&mut self) {
        self.functions = crate::action::get_use_fn_table();
    }

    fn bind_colors(&mut self) {
        self.color_map = crate::render::get_color_map();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let data = Data::read();
        assert_eq!(data.items["raw_crystal"].name, "Raw Crystal");
        assert_eq!(data.items["basic_cloner"].on_use.as_ref().unwrap(), "action_use_cloner");
    }
}
