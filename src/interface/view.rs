use rppal::i2c::I2c;
use ssd1306::{mode::GraphicsMode, interface::I2cInterface};
use super::buttons::ButtonSet;

pub enum ReturnStateEnum { Pop }
pub type ReturnState = Option<ReturnStateEnum>;

pub trait View {
    // For the user to implement
    fn render(&mut self, disp: &mut GraphicsMode<I2cInterface<I2c>>);
    fn handle_buttons(&mut self, buttons: &mut ButtonSet) -> ReturnState;
}
