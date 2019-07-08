mod map;

use std::default::Default;
use std::error::Error;

use rustbox::{Color, Event, Key, RustBox};

struct Entity {
    name: String,
    symbol: char,
    color: Color,
    blocks: bool,
}

struct Game {
    screen: rustbox::RustBox,
    entities: Vec<Entity>,
    current_map: Vec<Vec<usize>>,
    player_col: usize,
    player_row: usize,
}

impl Game {
    fn run(&mut self) {
        if self.welcome() {
            return ();
        }

        self.current_map[self.player_row][self.player_col] = 1;

        while self.step() {}
    }

    fn step(&mut self) -> bool {
        self.render();

        match self.screen.poll_event(false) {
            Ok(Event::KeyEvent(key)) => self.handle_key(key),
            Err(e) => panic!("{}", e.description()),
            _ => true,
        }
    }

    fn welcome(&mut self) -> bool {
        self.print_centered(11, "10K Types of Oatmeal");
        self.print_centered(13, "('n' for next, 'q' to quit)");
        self.screen.present();

        loop {
            match self.screen.poll_event(false) {
                Ok(Event::KeyEvent(key)) => match key {
                    Key::Char('n') => break false,
                    Key::Char('q') => break true,
                    _ => (),
                },
                Err(e) => panic!("{}", e.description()),
                _ => (),
            }
        }
    }

    fn print_centered(&mut self, row: usize, message: &str) {
        let x = (self.screen.width() / 2) - (message.len() / 2);
        self.screen.print(
            x,
            row,
            rustbox::RB_NORMAL,
            Color::White,
            Color::Black,
            message,
        );
    }

    fn handle_key(&mut self, key: Key) -> bool {
        let mut player_col = self.player_col;
        let mut player_row = self.player_row;

        let go = match key {
            Key::Esc | Key::Char('q') => false,
            Key::Right => {
                player_col += 1;
                true
            }
            Key::Left => {
                player_col -= 1;
                true
            }
            Key::Down => {
                player_row += 1;
                true
            }
            Key::Up => {
                player_row -= 1;
                true
            }
            _ => true,
        };

        if go {
            let entity = self.lookup_entity(player_row, player_col);
            if !entity.blocks {
                self.current_map[self.player_row][self.player_col] = map::EMPTY;
                self.current_map[player_row][player_col] = map::HERO;
                self.player_col = player_col;
                self.player_row = player_row;
            }
        }

        go
    }

    fn lookup_entity(&self, row: usize, col: usize) -> &Entity {
        let entity_index = self.current_map[row][col];
        &self.entities[entity_index]
    }

    fn render(&self) {
        self.screen.clear();
        for row in 0..self.screen.height() {
            for col in 0..self.screen.width() {
                let entity = self.lookup_entity(row, col);
                self.screen.print_char(
                    col,
                    row,
                    rustbox::RB_NORMAL,
                    entity.color,
                    Color::Black,
                    entity.symbol,
                );
            }
        }
        self.screen.present();
    }
}

fn make_entities() -> Vec<Entity> {
    let mut entities = Vec::new();
    entities.push(Entity {
        name: "Empty".to_string(),
        symbol: ' ',
        color: Color::Black,
        blocks: false,
    });
    entities.push(Entity {
        name: "Hero".to_string(),
        symbol: '@',
        color: Color::White,
        blocks: false,
    });
    entities.push(Entity {
        name: "Wall".to_string(),
        symbol: '#',
        color: Color::White,
        blocks: true,
    });
    entities
}

fn new_game() -> Game {
    let screen = RustBox::init(Default::default()).expect("couldn't init rustbox");
    let width = screen.width();
    let height = screen.height();

    let mut map_builder = map::new_builder_with_size(width, height);
    map_builder.wall_rect(0, 0, 5, 5);
    map_builder.wall_rect(0, 10, 25, 10);
    map_builder.wall_line(2, 5, true, 5);
    map_builder.wall_line(4, 5, true, 5);
    map_builder.clear(3, 4);
    map_builder.clear(3, 10);

    Game {
        screen: screen,
        entities: make_entities(),
        current_map: map_builder.build(),
        player_col: 1,
        player_row: 1,
    }
}

fn main() {
    let mut game = new_game();
    game.run();
}
