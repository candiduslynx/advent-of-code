use std::fs::read;
use std::io::BufRead;

pub(crate) fn scan(path: &str) -> Vec<Vec<u8>> {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.as_bytes().to_vec())
        .collect()
}
