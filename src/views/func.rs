use crate::{
    View, ReturnState, ReturnStateEnum::*, Display,
    buttons::ButtonSet,
};

pub struct FuncView<'a> {
    func: Box<&'a dyn Fn() -> ()>
}

impl<'a> FuncView<'a> {
    pub fn new(func: &'a dyn Fn() -> ()) -> FuncView<'a> {
        FuncView {
            func: Box::new(func)
        }
    }
}

impl<'a> View for FuncView<'a> {
    fn render(&mut self, disp: &mut Display) {}
    fn handle_buttons(&mut self, buttons: &mut ButtonSet) -> ReturnState { Some(Pop) }
    fn activate(&mut self) {
        (self.func)()
    }
}
