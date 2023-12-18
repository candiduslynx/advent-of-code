use lib::point::{Dir, Point};

use crate::contraption;

pub(crate) fn solve(path: &str) -> u64 {
    contraption::energy_from(&contraption::scan(path), (Point { x: 0, y: 0 }, Dir::L))
}
