use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::fs::read;
use std::hash::Hash;
use std::io::BufRead;

use lib::point::{Dir, Point};
use lib::pq::PriorityInsert;

pub(crate) fn scan(path: &str) -> Vec<Vec<u8>> {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.bytes().map(|b| b - b'0').collect())
        .collect()
}

#[derive(Copy, Clone, PartialEq, Eq, Ord, Debug, Hash)]
pub(crate) struct State {
    pub(crate) at: Point,
    pub(crate) dir: Dir,    // where we're face when we entered the location
    pub(crate) took: usize, // how many steps in direction specified were taken already
    pub(crate) to: Point,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.h().partial_cmp(&other.h()) {
            Some(Ordering::Equal) => {
                (self.at, self.dir, self.took).partial_cmp(&((other.at, other.dir, other.took)))
            }
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
    pub(crate) exclusive_x: usize, // <, not <=
    pub(crate) exclusive_y: usize,
}

// impl a_star, but the key is both point & dir
pub(crate) fn a_star(
    city: &Vec<Vec<u8>>,
    from: Point,
    to: Point,
    next: impl Fn(&Vec<Vec<u8>>, &State, &Limits) -> [Option<(State, u64)>; 3],
) -> u64 {
    let limits = &Limits {
        exclusive_x: city.len(),
        exclusive_y: city[0].len(),
    };

    let start = State {
        at: from,
        dir: Dir::R,
        took: 0,
        to,
    };

    let mut open = VecDeque::from([start]);
    let mut g_score: HashMap<State, u64> = HashMap::new();
    g_score.insert(start, 0);
    let mut f_score: HashMap<State, u64> = HashMap::new();
    f_score.insert(start, start.h()); // we just use flat dst as minimal amount of steps

    let mut res = u64::MAX;
    while !open.is_empty() {
        let node = open.pop_front().unwrap();
        if node.at == to {
            let score = *g_score.get(&node).unwrap();
            if score < res {
                res = score;
                // println!("possibility edit {:?}", score);
            }
            continue;
        }

        let &n_score = g_score.get(&node).unwrap();

        for (s, extra) in next(city, &node, limits).into_iter().filter_map(|s| s) {
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

pub(crate) fn solve(
    path: &str,
    next: impl Fn(&Vec<Vec<u8>>, &State, &Limits) -> [Option<(State, u64)>; 3],
) -> u64 {
    let c = scan(path);
    a_star(
        &c,
        Point::from_coords(0, 0),
        Point::from_coords(c.len() - 1, c[0].len() - 1),
        next,
    )
}
