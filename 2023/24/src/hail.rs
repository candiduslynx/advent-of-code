use std::fs::read;
use std::io::BufRead;
use std::ops::Sub;
use std::ops::{Add, Mul};
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct ParseError;

#[derive(Copy, Clone, Debug)]
struct Coord {
    x: f64,
    y: f64,
    z: f64,
}

impl FromStr for Coord {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<f64> = s
            .split(", ")
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        assert_eq!(s.len(), 3);
        Ok(Self {
            x: s[0],
            y: s[1],
            z: s[2],
        })
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

impl FromStr for Hailstone {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (coord, velocity) = s.split_once(" @ ").unwrap();
        let (coord, velocity) = (Coord::from_str(coord)?, Coord::from_str(velocity)?);
        Ok(Self { coord, velocity })
    }
}

pub(crate) fn scan(path: &str) -> Vec<Hailstone> {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| Hailstone::from_str(s.as_str()).unwrap())
        .collect()
}
