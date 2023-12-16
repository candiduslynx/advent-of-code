use std::fs::read;
use std::io::BufRead;

use lib::point::Point;

use crate::contraption;

#[derive(Clone, Copy)]
pub(crate) enum Dir {
    L = 1,
    R = 2,
    U = 4,
    D = 8,
}

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

    pub(crate) fn next(
        &mut self,
        point_dir: &PointDir,
    ) -> Option<(Option<PointDir>, Option<PointDir>)> {
        let fr = point_dir.1 as u8;
        return match self.entered & fr {
            0 => {
                self.entered |= fr;
                match self.c {
                    b'/' => Some((
                        Some(Node::passthrough(
                            &point_dir.0,
                            match point_dir.1 {
                                Dir::L => Dir::D,
                                Dir::R => Dir::U,
                                Dir::D => Dir::L,
                                Dir::U => Dir::R,
                            },
                        )),
                        None,
                    )),
                    b'\\' => Some((
                        Some(Node::passthrough(
                            &point_dir.0,
                            match point_dir.1 {
                                Dir::L => Dir::U,
                                Dir::R => Dir::D,
                                Dir::D => Dir::R,
                                Dir::U => Dir::L,
                            },
                        )),
                        None,
                    )),
                    b'-' => match point_dir.1 {
                        Dir::U | Dir::D => Some((
                            Some(Node::passthrough(&point_dir.0, Dir::L)),
                            Some(Node::passthrough(&point_dir.0, Dir::R)),
                        )),
                        _ => Some((Some(Node::passthrough(&point_dir.0, point_dir.1)), None)),
                    },
                    b'|' => match point_dir.1 {
                        Dir::L | Dir::R => Some((
                            Some(Node::passthrough(&point_dir.0, Dir::U)),
                            Some(Node::passthrough(&point_dir.0, Dir::D)),
                        )),
                        _ => Some((Some(Node::passthrough(&point_dir.0, point_dir.1)), None)),
                    },
                    _ => Some((Some(Node::passthrough(&point_dir.0, point_dir.1)), None)),
                }
            }
            _ => None,
        };
    }

    fn passthrough(coord: &Point, from: Dir) -> PointDir {
        match from {
            Dir::L => (coord.right(), from),
            Dir::R => (coord.left(), from),
            Dir::U => (coord.below(), from),
            Dir::D => (coord.above(), from),
        }
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

pub(crate) fn next(c: &mut Contraption, nodes: &Vec<PointDir>) -> Vec<PointDir> {
    nodes
        .iter()
        .filter_map(|n| c[n.0.ux()][n.0.uy()].next(n))
        .flat_map(|x| [x.0, x.1])
        .filter_map(|x| x)
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
        next = contraption::next(&mut c, &next)
            .into_iter()
            .filter(|(p, _)| p.is_valid(rows, cols))
            .collect();
    }
    energy(&c)
}
