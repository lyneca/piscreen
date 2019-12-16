mod buttons;
mod view;
pub mod views;

pub use buttons::*;
pub use view::*;

/// Create a new menu.
/// ```
/// menu_view![
///     ("Entry Name 2", text_view!("Text view with value!")),
///     ("Entry Name 2", menu_view![
///         ("Submenu!", text_view!("Text view inside submenu!"))
///     ]),
///     ("Entry Name 2", menu_view!("Submenu!")),
///     // Can also manually create view entries:
///     ("Entry Name 3", TextView::new("Hello, world!".to_owned())
/// ];
/// ```
#[macro_export]
macro_rules! menu_view {
    ( $( ($x:expr, $y:expr) ),* ) => {
        {
            let mut temp_menu = MenuView::new();
            $(
                temp_menu.add_entry(($x.to_owned(), Box::new($y)));
            )*
            temp_menu
        }
    };
}

/// Create a new text view.
/// ```
/// text_view!("Hello, world!");
/// ```
#[macro_export]
macro_rules! text_view {
    ( $x:expr ) => { { TextView::new($x) } }
}

/// Create a new file view from a path string.
/// ```
/// file_view!("/home/pi");
/// ```
#[macro_export]
macro_rules! file_view {
    ( $x:expr ) => { { FileView::new($x) } }
}
