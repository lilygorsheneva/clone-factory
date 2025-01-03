use crate::datatypes::Coordinate;
use crate::direction::AbsoluteDirection;
use crate::world::{World, WorldCell};
use crossterm::{
    cursor, execute, queue,
    style::{self, StyledContent, Stylize},
    terminal::{self,
        disable_raw_mode, enable_raw_mode, BeginSynchronizedUpdate, EndSynchronizedUpdate,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io::{self, Write};

impl WorldCell {
    fn get_drawable(&self) -> StyledContent<char> {
        match self {
            WorldCell {
                actor: Some(actor), ..
            } => {
                let glyph = match actor.facing {
                    AbsoluteDirection::N => 'A',
                    AbsoluteDirection::S => 'V',
                    AbsoluteDirection::E => '>',
                    AbsoluteDirection::W => '<',
                };
                let styled = match actor.isplayer {
                    true => glyph.red().on_black(),
                    false => glyph.white().on_black(),
                };
                styled
            },
            WorldCell {
                building: Some(_), ..
            } => 'B'.white().on_black(),
            WorldCell {items, .. } if items[0].is_some()=> {
                'i'.white().on_black()
            },
            _ => ' '.on_black()
        }
    }
}

pub fn init_render() -> io::Result<()>{
    execute!(
        io::stdout(),
        cursor::SetCursorStyle::SteadyUnderScore,
        cursor::Hide,
        EnterAlternateScreen
    )?;
    enable_raw_mode()?;
    Ok(())
}

pub fn deinit_render()-> io::Result<()> {
    execute!(io::stdout(), cursor::Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn renderworld(world: &World, center: &Coordinate)-> io::Result<()>  {
    let mut stdout = io::stdout();

    let size = terminal::size()?;
    let (cols, rows) = (size.0 as i16, size.1 as i16);
    let (centerx, centery) = (cols / 2 + center.x, rows / 2 + center.y);
    execute!(stdout, BeginSynchronizedUpdate)?;
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
                )?;
            } else {
                queue!(
                    stdout,
                    cursor::MoveTo(i as u16, j as u16),
                    style::PrintStyledContent(" ".on_dark_cyan())
                )?;
            }
        }
    }

    stdout.flush()?;
    execute!(stdout, EndSynchronizedUpdate)
}

// pub fn actionprompt
// pub fn show_inventory
// pub fn crafting_menu
// pub fn exit_prompt