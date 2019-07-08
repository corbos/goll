pub const EMPTY: usize = 0;
pub const HERO: usize = 1;
pub const WALL: usize = 2;

pub struct MapBuilder {
    map: Vec<Vec<usize>>,
}

impl MapBuilder {
    pub fn wall_rect(&mut self, x: usize, y: usize, width: usize, height: usize) {
        for current_x in 0..width {
            self.map[y][x + current_x] = WALL;
            self.map[y + height - 1][x + current_x] = WALL;
        }
        for current_y in 0..height {
            self.map[y + current_y][x] = WALL;
            self.map[y + current_y][x + width - 1] = WALL;
        }
    }

    pub fn wall_line(&mut self, x: usize, y: usize, is_vertical: bool, length: usize) {
        for current in 0..length {
            self.map[y + if is_vertical { current } else { 0 }]
                [x + if is_vertical { 0 } else { current }] = WALL;
        }
    }

    pub fn clear(&mut self, x: usize, y: usize) {
        self.map[y][x] = EMPTY;
    }

    pub fn build(&self) -> Vec<Vec<usize>> {
        self.map.clone()
    }
}

pub fn new_builder_with_size(width: usize, height: usize) -> MapBuilder {
    let mut map = Vec::with_capacity(height);
    for _row in 0..height {
        let mut row = Vec::with_capacity(width);
        for _col in 0..width {
            row.push(0);
        }
        map.push(row);
    }
    MapBuilder { map }
}
