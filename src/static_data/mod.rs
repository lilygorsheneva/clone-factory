//! Functions for loading external game data.

use crate::action::{get_use_fn_table, ItemUseFn};
use crate::buildings::{get_building_fn_table, BuildingUseFn};
use ratatui::style::Color;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use toml;

/// An object's definition.
/// All functional/internal apsects of the object should be contained here.
#[derive(Debug, Deserialize)]
pub struct ObjectDefinition {
    /// Internal name. Used for table lookups.
    pub name: String,

    /// The object's value when scored. This can probably be in another file.
    pub score_value: Option<i64>,


    /// The object's function when used as an item.
    pub on_use: Option<String>,
    #[serde(skip_deserializing)]
    pub on_use_fn: Option<ItemUseFn>,

    /// The object's function when used as a building.
    pub on_interact: Option<String>,
    #[serde(skip_deserializing)]
    pub on_interact_fn: Option<BuildingUseFn>,

    pub text: ObjectText,
    pub appearance: AppearanceDefiniton
}

/// Player-visible text related to an object. 
/// Swappable for localization
#[derive(Debug, Deserialize)]
pub struct ObjectText{
    /// Player-visible name.
    pub name: String,
    /// Long description.
    pub description: String
}



// The appearance of an item or entity. Ratatui-specific
#[derive(Debug, Deserialize)]
pub struct AppearanceDefiniton {
    pub glyph: String,
    pub glyph_n: Option<String>,
    pub glyph_s: Option<String>,
    pub glyph_e: Option<String>,
    pub glyph_w: Option<String>,

    pub color: Color,
}

// An item.
#[derive(Clone, Debug, Deserialize)]
pub struct ItemDefiniton {
    pub name: String,
    pub id: i64,
    pub glyph: String,
    pub color: Color,
 

    pub description: String,

    pub score_value: Option<i64>,

    pub on_use: Option<String>,
    #[serde(skip_deserializing)]
    pub on_use_fn: Option<ItemUseFn>,
}

// An item.
#[derive(Clone, Debug, Deserialize)]
pub struct BuildingDefinition {
    pub name: String,
    pub glyph: String,
    pub color: Color,

    pub description: String,
    pub on_interact: Option<String>,

    #[serde(skip_deserializing)]
    pub on_interact_fn: Option<BuildingUseFn>,
}

impl PartialEq for ItemDefiniton {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialEq for BuildingDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

// A crafting recipe.
#[derive(PartialEq, Eq, Clone, Debug, Deserialize)]
pub struct RecipeDefiniton {
    pub ingredients: Vec<String>,
    pub ingredient_counts: Vec<i64>,
    pub product: String,
    pub product_count: i64,
    pub name: String,
}

#[derive(Default, Deserialize)]

pub struct Data {
    pub actor_appearances: HashMap<String, AppearanceDefiniton>,
    pub buildings: HashMap<String, BuildingDefinition>,
    pub items: HashMap<String, ItemDefiniton>,
    pub recipes: HashMap<String, RecipeDefiniton>,
}

impl Data {
    pub fn get_config() -> &'static Data {
        let mut data = Data::read();
        data.bind_functions();
        let boxed = Box::new(data);
        Box::leak(boxed)
    }

    // Currently same as get_config, can be changed to read a smaller file.
    #[cfg(test)]
    pub fn get_test_config() -> &'static Data {
        let mut data = Data::read();
        data.bind_functions();
        let boxed = Box::new(data);
        Box::leak(boxed)
    }

    fn read() -> Data {
        let path = "src/static_data/data.toml";
        let s = fs::read_to_string(path).unwrap();
        toml::from_str(&s).unwrap()
    }

    fn bind_functions(&mut self) {
        let functions = get_use_fn_table();
        for (_, itemdef) in self.items.iter_mut() {
            if let Some(function) =
                functions.get(itemdef.on_use.as_ref().unwrap_or(&"default".to_string()))
            {
                itemdef.on_use_fn = Some(*function);
            }
        }

        let building_functions = get_building_fn_table();
        for (_, buildingdef) in self.buildings.iter_mut() {
            if let Some(function) = building_functions.get(
                buildingdef
                    .on_interact
                    .as_ref()
                    .unwrap_or(&"default".to_string()),
            ) {
                buildingdef.on_interact_fn = Some(*function);
            }
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
        assert_eq!(
            data.items["basic_cloner"].on_use.as_ref().unwrap(),
            "action_use_cloner"
        );
    }
}
