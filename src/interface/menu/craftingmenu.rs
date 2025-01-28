use std::{cell::RefCell, rc::Rc};

use crate::{
    action::{Action, SubAction},
    direction::{Direction, RelativeDirection},
    error::OkOrPopup,
    game_state::game::Game,
    interface::widgets::generate_popup_layout,
    inventory::BasicInventory,
    static_data::{Data, RecipeDefiniton},
};

use super::{gamemenu::GameMenu, MenuTrait, UILayer};

pub struct CraftingMenu<'a> {
    parent: &'a GameMenu,
    inventory: BasicInventory,
    recipes: Vec<CraftingMenuEntry>,
    game: Rc<RefCell<Game>>,
}

pub struct CraftingMenuEntry {
    definition: &'static RecipeDefiniton,
    widget: Paragraph<'static>,
}

impl CraftingMenuEntry {
    pub fn new(definition: &'static RecipeDefiniton, data: &Data) -> CraftingMenuEntry {
        let mut stringpieces = Vec::new();
        for i in 0..definition.ingredients.len() {
            let ingredient = data.items.get(&definition.ingredients[i]).expect("Non-existent item in crafting recipe.");
            stringpieces.push(format!("{} x {}", ingredient.text.name, definition.ingredient_counts[i]));
        }
        let product: &crate::static_data::ObjectDescriptor = data.items.get(&definition.product).expect("Non-existent item in crafting recipe.");
        stringpieces.push(format!("\n{}", product.text.description));

        let widget = Paragraph::new(stringpieces.join(" ")).block(Block::bordered().title(product.text.name.clone()));

        CraftingMenuEntry {
            definition,
            widget,
        }
    }
}

impl<'a> CraftingMenu<'a> {
    pub fn new(parent: &'a GameMenu, game: Rc<RefCell<Game>>) -> CraftingMenu<'a> {
        let gameref = game.borrow();
        let inventory = gameref.get_player_actor().unwrap().inventory;
        let recipes = Self::get_all_recipes(gameref.data);
        CraftingMenu {
            parent: parent,
            inventory,
            recipes,
            game: game.clone(),
        }
    }

    fn get_all_recipes(data: &'static Data) -> Vec<CraftingMenuEntry> {
        data.recipes
            .iter()
            .map(|(_, def)| CraftingMenuEntry::new (def, &data))
            .collect()
    }
}

pub enum CraftingMenuOptions {
    Craft(usize),
    Exit,
}
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Layout}, style::{Color, Stylize}, widgets::{Block, List, ListItem, Paragraph}
};
use CraftingMenuOptions::*;

impl UILayer for CraftingMenu<'_> {
    fn draw(&self, frame: &mut ratatui::Frame) {
        self.parent.draw(frame);

        let area = generate_popup_layout(frame);

        let slots = Layout::vertical(vec![Constraint::Min(1); self.recipes.len()]).split(area);
        for i in 0..self.recipes.len() {
            frame.render_widget(&self.recipes[i].widget, slots[i]);
        }
    }
}

impl MenuTrait for CraftingMenu<'_> {
    type MenuOptions = CraftingMenuOptions;

    fn enter_menu(&mut self, terminal: &mut ratatui::DefaultTerminal) {
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();

            match self.read() {
                None => {}
                Some(Exit) => break,
                Some(Craft(idx)) => {
                    if let Some(entry) = self.recipes.get(idx) {
                        let res = self.game.borrow_mut().player_action_and_turn(Action {
                            direction: Direction::Relative(RelativeDirection::F),
                            action: SubAction::Craft(entry.definition),
                        });
                        res.ok_or_popup(self, terminal);
                    }
                }
            }
        }
    }

    fn parsekey(&self, key: crossterm::event::KeyEvent) -> Option<Self::MenuOptions> {
        match key.code {
            KeyCode::Char('1') => Some(Craft(0)),
            KeyCode::Char('2') => Some(Craft(1)),
            KeyCode::Char('3') => Some(Craft(2)),
            KeyCode::Char('4') => Some(Craft(3)),
            KeyCode::Char('5') => Some(Craft(4)),
            KeyCode::Char('6') => Some(Craft(5)),
            KeyCode::Char('7') => Some(Craft(6)),
            KeyCode::Char('8') => Some(Craft(7)),
            KeyCode::Char('9') => Some(Craft(8)),
            KeyCode::Char('0') => Some(Craft(9)),
            KeyCode::Esc => Some(Exit),
            _ => None,
        }
    }
}
