use crate::datatypes::Coordinate;
use crate::direction::AbsoluteDirection;
use crate::game::{self, Game};
use crate::world::{World, WorldCell};
use ratatui::buffer::Cell;
use ratatui::layout::Rect;
use ratatui::prelude::Buffer;
use ratatui::style::{Color, Style};
use ratatui::widgets::Widget;
use ratatui::{self, DefaultTerminal, Frame};
use std::io::{self, Write};

impl WorldCell {
    fn draw(&self, cell: &mut Cell) {
        let generic_style = Style::default().fg(Color::White).bg(Color::DarkGray);

        match self {
            WorldCell {
                actor: Some(actor), ..
            } => {
                let glyph = match actor.facing {
                    AbsoluteDirection::N => "A",
                    AbsoluteDirection::S => "V",
                    AbsoluteDirection::E => ">",
                    AbsoluteDirection::W => "<",
                };
                let style = match actor.isplayer {
                    true => Style::default().fg(Color::Red).bg(Color::Black),
                    false => generic_style,
                };
                cell.set_symbol(glyph).set_style(style);
            }
            WorldCell {
                building: Some(_), ..
            } => {
                cell.set_symbol("B").set_style(generic_style);
            }
            WorldCell { items, .. } if items[0].is_some() => {
                cell.set_symbol("i").set_style(generic_style);
            }
            _ => {
                cell.set_symbol(" ").set_style(generic_style);
            }
        }
    }
}

pub fn init_render() -> DefaultTerminal {
    ratatui::init()
}

pub fn deinit_render() {
    ratatui::restore();
}

pub struct WorldWindow<'a> {
    pub world: &'a World,
    pub center: Coordinate,
}

impl<'a> Widget for WorldWindow<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (cols, rows) = (area.width, area.height);
        let (centerx, centery) = (
            cols as i16 / 2 + self.center.x,
            rows as i16 / 2 + self.center.y,
        );

        for i in 0..cols {
            for j in 0..rows {
                let x = centerx - i as i16;
                let y = centery - j as i16;
                if let Some(worldcell) = self.world.get(&Coordinate { x: x, y: y }) {
                    worldcell.draw(&mut buf[(i, j)]);
                } else {
                    buf[(i, j)].set_symbol(" ");
                }
            }
        }
    }
}

pub fn draw(game: &Game, frame: &mut Frame) {
    let window = WorldWindow {
        world: &game.world,
        center: game.get_player_coords().unwrap(),
    };
    frame.render_widget(window, frame.area());
}
// pub fn actionprompt
// pub fn show_inventory
// pub fn crafting_menu
// pub fn exit_prompt
