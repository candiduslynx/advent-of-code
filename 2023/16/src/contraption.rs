use std::fs::read;
use std::io::BufRead;

use lib::point::{Dir, Point};

#[derive(Clone, Copy)]
pub(crate) struct Node {
    pub c: u8,       // char at position
    pub entered: u8, // entered & Dir => we saw entering from that dir
}

pub(crate) type PointDir = (Point, Dir);

impl Node {
    pub(crate) fn from_byte(b: &u8) -> Self {
        Node {
            c: *b,
            entered: 0u8,
        }
    }

    pub(crate) fn next(&mut self, (point, from): &PointDir) -> [Option<PointDir>; 2] {
        let fr = *from as u8;
        return match self.entered & fr {
            0 => {
                self.entered |= fr;
                match self.c {
                    b'/' => [
                        Some(Node::passthrough(
                            point,
                            match from {
                                Dir::L => Dir::D,
                                Dir::R => Dir::U,
                                Dir::D => Dir::L,
                                Dir::U => Dir::R,
                            },
                        )),
                        None,
                    ],
                    b'\\' => [
                        Some(Node::passthrough(
                            point,
                            match from {
                                Dir::L => Dir::U,
                                Dir::R => Dir::D,
                                Dir::D => Dir::R,
                                Dir::U => Dir::L,
                            },
                        )),
                        None,
                    ],
                    b'-' => match from {
                        Dir::U | Dir::D => [
                            Some(Node::passthrough(point, Dir::L)),
                            Some(Node::passthrough(point, Dir::R)),
                        ],
                        _ => [Some(Node::passthrough(point, *from)), None],
                    },
                    b'|' => match from {
                        Dir::L | Dir::R => [
                            Some(Node::passthrough(point, Dir::U)),
                            Some(Node::passthrough(point, Dir::D)),
                        ],
                        _ => [Some(Node::passthrough(point, *from)), None],
                    },
                    _ => [Some(Node::passthrough(point, *from)), None],
                }
            }
            _ => [None, None],
        };
    }

    fn passthrough(coord: &Point, from: Dir) -> PointDir {
        (
            match from {
                Dir::L => coord.right(),
                Dir::R => coord.left(),
                Dir::U => coord.below(),
                Dir::D => coord.above(),
            },
            from,
        )
    }

    fn energized(&self) -> bool {
        self.entered > 0
    }
}

pub(crate) type Contraption = Vec<Vec<Node>>;

pub(crate) fn scan(path: &str) -> Contraption {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.as_bytes().iter().map(|b| Node::from_byte(b)).collect())
        .collect()
}

pub(crate) fn energy(c: &Contraption) -> u64 {
    c.iter()
        .map(|row| row.iter().filter(|n| n.energized()).count() as u64)
        .sum()
}

pub(crate) fn energy_from(init: &Contraption, from: PointDir) -> u64 {
    let (rows, cols) = (init.len(), init[0].len());
    let mut c = init.to_vec();
    let mut next: Vec<PointDir> = vec![from];
    while next.len() > 0 {
        next = next
            .iter()
            .flat_map(|n| c[n.0.ux()][n.0.uy()].next(n))
            .filter_map(|x| x)
            .filter(|(p, _)| p.is_valid(rows, cols))
            .collect();
    }
    energy(&c)
}
