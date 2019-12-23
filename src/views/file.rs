extern crate embedded_graphics;

use std::fs::DirEntry;
use std::path::{PathBuf};

use crate::{
    View, ReturnState, Display,
    buttons::ButtonSet,
    views::{MenuView,TextView},
    views::menu::MenuEntry
};

/// A view that shows a list of items in a folder.
pub struct FileView {
    path: PathBuf,
    menu: MenuView
}

impl FileView {
    /// Create a new file view from a path.
    pub fn new(dir: &str) -> FileView {
        FileView {
            path: PathBuf::from(dir),
            menu: MenuView::new()
        }
    }

    pub fn get_name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_owned()
    }
}

impl View for FileView {
    fn render(&mut self, disp: &mut Display) {
        self.menu.render(disp);
    }

    fn handle_buttons(&mut self, buttons: &mut ButtonSet) -> ReturnState {
        self.menu.handle_buttons(buttons)
    }

    fn activate(&mut self) {
        self.menu.set_name(self.get_name().as_ref());
        self.menu.set_entries(
            self.path.read_dir().unwrap().map(|dir: Result<DirEntry, std::io::Error>| -> MenuEntry {
                let dir = dir.unwrap();
                let name = dir.file_name().to_str().unwrap().to_owned();
                if let Ok(file_type) = dir.file_type() {
                    if file_type.is_dir() {
                        return (name.clone(), Box::new(FileView::from(self.path.join(dir.path()))));
                    }
                }
                return (name.clone(), Box::new(TextView::new(name.as_ref())));
            }).collect()
        );
    }
}

impl From<PathBuf> for FileView {
    /// Create a `FileView` from an existing path reference.
    fn from(p: PathBuf) -> FileView {
        FileView {
            path: p,
            menu: MenuView::new()
        }
    }
}
