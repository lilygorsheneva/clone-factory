use std::{cell::RefCell, rc::Rc};

use crate::{
    action::{Action, SubAction},
    direction::{Direction, RelativeDirection},
    game_state::game::Game,
    inventory::BasicInventory,
    static_data::{Data, RecipeDefiniton},
};

pub struct CraftingMenu {
    inventory: BasicInventory,
    recipes: Vec<CraftingMenuEntry>,
    game: Rc<RefCell<Game>>,
}

pub struct CraftingMenuEntry {
    definition: &'static RecipeDefiniton,
    text: String,
}

impl CraftingMenuEntry {
    pub fn new(definition: &'static RecipeDefiniton, data: &Data) -> CraftingMenuEntry {
        let mut stringpieces = Vec::new();
        let product: &crate::static_data::ObjectDescriptor = data
        .items
        .get(&definition.product)
        .expect("Non-existent item in crafting recipe.");
        stringpieces.push(format!("{}: {}\n", product.text.name, product.text.description));
        
        for i in 0..definition.ingredients.len() {
            let ingredient = data
                .items
                .get(&definition.ingredients[i])
                .expect("Non-existent item in crafting recipe.");
            stringpieces.push(format!(
                "{} x {}",
                ingredient.text.name, definition.ingredient_counts[i]
            ));
        }
  
        CraftingMenuEntry {
            definition,
            text: stringpieces.join(" "),
        }
    }
}

impl CraftingMenu {
    pub fn new(game: Rc<RefCell<Game>>) -> CraftingMenu {
        let gameref = game.borrow();
        let inventory = gameref.get_player_actor().unwrap().inventory;
        let recipes = Self::get_all_recipes(gameref.data);
        CraftingMenu {
            inventory,
            recipes,
            game: game.clone(),
        }
    }

    fn get_all_recipes(data: &'static Data) -> Vec<CraftingMenuEntry> {
        data.recipes
            .iter()
            .map(|(_, def)| CraftingMenuEntry::new(def, &data))
            .collect()
    }

    pub fn show(&self, ctx: &egui::Context) {
        let window = egui::Window::new("Crafting").show(ctx, |ui| {
            for entry in &self.recipes {
                if ui.button(&entry.text).clicked() {
                    let res = self.game.borrow_mut().player_action_and_turn(Action {
                        direction: Direction::Relative(RelativeDirection::F),
                        action: SubAction::Craft(entry.definition),
                    });
                }
            }
        });
    }
}
