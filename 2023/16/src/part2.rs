use rayon::prelude::*;

use lib::point::{Dir, Point};

use crate::contraption;

pub(crate) fn solve(path: &str) -> u64 {
    let c = contraption::scan(path);
    let (rows, cols) = (c.len() as i64, c[0].len() as i64);

    (0..rows)
        .flat_map(|x| {
            [
                (Point { x, y: 0 }, Dir::L),
                (Point { x, y: cols - 1 }, Dir::R),
            ]
        })
        .chain((0..cols).flat_map(|y| {
            [
                (Point { x: 0, y }, Dir::U),
                (Point { x: rows - 1, y }, Dir::D),
            ]
        }))
        .par_bridge()
        .map(|p| contraption::energy_from(&c, p))
        .max()
        .unwrap()
}
