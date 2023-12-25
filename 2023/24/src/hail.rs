use std::fs::read;
use std::io::BufRead;
use std::ops::Sub;
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug)]
struct Coord {
    x: f64,
    y: f64,
    z: f64,
}

impl From<&str> for Coord {
    fn from(s: &str) -> Self {
        let s: Vec<f64> = s
            .split(", ")
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        assert_eq!(s.len(), 3);
        Self {
            x: s[0],
            y: s[1],
            z: s[2],
        }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Coord {
    type Output = Coord;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Hailstone {
    coord: Coord,
    velocity: Coord,
}

impl From<&str> for Hailstone {
    fn from(s: &str) -> Self {
        let (coord, velocity) = s.split_once(" @ ").unwrap();
        let (coord, velocity) = (Coord::from(coord), Coord::from(velocity));
        Self { coord, velocity }
    }
}

impl Hailstone {
    fn at(&self, time: f64) -> Coord {
        self.coord + self.velocity * time
    }

    /// return (x,vx)
    fn x(&self) -> (f64, f64) {
        (self.coord.x, self.velocity.x)
    }

    /// return (y,vy)
    fn y(&self) -> (f64, f64) {
        (self.coord.y, self.velocity.y)
    }

    /// return (z,vz)
    fn z(&self) -> (f64, f64) {
        (self.coord.z, self.velocity.z)
    }

    fn intersect_xy(&self, rhs: &Self) -> Option<Coord> {
        let ((x1, vx1), (y1, vy1)) = (self.x(), self.y());
        let ((x2, vx2), (y2, vy2)) = (rhs.x(), rhs.y());

        match solve_linear_2_variables((vx1, -vx2, x1 - x2), (vy1, -vy2, y1 - y2)) {
            None => None,
            Some((x, y)) => {
                if x < 0.0 || y < 0.0 {
                    None
                } else {
                    Some(self.at(x))
                }
            }
        }
    }
}

fn solve_linear_2_variables(
    (a1, b1, c1): (f64, f64, f64),
    (a2, b2, c2): (f64, f64, f64),
) -> Option<(f64, f64)> {
    let d = b1 * a2 - b2 * a1;
    if d == 0.0 {
        None
    } else {
        Some(((b2 * c1 - b1 * c2) / d, (c2 * a1 - c1 * a2) / d))
    }
}

pub(crate) fn scan(path: &str) -> Vec<Hailstone> {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| Hailstone::from(s.as_str()))
        .collect()
}

pub(crate) fn intersect_xy(stones: &Vec<Hailstone>, from: i64, to: i64) -> u64 {
    (0..stones.len())
        .flat_map(|i| (i + 1..stones.len()).map(move |j| (i, j)))
        .map(|(i, j)| (&stones[i], &stones[j]))
        .filter_map(|(a, b)| a.intersect_xy(b))
        .filter(|c| from <= c.x.floor() as i64 && c.x.ceil() as i64 <= to)
        .filter(|c| from <= c.y.floor() as i64 && c.y.ceil() as i64 <= to)
        .count() as u64
}
