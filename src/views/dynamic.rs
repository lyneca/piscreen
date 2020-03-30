extern crate embedded_graphics;

// use embedded_graphics::fonts::{Font,Font6x8};
// use embedded_graphics::coord::Coord;
// use embedded_graphics::prelude::*;
// use embedded_graphics::image::Image1BPP;
// use embedded_graphics::primitives::Rect;
// use embedded_graphics::primitives::Line;
// use embedded_graphics::Drawing;

use crate::{
    buttons::ButtonSet,
    views::{ON, OFF},
    View, ReturnState, ReturnStateEnum::*, Display
};

pub struct DynamicView {
    
}

impl View for DynamicView {
    fn render(&mut self, disp: &mut Display) {

    }
    fn handle_buttons(&mut self, buttons: &mut ButtonSet) -> ReturnState { None }
}

/*
{
    root: {
        type: "menu",
        entries: [
            {
                type: "file",
                dir: "~"
            },
            {
                type: "text",
                text: "Hello, world!"
            }
        ]
    }
}
*/
