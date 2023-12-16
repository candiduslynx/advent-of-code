use crate::contraption;
use crate::contraption::{Dir, PointDir};
use lib::point::Point;

pub(crate) fn solve(path: &str) -> u64 {
    let mut c = contraption::scan(path);
    let (rows, cols) = (c.len(), c[0].len());
    let mut next: Vec<PointDir> = vec![(Point { x: 0, y: 0 }, Dir::L)];
    while next.len() > 0 {
        next = contraption::next(&mut c, &next)
            .into_iter()
            .filter(|(p, _)| p.is_valid(rows, cols))
            .collect();
    }
    contraption::energy(&c)
}
