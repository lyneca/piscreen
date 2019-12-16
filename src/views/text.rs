use crate::{
    buttons::ButtonSet,
    View, ReturnState, ReturnStateEnum::*, Display,
};

use embedded_graphics::{
    fonts::{Font,Font6x8},
    coord::Coord,
    prelude::*,
    pixelcolor::PixelColorU8,
    primitives::Rect,
    Drawing
};

const SCROLL_AMOUNT: u16 = 7;
const SCROLL_AMOUNT_HOLD: u16 = 4;

/// A view that renders wrapped, scrolling text.
pub struct TextView { text: String, offset: u16 }

impl TextView {
    pub fn new(text: &str) -> TextView {
        TextView { text: text.to_owned(), offset: 0 }
    }

    /// Wrap the text in the view over several lines.
    pub fn get_lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        let mut next: String = String::new();
        for word in self.text.split_ascii_whitespace() {
            if next.len() + word.len() < 20 {
                next.push(' ');
                next.push_str(word.as_ref());
            } else {
                lines.push(next.trim().to_owned());
                next.clear();
                next.push_str(word.as_ref());
            }
        }
        lines.push(next.trim().to_owned());
        lines
    }

    /// Get the maximum vertical offset of the contained text when rendered to
    /// the screen.
    fn get_max_offset(&self) -> u16 {
        return (self.get_lines().len()) as u16 * 9 + 1 - 60;
    }
}

impl View for TextView {
    fn render(&mut self, disp: &mut Display) {
        for (i, line) in self.get_lines().iter().enumerate() {
            disp.draw(Font6x8::render_str(line).translate(Coord::new(3, 3 + 9 * i as i32 - self.offset as i32)).into_iter());
        }
        disp.draw(Rect::new(
            Coord::new(0, 0),
            Coord::new(127, 63),
        ).with_stroke(Some(PixelColorU8(1))).into_iter());
    }

    fn handle_buttons(&mut self, buttons: &mut ButtonSet) -> ReturnState {
        if buttons.up.was_pressed() {
            self.offset = self.offset.saturating_sub(
                if buttons.up.is_hold {SCROLL_AMOUNT_HOLD} else {SCROLL_AMOUNT}
            );
        }
        if buttons.down.was_pressed() {
            self.offset = std::cmp::min(self.offset.saturating_add(
                    if buttons.up.is_hold { SCROLL_AMOUNT_HOLD } else { SCROLL_AMOUNT }
            ), self.get_max_offset());
        }
        if buttons.left.was_pressed() { self.offset = 0; }
        if buttons.right.was_pressed() { self.offset = self.get_max_offset(); }
        if buttons.b.was_pressed() { return Some(Pop) }
        None
    }
}

