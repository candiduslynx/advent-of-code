use std::fs::read;
use std::io::BufRead;

use crate::cards::Hand;

pub(crate) fn solve(path: &str) -> u64 {
    let mut hands: Vec<(Hand, u64)> = read(path)
        .unwrap()
        .lines()
        .filter_map(|s| s.ok())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let parts: Vec<&str> = s
                .split_whitespace()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();
            assert_eq!(parts.len(), 2);
            (Hand::from_str(parts[0]), parts[1].parse().unwrap())
        })
        .collect();
    hands.sort_by(|(a, _), (b, _)| a.cmp(&b));
    hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bet))| (1u64 + idx as u64) * bet)
        .sum()
}
