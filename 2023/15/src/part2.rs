use std::fs::read;
use std::io::BufRead;

use crate::lens::LensHasher;

pub(crate) fn solve(path: &str) -> u64 {
    let mut boxes = std::iter::repeat(vec![])
        .take(256)
        .collect::<Vec<Vec<Option<(String, u8)>>>>();

    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .for_each(|s| {
            s.split(",").for_each(|part| match part.as_bytes().last() {
                Some(&b'-') => {
                    let key = &part[..part.len() - 1];
                    match boxes[LensHasher::calc(key) as usize]
                        .iter_mut()
                        .find(|s| s.as_ref().is_some_and(|(s, _)| s == key))
                    {
                        None => {}
                        Some(k) => *k = None,
                    }
                }
                _ => {
                    let parts: Vec<&str> = part.split("=").collect();
                    assert_eq!(parts.len(), 2);
                    let key = parts[0];
                    let lens_box = &mut boxes[LensHasher::calc(key) as usize];
                    let val = parts[1].parse::<u8>().unwrap();
                    match lens_box
                        .iter_mut()
                        .find(|s| s.as_ref().is_some_and(|(s, _)| s == key))
                    {
                        None => lens_box.push(Some((String::from(key), val))),
                        Some(Some(k)) => k.1 = val,
                        _ => {}
                    }
                }
            })
        });

    boxes
        .iter()
        .enumerate()
        .filter(|(_, v)| !v.is_empty())
        .map(|(i, v)| {
            let k = i + 1;
            v.iter()
                .filter(|s| s.is_some())
                .enumerate()
                .map(|(j, kv)| (k * (j + 1) * usize::from(kv.as_ref().unwrap().1)) as u64)
                .sum::<u64>()
        })
        .sum()
}
