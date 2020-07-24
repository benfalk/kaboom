#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn surrounding_points(&self) -> [Self; 8] {
        [
            Self { x: self.x - 1, ..*self },
            Self { x: self.x - 1, y: self.y - 1 },
            Self { y: self.y - 1, ..*self },
            Self { x: self.x + 1, y: self.y - 1 },
            Self { x: self.x + 1, ..*self },
            Self { x: self.x + 1, y: self.y + 1 },
            Self { y: self.y + 1, ..*self },
            Self { x: self.x - 1, y: self.y + 1 },
        ]
    }
}
