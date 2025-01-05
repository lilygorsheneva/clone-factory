use crate::datatypes::{Coordinate, Item};
use crate::direction::AbsoluteDirection;
use crate::game::{self, Game};
use crate::world::{World, WorldCell};
use ratatui::buffer::Cell;
use ratatui::layout::{self, Constraint, Direction, Layout, Rect};
use ratatui::prelude::Buffer;
use ratatui::style::{Color, Style};
use ratatui::widgets::{self, Widget};
use ratatui::{self, DefaultTerminal, Frame};

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

fn render_items(items: &[Option<Item>; 5], area: Rect, frame: &mut Frame) {
    let row = widgets::Row::new(items.map(|i| {
        if let Some(item) = i {
            // Get item name from some table instead.
            item.name.to_string()
        } else {
            "Empty".to_string()
        }
    }));
    // Construct blocks instead, with colors based on contents (and bordered)
    frame.render_widget(
        widgets::Table::new([row], [Constraint::Ratio(1, 5); 5])
            .style(Style::default().fg(Color::Black).bg(Color::Blue)),
        area,
    );
}

pub fn draw(game: &Game, frame: &mut Frame) {
    let window = WorldWindow {
        world: &game.world,
        center: game.get_player_coords().unwrap(),
    };
    let [main, bottom] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1)])
        .areas(frame.area());
    frame.render_widget(window, main);
    let actor = game.get_player_actor().unwrap();
    render_items(&actor.inventory, bottom, frame);
}
// pub fn actionprompt
// pub fn crafting_menu
// pub fn exit_prompt
