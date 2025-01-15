//! Functions for loading external game data.

use crate::action::{ItemUseFn, get_use_fn_table};
use crate::interface::render::get_color_map;
use map::StaticDataMap;
use ratatui::style::Color;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use toml;

pub mod map;

// The appearance of an item or entity. Ratatui-specific
#[derive(Clone,Deserialize)]
pub struct AppearanceDefiniton {
    pub glyph: String,
    pub glyph_n: Option<String>,
    pub glyph_s: Option<String>,
    pub glyph_e: Option<String>,
    pub glyph_w: Option<String>,


    pub color: String,
    // TODO: write a custom Deserialize for Color
    #[serde(skip_deserializing)]
    pub color_object: Color
}

// An item.
#[derive(Clone, Debug, Deserialize)]
pub struct ItemDefiniton {
    

    pub name: String,
    pub id: i64,
    pub glyph: String,
    pub color: String,
    // TODO: write a custom Deserialize for Color
    #[serde(skip_deserializing)]
    pub color_object: Color,

    pub description: String,
    pub on_use: Option<String>,

    #[serde(skip_deserializing)]
    pub on_use_fn: Option<Box<ItemUseFn>>
}

impl PartialEq for ItemDefiniton {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

// A crafting recipe.
#[derive(Clone, Deserialize)]
pub struct RecipeDefiniton {
    pub ingredients: Vec<String>,
    pub ingredient_counts: Vec<i64>,
    pub product: String,
    pub product_count: i64,
    pub name: String,
}

#[derive(Default, Deserialize)]

pub struct Data {
    actor_appearances: HashMap<String, AppearanceDefiniton>,
    building_appearances: HashMap<String, AppearanceDefiniton>,
    items: HashMap<String, ItemDefiniton>,
    recipes: HashMap<String, RecipeDefiniton>,
}

pub struct StaticData {
    pub actor_appearances: StaticDataMap<String, AppearanceDefiniton>,
    pub building_appearances: StaticDataMap<String, AppearanceDefiniton>,
    pub items: StaticDataMap<String, ItemDefiniton>,
    pub recipes: StaticDataMap<String, RecipeDefiniton>,
}

impl StaticData {
    pub fn from_data(data: &Data ) -> StaticData {
            StaticData {
            actor_appearances: StaticDataMap::from_map(&data.actor_appearances),
            building_appearances: StaticDataMap::from_map(&data.building_appearances),
            items: StaticDataMap::from_map(&data.items),
            recipes: StaticDataMap::from_map(&data.recipes),
        }
    }

    pub fn get_config() -> StaticData {
        let mut data = Data::read();
        data.bind_functions();
        data.bind_colors();
        StaticData::from_data(&data)
    }
    
    // This should be a test fixture; otherwise it's a leak. 
    #[cfg(test)]
    pub fn get_test_config() -> StaticData {
        let mut data = Data::read();
        data.bind_functions();
        data.bind_colors();
        StaticData::from_data(&data)
    }
}


impl Data {
    pub fn get_config() -> Data {
        let mut data = Data::read();
        data.bind_functions();
        data.bind_colors();
        data
    }
    
    // Currently same as get_config, can be changed to read a smaller file.
    #[cfg(test)]
    pub fn get_test_config() -> Data {
        let mut data = Data::read();
        data.bind_functions();
        data.bind_colors();
        data
    }
    

    fn read() -> Data {
        let path = "src/static_data/data.toml";
        let s = fs::read_to_string(path).unwrap();
        toml::from_str(&s).unwrap()
    }

    fn bind_functions(&mut self) {
        let functions = get_use_fn_table();
        for (_, itemdef) in self.items.iter_mut() {
            if let Some(function ) =  functions.get(itemdef.on_use.as_ref().unwrap_or(&"default".to_string())) {
                itemdef.on_use_fn =  Some(Box::new(*function.clone()));
            }
        }
    }

    fn bind_colors(&mut self) {
        let color_map = get_color_map();
        for (_, itemdef) in self.items.iter_mut() {
            itemdef.color_object = *color_map.get(&itemdef.color).unwrap_or(&Color::White);
        }
        for (_, actordef) in self.actor_appearances.iter_mut() {
            actordef.color_object = *color_map.get(&actordef.color).unwrap_or(&Color::White);
        }
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
