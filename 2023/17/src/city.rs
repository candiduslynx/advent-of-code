use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::fs::read;
use std::hash::{Hash, Hasher};
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

#[derive(Copy, Clone, PartialEq, Eq, Ord, Debug)]
pub(crate) struct State {
    pub(crate) at: Point,
    pub(crate) dir: Dir,   // where we're face when we entered the location
    pub(crate) took: u8,   // how many steps in direction specified were taken already
    pub(crate) end: Point, // for cmp
    pub(crate) exclusive_end: Point, // for limits
    pub(crate) total: u64,
}

impl State {
    fn h(&self) -> u64 {
        self.at.dst_flat(&self.end) as u64 + self.total
    }
}
impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.h().partial_cmp(&other.h())
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.at.hash(state);
        self.dir.hash(state);
        self.took.hash(state);
    }
}

// after each step we have 3 possibilities: forward, left or right
// we also need to take the validity into account
fn next_moves(city: &Vec<Vec<u8>>, state: &State) -> [Option<State>; 3] {
    let possible = match state.dir {
        Dir::U => [
            State {
                at: state.at.below(),
                dir: Dir::U,
                took: state.took + 1,
                ..*state
            }, // forward
            State {
                at: state.at.left(),
                dir: Dir::L,
                took: 1,
                ..*state
            }, // left
            State {
                at: state.at.right(),
                dir: Dir::R,
                took: 1,
                ..*state
            }, // right
        ],
        Dir::D => [
            State {
                at: state.at.below(),
                dir: Dir::D,
                took: state.took + 1,
                ..*state
            }, // forward
            State {
                at: state.at.right(),
                dir: Dir::R,
                took: 1,
                ..*state
            }, // left
            State {
                at: state.at.left(),
                dir: Dir::L,
                took: 1,
                ..*state
            }, // right
        ],
        Dir::L => [
            State {
                at: state.at.left(),
                dir: Dir::L,
                took: state.took + 1,
                ..*state
            }, // forward
            State {
                at: state.at.below(),
                dir: Dir::D,
                took: 1,
                ..*state
            }, // left
            State {
                at: state.at.above(),
                dir: Dir::U,
                took: 1,
                ..*state
            }, // right
        ],
        Dir::R => [
            State {
                at: state.at.right(),
                dir: Dir::R,
                took: state.took + 1,
                ..*state
            }, // forward
            State {
                at: state.at.above(),
                dir: Dir::U,
                took: 1,
                ..*state
            }, // left
            State {
                at: state.at.below(),
                dir: Dir::D,
                took: 1,
                ..*state
            }, // right
        ],
    };
    [
        validate(city, possible[0], state.total),
        validate(city, possible[1], state.total),
        validate(city, possible[2], state.total),
    ]
}

fn validate(city: &Vec<Vec<u8>>, mut state: State, total: u64) -> Option<State> {
    let (max_x, max_y) = state.exclusive_end.coords();
    if !state.at.is_valid(max_x, max_y) {
        return None;
    }
    let (x, y) = state.at.coords();
    state.total = total + city[x][y] as u64;

    match state.took {
        (0..=3) => Some(state),
        _ => None,
    }
}

// impl a_star, but the key is both point & dir
pub(crate) fn a_star(city: &Vec<Vec<u8>>, at: Point, end: Point) -> u64 {
    let start = State {
        at,
        dir: Dir::R,
        took: 0,
        end,
        exclusive_end: Point::from_coords(city.len(), city[0].len()),
        total: 0,
    };
    let mut open = VecDeque::from([start]);
    let mut score: HashMap<State, u64> = HashMap::new();
    score.insert(start, 0);

    while !open.is_empty() {
        let node = open.pop_front().unwrap();
        if node.at == node.end {
            return node.total;
        }

        next_moves(city, &node)
            .into_iter()
            .filter_map(|s| s)
            .for_each(|s| match score.entry(s) {
                Entry::Vacant(e) => {
                    e.insert(s.total);
                    open.p_insert(s);
                }
                Entry::Occupied(mut e) => {
                    if &s.total < e.get() {
                        e.insert(s.total);
                        open.p_insert(s);
                    }
                }
            });
    }
    0
}
