use lib::point::{Dir, Point};
use std::fs::read;
use std::io::BufRead;

pub(crate) fn scan(path: &str) -> Vec<Vec<u8>> {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.bytes().map(|b| b - b'0').collect())
        .collect()
}

pub(crate) struct State {
    pub(crate) at: Point,
    pub(crate) dir: Dir, // where we're face when we entered the location
    pub(crate) took: u8, // how many steps in direction specified were taken already
}

// after each step we have 3 possibilities: forward, left or right
fn next(state: State) -> [Option<State>; 3] {}
