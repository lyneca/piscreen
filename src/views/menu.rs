extern crate embedded_graphics;

use embedded_graphics::fonts::{Font,Font6x8};
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::image::Image1BPP;
use embedded_graphics::primitives::Rect;
use embedded_graphics::primitives::Line;
use embedded_graphics::Drawing;

use crate::{
    buttons::ButtonSet,
    views::{ON, OFF},
    View, ReturnState, ReturnStateEnum::*, Display
};

/// Number of entries shown on the screen.
const NUM_ENTRIES_SHOWN: u8 = 4;

/// Type representing a text entry and a Viewable object.
pub type MenuEntry = (String, Box<dyn View>);

/// A view that provies a scrolling list of selectable entries.
pub struct MenuView {
    name: Option<String>,
    active: bool,
    entries: Vec<MenuEntry>,
    selected: u8,
    first_visible_item: u8,
    text_scroll_offset: u8
}

const ARROW_DOWN: &[u8] = &[
    0b00000000,
    0b00100000,
    0b00100000,
    0b00100000,
    0b10101000,
    0b01110000,
    0b00100000,
    0b00000000,
];

impl MenuView {
    /// Create a new, empty menu.
    pub fn new() -> MenuView {
        MenuView {
            name: None,
            entries: vec![],
            selected: 0,
            active: false,
            first_visible_item: 0,
            text_scroll_offset: 0
        }
    }

    /// Add an entry to a menu.
    pub fn add_entry(&mut self, entry: MenuEntry) {
        self.entries.push(
            (entry.0.to_owned(), entry.1)
        );
    }

    /// Set the list of entries on an existing menu.
    pub fn set_entries(&mut self, entries: Vec<MenuEntry>) {
        self.entries = entries;
    }

    /// Select the name of the menu
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_owned());
    }

    /// Select the next entry in the menu.
    pub fn next_entry(&mut self) {
        self.selected += 1;
        self.selected %= self.entries.len() as u8;
        if self.selected == 0 {
            self.first_visible_item = 0;
        }
        if self.selected >= self.first_visible_item + NUM_ENTRIES_SHOWN {
            self.first_visible_item = self.selected - NUM_ENTRIES_SHOWN + 1;
        }
    }

    /// Select the previous entry in the menu.
    pub fn prev_entry(&mut self) {
        if self.selected == 0 {
            self.selected = self.entries.len() as u8 - 1;
            self.first_visible_item = self.entries.len() as u8 - NUM_ENTRIES_SHOWN;
        } else {
            self.selected -= 1;
        }
        if self.selected <= self.first_visible_item {
            self.first_visible_item = self.selected;
        }
    }

    /// Select the first entry
    pub fn first_entry(&mut self) {
        self.selected = 0;
        self.first_visible_item = 0;
    }

    /// Select the last entry
    pub fn last_entry(&mut self) {
        self.selected = self.entries.len() as u8 - 1;
        if self.entries.len() > NUM_ENTRIES_SHOWN as usize {
            self.first_visible_item = self.selected - (NUM_ENTRIES_SHOWN - 1);
        }
    }

    /// Create a new menu with the provided entries.
    pub fn with_entries(entries: Vec<MenuEntry>) -> MenuView {
        MenuView {
            name: None,
            entries,
            selected: 0,
            active: false,
            first_visible_item: 0,
            text_scroll_offset: 0
        }
    }

    /// Handle the buttons from the menu itself (i.e. don't pass down to any children).
    fn handle_buttons_self(&mut self, buttons: &mut ButtonSet) -> ReturnState {
        if buttons.up.was_pressed() { self.prev_entry() }
        if buttons.down.was_pressed() { self.next_entry() }

        if buttons.left.was_pressed() { self.first_entry() }
        if buttons.right.was_pressed() { self.last_entry() }

        if buttons.a.was_pressed() {
            self.active = true;
            self.entries[self.selected as usize].1.activate();
        }
        if buttons.b.was_pressed() {
            return Some(Pop)
        }
        if buttons.c.was_pressed() { }
        ReturnState::None
    }

    /// Handle rendering from the menu itself (i.e. don't pass down to any children).
    fn render_self(&mut self, disp: &mut Display) {
        // let has_scroll = self.entries.len() > 4;
        let has_scroll = false;
        let width = match has_scroll {
            true => 123,
            false => 127,
        };
        for (i, entry) in self.entries.iter().skip(self.first_visible_item as usize).take(NUM_ENTRIES_SHOWN as usize).enumerate() {
            let is_selected = std::ptr::eq(&self.entries[self.selected as usize], entry);
            disp.draw(Rect::new(
                    Coord::new(0, i as i32 * 13),
                    Coord::new(width, (i + 1) as i32 * 13))
                .with_stroke(ON)
                .with_fill(match is_selected { true => ON, false => OFF })
                .into_iter());
            disp.draw(Font6x8::render_str(match entry.0.get(..20) {
                Some(s) => s,
                None => entry.0.as_str()
            })
                .with_stroke(match !is_selected { true => ON, false => OFF })
                .with_fill(match is_selected { true => ON, false => OFF })
                .translate(Coord::new(3, i as i32 * 13 + 3))
                .into_iter());
            disp.draw(Rect::new(
                    Coord::new(0, i as i32 * 13),
                    Coord::new(width, (i + 1) as i32 * 13))
                .with_stroke(ON)
                .into_iter());
        }
        if has_scroll {
            disp.draw(Line::new(
                    Coord::new(127 - 2, 0),
                    Coord::new(127, 0))
                .with_stroke(ON)
                .into_iter());
            disp.draw(Line::new(
                    Coord::new(127 - 2, 63),
                    Coord::new(127, 63))
                .with_stroke(ON)
                .into_iter());
            disp.draw(Line::new(
                    Coord::new(127 - 1, 0),
                    Coord::new(127 - 1, 63))
                .with_stroke(ON)
                .into_iter());
            let height = 60f32 / self.entries.len() as f32;
            let offset = (2f32 + self.selected as f32 * height as f32) as u8;
            disp.draw(Rect::new(
                    Coord::new(127 - 2, offset as i32),
                    Coord::new(127, (offset + height as u8) as i32))
                .with_fill(ON)
                .into_iter());
        } else {

        }
        match &self.name {
            Some(name) => disp.draw(Font6x8::render_str(name.as_ref())
                .translate(Coord::new(3, 4 * 13 + 3))
                .into_iter()),
            None => {}
        };
        if self.first_visible_item < self.entries.len() as u8 - NUM_ENTRIES_SHOWN
            && NUM_ENTRIES_SHOWN <= self.entries.len() as u8 {
            disp.draw(Image1BPP::new(ARROW_DOWN, 5, 8)
                .translate(Coord::new(width - 7, 4 * 13 + 3))
                .into_iter());
        }
        self.text_scroll_offset = 0;
    }
}

impl View for MenuView {
    fn handle_buttons(&mut self, buttons: &mut ButtonSet) -> ReturnState {
        if self.active {
            match self.entries[self.selected as usize].1.handle_buttons(buttons) {
                Some(Pop) => {
                    self.active = false;
                    None
                },
                _ => None
            }
        } else {
            self.handle_buttons_self(buttons)
        }
    }

    fn render(&mut self, disp: &mut Display) {
        match self.active {
            true => self.entries[self.selected as usize].1.render(disp),
            false => self.render_self(disp)
        }
    }
}
