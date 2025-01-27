use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    widgets::{self, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

use crate::{
    action::{Action, SubAction},
    datatypes::Coordinate,
    error::OkOrPopup,
    game_state::{game::Game, world::WorldCell},
    interface::widgets::{generate_main_layout, ItemBar, WorldWindowWidget},
    recording::interface::RecordingMenu,
};

use crate::direction::{
    AbsoluteDirection::{E, N, S, W},
    Direction::{Absolute, Relative},
    RelativeDirection::F,
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

        let (main, side, bottom, corner) = generate_main_layout(frame);

        cell.render_as_list(side, frame.buffer_mut());
        //frame.render_widget(get_guide, side);

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
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();

            match self.read() {
                None => {}
                Some(L) => {
                    self.coordinates.x -= 1;
                }
                Some(R) => {
                    self.coordinates.x += 1;
                }
                Some(U) => {
                    self.coordinates.y += 1;
                }
                Some(D) => {
                    self.coordinates.y -= 1;
                }
                Some(Exit) => break,
            }
        }
    }
}
