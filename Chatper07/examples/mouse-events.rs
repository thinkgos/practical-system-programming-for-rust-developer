use std::io::{self, Write};
use termion::cursor::{self, DetectCursorPos};
use termion::event::*;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

fn main() {
    let stdin = io::stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    writeln!(
        stdout,
        "{}{} Type q to exit.",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(m) => match m {
                MouseEvent::Press(_, a, b) | MouseEvent::Release(a, b) | MouseEvent::Hold(a, b) => {
                    write!(stdout, "{}", cursor::Goto(a, b)).unwrap();
                    let (x, y) = stdout.cursor_pos().unwrap();
                    write!(
                        stdout,
                        "{}{}Cursor is at: ({},{}){}",
                        cursor::Goto(5, 5),
                        termion::clear::UntilNewline,
                        x,
                        y,
                        cursor::Goto(a, b)
                    )
                    .unwrap();
                }
            },
            _ => {}
        }

        stdout.flush().unwrap();
    }
}
