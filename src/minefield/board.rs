use super::location::{
    Location,
    Status::*,
};
use super::point::Point;
use super::state::State;
use rand::seq::IteratorRandom;

#[derive(Debug)]
pub struct Board {
    locations: Vec<Location>,
    width: i32,
    height: i32,
    bomb_count: i32,
    tiles_remaining: usize,
    tiles_flagged: usize,
    state: State,
}

impl Board {
    pub fn new(width: i32, height: i32, bomb_count: i32) -> Result<Self, String> {

        if width < 2 || height < 2 {
            return Err(String::from("Width and Height must be larger then 1"));
        }

        if bomb_count < 1 {
            return Err(String::from("Bomb count cannot be zero or less"));
        }

        if bomb_count > width * height {
            return Err(String::from("Bomb count may not exceed board size"));
        }

        let mut board = Self {
            locations: Vec::with_capacity(width as usize * height as usize),
            width,
            height,
            bomb_count,
            tiles_remaining: (width * height) as usize,
            tiles_flagged: 0,
            state: State::default(),
        };

        board.init_locations();
        board.seed_bombs();

        Ok(board)
    }

    pub fn reset(&mut self) {
        for mut loc in self.locations.iter_mut() {
            loc.surrounding_bomb_count = 0;
            loc.status = Covered;
            loc.has_bomb = false;
        }
        self.seed_bombs();
        self.tiles_remaining = self.locations.len();
        self.tiles_flagged = 0;
        self.state = State::default();
    }

    pub fn bombs_remaining(&self) -> i32 {
        self.bomb_count - self.tiles_flagged as i32
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn width(&self) -> i32 { self.width }

    pub fn height(&self) -> i32 { self.height }

    pub fn location_at(&self, x: i32, y: i32) -> &Location {
        let index = self.index_for_point(&Point { x, y }).unwrap();
        &self.locations[index]
    }

    pub fn uncover_location_at(&mut self, x: i32, y: i32) {
        let index = self.index_for_point(&Point { x, y }).unwrap();

        if self.state.is_ready() {
            self.state = State::start();
        }

        match self.locations.get_mut(index).unwrap() {

            Location { status, .. }
            if status == &Uncovered || status == &Flagged =>
                return,

            location => {
                location.status = Uncovered;
                self.tiles_remaining -= 1;

                if location.has_bomb {
                    self.state = self.state.to_failure();
                } else if location.surrounding_bomb_count == 0 {
                    for point in location.point.surrounding_points().iter() {
                        if self.index_for_point(&point).is_some() {
                            self.uncover_location_at(point.x, point.y);
                        }
                    }
                }
            }
        }

        if self.tiles_remaining == self.bomb_count as usize {
            self.state = self.state.to_victory();
        }
    }

    pub fn toggle_flag(&mut self, x: i32, y: i32) {
        let index = self.index_for_point(&Point { x, y }).unwrap();
        let mut loc = self.locations.get_mut(index).unwrap();

        match loc {
            Location { status: Covered, .. } => {
                loc.status = Flagged;
                self.tiles_flagged += 1;
            },
            Location { status: Flagged, .. } => {
                loc.status = Covered;
                self.tiles_flagged -= 1;
            },
            _ => (),
        }
    }

    // Private Functions

    fn init_locations(&mut self) {
        for idx in self.location_index_range() {
            self.locations.push(
                Location::new(self.point_for_index(idx))
            );
        }
    }

    fn seed_bombs(&mut self) {
        for idx in self.random_bomb_location_indexes() {
            self.locations[idx].has_bomb = true;

            for point in self.locations[idx].point.surrounding_points().iter() {
                if let Some(i) = self.index_for_point(&point) {
                    self.locations[i].surrounding_bomb_count += 1;
                };
            }
        }
    }

    fn location_index_range(&self) -> std::ops::Range<usize> {
        0..(self.width as usize * self.height as usize)
    }

    fn point_for_index(&self, index: usize) -> Point {
        let index = index as i32;

        Point {
            x: index % self.width,
            y: index / self.width,
        }
    }

    fn random_bomb_location_indexes(&self) -> Vec<usize> {
        self.location_index_range()
            .choose_multiple(&mut rand::thread_rng(), self.bomb_count as usize)
    }

    fn index_for_point(&self, point: &Point) -> Option<usize> {
        if point.x < 0 || point.y < 0 ||
           point.x >= self.width || point.y >= self.height {
               return None;
        }

        Some(point.x as usize + ( point.y as usize * self.width as usize))
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut line = String::with_capacity(self.width as usize * 2 + 5);
        for _ in 0..self.width { line.push_str("--"); }
        writeln!(f, "{}", line)?;
        line.truncate(0);

        for (idx, loc) in self.locations.iter().enumerate() {
            line.push_str(match loc {
                Location { has_bomb: true, .. } => " x",
                Location { surrounding_bomb_count: 0, .. } => "  ",
                Location { surrounding_bomb_count: 1, .. } => " 1",
                Location { surrounding_bomb_count: 2, .. } => " 2",
                Location { surrounding_bomb_count: 3, .. } => " 3",
                Location { surrounding_bomb_count: 4, .. } => " 4",
                Location { surrounding_bomb_count: 5, .. } => " 5",
                Location { surrounding_bomb_count: 6, .. } => " 6",
                Location { surrounding_bomb_count: 7, .. } => " 7",
                Location { surrounding_bomb_count: 8, .. } => " 8",
                _ => " ?"
            });

            if ( idx + 1 ) % self.width as usize == 0 {
                writeln!(f, "{}", line)?;
                line.truncate(0);
            }
        }

        for _ in 0..self.width { line.push_str("--"); }
        writeln!(f, "{}", line)
    }
}
