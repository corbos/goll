//! `ui` hides the concrete details of our console/terminal interface.
//! Currently it's crossterm,
//! This abstraction will let us change our mind.
const SPACE: char = ' ';
const RESET: &str = "\x1b[0m";
const FG_BLACK: &str = "\x1b[30m";
const FG_RED: &str = "\x1b[31m";
const FG_GREEEN: &str = "\x1b[32m";
const FG_YELLOW: &str = "\x1b[33m";
const FG_WHITE: &str = "\x1b[37m";

const BG_BLACK: &str = "\x1b[40m";
const BG_YELLOW: &str = "\x1b[43m";

pub enum Key {
    Char(char),
    Ctrl(char),
    Alt(char),
    Esc,
    Left,
    Right,
    Up,
    Down,
    Unknown,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    White,
    Unknown,
}

impl Color {
    fn fg(&self) -> &str {
        match self {
            Color::Red => FG_RED,
            Color::White => FG_WHITE,
            _ => FG_BLACK,
        }
    }
    fn bg(&self) -> &str {
        match self {
            Color::Yellow => BG_YELLOW,
            _ => BG_BLACK,
        }
    }
}

struct Cell {
    symbol: char,
    fg: Color,
    bg: Color,
}

pub struct Buffer {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Buffer {
    pub fn print_centered(&mut self, message: &str, delta_off_horizontal_center: i32) {
        let mut x = (self.width / 2) - (message.len() / 2);
        let y = (self.height as i32 / 2 + delta_off_horizontal_center) as usize;
        for c in message.chars() {
            self.print_symbol(y, x, c, Color::White, Color::Black);
            x = x + 1;
        }
    }

    pub fn print_lower_left(&mut self, message: &str) {
        let mut x = 0;
        for c in message.chars() {
            self.print_symbol(self.height - 1, x, c, Color::Black, Color::Yellow);
            x = x + 1;
        }
        self.print_symbol(self.height - 1, x, SPACE, Color::Black, Color::Yellow);
    }

    pub fn print_symbol(&mut self, row: usize, col: usize, c: char, fg: Color, bg: Color) {
        self.cells[self.width * row + col] = Cell {
            symbol: c,
            fg: fg,
            bg: bg,
        }
    }
}

pub fn new_buffer(width: usize, height: usize) -> Buffer {
    let mut cells = Vec::with_capacity(width * height);
    for _i in 0..(width * height) {
        cells.push(Cell {
            symbol: SPACE,
            fg: Color::Black,
            bg: Color::Black,
        });
    }
    Buffer {
        width: width,
        height: height,
        cells: cells,
    }
}

// UI manages user input and output to the user.
pub struct UI {
    _screen: crossterm::RawScreen,
    cursor: crossterm::TerminalCursor,
    stdin: crossterm::SyncReader,
}

impl Drop for UI {
    fn drop(&mut self) {
        crossterm::cursor().show().expect("couldn't show cursor");
    }
}

impl UI {
    pub fn read_key(&mut self) -> Key {
        use crossterm::KeyEvent::*;
        loop {
            if let Some(key_event) = self.stdin.next() {
                if let crossterm::InputEvent::Keyboard(evt) = key_event {
                    let result = match evt {
                        Ctrl(c) => Key::Ctrl(c),
                        Char(c) => Key::Char(c),
                        Alt(c) => Key::Alt(c),
                        Esc => Key::Esc,
                        Left => Key::Left,
                        Right => Key::Right,
                        Up => Key::Up,
                        Down => Key::Down,
                        _ => Key::Unknown,
                    };

                    if let Key::Unknown = result {
                        ()
                    } else {
                        break result;
                    }
                }
            }
        }
    }

    pub fn render(&self, buffer: &Buffer) {
        self.cursor.goto(0, 0).expect("couldn't goto(0,0)");
        let mut fg = Color::Unknown;
        let mut bg = Color::Unknown;

        let mut string_buffer = String::new();
        for cell in buffer.cells.iter() {
            if cell.fg != fg {
                string_buffer.push_str(cell.fg.fg());
                fg = cell.fg;
            }
            if cell.bg != bg {
                string_buffer.push_str(cell.bg.bg());
                bg = cell.bg;
            }
            string_buffer.push(cell.symbol);
        }
        string_buffer.push_str(RESET);
        print!("{}", string_buffer);
    }
}

pub fn new_ui() -> UI {
    // Don't let `screen` drop out of scope. If it does, raw mode goes away.
    let screen = crossterm::RawScreen::into_raw_mode().expect("couldn't into_raw_mode");
    let cursor = crossterm::cursor();
    let input = crossterm::input();
    let stdin = input.read_sync();

    cursor.hide().expect("couldn't hide cursor");

    UI {
        _screen: screen,
        cursor: cursor,
        stdin: stdin,
    }
}
