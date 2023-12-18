use std::fs::read;
use std::io::BufRead;
use std::str::FromStr;

use lib::point::{Dir, Point};

pub(crate) struct Polygon {
    points: Vec<Point>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ParseError;
impl Polygon {
    fn area1(&self) -> u64 {
        let (s1, s2) = self
            .points
            .iter()
            .zip(self.points.iter().cycle().skip(1))
            .fold((0, 0), |(a, b), (p1, p2)| {
                (a + (p1.x * p2.y) as i64, b + (p1.y * p2.x) as i64)
            });
        s1.abs_diff(s2) >> 1
    }

    fn circumference(&self) -> u64 {
        self.points
            .iter()
            .zip(self.points.iter().cycle().skip(1))
            .map(|(a, b)| a.dst_flat(b) as u64)
            .sum()
    }

    fn area(&self) -> u64 {
        self.area1() + (self.circumference() / 2 + 1)
    }

    fn from_steps(steps: &Vec<Step>) -> Result<Self, ParseError> {
        let points: Vec<Point> = steps
            .iter()
            .scan(Point { x: 0, y: 0 }, |from, step| {
                *from = match step.dir {
                    Dir::U => Point {
                        x: from.x - step.len,
                        ..*from
                    },
                    Dir::D => Point {
                        x: from.x + step.len,
                        ..*from
                    },
                    Dir::L => Point {
                        y: from.y - step.len,
                        ..*from
                    },
                    Dir::R => Point {
                        y: from.y + step.len,
                        ..*from
                    },
                };
                Some(*from)
            })
            .collect();

        let p = points.last().unwrap();
        if p.x == 0 && p.y == 0 {
            Ok(Self { points })
        } else {
            Err(ParseError)
        }
    }
}

pub(crate) struct Step {
    dir: Dir,
    len: isize,
}

impl Step {
    fn from_hex_str(s: &str) -> Result<Self, ParseError> {
        let p = s.split_once("#").unwrap().1;
        let p = &p[..p.len() - 1];
        assert_eq!(p.len(), 6);

        let dir = match p.as_bytes()[5] {
            b'0' => Ok(Dir::R),
            b'1' => Ok(Dir::D),
            b'2' => Ok(Dir::L),
            b'3' => Ok(Dir::U),
            _ => Err(ParseError),
        }?;

        let len = isize::from_str_radix(&p[..5], 16).map_err(|_| ParseError)?;
        Ok(Self { dir, len })
    }
}
impl FromStr for Step {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .split_whitespace()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        assert_eq!(parts.len(), 3);

        assert_eq!(parts[0].len(), 1);
        let dir = match parts[0].as_bytes()[0] {
            b'R' => Ok(Dir::R),
            b'D' => Ok(Dir::D),
            b'L' => Ok(Dir::L),
            b'U' => Ok(Dir::U),
            _ => Err(ParseError),
        }?;

        let len = parts[1].parse::<isize>().map_err(|_| ParseError)?;

        Ok(Self { dir, len })
    }
}

pub(crate) fn solve(path: &str, hex: bool) -> Option<u64> {
    let Ok(steps) = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| {
            if !hex {
                Step::from_str(s.as_str())
            } else {
                Step::from_hex_str(s.as_str())
            }
        })
        .collect()
    else {
        return None;
    };

    let Ok(p) = Polygon::from_steps(&steps) else {
        return None;
    };

    Some(p.area())
}
