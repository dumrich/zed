use crate::cli::Target;
use crate::error::Error;
use zui_core::style::{self, Style};

use super::finder::FileFinder;
use super::Component;
use std::convert::TryFrom;
use std::io::Write;
use std::path::PathBuf;
use zui_core::color::{self, Color};
use zui_core::key::{Key, KeyIterator};
use zui_core::term::Terminal;

#[derive(Debug, Clone)]
pub struct Dashboard {
    pub banner: &'static str,
    pub dir: PathBuf,
    selected_option: u8,
}

impl Dashboard {
    pub fn set_dir(mut self, p: PathBuf) -> Dashboard {
        self.dir = p;
        self
    }
}

impl Component for Dashboard {
    type Widget = Dashboard;

    // WidgetReturn
    type WidgetReturn = Target;

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
            dir: PathBuf::new(),
            selected_option: 1,
        }
    }

    fn destroy<T: Write>(&mut self, term: &mut Terminal<T>) -> super::ZedError {
        Ok(())
    }

    // Fix all these unwraps
    fn view<T: Write>(&mut self, term: &mut Terminal<T>) -> super::ZedError {
        // Inital values
        let (x, y) = term.get_size(); // TODO: Fix this
                                      // Setup Rendering
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

        // TODO: Fix these
        term.set_cursor_to(pos_1, term.y_pos + 2).unwrap();
        let o_string1 = format!(
            "{} \u{f002}  Find File{}\t\tSPC f{}",
            color::fg(Color::GreenLight),
            color::fg(Color::YellowLight),
            color::fg(Color::Reset)
        );
        term.print(&o_string1).unwrap();

        term.set_cursor_to(pos_1, term.y_pos + 2).unwrap();
        let o_string2 = format!(
            "{} \u{f1fc}  Change Color{}\t\tSPC c{}",
            color::fg(Color::GreenLight),
            color::fg(Color::YellowLight),
            color::fg(Color::Reset)
        );
        term.print(&o_string2).unwrap();

        term.set_cursor_to(pos_1, term.y_pos + 2).unwrap();
        let o_string3 = format!(
            "{} \u{f15c}  Live Grep{}\t\tSPC g{}",
            color::fg(Color::GreenLight),
            color::fg(Color::YellowLight),
            color::fg(Color::Reset)
        );
        term.print(&o_string3).unwrap();

        // Custom Message
        term.set_cursor_to((x as f64 / 2.19) as u16, term.y_pos + 3)
            .unwrap();
        let o_string4 = format!(
            "{}\u{f004}  {}{}by{}{} dumrich{}",
            color::fg(Color::Red),
            color::fg(Color::Reset),
            style::set(Style::Bold),
            style::set(Style::Reset),
            color::fg(Color::Purple),
            color::fg(Color::Reset)
        );
        term.print(&o_string4).unwrap();

        // End Render
        term.set_cursor_to(pos_1 + 4, term.y_pos - 7).unwrap();
        term.show_cursor().unwrap();

        Ok(())
    }

    fn handle_key<T: Write>(
        &mut self,
        term: &mut Terminal<T>,
        keys: KeyIterator,
    ) -> Result<Self::WidgetReturn, Error> {
        for key in keys.clone() {
            match key {
                Key::Ctrl('q') => {
                    // Destroy dashboard
                    self.destroy(term).unwrap();
                    return Ok(Target::Empty);
                }
                Key::Enter | Key::Tab => match self.selected_option {
                    1 => {
                        // Render FileFinder
                        term.clear_screen().unwrap();
                        let mut finder = FileFinder::new().set_dir(self.dir.clone());
                        let f = finder.render(term, keys.clone());
                        if let Ok(Target::File(x)) = f {
                            return Ok(Target::File(x));
                        }
                        self.view(term).unwrap();
                    }
                    _ => continue,
                },
                Key::Down | Key::Char('j') => {
                    // Store each option as number
                    if self.selected_option != 3 {
                        term.set_cursor_to(
                            (term.get_size().0 as f64 / 2.5) as u16 + 4,
                            term.y_pos + 2,
                        )
                        .unwrap();
                        // Change the selected Option
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
                        // Change the selected Option
                        self.selected_option -= 1;
                    }
                }

                // User Space Bindings
                Key::Char(' ') => match keys.clone().next() {
                    // If the next key...
                    Some(x) => match x {
                        // Render the File Finder
                        Key::Char('f') => {
                            // Render FileFinder
                            term.clear_screen().unwrap();
                            let mut finder = FileFinder::new().set_dir(self.dir.clone());
                            let f = finder.render(term, keys.clone());
                            if let Ok(Target::File(x)) = f {
                                return Ok(Target::File(x));
                            }
                            self.view(term).unwrap();
                        }
                        // Render the color switcher
                        Key::Char('c') => {
                            continue;
                        }
                        // Live Grepper
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
        Ok(Target::Empty)
    }

    fn render<T: Write>(
        &mut self,
        term: &mut Terminal<T>,
        keys: KeyIterator,
    ) -> Result<Self::WidgetReturn, Error> {
        self.view(term).unwrap();

        match self.handle_key(term, keys.clone()) {
            Ok(s) => match s {
                Target::File(x) => return Ok(Target::File(x)),
                _ => return Ok(Target::Empty),
            },
            Err(s) => Err(s),
        }
    }
}
