#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Eq + Ord> Point<T> {
    pub fn neighbors(&self) -> [Point; 8] {
        [
            Point { x: self.x - 1, y: self.y - 1 },
            Point { x: self.x - 1, y: self.y },
            Point { x: self.x - 1, y: self.y + 1 },
            Point { x: self.x, y: self.y - 1 },
            Point { x: self.x, y: self.y + 1 },
            Point { x: self.x + 1, y: self.y - 1 },
            Point { x: self.x + 1, y: self.y },
            Point { x: self.x + 1, y: self.y + 1 },
        ]
    }

    pub fn above(&self) -> Point { Point { x: self.x - 1, y: self.y } }
    pub fn neighbors_above(&self) -> [Point; 3] {
        [
            Point { x: self.x - 1, y: self.y - 1 },
            Point { x: self.x - 1, y: self.y },
            Point { x: self.x - 1, y: self.y + 1 },
        ]
    }

    pub fn below(&self) -> Point { Point { x: self.x + 1, y: self.y } }
    pub fn neighbors_below(&self) -> [Point; 3] {
        [
            Point { x: self.x + 1, y: self.y - 1 },
            Point { x: self.x + 1, y: self.y },
            Point { x: self.x + 1, y: self.y + 1 },
        ]
    }

    pub fn left(&self) -> Point { Point { x: self.x, y: self.y - 1 } }
    pub fn neighbors_left(&self) -> [Point; 3] {
        [
            Point { x: self.x - 1, y: self.y - 1 },
            Point { x: self.x, y: self.y - 1 },
            Point { x: self.x + 1, y: self.y - 1 },
        ]
    }

    pub fn right(&self) -> Point { Point { x: self.x, y: self.y + 1 } }
    pub fn neighbors_right(&self) -> [Point; 3] {
        [
            Point { x: self.x - 1, y: self.y + 1 },
            Point { x: self.x, y: self.y + 1 },
            Point { x: self.x + 1, y: self.y + 1 },
        ]
    }

    pub fn is_valid(&self, exclusive_max_x: T, exclusive_max_y: T) -> bool {
        self.x >= 0 && self.x < exclusive_max_x && self.y >= 0 && self.y < exclusive_max_y
    }
}