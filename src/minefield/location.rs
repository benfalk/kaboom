use super::point::Point;

#[derive(Debug)]
pub struct Location {
    pub point: Point,
    pub has_bomb: bool,
    pub surrounding_bomb_count: u8,
    pub status: Status,
}

#[derive(Debug)]
pub enum Status {
    Covered,
    Flagged,
    Uncovered,
}

impl Location {
    pub fn new(point: Point) -> Self {
        Self {
            point,
            has_bomb: false,
            surrounding_bomb_count: 0,
            status: Status::Covered,
        }
    }
}
