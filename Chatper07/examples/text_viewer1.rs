use std::env::args;
use std::fs;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};

#[derive(Debug)]
struct Doc {
    lines: Vec<String>,
}

#[derive(Debug)]
struct Coordinates {
    pub x: usize,
    pub y: usize,
}

struct TextViewer {
    file_name: String,
    doc: Doc,
    cur_pos: Coordinates,
    terminal_size: Coordinates,
}

impl TextViewer {
    fn init(filename: &str) -> Self {
        let lines: Vec<_> = fs::read_to_string(filename)
            .unwrap()
            .lines()
            .map(|line| line.to_owned())
            .collect();

        let doc_length = lines.len();
        let size = termion::terminal_size().unwrap();
        Self {
            doc: Doc { lines },
            cur_pos: Coordinates {
                x: 1,
                y: doc_length,
            },
            terminal_size: Coordinates {
                x: size.0 as usize,
                y: size.1 as usize,
            },
            file_name: filename.into(),
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
        let doc_length = self.doc.lines.len();
        for line in 0..self.doc.lines.len() {
            println!("{}\r", self.doc.lines[line as usize]);
        }

        println!(
            "{}",
            termion::cursor::Goto(0, (self.terminal_size.y - 2) as u16),
        );
        println!(
            "{}{} line-count={} Filename: {}{}",
            color::Fg(color::Red),
            style::Bold,
            doc_length,
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

    fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => {
                    break;
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }
}

// 页眉(标题)栏
// 文本区域
// 页脚栏
// 查看文件并在终端上显示, 如果文件行数超过终端高度, 则可以滚动查看,
// 可以向上, 向下, 向左, 向右等按键进行滚动查看
// Ctrl + Q 退出文本查看器

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
    // Open file & load into struct
    println!("{}", termion::cursor::Show);
    // Initialize viewer
    let mut viewer = TextViewer::init(&args[1]);
    viewer.show_document();
    viewer.run();
}
