//! Functions related to UI rendering.
//! Anything backend specific (ratatui) should be contained here.
use std::collections::HashMap;

use crate::actor::Actor;
use crate::buildings:: Building;
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
use ratatui::style::{Color, Modifier, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};
use ratatui::{self, DefaultTerminal, Frame};

impl<'a> WorldCell<'a> {
    fn draw(&'a self, data: &Data, cell: &mut Cell) {


        // Handle floor color.
        let (r, g, b):(u8,u8,u8) = match self.floor {
            FloorTile::Water => (0, 0, 200),
            FloorTile::Stone => (69, 69, 69),
            FloorTile::Dirt => (69, 35, 10),
        };

        // Handle Paradox color overlay
        let pdx_overlay = self.paradox.0 as u8;
        let mut bgcolor = Color::Rgb(
            r.saturating_add(pdx_overlay),
            g.saturating_add(pdx_overlay),
            b.saturating_add(pdx_overlay),
        );
        let mut fgcolor = Color::Rgb(b, g, r);
        let mut fg_glyph = " ";
        let mut modifiers = Modifier::empty();

        if let Some(actor) = self.actor {
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
            fgcolor = actor_def.color;
            fg_glyph = glyph; 
        }
        
        if let Some(building) = self.building {
            if fg_glyph == " " {
                fg_glyph = &building.definition.glyph;
                fgcolor = Color::Black;
            } else {
                modifiers = modifiers | Modifier::UNDERLINED;
            }
        }

        if let Some(item) = self.items[0] {
            if fg_glyph == " " {
                fg_glyph = &item.definition.glyph;
                fgcolor = Color::Black;
            } else {
                modifiers = modifiers | Modifier::UNDERLINED;
            }
        }

       cell.set_symbol(fg_glyph).set_fg(fgcolor).set_bg(bgcolor);
       cell.modifier = modifiers;
       
    }


    fn as_list(&'a self) -> Vec<Paragraph<'a>> {
        let mut tmp = Vec::new();
        if let Some(actor) =self.actor {
            tmp.push(actor.textbox());
        }
        if let Some(building) =self.building {
            tmp.push(building.textbox());
        }
        if let Some(item) =self.items[0] {
            tmp.push( Paragraph::new(item.definition.name.clone()).block(
                Block::default()
                    .title(Line::from(item.definition.glyph.clone()).centered())
                    .borders(Borders::ALL)));
        }
        tmp.push(self.floor.textbox());
        tmp
    }

    pub fn render_as_list(&self, area: Rect, buf: &mut Buffer) {
        let list = self.as_list();
        let slots = Layout::vertical(vec![Constraint::Min(1); list.len()]).split(area);
        for i in 0..list.len() {
            // TODO: clean this up.
            list[i].clone().render(slots[i], buf)
        }
    }
}


impl FloorTile {
    fn textbox(&self) -> Paragraph<'static> {
        match self {
            FloorTile::Water => Paragraph::new("Impassable").bg(Color::Red).block(Block::new().title("Water")),
            FloorTile::Dirt => Paragraph::new("").block(Block::new().title("Dirt")),
            FloorTile::Stone => Paragraph::new("").block(Block::new().title("Stone"))
        }
    }
}

impl Actor {
    fn textbox(&self) -> Paragraph<'static> {
        match self.isplayer {
            true => Paragraph::new("It's you"),
            false => Paragraph::new("It's your clone"),
        }
    }
}

impl Building {
    fn textbox(&self) -> Paragraph<'static> {
        Paragraph::new(self.definition.name.clone())
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
    pub center: Coordinate,
    data: &'a Data,
    pub show_cursor: bool
}

impl<'a> WorldWindowWidget<'a> {
    pub fn new(game: &'a Game) -> WorldWindowWidget<'a> {
        WorldWindowWidget {
            world: &game.world,
            center: *game
                .get_player_coords()
                .unwrap_or(&Coordinate { x: 0, y: 0 }),
            data: &game.data,
            show_cursor: false
        }
    }
}

impl<'a> Widget for WorldWindowWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (cols, rows) = (area.width as i32, area.height  as i32);
        let (centerx, centery) = (
            cols  / 2 - self.center.x,
            rows  / 2 + self.center.y,
        );

        for i in 0..cols {
            for j in 0..rows {
                let x = i - centerx;
                let y = centery - j;
                let coord = Coordinate { x, y };
                let buf_idx = (i as u16, j as u16);
                if self.world.actors.in_bounds(&coord) {
                    let cell = self
                        .world
                        .get_cell(&coord)
                        .expect("Cell out of bounds but was checked in bounds.");
                    cell.draw(self.data, &mut buf[buf_idx]);
                } else {
                    buf[buf_idx].set_symbol(" ").set_bg(Color::DarkGray);
                }
                if self.show_cursor {
                    if (i - (cols/2)).abs() + (j - (rows/2)).abs()  == 1  {
                        let style = buf[buf_idx].style(); 
                        buf[buf_idx].set_style(style.add_modifier(Modifier::REVERSED));
                    }
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
        Paragraph::new(format!("Turn {}\n{} points.",self.turn, self.score))
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
