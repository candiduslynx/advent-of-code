use lib::point::Point;
use std::collections::HashSet;
use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve(path: &str, scale: u64) -> u64 {
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
                    x: x as i64,
                    y: y as i64,
                })
        })
        .collect::<Vec<Point>>();

    let empty_x: Vec<i64> = galaxies
        .iter()
        .map(|p| p.x)
        .fold(
            HashSet::from_iter(0i64..sky.len() as i64),
            |mut left: HashSet<i64>, x| {
                left.remove(&x);
                left
            },
        )
        .into_iter()
        .collect();

    let empty_y: Vec<i64> = galaxies
        .iter()
        .map(|p| p.y)
        .fold(
            HashSet::from_iter(0i64..sky[0].len() as i64),
            |mut left: HashSet<i64>, y| {
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
                        + (scale - 1) * (intersections(g.x, o.x, &empty_x) as u64)
                        + (scale - 1) * (intersections(g.y, o.y, &empty_y) as u64);
                    dst
                })
                .sum::<u64>()
        })
        .sum()
}

fn intersections(start: i64, end: i64, points_of_interest: &Vec<i64>) -> usize {
    let (mut s, mut e) = (start, end);
    if start > end {
        (s, e) = (end, start);
    }
    points_of_interest
        .iter()
        .filter(|&&p| s <= p && p <= e)
        .count()
}
