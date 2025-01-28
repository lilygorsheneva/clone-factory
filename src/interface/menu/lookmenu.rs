use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    widgets::{List, ListItem},
    DefaultTerminal, Frame,
};

use crate::{
    datatypes::Coordinate,
    game_state::game::Game,
    interface::widgets::{generate_main_layout, ItemBar, WorldWindowWidget},
};


use super::{MenuTrait, UILayer};

pub struct LookMenu {
    game: Rc<RefCell<Game>>,
    coordinates: Coordinate,
}

pub enum LookMenuOptions {
    Exit,
    L,
    R,
    U,
    D,
}
use LookMenuOptions::*;

impl LookMenu {
    pub fn new(game: Rc<RefCell<Game>>) -> LookMenu {
        let coordinates = *game.borrow().get_player_coords().unwrap();
        LookMenu { game, coordinates }
    }
}

impl UILayer for LookMenu {
    fn draw(&self, frame: &mut Frame) {
        let game = self.game.borrow();
        let mut window = WorldWindowWidget::new(&game);
        window.center = self.coordinates;
        window.show_cursor = true;
        let item_widget = ItemBar::new(&game);
        let score = &game.score;

        let cell = game.world.get_cell(&self.coordinates).unwrap();

        let (main, sideup, sidedown, bottom, corner) = generate_main_layout(frame);

        cell.render_as_list(sidedown, frame.buffer_mut());
        frame.render_widget(get_guide(), sideup);

        frame.render_widget(item_widget, bottom);
        frame.render_widget(window, main);
        frame.render_widget(score, corner);
    }
}

fn get_guide() -> List<'static> {
    List::new([
        ListItem::new("arrow keys: move cursor"),
        ListItem::new("Esc: return"),
    ])
}

impl MenuTrait for LookMenu {
    type MenuOptions = LookMenuOptions;

    fn parsekey(&self, key: KeyEvent) -> Option<Self::MenuOptions> {
        match key.code {
            KeyCode::Left => Some(L),
            KeyCode::Right => Some(R),
            KeyCode::Up => Some(U),
            KeyCode::Down => Some(D),
            KeyCode::Esc => Some(Exit),
            _ => None,
        }
    }

    fn enter_menu(&mut self, terminal: &mut DefaultTerminal) {
        let dimensions = self.game.borrow().world.dimensions();
        let zero = Coordinate{x:0,y:0};
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();

            match self.read() {
                None => {}
                Some(L) => {
                    self.coordinates.x -= 1;
                    self.coordinates = self.coordinates.clamp(zero,dimensions);
                }
                Some(R) => {
                    self.coordinates.x += 1;
                    self.coordinates = self.coordinates.clamp(zero,dimensions);
                }
                Some(U) => {
                    self.coordinates.y += 1;
                    self.coordinates = self.coordinates.clamp(zero,dimensions);
                }
                Some(D) => {
                    self.coordinates.y -= 1;
                    self.coordinates = self.coordinates.clamp(zero,dimensions);
                }
                Some(Exit) => break,
            }
        }
    }
}
