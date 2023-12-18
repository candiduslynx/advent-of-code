use lib::point::Point;

use crate::city;

pub(crate) fn solve(path: &str) -> u64 {
    let c = city::scan(path);
    city::a_star(
        &c,
        Point::from_coords(0, 0),
        Point::from_coords(c.len() - 1, c[0].len() - 1),
        3,
    )
}
