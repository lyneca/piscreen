extern crate embedded_graphics;

use std::fs::DirEntry;
use std::path::Path;

use crate::{
    View, ReturnState, Display,
    buttons::ButtonSet,
    views::{MenuView,TextView},
    views::menu::MenuEntry
};

/// A view that shows a list of items in a folder.
pub struct FileView<'a> {
    path: &'a Path,
    menu: MenuView
}

impl<'a> FileView<'a> {
    /// Create a new file view from a path.
    pub fn new(dir: &str) -> FileView {
        FileView {
            path: Path::new(dir),
            menu: MenuView::new()
        }
    }

    pub fn get_name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_owned()
    }
}

impl<'a> View for FileView<'a> {
    fn render(&mut self, disp: &mut Display) {
        self.menu.set_name(self.get_name().as_ref());
        self.menu.set_entries(
            self.path.read_dir().unwrap().map(|dir: Result<DirEntry, std::io::Error>| -> MenuEntry {
                let name = dir.unwrap().file_name().to_str().unwrap().to_owned();
                (name.clone(), Box::new(TextView::new(name.as_ref())))
            }).collect()
        );
        self.menu.render(disp);
        // self.draw_dirname(disp);
        // for (i, dir) in self.path.read_dir().unwrap().enumerate() {
            // self.draw_dir_entry(i, &dir.unwrap(), disp);
        // }
    }
    fn handle_buttons(&mut self, buttons: &mut ButtonSet) -> ReturnState {
        let ret = self.menu.handle_buttons(buttons);
        println!("{:?}", ret);
        ret
    }
}

impl<'a> From<&'a Path> for FileView<'a> {
    /// Create a `FileView` from an existing path reference.
    fn from(p: &'a Path) -> FileView<'a> {
        FileView {
            path: p,
            menu: MenuView::new()
        }
    }
}
