use crate::{
    action::{Action, SubAction},
    direction::{Direction, RelativeDirection},
    game_state::game::{self, Game},
    inventory::{self, BasicInventory},
    static_data::{RecipeDefiniton, StaticData},
};

use super::MenuTrait;

pub struct CraftingMenu<'a> {
    inventory: BasicInventory,
    recipes: Vec<CraftingMenuEntry>,
    game: &'a mut Game,
}

pub struct CraftingMenuEntry {
    definition: &'static RecipeDefiniton,
}

impl CraftingMenu<'_> {
    pub fn new(game: &mut Game) -> CraftingMenu {
        CraftingMenu {
            inventory: game.get_player_actor().unwrap().inventory,
            recipes: Self::get_all_recipes(game.data),
            game,
        }
    }

    fn get_all_recipes(data: &'static StaticData) -> Vec<CraftingMenuEntry> {
        data.recipes
            .iter()
            .map(|(_, def)| CraftingMenuEntry { definition: def })
            .collect()
    }
}

pub enum CraftingMenuOptions {
    Craft(usize),
    Exit,
}
use crossterm::event::KeyCode;
use ratatui::widgets::{List, ListItem};
use CraftingMenuOptions::*;

impl MenuTrait for CraftingMenu<'_> {
    type MenuOptions = CraftingMenuOptions;

    fn draw(&self, frame: &mut ratatui::Frame) {
        let items = self
            .recipes
            .iter()
            .map(|i| ListItem::new(i.definition.name.clone()));
        let list = List::new(items);
        frame.render_widget(list, frame.area());
    }

    fn call(&mut self, terminal: &mut ratatui::DefaultTerminal) {
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();

            match self.read() {
                None => {}
                Some(Exit) => break,
                Some(Craft(idx)) => {
                    if let Some(entry) = self.recipes.get(idx) {
                        self.game
                            .player_action(Action {
                                direction: Direction::Relative(RelativeDirection::F),
                                action: SubAction::Craft(entry.definition),
                            })
                            .unwrap();
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