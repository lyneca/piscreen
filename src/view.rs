use rppal::i2c::I2c;
use ssd1306::{mode::GraphicsMode, interface::I2cInterface};
use crate::ButtonSet;

/// An I2C interface that is able to be rendered to.
pub type Display = GraphicsMode<I2cInterface<I2c>>;

/// Enum holding data returned from a child view to a parent view.
#[derive(Debug)]
pub enum ReturnStateEnum {
    Pop,
    Text(String)
}

/// Type for holding a return state in an option.
pub type ReturnState = Option<ReturnStateEnum>;

/// Trait for items that can be rendered to the screen
/// and accept button input.
pub trait View {
    /// Render the view to the screen.
    fn render(&mut self, disp: &mut Display);

    /// Handle button inputs.
    fn handle_buttons(&mut self, buttons: &mut ButtonSet) -> ReturnState;

    /// Activate the view before being rendered for the first time
    fn activate(&mut self) {}
}
