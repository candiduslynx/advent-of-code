use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve(path: &str) -> u32 {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| digits(s.as_str()))
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum()
}

fn digits(s: &str) -> Vec<u32> {
    s.char_indices()
        .filter_map(|(idx, _)| digit(&s[idx..]))
        .collect()
}

fn digit(s: &str) -> Option<u32> {
    if s.is_empty() {
        return None;
    }

    match s.chars().next() {
        None => return None,
        Some(c) => {
            if c.is_ascii_digit() {
                return c.to_digit(10);
            }
        }
    }

    [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .find_map(|(i, &pfx)| {
        if s.starts_with(pfx) {
            Some(i as u32)
        } else {
            None
        }
    })
}
