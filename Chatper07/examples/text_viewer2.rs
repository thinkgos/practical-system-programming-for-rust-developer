use std::env::args;
use std::fs;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};
struct TextViewer {
    doc: Doc,
    doc_length: usize,
    cur_pos: Coordinates,
    terminal_size: Coordinates,
    file_name: String,
}
impl TextViewer {
    fn init(file_name: &str) -> Self {
        let mut doc_file = Doc { lines: vec![] };
        let file_handle = fs::read_to_string(file_name).unwrap();
        for doc_line in file_handle.lines() {
            doc_file.lines.push(doc_line.to_string());
        }
        let mut doc_length = file_handle.lines().count();
        if doc_length == 0 {
            doc_file.lines.push("".into());
            doc_length += 1;
        }
        let size = termion::terminal_size().unwrap();
        Self {
            doc: doc_file,
            cur_pos: Coordinates {
                x: 1,
                y: doc_length,
            },
            doc_length: doc_length,
            terminal_size: Coordinates {
                x: size.0 as usize,
                y: size.1 as usize,
            },
            file_name: file_name.into(),
        }
    }

    fn show_document(&mut self) {
        let pos = &self.cur_pos;
        let (old_x, old_y) = (pos.x, pos.y);
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        println!(
            "{}{}Welcome to Super text viewer\r{}",
            color::Bg(color::Black),
            color::Fg(color::White),
            style::Reset
        );
        if self.doc_length < self.terminal_size.y {
            for line in 0..self.doc_length {
                println!("{}\r", self.doc.lines[line as usize]);
            }
        } else {
            if pos.y <= self.terminal_size.y {
                for line in 0..self.terminal_size.y - 3 {
                    println!("{}\r", self.doc.lines[line as usize]);
                }
            } else {
                for line in pos.y - (self.terminal_size.y - 3)..pos.y {
                    println!("{}\r", self.doc.lines[line as usize]);
                }
            }
        }
        println!(
            "{}",
            termion::cursor::Goto(0, (self.terminal_size.y - 2) as u16),
        );
        println!(
            "{}X={},Y={}, line-count={} Filename: {}{}",
            color::Fg(color::Red),
            old_x,
            old_y,
            self.doc_length,
            self.file_name,
            style::Reset
        );
        self.set_pos(old_x, old_y);
    }
    fn set_pos(&mut self, x: usize, y: usize) {
        self.cur_pos.x = x;
        self.cur_pos.y = y;
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, (self.cur_pos.y) as u16)
        );
    }
    fn inc_x(&mut self) {
        if self.cur_pos.x < self.terminal_size.x {
            self.cur_pos.x += 1;
        }
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }
    fn dec_x(&mut self) {
        if self.cur_pos.x > 1 {
            self.cur_pos.x -= 1;
        }
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }
    fn inc_y(&mut self) {
        if self.cur_pos.y < self.doc_length {
            self.cur_pos.y += 1;
        }

        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }
    fn dec_y(&mut self) {
        if self.cur_pos.y > 1 {
            self.cur_pos.y -= 1;
        }
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }

    fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => {
                    break;
                }
                Key::Left => {
                    self.dec_x();
                    self.show_document();
                }
                Key::Right => {
                    self.inc_x();
                    self.show_document();
                }
                Key::Up => {
                    self.dec_y();
                    self.show_document();
                }
                Key::Down => {
                    self.inc_y();
                    self.show_document();
                }
                Key::Backspace => {
                    self.dec_x();
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }
}
struct Doc {
    lines: Vec<String>,
}
#[derive(Debug)]
struct Coordinates {
    pub x: usize,
    pub y: usize,
}
fn main() {
    //Get arguments from command line
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Please provide file name as argument");
        std::process::exit(0);
    }
    //Check if file exists. If not, print error message and exit process
    if !std::path::Path::new(&args[1]).exists() {
        println!("File does not exist");
        std::process::exit(0);
    }
    // Clear full screen
    println!("{}", termion::clear::All);
    // Open file & load into struct
    println!("{}", termion::cursor::Show);
    println!("{}", termion::cursor::Goto(1, 1));
    // Initialize editor
    let mut editor = TextViewer::init(&args[1]);
    editor.set_pos(1, 1);
    editor.show_document();
    editor.run();
}
