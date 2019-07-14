mod map;
mod ui;
use ui::{Color, Key, UI};

enum CommandResult {
    Ok,
    Ignored,
    Next(usize),
    Quit,
}

trait Level {
    fn get_buffer(&self, width: usize, height: usize) -> ui::Buffer;
    fn execute(&mut self, command: ui::Key) -> CommandResult;
}

struct WelcomeLevel {}

impl Level for WelcomeLevel {
    fn get_buffer(&self, width: usize, height: usize) -> ui::Buffer {
        let mut buffer = ui::new_buffer(width, height);
        buffer.print_centered("10K Types of Oatmeal", -1);
        buffer.print_centered("('n' for next, 'q' to quit)", 1);
        buffer
    }

    fn execute(&mut self, command: ui::Key) -> CommandResult {
        match command {
            ui::Key::Char('n') => CommandResult::Next(1),
            ui::Key::Char('q') => CommandResult::Quit,
            _ => CommandResult::Ignored,
        }
    }
}

struct DungeonLevel {
    level_index: usize,
    entities: Vec<Entity>,
    current_map: Vec<Vec<usize>>,
    player_col: usize,
    player_row: usize,
}

impl Level for DungeonLevel {
    fn get_buffer(&self, width: usize, height: usize) -> ui::Buffer {
        let mut buffer = ui::new_buffer(width, height);
        for row in 0..height {
            for col in 0..width {
                let entity = self.lookup_entity(row, col);
                buffer.print_symbol(row, col, entity.symbol, entity.color, Color::Black);
            }
        }
        buffer
    }

    fn execute(&mut self, command: ui::Key) -> CommandResult {
        let mut player_col = self.player_col;
        let mut player_row = self.player_row;

        let go = match command {
            Key::Esc | Key::Char('q') => CommandResult::Next(self.level_index + 1),
            Key::Right => {
                player_col += 1;
                CommandResult::Ok
            }
            Key::Left => {
                player_col -= 1;
                CommandResult::Ok
            }
            Key::Down => {
                player_row += 1;
                CommandResult::Ok
            }
            Key::Up => {
                player_row -= 1;
                CommandResult::Ok
            }
            _ => CommandResult::Ignored,
        };

        if let CommandResult::Ok = go {
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
}

impl DungeonLevel {
    fn lookup_entity(&self, row: usize, col: usize) -> &Entity {
        let entity_index = self.current_map[row][col];
        &self.entities[entity_index]
    }
    fn new(level: usize, width: usize, height: usize) -> DungeonLevel {
        let mut map_builder = map::new_builder_with_size(width, height);
        map_builder.wall_rect(0, 0, 5, 5);
        map_builder.wall_rect(0, 10, 25, 10);
        map_builder.wall_line(2, 5, true, 5);
        map_builder.wall_line(4, 5, true, 5);
        map_builder.clear(3, 4);
        map_builder.clear(3, 10);

        let mut map = map_builder.build();
        map[1][1] = 1;

        DungeonLevel {
            level_index: level,
            entities: make_entities(),
            current_map: map,
            player_col: 1,
            player_row: 1,
        }
    }
}

struct FarewellLevel {}

impl Level for FarewellLevel {
    fn get_buffer(&self, width: usize, height: usize) -> ui::Buffer {
        let mut buffer = ui::new_buffer(width, height);
        buffer.print_centered("Goodbye!", -1);
        buffer.print_centered("(Press any key to quit)", 1);
        buffer
    }

    fn execute(&mut self, _command: ui::Key) -> CommandResult {
        CommandResult::Quit
    }
}

struct Entity {
    name: String,
    symbol: char,
    color: Color,
    blocks: bool,
}

struct Game {
    screen_width: usize,
    screen_height: usize,
    ui: UI,
}

impl Game {
    fn run(&mut self) {
        let mut index = 0;
        while let Some(level) = self.next_level(index) {
            index = self.run_level(level);
        }
    }

    fn run_level(&mut self, mut level: Box<Level>) -> usize {
        loop {
            self.ui
                .render(&level.get_buffer(self.screen_width, self.screen_height));

            let next_index = loop {
                match level.execute(self.ui.read_key()) {
                    CommandResult::Ok => break 0,
                    CommandResult::Ignored => (),
                    CommandResult::Next(index) => break index,
                    CommandResult::Quit => break 1000,
                }
            };

            if next_index > 0 {
                break next_index;
            }
        }
    }

    fn next_level(&self, level: usize) -> Option<Box<Level>> {
        match level {
            0 => Some(Box::new(WelcomeLevel {})),
            1 => Some(Box::new(DungeonLevel::new(
                level,
                self.screen_width,
                self.screen_height,
            ))),
            2 => Some(Box::new(FarewellLevel {})),
            _ => None,
        }
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
        color: Color::Red,
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

fn new_game(width: usize, height: usize) -> Game {
    Game {
        screen_width: width,
        screen_height: height,
        ui: ui::new_ui(),
    }
}

fn main() {
    // User can specify the terminal size, but no fancy resizing for now.
    let mut width = 80;
    let mut height = 24;

    let args: Vec<String> = std::env::args().collect();
    if let Some(value) = args.get(1) {
        if let Ok(num) = value.trim().parse::<usize>() {
            width = num;
        }
    }

    if let Some(value) = args.get(2) {
        if let Ok(num) = value.trim().parse::<usize>() {
            height = num;
        }
    }

    let mut game = new_game(width, height);
    game.run();
}
