use crate::datatypes::Coordinate;
use crate::direction::AbsoluteDirection;
use crate::world::{World, WorldCell};
use crossterm::{
    cursor, execute, queue,
    style::{self, StyledContent, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, BeginSynchronizedUpdate, EndSynchronizedUpdate,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io::{self, Write};

impl WorldCell {
    fn get_drawable(&self) -> StyledContent<char> {
        if self.actor.is_some() {
            let actor = self.actor.as_ref().unwrap();
            let glyph =  match actor.facing {
                AbsoluteDirection::N => 'A',
                AbsoluteDirection::S => 'V',
                AbsoluteDirection::E => '>',
                AbsoluteDirection::W => '<',
            };
            let styled = match actor.isplayer {
                true => glyph.red().on_black(),
                false => glyph.white().on_black(),
            };
            return styled
        } else if self.building.is_some() {
            'B'.white().on_black()
        } else if !self.items[0].is_none() {
            'i'.white().on_black()
        } else {
            ' '.on_black()
        }
    }
}

pub fn init_render() {
    execute!(io::stdout(),  cursor::SetCursorStyle::SteadyUnderScore, cursor::Hide, EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
}

pub fn deinit_render() {
    execute!(io::stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

pub fn render(world: &World, center: &Coordinate) {
    let mut stdout = io::stdout();

    let size = size().unwrap();
    let (cols, rows) = (size.0 as i16, size.1 as i16);
    let (centerx, centery) = (cols / 2 + center.x, rows / 2 + center.y);
    execute!(stdout, BeginSynchronizedUpdate).unwrap();
    for i in 0..cols {
        for j in 0..rows {
            let x = centerx - i;
            let y = centery - j;

            if let Some(cell) = world.get(&Coordinate { x: x, y: y }) {
                // Render that Cell
                queue!(
                    stdout,
                    cursor::MoveTo(i as u16, j as u16),
                    style::PrintStyledContent(cell.get_drawable())
                )
                .unwrap();
            } else {
                queue!(
                    stdout,
                    cursor::MoveTo(i as u16, j as u16),
                    style::PrintStyledContent(" ".on_dark_cyan())
                )
                .unwrap();
            }
        }
    }

    stdout.flush().unwrap();
    execute!(stdout, EndSynchronizedUpdate).unwrap();
}
