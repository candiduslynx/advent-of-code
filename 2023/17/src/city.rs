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
