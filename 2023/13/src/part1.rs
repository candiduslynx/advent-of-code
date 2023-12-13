use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve(path: &str) -> u64 {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .fold(vec![Vec::<Vec<u8>>::new()], |mut state, s| {
            match s.len() {
                0 => state.push(vec![]),
                _ => state.last_mut().unwrap().push(s.as_bytes().to_vec()),
            }
            state
        })
        .iter()
        .for_each(|v| {
            let x = v.len();
            let y = v[0].len();
            println!("{x} by {y}");
        });
    124
}
