use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::fs::read;
use std::hash::Hash;
use std::io::BufRead;

use lib::point::{Dir, Point};
use lib::pq::PriorityInsert;

#[derive(Copy, Clone, PartialEq, Eq, Ord, Debug, Hash)]
pub(crate) struct State {
    pub(crate) at: Point,
    pub(crate) dir: Dir, // where we're face when we entered the location
    pub(crate) to: Point,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.h().partial_cmp(&other.h()) {
            Some(Ordering::Equal) => (self.at, self.dir).partial_cmp(&((other.at, other.dir))),
            o => o,
        }
    }
}

impl State {
    fn h(&self) -> u64 {
        self.at.dst_flat(&self.to) as u64
    }
}

pub(crate) struct Limits {
    pub(crate) exclusive_x: u64,
    pub(crate) exclusive_y: u64,
}

// impl a_star, but the key is both point & dir
pub(crate) fn a_star(
    city: &Vec<Vec<u8>>,
    from: Point,
    to: Point,
    next: impl Fn(&Vec<Vec<u8>>, &State, &Limits) -> Vec<(State, u64)>,
) -> u64 {
    let limits = &Limits {
        exclusive_x: city.len() as u64,
        exclusive_y: city[0].len() as u64,
    };

    let start = [
        State {
            at: from,
            dir: Dir::R,
            to,
        },
        State {
            at: from,
            dir: Dir::D,
            to,
        },
    ];

    let mut open = VecDeque::from(start);
    let mut g_score: HashMap<State, u64> = HashMap::new();
    let mut f_score: HashMap<State, u64> = HashMap::new();
    start.iter().for_each(|&s| {
        g_score.insert(s, 0);
        f_score.insert(s, s.h());
    });

    let mut res = u64::MAX;
    while !open.is_empty() {
        let node = open.pop_front().unwrap();
        if node.at == to {
            let score = *g_score.get(&node).unwrap();
            if score < res {
                res = score;
            }
            continue;
        }

        let &n_score = g_score.get(&node).unwrap();

        for (s, extra) in next(city, &node, limits).into_iter() {
            let t = n_score + extra;
            match g_score.entry(s) {
                Entry::Vacant(e) => {
                    e.insert(t);
                    f_score.insert(s, t + s.h());
                    open.p_insert(s);
                }
                Entry::Occupied(mut e) => {
                    if &t < e.get() {
                        e.insert(t);
                        f_score.insert(s, t + s.h());
                        open.p_insert(s);
                    }
                }
            }
        }
    }
    res
}

pub(crate) fn scan(path: &str) -> Vec<Vec<u8>> {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.bytes().map(|b| b - b'0').collect())
        .collect()
}

pub(crate) fn solve(
    path: &str,
    next: impl Fn(&Vec<Vec<u8>>, &State, &Limits) -> Vec<(State, u64)>,
) -> u64 {
    let c = scan(path);
    a_star(
        &c,
        Point::from_coords(0, 0),
        Point::from_coords(c.len() - 1, c[0].len() - 1),
        next,
    )
}

pub(crate) fn walk(
    c: &Vec<Vec<u8>>,
    from: &State,
    dir: Dir,
    limits: &Limits,
    steps: usize,
    skip: usize,
) -> Vec<(State, u64)> {
    (0..steps)
        .scan(from.at, |x, _| {
            *x = x.neighbour(dir);
            Some(*x)
        })
        .scan((*from, 0u64), |n, at| {
            if !at.is_valid(limits.exclusive_x, limits.exclusive_y) {
                None
            } else {
                *n = (State { at, dir, ..*from }, n.1 + c[at.ux()][at.uy()] as u64);
                Some(*n)
            }
        })
        .skip(skip)
        .collect()
}
