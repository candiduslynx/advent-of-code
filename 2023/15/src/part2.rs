use std::fs::read;
use std::io::BufRead;

use crate::hash;

pub(crate) fn solve(path: &str) -> u64 {
    type KeyValue = (String, usize);
    let mut boxes: Vec<Vec<Option<KeyValue>>> = std::iter::repeat(vec![])
        .take(256)
        .collect::<Vec<Vec<Option<KeyValue>>>>();

    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .for_each(|s| {
            s.split(",").for_each(|part| {
                if part.ends_with("-") {
                    let key = part.strip_suffix("-").unwrap();
                    match boxes[hash::hash(key)]
                        .iter_mut()
                        .find(|s| s.as_ref().is_some_and(|(s, _)| s == key))
                    {
                        None => {}
                        Some(k) => *k = None,
                    }
                } else {
                    let parts: Vec<&str> = part.split("=").collect();
                    assert_eq!(parts.len(), 2);
                    let key = parts[0];
                    let val = parts[1].parse::<usize>().unwrap();
                    match boxes[hash::hash(key)]
                        .iter_mut()
                        .find(|s| s.as_ref().is_some_and(|(s, _)| s == key))
                    {
                        None => boxes[hash::hash(key)].push(Some((String::from(key), val))),
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
                .map(|(j, kv)| (k * (j + 1) * kv.as_ref().unwrap().1) as u64)
                .sum::<u64>()
        })
        .sum()
}
