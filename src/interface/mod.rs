pub mod buttons;
pub mod view;
pub mod views;

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

macro_rules! text_view {
    ( $x:expr ) => { { TextView::new($x) } }
}
