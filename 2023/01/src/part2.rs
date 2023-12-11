use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve(path: &str) -> u32 {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| num(s.as_str()))
        .sum()
}

fn num(s: &str) -> u32 {
    let mut padded = String::from("____");
    padded.push_str(s);
    padded.push_str("____");
    let first = s
        .as_bytes()
        .iter()
        .enumerate()
        .find_map(|(i, _)| first_digit(&padded[i + 4..]))
        .unwrap();
    let len = padded.len();
    let last = s
        .as_bytes()
        .iter()
        .enumerate()
        .find_map(|(i, _)| last_digit(&padded[..len - i - 4]))
        .unwrap();

    (first * 10 + last) as u32
}

fn first_digit(s: &str) -> Option<u8> {
    let bytes = s.as_bytes();
    let c = &bytes[0];
    if c.is_ascii_digit() {
        return Some(c - b'0');
    }

    [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .find_map(|(i, &pfx)| {
        if s.starts_with(pfx) {
            Some(i as u8)
        } else {
            None
        }
    })
}

fn last_digit(s: &str) -> Option<u8> {
    let bytes = s.as_bytes();
    let c = &bytes[bytes.len() - 1];
    if c.is_ascii_digit() {
        return Some(c - b'0');
    }

    [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .find_map(|(i, &pfx)| {
        if s.ends_with(pfx) {
            Some(i as u8)
        } else {
            None
        }
    })
}
