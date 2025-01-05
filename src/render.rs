use std::collections::HashMap;

use crate::data::Data;
use crate::datatypes::{Coordinate, Item};
use crate::direction::AbsoluteDirection;
use crate::game::{self, Game};
use crate::world::{World, WorldCell};
use ratatui::buffer::Cell;
use ratatui::layout::{self, Constraint, Direction, Layout, Rect};
use ratatui::prelude::Buffer;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{self, Block, Borders, List, ListItem, Paragraph, Widget};
use ratatui::{self, DefaultTerminal, Frame};

impl WorldCell {
    fn draw(&self, data: &Data, cell: &mut Cell) {
        let generic_style = Style::default().fg(Color::White).bg(Color::Black);

        match self {
            WorldCell {
                actor: Some(actor), ..
            } => {
                let actor_name = match actor.isplayer {
                    true => "player",
                    false => "clone",
                };
                let actor_def = data.actor_appearances.get(actor_name).unwrap();
                let glyph = match actor.facing {
                    AbsoluteDirection::N => actor_def.glyph_n.as_ref().unwrap_or(&actor_def.glyph),
                    AbsoluteDirection::S => actor_def.glyph_s.as_ref().unwrap_or(&actor_def.glyph),
                    AbsoluteDirection::E => actor_def.glyph_e.as_ref().unwrap_or(&actor_def.glyph),
                    AbsoluteDirection::W => actor_def.glyph_w.as_ref().unwrap_or(&actor_def.glyph),
                };
                cell.set_symbol(glyph)
                    .set_fg(actor_def.color_object)
                    .set_bg(Color::Black);
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

pub fn get_color_map() -> HashMap<String, Color> {
    HashMap::from([
        ("white".to_string(), Color::White),
        ("red".to_string(), Color::Red),
    ])
}

pub fn deinit_render() {
    ratatui::restore();
}

pub struct WorldWindow<'a> {
    pub world: &'a World,
    pub center: Coordinate,
    pub data: &'a Data,
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
                    worldcell.draw(self.data, &mut buf[(i, j)]);
                } else {
                    buf[(i, j)].set_symbol(" ").set_bg(Color::DarkGray);
                }
            }
        }
    }
}

fn render_items(items: &[Option<Item>; 5], data: &Data, area: Rect, frame: &mut Frame) {
    let slots: [Rect; 5] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 5); 5])
        .areas(area);

    for i in 0..items.len() {
        if let Some(item) = items[i] {
            let itemdef = data.items.get(item.name).unwrap();
            frame.render_widget(
                Paragraph::new(itemdef.name.clone()).block(
                    Block::default()
                        .title(Line::from((i+1).to_string()).left_aligned())
                        .title(Line::from(itemdef.glyph.clone()).centered())
                        .title(Line::from(item.quantity.to_string()).right_aligned())
                        .borders(Borders::ALL),
                ),
                slots[i],
            )
        } else {
            frame.render_widget(Paragraph::new("").block(Block::default()), slots[i])
        }
    }
}

fn render_recipes(data: &Data, area: Rect, frame: &mut Frame) {
    // TODO filter by unlocks
    // TODO cache
    // TODO show requirements
    let items: Vec<ListItem> = data.recipes.iter().map(|(_, def)| {ListItem::new(def.name.clone())}).collect();
    let list = List::new(items);
    frame.render_widget(list, area);
}


pub fn draw(game: &Game, frame: &mut Frame) {
    let window = WorldWindow {
        world: &game.world,
        center: game
            .get_player_coords()
            .unwrap_or(Coordinate { x: 0, y: 0 }),
        data: &game.data,
    };

    let [main, side] =  Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Fill(1), Constraint::Length(20)])
    .areas(frame.area());

    let [main, bottom] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(3)])
        .areas(main);
    frame.render_widget(window, main);
    if let Ok(actor) = game.get_player_actor() {
        render_items(&actor.inventory, &game.data, bottom, frame)
    }
    render_recipes(&game.data, side, frame);
}
// pub fn actionprompt
// pub fn crafting_menu
// pub fn exit_prompt
