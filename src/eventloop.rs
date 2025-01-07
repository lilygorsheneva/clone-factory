// TODO move the npc turns event loop here.

use ratatui::DefaultTerminal;
use crate::interface::input;
use crate::interface::render;
use crate::game_state::game::Game;


pub fn main_event_loop(game: &mut Game, terminal: &mut DefaultTerminal) {

    loop {
        match input::readinput() {
            Some(input::InputResult::Exit) => break,
            Some(input::InputResult::Redraw) => {
                terminal.draw(|frame| render::draw(&game, frame)).unwrap();
            }
            Some(input::InputResult::Act(act)) => {
                game.player_action(act).unwrap();
                game.do_npc_turns().unwrap();
                terminal.draw(|frame| render::draw(&game, frame)).unwrap();
            }
            Some(input::InputResult::Record) => match game.current_recording {
                Some(_) => game.end_record().unwrap(),
                None => game.init_record().unwrap(),
            },
            _ => {}
        };
    }

}

fn craft(game: &mut Game, terminal: &mut DefaultTerminal) {
    // let list compose_recipe_list
    //  render::request_crafting_recipe(list)
    // if let Some(Numeral(i)) = input::read_numeral
    // game.player_action(craft(list(i)))
}