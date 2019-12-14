extern crate embedded_graphics;

use std::cmp;

use rppal::i2c::I2c;
use ssd1306::{mode::GraphicsMode, interface::I2cInterface};

use embedded_graphics::fonts::{Font,Font6x8};
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::image::Image1BPP;
use embedded_graphics::pixelcolor::PixelColorU8;
use embedded_graphics::primitives::Rect;
use embedded_graphics::Drawing;

use crate::interface::{
    buttons::ButtonSet,
    view::{View, ReturnState, ReturnStateEnum::*}
};

pub type MenuEntry = (String, Box<dyn View>);

pub struct MenuView {
    active: bool,
    entries: Vec<MenuEntry>,
    selected: u8,
    text_scroll_offset: u8
}

const ARROW: &[u8] = &[
    0b01000000,
    0b11111100,
    0b11111100,
    0b01000000,
];

impl MenuView {
    pub fn new() -> MenuView {
        MenuView {
            entries: vec![],
            selected: 0,
            active: false,
            text_scroll_offset: 0
        }
    }

    pub fn add_entry(&mut self, entry: MenuEntry) {
        self.entries.push(
            (entry.0[..cmp::min(19, entry.0.len())].to_owned(), entry.1)
        );
    }

    pub fn next_entry(&mut self) {
        self.selected += 1;
        self.selected %= self.entries.len() as u8;
    }

    pub fn prev_entry(&mut self) {
        if self.selected == 0 {
            self.selected = self.entries.len() as u8 - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn with_entries(entries: Vec<MenuEntry>) -> MenuView {
        MenuView {
            entries,
            selected: 0,
            active: false,
            text_scroll_offset: 0
        }
    }

    pub fn handle_buttons_self(&mut self, buttons: &mut ButtonSet) -> ReturnState {
        if buttons.b.was_pressed() { }
        if buttons.c.was_pressed() { }
        if buttons.left.was_pressed() { }
        if buttons.right.was_pressed() { }

        if buttons.up.was_pressed() { self.prev_entry() }
        if buttons.down.was_pressed() { self.next_entry() }

        if buttons.a.was_pressed() { self.active = true }
        ReturnState::None
    }

    fn render_self(&mut self, disp: &mut GraphicsMode<I2cInterface<I2c>>) {
        for (i, entry) in self.entries.iter().enumerate() {
            if i as u8 == self.selected {
                disp.draw(Image1BPP::<PixelColorU8>::new(ARROW, 6, 4)
                    .translate(Coord::new(120, i as i32 * 13 + 5))
                    .into_iter()
                );
                self.text_scroll_offset = 0;
            }
            disp.draw(Font6x8::render_str(entry.0.as_ref()).translate(Coord::new(3, i as i32 * 13 + 3)).into_iter());
            disp.draw(Rect::new(
                    Coord::new(0, i as i32 * 13),
                    Coord::new(117, (i + 1) as i32 * 13),
            ).with_stroke(Some(PixelColorU8(1))).into_iter());
        }
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
                None => None
            }
        } else {
            self.handle_buttons_self(buttons)
        }
    }

    fn render(&mut self, disp: &mut GraphicsMode<I2cInterface<I2c>>) {
        match self.active {
            true => self.entries[self.selected as usize].1.render(disp),
            false => self.render_self(disp)
        }
    }
}
