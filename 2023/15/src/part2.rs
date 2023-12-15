use crate::lens::LensHasher;
use std::fs::read;
use std::io::BufRead;

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
            s.split(",").for_each(|part| {
                if part.ends_with("-") {
                    let key = part.strip_suffix("-").unwrap();
                    match boxes[LensHasher::calc(key) as usize]
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
                    let h = usize::try_from(LensHasher::calc(key)).unwrap();
                    let val = parts[1].parse::<u8>().unwrap();
                    match boxes[h]
                        .iter_mut()
                        .find(|s| s.as_ref().is_some_and(|(s, _)| s == key))
                    {
                        None => boxes[h].push(Some((String::from(key), val))),
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