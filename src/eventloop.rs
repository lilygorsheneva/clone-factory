// TODO move the npc turns event loop here.
use crate::datatypes::Coordinate;
use crate::engine::update::Updatable;
use crate::game_state::game::Game;
use crate::interface;
use crate::interface::input;
use crate::interface::render;
use crate::inventory::Item;
use crate::static_data::StaticData;
use crossterm::event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::{Event, KeyModifiers};
use ratatui::DefaultTerminal;

pub struct Application {
    pub game: Option<Game>,
    pub data: &'static StaticData,
    pub terminal: DefaultTerminal,
}

impl Application {
    pub fn new() -> Self {
        Application {
            game: None,
            terminal: interface::render::init_render(),
            data: StaticData::get_config(),
        }
    }

    pub fn start_game(&mut self) {
        let mut game = Game::new(Coordinate { x: 20, y: 10 }, &self.data);

        game.spawn(&Coordinate { x: 1, y: 1 }).unwrap();

        let item_def = self.data.items.get(&"raw_crystal".to_string()).unwrap();
        let foo = Item::new(item_def, 1);

        game.world
            .items
            .mut_set(&Coordinate { x: 10, y: 5 }, &[Some(foo)])
            .unwrap();

        self.game = Some(game);
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        interface::render::deinit_render();
    }
}

const ESC: KeyEvent = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
const ENTER: KeyEvent = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);

impl Application {
    pub fn main_menu(&mut self) {
        loop {

        let _ = &self
        .terminal
        .draw(|frame| render::draw_main_menu(frame));

            let event = event::read().expect("Error getting input");
            if event == Event::Key(ESC) {
                return;
            }
            if event == Event::Key(ENTER) {
                if self.game.is_none() {
                    self.start_game();
                }
                self.game_loop();
            }
        }
    }

    fn game_loop(&mut self) {
        let game = self
            .game
            .as_mut()
            .expect("Entered main game loop without an active game");
        let menu = input::normal_menu();

        loop {
            let _ = &self
            .terminal
            .draw(|frame| render::draw_game_window(&game, &menu, frame));


            match input::readinput(&menu) {
                input::InputResult::Exit => break,
                input::InputResult::Act(act) => {
                    game.player_action(act).unwrap();
                    game.do_npc_turns().unwrap();
                   ;
                }
                input::InputResult::Record => match game.current_recording {
                    Some(_) => game.end_record().unwrap(),
                    None => game.init_record().unwrap(),
                },
                _ => {}
            };
        }
    }
}
