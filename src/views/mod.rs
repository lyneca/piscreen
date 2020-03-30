mod menu;
mod dynamic;
mod text;
mod text_input;
mod empty;
mod file;
mod func;

use embedded_graphics::pixelcolor::PixelColorU8;

pub const ON: Option<PixelColorU8> = Some(PixelColorU8(1));
pub const OFF: Option<PixelColorU8> = Some(PixelColorU8(0));

pub use menu::MenuView;
pub use menu::MenuEntry;
pub use text::TextView;
pub use text_input::TextInputView;
pub use empty::EmptyView;
pub use file::FileView;
pub use func::FuncView;
pub use dynamic::DynamicView;
