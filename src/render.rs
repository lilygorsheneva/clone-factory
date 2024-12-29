use crate::datatypes::{Coordinate, World, WorldCell};
use crossterm::{
    cursor, queue, execute,
    style::{self, Attribute, Color, StyledContent, Stylize},
    terminal::{size, enable_raw_mode, disable_raw_mode,EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout, Write};

impl WorldCell {
    fn get_drawable(&self) -> StyledContent<char> {
        if self.actor.is_some() {
            'A'.white().on_black()
        } else if self.building.is_some(){
            'B'.white().on_black()
        } else if !self.items.is_empty() {
            'i'.white().on_black()
        } else {
            ' '.on_black()
        }
    }
}

pub fn init_render() -> io::Result<()> {
    execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()
}

pub fn deinit_render() -> io::Result<()> {
    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()
}

pub fn render(world: World, center: Coordinate) -> io::Result<()> {
    let mut stdout = io::stdout();

    let size= size()?;
    let rows = size.1 as i16;
    let cols  = size.0 as i16; 
    let (centerx, centery) = (cols/2 + center.x, rows/2 +center.y);

    for i in 1..cols {
        for j in 1..rows {
            let x=  centerx - i;
            let y = centery - j;
        
            if let Some(cell) = world.get(Coordinate{x:x, y:y}) {
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
                    style::PrintStyledContent(" ".on_grey())
                )?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}
