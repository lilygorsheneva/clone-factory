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

fn render(world: World, location: Coordinate) -> io::Result<()> {
    let mut stdout = io::stdout();

    let zero = Coordinate::zero();
    let (rows, cols) = terminal::size()?;

    for x in 0..cols {
        for y in 0..rows {
            let location = Coordinate {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            };
            if location.in_rect(&zero, &world.dimensions) {
                // Render that Cell
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent(world.data[x as usize][y as usize].get_drawable())
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
