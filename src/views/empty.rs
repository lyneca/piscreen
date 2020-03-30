use crate::buttons::ButtonSet;
use crate::{View, ReturnState, Display};


/// An empty view that has no interactions and doesn't render anything.
pub struct EmptyView { }
impl EmptyView { pub fn new() -> EmptyView { EmptyView { } } }
impl View for EmptyView {
    fn render(&mut self, disp: &mut Display) {}
    fn handle_buttons(&mut self, buttons: &mut ButtonSet) -> ReturnState { None }
}

