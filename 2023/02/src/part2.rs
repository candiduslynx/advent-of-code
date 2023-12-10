use std::fs::read;
use std::io::BufRead;

use crate::game;

pub(crate) fn solve(path: &str) -> u32 {
    read(path).unwrap().lines().
        map(|s| game::Game::from_str(s.unwrap()).power()).
        sum()
}
