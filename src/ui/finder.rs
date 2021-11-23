// Generic Finder
use super::ZedError;
use crate::error::Error;
use crate::ui::Component;
use std::io::Write;
use zui_core::key::{Key, KeyIterator};
use zui_core::term::Terminal;
use zui_core::widgets::popup::Popup;
use zui_core::widgets::{Position, Widget};

fn finder<T: Write>(term: &mut Terminal<T>, x: impl Iterator) {
    // Generic Finder
    let x = Popup::new(term)
        .title("Find")
        .width(60)
        .height(2)
        .y_offset(14);
    let x_deets = x.render(term).unwrap();

    let p = Popup::new(term).title("").width(60).height(25);
    let p_deets = p.render(term).unwrap();

    term.set_cursor_to(x_deets.starting_pos.0 + 2, x_deets.starting_pos.1 + 1)
        .unwrap();
}

pub struct FileFinder {
    search: String,
    show_icon: bool,
}

impl Component for FileFinder {
    type Widget = FileFinder;

    fn new() -> Self::Widget {
        FileFinder {
            // If you change the banner, make sure to edit cursor (view method)
            // No spaces on lines
            show_icon: true,
            search: String::new(),
        }
    }

    fn destroy<T: Write>(&mut self, term: &mut Terminal<T>) -> super::ZedError {
        match term.clear_screen() {
            Ok(_) => Ok(()),
            Err(_e) => Err(Error::CouldNotRender),
        }
    }

    // Fix all these unwraps
    fn view<T: Write>(&mut self, term: &mut Terminal<T>) -> super::ZedError {
        // Inital values
        finder(term, vec![1, 2, 3].iter());

        Ok(())
    }

    fn handle_key<T: Write>(
        &mut self,
        term: &mut Terminal<T>,
        keys: KeyIterator,
    ) -> super::ZedError {
        for key in keys.clone() {
            match key {
                Key::Esc => {
                    self.destroy(term).unwrap();
                    return Ok(());
                }
                Key::Char(x) => {
                    self.search.push(x);
                    term.print(x.to_string().as_str()).unwrap();
                    term.set_cursor_to(term.x_pos + 1, term.y_pos).unwrap();
                    term.show_cursor().unwrap();
                }
                Key::Backspace => {
                    if self.search.len() >= 1 {
                        self.search = self.search[..self.search.len() - 1].to_string();
                        term.set_cursor_to(term.x_pos - 1, term.y_pos).unwrap();
                        term.print(" ").unwrap();
                        term.set_cursor_to(term.x_pos, term.y_pos).unwrap();
                        term.show_cursor().unwrap();
                    }
                } //TODO: Add Arrow Keys
                _ => continue,
            }
        }
        Ok(())
    }
}
