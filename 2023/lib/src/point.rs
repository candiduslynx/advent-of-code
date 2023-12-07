#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Eq + Ord + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + From<i32>> Point<T> {
    pub fn neighbors(&self) -> [Point<T>; 8] {
        let one: T = T::from(1);

        [
            Point { x: self.x - one, y: self.y - one },
            Point { x: self.x - one, y: self.y },
            Point { x: self.x - one, y: self.y + one },
            Point { x: self.x, y: self.y - one },
            Point { x: self.x, y: self.y + one },
            Point { x: self.x + one, y: self.y - one },
            Point { x: self.x + one, y: self.y },
            Point { x: self.x + one, y: self.y + one },
        ]
    }

    pub fn above(&self) -> Point<T> {
        let one: T = T::from(1);

        Point { x: self.x - one, y: self.y }
    }

    pub fn neighbors_above(&self) -> [Point<T>; 3] {
        let one: T = T::from(1);

        [
            Point { x: self.x - one, y: self.y - one },
            Point { x: self.x - one, y: self.y },
            Point { x: self.x - one, y: self.y + one },
        ]
    }

    pub fn below(&self) -> Point<T> {
        let one: T = T::from(1);

        Point { x: self.x + one, y: self.y }
    }

    pub fn neighbors_below(&self) -> [Point<T>; 3] {
        let one: T = T::from(1);

        [
            Point { x: self.x + one, y: self.y - one },
            Point { x: self.x + one, y: self.y },
            Point { x: self.x + one, y: self.y + one },
        ]
    }

    pub fn left(&self) -> Point<T> {
        let one: T = T::from(1);

        Point { x: self.x, y: self.y - one }
    }

    pub fn neighbors_left(&self) -> [Point<T>; 3] {
        let one: T = T::from(1);

        [
            Point { x: self.x - one, y: self.y - one },
            Point { x: self.x, y: self.y - one },
            Point { x: self.x + one, y: self.y - one },
        ]
    }

    pub fn right(&self) -> Point<T> {
        let one: T = T::from(1);

        Point { x: self.x, y: self.y + one }
    }

    pub fn neighbors_right(&self) -> [Point<T>; 3] {
        let one: T = T::from(1);

        [
            Point { x: self.x - one, y: self.y + one },
            Point { x: self.x, y: self.y + one },
            Point { x: self.x + one, y: self.y + one },
        ]
    }

    pub fn is_valid(&self, exclusive_max_x: T, exclusive_max_y: T) -> bool {
        let zero: T = T::from(0);
        self.x >= zero && self.x < exclusive_max_x && self.y >= zero && self.y < exclusive_max_y
    }
}