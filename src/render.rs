use crate::datatypes::{Coordinate, World, WorldCell};
use crossterm::{
    cursor, queue,
    style::{self, Attribute, Color, StyledContent, Stylize},
    terminal,
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

fn render(world: World, center: Coordinate) -> io::Result<()> {
    let mut stdout = io::stdout();

    let size= terminal::size()?;
    let rows = size.1 as i16;
    let cols  = size.0 as i16; 
    let (centerx, centery) = (cols/2 + center.x, rows/2 +center.y);

    for i in 1..cols {
        for j in 1..rows {
            let x=  i - centerx;
            let y = j - centery;
        
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
