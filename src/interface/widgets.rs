//! Functions related to UI rendering.
//! Anything backend specific (ratatui) should be contained here.
use std::collections::HashMap;

use crate::datatypes::Coordinate;
use crate::direction::AbsoluteDirection;
use crate::game_state::game::Game;
use crate::game_state::world::{FloorTile, World, WorldCell};
use crate::inventory::{BasicInventory, Item};
use crate::score::Score;
use crate::static_data::{Data, ItemDefiniton};
use ratatui::buffer::Cell;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::prelude::Buffer;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};
use ratatui::{self, DefaultTerminal, Frame};

impl<'a> WorldCell<'a> {
    fn draw(&'a self, data: &Data, cell: &mut Cell) {
        let (r, g, b) = match self.floor {
            FloorTile::Water => (0, 0, 200),
            FloorTile::Stone => (69, 69, 69),
            FloorTile::Dirt => (69, 35, 10),
        };

        let pdx_overlay = self.paradox.0 as u8;
        let bgcolor = Color::Rgb(
            r + pdx_overlay,
            g + pdx_overlay,
            b + pdx_overlay,
        );

        let generic_style = Style::default().fg(Color::White).bg(bgcolor);

        match self {
            WorldCell {
                actor: Some(actor), ..
            } => {
                let actor_name = match actor.isplayer {
                    true => "player",
                    false => "clone",
                };
                let actor_def = data.actor_appearances.get(&actor_name.to_string()).unwrap();
                let glyph = match actor.facing {
                    AbsoluteDirection::N => actor_def.glyph_n.as_ref().unwrap_or(&actor_def.glyph),
                    AbsoluteDirection::S => actor_def.glyph_s.as_ref().unwrap_or(&actor_def.glyph),
                    AbsoluteDirection::E => actor_def.glyph_e.as_ref().unwrap_or(&actor_def.glyph),
                    AbsoluteDirection::W => actor_def.glyph_w.as_ref().unwrap_or(&actor_def.glyph),
                };
                cell.set_symbol(glyph)
                    .set_fg(actor_def.color_object)
                    .set_bg(bgcolor);
            }
            WorldCell {
                building: Some(building),
                items: [maybe_item],
                ..
            } => {
                if maybe_item.is_some() {
                    cell.set_symbol(&building.definition.glyph)
                        .set_style(generic_style.add_modifier(Modifier::UNDERLINED));
                } else {
                    cell.set_symbol(&building.definition.glyph)
                        .set_style(generic_style);
                }
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

pub struct WorldWindowWidget<'a> {
    world: &'a World,
    center: Coordinate,
    data: &'a Data,
}

impl<'a> WorldWindowWidget<'a> {
    pub fn new(game: &'a Game) -> WorldWindowWidget<'a> {
        WorldWindowWidget {
            world: &game.world,
            center: *game
                .get_player_coords()
                .unwrap_or(&Coordinate { x: 0, y: 0 }),
            data: &game.data,
        }
    }
}

impl<'a> Widget for WorldWindowWidget<'a> {
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
                let coord = Coordinate { x, y };
                if self.world.actors.in_bounds(&coord) {
                    let cell = self
                        .world
                        .get_cell(&coord)
                        .expect("Cell out of bounds but was checked in bounds.");
                    cell.draw(self.data, &mut buf[(i, j)]);
                } else {
                    buf[(i, j)].set_symbol(" ").set_bg(Color::DarkGray);
                }
            }
        }
    }
}

struct ItemWidget {
    item: Item,
    idx: usize,
    itemdef: &'static ItemDefiniton,
}
impl ItemWidget {
    fn new(item: Item, idx: usize) -> ItemWidget {
        ItemWidget {
            item,
            idx,
            itemdef: item.definition,
        }
    }
}

impl Widget for ItemWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Paragraph::new(self.itemdef.name.clone()).block(
            Block::default()
                .title(Line::from((self.idx + 1).to_string()).left_aligned())
                .title(Line::from(self.itemdef.glyph.clone()).centered())
                .title(Line::from(self.item.quantity.to_string()).right_aligned())
                .borders(Borders::ALL),
        );
        block.render(area, buf);
    }
}

pub struct ItemBar {
    items: BasicInventory,
}

impl ItemBar {
    pub fn new(game: &Game) -> ItemBar {
        if let Ok(actor) = game.get_player_actor() {
            ItemBar {
                items: actor.inventory,
            }
        } else {
            ItemBar {
                items: Default::default(),
            }
        }
    }
}

impl Widget for ItemBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let slots: [Rect; 5] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 5); 5])
            .areas(area);
        for i in 0..self.items.get_items().len() {
            if let Some(item) = self.items.get_items()[i] {
                ItemWidget::new(item, i).render(slots[i], buf);
            } else {
                Paragraph::new("")
                    .block(Block::default())
                    .render(slots[i], buf);
            }
        }
    }
}

impl Widget for &Score {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!("{} points", self.0))
            .block(Block::default())
            .render(area, buf);
    }
}

pub fn generate_popup_layout(frame: &mut Frame) -> Rect {
    let [area] = Layout::horizontal([Constraint::Percentage(50)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Constraint::Percentage(50)])
        .flex(Flex::Center)
        .areas(area);

    Clear.render(area, frame.buffer_mut());

    area
}

pub fn generate_main_layout(frame: &Frame) -> (Rect, Rect, Rect, Rect) {
    let [tmp_main, tmp_side] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Length(20)]).areas(frame.area());

    let [main, bottom] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(tmp_main);

    let [side, corner] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(tmp_side);

    (main, side, bottom, corner)
}
