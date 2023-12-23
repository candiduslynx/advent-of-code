use std::collections::HashSet;
use std::fs::read;
use std::io::BufRead;

use lib::point::Point;

pub(crate) fn scan(path: &str) -> (Vec<Vec<u8>>, Point) {
    let mut f: Vec<Vec<u8>> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| Vec::from(s.as_bytes()))
        .collect();

    let x = f
        .iter()
        .enumerate()
        .find_map(|(x, row)| if row.contains(&b'S') { Some(x) } else { None })
        .unwrap();
    let y = f[x]
        .iter_mut()
        .enumerate()
        .find_map(|(y, el)| {
            if *el == b'S' {
                *el = b'.';
                Some(y)
            } else {
                None
            }
        })
        .unwrap();
    (f, Point::from_coords(x, y))
}

/// `distances[idx] = amount of nodes with minimal distance = idx`
pub(crate) fn distances(f: &Vec<Vec<u8>>, from: &Point) -> Vec<u64> {
    let (max_x, max_y) = (f.len() as u64, f[0].len() as u64);
    let mut visited = HashSet::from([from.clone()]);
    let mut result = vec![1]; // from
    let mut open = vec![from.clone()];

    while !open.is_empty() {
        open = open
            .iter()
            .flat_map(|p| p.neighbors_straight())
            .filter_map(|p| {
                if visited.contains(&p) || !p.is_valid(max_x, max_y) {
                    None
                } else {
                    let (x, y) = p.coords();
                    match f[x][y] {
                        b'.' => {
                            visited.insert(p.clone());
                            Some(p)
                        }
                        _ => None,
                    }
                }
            })
            .collect();

        result.push(open.len() as u64);
    }
    result
}
