use std::fs::read;
use std::io::BufRead;

use crate::game;

pub(crate) fn solve() -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    return read("./input.txt").unwrap().lines().
        map(|s| game::Game::from_str(s.unwrap())).
        filter(|g| g.possible(MAX_RED, MAX_GREEN, MAX_BLUE)).
        fold(0u32, |sum, x| sum + x.id);
}
