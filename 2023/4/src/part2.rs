use std::iter;
use std::fs::read;
use std::io::BufRead;
use crate::card;

pub(crate) fn solve() -> u64 {
    let wins: Vec<usize> = read("./input.txt").unwrap().lines().
        map(|s|card::Card::from_string(s.unwrap())).
        map(|c| c.winning_numbers().len()).collect();

    let mut cards:Vec<u64> = iter::repeat(1).take(wins.len()).collect();

    for i in 0..wins.len()-1 {
        for j in i+1..i+1+wins[i] {
            cards[j]+=cards[i];
        }
    }

    cards.iter().sum()
}
