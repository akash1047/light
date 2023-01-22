use std::io::Write;
use std::io::{stdin, stdout};

use termion::cursor;
use termion::{
    cursor::Goto,
    event::{Event, Key::*, MouseButton, MouseEvent},
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::IntoAlternateScreen,
};

fn main() {
    println!("press a key to start");

    let mut screen = MouseTerminal::from(
        stdout()
            .into_raw_mode()
            .and_then(IntoAlternateScreen::into_alternate_screen)
            .unwrap(),
    );

    if let Err(_) = write!(screen, "{}", cursor::Hide) {
        return;
    }

    for event in stdin().events() {
        let event = match event {
            Ok(e) => e,
            Err(_) => {
                break;
            }
        };

        match event {
            Event::Key(Char('q')) => {
                break;
            }

            Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y)) => {
                if write!(screen, "{}x", Goto(x, y)).is_err() {
                    break;
                }
            }

            Event::Mouse(MouseEvent::Hold(x, y)) => {
                if write!(screen, "{}x", Goto(x, y)).is_err() {
                    break;
                }
            }

            _ => {}
        }

        if write!(screen, "{}press q to exit", Goto(1, 1)).is_err() {
            break;
        }

        if screen.flush().is_err() {
            break;
        }
    }

    if let Err(_) = write!(screen, "{}", cursor::Show) {
        // well have to restart terminal to bring back cursor.
    }
}
