use std::collections::HashSet;
use std::fs::read;
use std::io::BufRead;

use lib::point::Point;

pub(crate) fn solve(path: &str) -> u64 {
    let sky: Vec<Vec<u8>> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.into_bytes())
        .collect();

    let galaxies = sky
        .iter()
        .enumerate()
        .flat_map(move |(x, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| b'#'.eq(c))
                .map(move |(y, _)| Point {
                    x: x as isize,
                    y: y as isize,
                })
        })
        .collect::<Vec<Point>>();

    let empty_x: Vec<isize> = galaxies
        .iter()
        .map(|p| p.x)
        .fold(
            HashSet::from_iter(0isize..sky.len() as isize),
            |mut left: HashSet<isize>, x| {
                left.remove(&x);
                left
            },
        )
        .into_iter()
        .collect();

    let empty_y: Vec<isize> = galaxies
        .iter()
        .map(|p| p.y)
        .fold(
            HashSet::from_iter(0isize..sky[0].len() as isize),
            |mut left: HashSet<isize>, y| {
                left.remove(&y);
                left
            },
        )
        .into_iter()
        .collect();

    galaxies
        .iter()
        .enumerate()
        .map(|(i, g)| {
            galaxies[i + 1..]
                .iter()
                .map(|o| {
                    let dst = g.dst_flat(o) as u64
                        + (intersections(g.x, o.x, &empty_x) as u64)
                        + (intersections(g.y, o.y, &empty_y) as u64);
                    dst
                })
                .sum::<u64>()
        })
        .sum()
}

fn intersections(start: isize, end: isize, poi: &Vec<isize>) -> usize {
    let (mut s, mut e) = (start, end);
    if start > end {
        (s, e) = (end, start);
    }
    poi.iter().filter(|&&p| s <= p && p <= e).count()
}
