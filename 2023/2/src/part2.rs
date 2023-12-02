use std::fs::read;
use std::io::BufRead;

use crate::game;

pub(crate) fn solve() -> u32 {
    return read("./input.txt").unwrap().lines().
        map(|s| game::Game::from_str(s.unwrap()).power()).
        fold(0u32, |sum, x| sum + x);
}
