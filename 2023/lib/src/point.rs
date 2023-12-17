#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn ux(&self) -> usize {
        self.x as usize
    }

    pub fn uy(&self) -> usize {
        self.y as usize
    }

    pub fn from_coords(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
    pub fn coords(&self) -> (usize, usize) {
        (self.ux(), self.uy())
    }

    pub fn dst_flat(&self, other: &Self) -> usize {
        (self.x - other.x).abs() as usize + (self.y - other.y).abs() as usize
    }

    pub fn neighbors(&self) -> [Self; 8] {
        [
            Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }

    pub fn above(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn neighbors_above(&self) -> [Self; 3] {
        [
            Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
        ]
    }

    pub fn below(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn neighbors_below(&self) -> [Self; 3] {
        [
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }

    pub fn left(&self) -> Self {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn neighbors_left(&self) -> [Self; 3] {
        [
            Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
        ]
    }

    pub fn right(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn neighbors_right(&self) -> [Self; 3] {
        [
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }

    pub fn is_valid(&self, exclusive_max_x: usize, exclusive_max_y: usize) -> bool {
        self.x >= 0
            && self.x < exclusive_max_x.try_into().unwrap()
            && self.y >= 0
            && self.y < exclusive_max_y.try_into().unwrap()
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum Dir {
    L = 1,
    R = 2,
    U = 4,
    D = 8,
}
