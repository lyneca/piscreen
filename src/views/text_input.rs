use embedded_graphics::fonts::{Font,Font6x8};
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::image::Image1BPP;
use embedded_graphics::pixelcolor::PixelColorU8;
use embedded_graphics::primitives::Rect;
use embedded_graphics::primitives::Line;
use embedded_graphics::Drawing;

use std::time::SystemTime;

use crate::{
    buttons::ButtonSet,
    View, ReturnState, ReturnStateEnum::*, Display
};

enum TextInputMode { Viewing, Entering }
enum Direction { Up, Down, Left, Right }

use TextInputMode::*;
use Direction::*;

struct InputTree {
    caps: bool,
    group: &'static str,
}

/*
 * A..Z 0..9 !#@$ ()[]
 * A..Z: A-M M-Z
 * A..M: abcd efgh ijkl mnop
 * N..Z: qrst uvwx yz
 * 0..9: 0123 4567 89
 * .,!?: .,!? :;-_ #@$% &^*~
 * ()[]: ()[] <>{} "'`| /\+=
 */

impl InputTree {
    fn get_group(group: &str, pressed: Direction) -> &str {
        let group_case = group.to_uppercase();
        match (group_case.as_str(), pressed) {
            ("", d) => match d { Up => "A..Z", Right => "0..9", Down => "PUNC", Left => "EXTR" }
            ("A..Z", d) => match d { Up => "A..M", Down => "N..Z", _ => ""}
            ("A..M", d) => match d { Up => "ABCD", Right => "EFGH", Left => "IJKL", Down => "M"}
            ("N..Z", d) => match d { Up => "NOPQ", Right => "RSTU", Left => "VWXY", Down => "Z"}
            ("ABCD", d) => match d { Left => "A", Down => "B", Up => "C", Right => "D" }
            ("EFGH", d) => match d { Left => "E", Down => "F", Up => "G", Right => "H" }
            ("IJKL", d) => match d { Left => "I", Down => "J", Up => "K", Right => "L" }
            ("NOPQ", d) => match d { Left => "N", Down => "O", Up => "P", Right => "Q" }
            ("RSTU", d) => match d { Left => "R", Down => "S", Up => "T", Right => "U" }
            ("VWXY", d) => match d { Left => "V", Down => "W", Up => "X", Right => "Y" }
            ("0..9", d) => match d { Up => "0123", Right => "4567", Down => "8", Left => "9" }
            ("0123", d) => match d { Left => "0", Down => "1", Up => "2", Right => "3" }
            ("4567", d) => match d { Left => "4", Down => "5", Up => "6", Right => "7" }
            ("PUNC", d) => match d { Up => ".,!?", Right => ":;-_", Down => "#@$%", Left => "&^*~" }
            (".,!?", d) => match d { Left => ".", Down => ",", Up => "!", Right => "?" }
            (":;-_", d) => match d { Left => ":", Down => ";", Up => "-", Right => "_" }
            ("#@$%", d) => match d { Left => "#", Down => "@", Up => "$", Right => "%" }
            ("&^*~", d) => match d { Left => "&", Down => "^", Up => "*", Right => "~" }
            ("EXTR", d) => match d { Up => "()[]", Right => "<>{}", Down => "\"'`|", Left => "/\\+=" }
            ("()[]", d) => match d { Left => "(", Down => ")", Up => "[", Right => "]" }
            ("<>{}", d) => match d { Left => "<", Down => ">", Up => "{", Right => "}" }
            ("\"'`|", d) => match d { Left => "\"", Down => "'", Up => "`", Right => "|" }
            ("/\\+=", d) => match d { Left => "/", Down => "\\", Up => "+", Right => "=" }
            (_, _) => ""
        }
    }

    fn new() -> InputTree {
        InputTree {
            caps: false,
            group: "",
        }
    }

    fn reset(&mut self) {
        self.group = "";
    }
    
    fn render(&self, disp: &mut Display) {

        let up: String;
        let down: String;
        let left: String;
        let right: String;

        if self.caps {
            up = InputTree::get_group(self.group, Up).to_uppercase();
            right = InputTree::get_group(self.group, Right).to_uppercase();
            down = InputTree::get_group(self.group, Down).to_uppercase();
            left = InputTree::get_group(self.group, Left).to_uppercase();
        } else {
            up = InputTree::get_group(self.group, Up).to_lowercase();
            right = InputTree::get_group(self.group, Right).to_lowercase();
            down = InputTree::get_group(self.group, Down).to_lowercase();
            left = InputTree::get_group(self.group, Left).to_lowercase();
        }

        if up.len() > 0 {
            disp.draw(Font6x8::render_str(up.as_str())
                .translate(Coord::new(64 - (up.len() as i32 * 6) / 2, 24 - 16))
                .into_iter());
            disp.draw(Line::new(Coord::new(63, 23), Coord::new(63, 23 - 4))
                .with_stroke(Some(PixelColorU8(1)))
                .into_iter())
        }
        if down.len() > 0 {
            disp.draw(Font6x8::render_str(down.as_str())
                .translate(Coord::new(64 - (down.len() as i32 * 6) / 2, 24 + 8))
                .into_iter());
            disp.draw(Line::new(Coord::new(63, 23), Coord::new(63, 23 + 4))
                .with_stroke(Some(PixelColorU8(1)))
                .into_iter())
        }
        if left.len() > 0 {
            disp.draw(Font6x8::render_str(left.as_str())
                .translate(Coord::new(64 - 8 - left.len() as i32 * 6, 24 - 4))
                .into_iter());
            disp.draw(Line::new(Coord::new(63, 23), Coord::new(63 - 4, 23))
                .with_stroke(Some(PixelColorU8(1)))
                .into_iter())
        }
        if right.len() > 0 {
            disp.draw(Font6x8::render_str(right.as_str())
                .translate(Coord::new(64 + 8, 24 - 4))
                .into_iter());
            disp.draw(Line::new(Coord::new(63, 23), Coord::new(63 + 4, 23))
                .with_stroke(Some(PixelColorU8(1)))
                .into_iter())
        }
    }

    fn handle_buttons(&mut self, buttons: &mut ButtonSet) -> ReturnState {
        if buttons.b.was_pressed() {
            return Some(Pop);
        }

        if buttons.c.was_pressed() {
            self.caps = !self.caps;
        }

        // Defs a hack but I don't think it's easily possible to hit two d-pad buttons at once
        let pressed: Direction = if buttons.left.was_pressed() { Left }
            else if buttons.right.was_pressed() { Right }
            else if buttons.up.was_pressed() { Up }
            else if buttons.down.was_pressed() { Down }
            else { return None };
        
        self.group = InputTree::get_group(self.group, pressed);

        println!("{}", self.group);
        match self.group.len() {
            0 => Some(Text(" ".to_owned())),
            1 => Some(Text(self.group.to_owned())),
            _ => None
        }
    }
}

pub struct TextInputView {
    text: String,
    input_tree: InputTree,
    mode: TextInputMode,
    selected_char: u8
}

impl TextInputView {
    pub fn new() -> TextInputView {
        TextInputView {
            text: String::from(" "),
            input_tree: InputTree::new(),
            mode: Viewing,
            selected_char: 0
        }
    }
    
    fn render_text(&self, disp: &mut Display) {
        disp.draw(Rect::new(
                Coord::new(0, 64 - 14),
                Coord::new(127, 63))
            .with_stroke(Some(PixelColorU8(1)))
            .into_iter());
        for (i, char) in self.text.as_str().chars().enumerate() {
            if i == self.selected_char as usize {
                disp.draw(Line::new(Coord::new(0, 0), Coord::new(6, 0))
                    .with_stroke(Some(PixelColorU8(1)))
                    .translate(Coord::new(2 + 6 * i as i32, 61))
                    .into_iter())
            }
            disp.draw(Font6x8::render_str(
                    std::str::from_utf8(&[char as u8]).unwrap())
                    .with_stroke(Some(PixelColorU8(1)))
                    .translate(Coord::new(3 + 6 * i as i32, 64 - 11))
                    .into_iter())
        };
    }
}

impl View for TextInputView {
    fn render(&mut self, disp: &mut Display) {
        self.render_text(disp);
        match self.mode {
            Viewing => { }
            Entering => { self.input_tree.render(disp) }
        }
    }
    fn handle_buttons(&mut self, buttons: &mut ButtonSet) -> ReturnState {
        match self.mode {
            Viewing => {
                if buttons.b.was_pressed() { return Some(Pop) }
                if buttons.a.was_pressed() { 
                    self.input_tree.reset();
                    self.mode = Entering
                }
                if buttons.left.was_pressed() {
                    if self.selected_char > 0 {
                        self.selected_char -= 1;
                    }
                }

                if buttons.right.was_pressed() {
                    if self.selected_char as usize == self.text.len() - 1 {
                        self.text.push(' ');
                    }
                    self.selected_char += 1;
                }
                None
            }
            Entering => {
                match self.input_tree.handle_buttons(buttons) {
                    Some(Pop) => {
                        self.mode = Viewing;
                        None
                    }
                    Some(Text(t)) => {
                        self.mode = Viewing;
                        let mut out = String::new();
                        for (i, byte) in self.text.as_bytes().iter().enumerate() {
                            if i == self.selected_char as usize {
                                if self.input_tree.caps {
                                    out.push_str(t.to_uppercase().as_str());
                                } else {
                                    out.push_str(t.to_lowercase().as_str());
                                }
                            } else {
                                out.push(char::from(*byte))
                            }
                        }
                        if !out.ends_with(" ") {
                            out.push_str(" ")
                        }
                        self.text = out;
                        None
                    }
                    None => None
                }
            }
        }
    }
    fn activate(&mut self) { }
}
