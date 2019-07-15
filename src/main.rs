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
    frame_top: usize,
    frame_left: usize,
    buffer_width: usize,
    buffer_height: usize,
    player_col: usize,
    player_row: usize,
    looking: bool,
    look_row: usize,
    look_col: usize,
}

impl Level for DungeonLevel {
    fn get_buffer(&self, width: usize, height: usize) -> ui::Buffer {
        let mut buffer = ui::new_buffer(width, height);
        for row in 0..height {
            for col in 0..width {
                let entity = self.lookup_entity(row + self.frame_top, col + self.frame_left);
                buffer.print_symbol(row, col, entity.symbol, entity.color, Color::Black);
            }
        }
        if self.looking {
            let entity = self.lookup_entity(
                self.look_row + self.frame_top,
                self.look_col + self.frame_left,
            );
            buffer.print_symbol(
                self.look_row,
                self.look_col,
                entity.symbol,
                entity.color,
                Color::Yellow,
            );
            buffer.print_lower_left(entity.name);
        }
        buffer
    }

    fn execute(&mut self, command: ui::Key) -> CommandResult {
        if self.looking {
            self.look(command)
        } else {
            self.move_player(command)
        }
    }
}

impl DungeonLevel {
    fn lookup_entity(&self, row: usize, col: usize) -> &Entity {
        let entity_index = self.current_map[row][col];
        &self.entities[entity_index]
    }
    fn move_player(&mut self, command: ui::Key) -> CommandResult {
        let mut player_col = self.player_col;
        let mut player_row = self.player_row;

        let go = match command {
            Key::Esc | Key::Char('q') => CommandResult::Next(self.level_index + 1),
            Key::Right => {
                player_col += 1;
                if player_col > self.buffer_width + self.frame_left - 2 {
                    self.frame_left = self.frame_left + 1;
                }
                CommandResult::Ok
            }
            Key::Left => {
                player_col -= 1;
                if self.frame_left > 0 && player_col < self.frame_left + 1 {
                    self.frame_left = self.frame_left - 1;
                }
                CommandResult::Ok
            }
            Key::Down => {
                player_row += 1;
                if player_row > self.buffer_height + self.frame_top - 2 {
                    self.frame_top = self.frame_top + 1;
                }
                CommandResult::Ok
            }
            Key::Up => {
                player_row -= 1;
                if self.frame_top > 0 && player_row < self.frame_top + 1 {
                    self.frame_top = self.frame_top - 1;
                }
                CommandResult::Ok
            }
            Key::Char('l') => {
                self.looking = true;
                self.look_row = self.player_row;
                self.look_col = self.player_col;
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

    fn look(&mut self, command: ui::Key) -> CommandResult {
        match command {
            Key::Char('l') | Key::Esc => {
                self.looking = false;
                CommandResult::Ok
            }
            Key::Left => {
                self.look_col = self.look_col - 1;
                CommandResult::Ok
            }
            Key::Right => {
                self.look_col = self.look_col + 1;
                CommandResult::Ok
            }
            Key::Up => {
                self.look_row = self.look_row - 1;
                CommandResult::Ok
            }
            Key::Down => {
                self.look_row = self.look_row + 1;
                CommandResult::Ok
            }
            _ => CommandResult::Ignored,
        }
    }
    fn new(level: usize, buffer_width: usize, buffer_height: usize) -> DungeonLevel {
        let mut map_builder = map::new_builder(1000, 1000, map::WALL);

        for y in 0..100 {
            for x in 0..100 {
                map_builder.carve_out_rect(x * 10 + 1, y * 10 + 1, 8, 8);
            }
            map_builder.carve_out_line(2, y * 10 + 5, false, 900);
            map_builder.carve_out_line(y * 10 + 5, 2, true, 900);
        }

        // map_builder.wall_rect(0, 0, 5, 5);
        // map_builder.wall_rect(0, 10, 25, 10);
        // map_builder.wall_line(2, 5, true, 5);
        // map_builder.wall_line(4, 5, true, 5);
        // map_builder.clear(3, 4);
        // map_builder.clear(3, 10);

        let mut map = map_builder.build();
        map[1][1] = 1;

        DungeonLevel {
            level_index: level,
            entities: make_entities(),
            current_map: map,
            frame_top: 0,
            frame_left: 0,
            buffer_width: buffer_width,
            buffer_height: buffer_height,
            player_col: 1,
            player_row: 1,
            looking: false,
            look_row: 0,
            look_col: 0,
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
    name: &'static str,
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
        name: "An Empty Space",
        symbol: ' ',
        color: Color::Black,
        blocks: false,
    });
    entities.push(Entity {
        name: "Our Hero",
        symbol: '@',
        color: Color::Red,
        blocks: false,
    });
    entities.push(Entity {
        name: "A Wall",
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
