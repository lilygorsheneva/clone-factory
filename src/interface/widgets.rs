//! Functions related to UI rendering.
//! Anything backend specific (ratatui) should be contained here.

use crate::actor::Actor;
use crate::buildings:: Building;
use crate::datatypes::Coordinate;
use crate::direction::AbsoluteDirection;
use crate::game_state::game::Game;
use crate::game_state::world::{FloorTile, World, WorldCell};
use crate::inventory::{BasicInventory, Item};
use crate::score::Score;
use crate::static_data::{Data, ObjectDescriptor};
use ratatui::buffer::Cell;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::prelude::Buffer;
use ratatui::style::{Color, Modifier, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};
use ratatui::{self, DefaultTerminal, Frame};

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
        Paragraph::new(self.descriptor.text.description.clone())
    }
}

impl Building {
    fn textbox(&self) -> Paragraph<'static> {
        Paragraph::new(self.definition.text.name.clone())
    }
}

struct ItemWidget {
    item: Item,
    idx: usize,
    itemdef: &'static ObjectDescriptor,
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
        let block = Paragraph::new(self.itemdef.text.name.clone()).block(
            Block::default()
                .title(Line::from((self.idx + 1).to_string()).left_aligned())
                .title(Line::from(self.itemdef.appearance.glyph.clone()).centered())
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
