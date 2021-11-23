use std::io::Write;

use crate::error::Error;

use super::Component;
use crate::ui::finder;
use zui_core::key::{Key, KeyIterator};
use zui_core::term::Terminal;

pub struct Dashboard {
    pub banner: &'static str,
    selected_option: u8,
}

impl Dashboard {}

impl Component for Dashboard {
    type Widget = Dashboard;

    fn new() -> Self::Widget {
        Dashboard {
            // If you change the banner, make sure to edit cursor (view method)
            // No spaces on lines
            banner: r"
███████╗███████╗██████╗     
╚══███╔╝██╔════╝██╔══██╗    
  ███╔╝ █████╗  ██║  ██║    
 ███╔╝  ██╔══╝  ██║  ██║    
███████╗███████╗██████╔╝    
╚══════╝╚══════╝╚═════╝     
    ",
            selected_option: 1,
        }
    }

    fn destroy<T: Write>(&mut self, term: &mut Terminal<T>) -> super::ZedError {
        match term.switch_main() {
            Ok(_) => Ok(()),
            Err(_e) => Err(Error::CouldNotRender),
        }
    }

    // Fix all these unwraps
    fn view<T: Write>(&mut self, term: &mut Terminal<T>) -> super::ZedError {
        // Inital values
        let (x, y) = term.get_size(); // TODO: Fix this
                                      // Setup Rendering
        term.switch_screen().unwrap();
        term.clear_screen().unwrap();

        // Render Logo
        term.set_cursor_to((x as f64 / 2.35) as u16, (y as f64 / 4.3) as u16)
            .unwrap(); // If you change the banner, modify this
        for line in self.banner.lines() {
            term.print(line).unwrap();
            term.set_cursor_to(term.x_pos, term.y_pos + 1).unwrap();
        }

        // Render Options
        let pos_1 = (x as f64 / 2.5) as u16;
        term.set_cursor_to(pos_1, term.y_pos + 2).unwrap();
        term.print(" \u{f002}  Find File\t\tSPC f").unwrap();

        term.set_cursor_to(pos_1, term.y_pos + 2).unwrap();
        term.print(" \u{f1fc}  Change Color\t\tSPC c").unwrap();

        term.set_cursor_to(pos_1, term.y_pos + 2).unwrap();
        term.print(" \u{f15c}  Live Grep\t\tSPC g").unwrap();

        term.set_cursor_to((x as f64 / 2.19) as u16, term.y_pos + 3)
            .unwrap();
        term.print("Loaded 1 Plugin").unwrap();

        // End
        term.set_cursor_to(pos_1 + 4, term.y_pos - 7).unwrap();
        term.show_cursor().unwrap();

        Ok(())
    }

    fn handle_key<T: Write>(
        &mut self,
        term: &mut Terminal<T>,
        keys: KeyIterator,
    ) -> super::ZedError {
        for key in keys.clone() {
            match key {
                Key::Ctrl('q') => {
                    self.destroy(term).unwrap();
                    return Ok(());
                }
                Key::Down | Key::Char('j') => {
                    if self.selected_option != 3 {
                        term.set_cursor_to(
                            (term.get_size().0 as f64 / 2.5) as u16 + 4,
                            term.y_pos + 2,
                        )
                        .unwrap();
                        self.selected_option += 1;
                    }
                    continue;
                }
                Key::Up | Key::Char('k') => {
                    if self.selected_option != 1 {
                        term.set_cursor_to(
                            (term.get_size().0 as f64 / 2.5) as u16 + 4,
                            term.y_pos - 2,
                        )
                        .unwrap();
                        self.selected_option -= 1;
                    }
                    continue;
                }

                // User Space Bindings
                Key::Char(' ') => match keys.clone().next() {
                    Some(x) => match x {
                        Key::Char('f') => {
                            finder::finder(term, vec![1, 2, 3].iter());
                            continue;
                        }
                        Key::Char('c') => {
                            continue;
                        }
                        Key::Char('g') => {
                            continue;
                        }
                        _ => continue,
                    },
                    None => continue,
                },
                _ => continue,
            }
        }
        Ok(())
    }
}
