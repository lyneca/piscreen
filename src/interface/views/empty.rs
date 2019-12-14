use rppal::i2c::I2c;
use ssd1306::{mode::GraphicsMode, interface::I2cInterface};

use crate::interface::{
    buttons::ButtonSet,
    view::{View, ReturnState}
};


pub struct EmptyView<'a> { child: Option<&'a Box<dyn View>> }
impl<'a> EmptyView<'a> { fn new() -> EmptyView<'a> { EmptyView { child: None } } }
impl<'a> View for EmptyView<'a> {
    fn render(&mut self, disp: &mut GraphicsMode<I2cInterface<I2c>>) {}
    fn handle_buttons(&mut self, buttons: &mut ButtonSet) -> ReturnState { None }
}

