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

    let zero = Coordinate::zero();
    let (rows, cols) = terminal::size()?;
    let (centerx, centery) = (cols/2, rows/2);

    for i in 0..cols {
        for j in 0..rows {
            let x=  i - centerx;
            let y = j - centery;
        
            if let Some(cell) = world.get(Coordinate{x:x.try_into().unwrap(), y:y.try_into().unwrap()}) {
                // Render that Cell
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent(cell.get_drawable())
                )?;
            } else {
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent(" ".on_grey())
                )?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}
