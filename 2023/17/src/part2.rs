use crate::city;
use crate::city::{Limits, State};
use lib::point::Point;

pub(crate) fn solve(path: &str) -> u64 {
    city::solve(path, next_moves)
}

// after each step we have 3 possibilities: forward, left or right
// we also need to take the validity into account
fn next_moves(city: &Vec<Vec<u8>>, s: &State, limits: &Limits) -> [Option<(State, u64)>; 3] {
    // next state + cost incurred
    let (l, r) = (s.dir.turn_counterclockwise(), s.dir.turn_clockwise());
    let f = s.at.neighbour(&s.dir);
    let (l1, r1) = (s.at.neighbour(&l), s.at.neighbour(&r));
    let (l2, r2) = (l1.neighbour(&l), r1.neighbour(&r));
    let (l3, r3) = (l2.neighbour(&l), r2.neighbour(&r));
    let (l4, r4) = (l3.neighbour(&l), r3.neighbour(&r));
    [
        (
            State {
                // forward
                at: f,
                took: s.took + 1,
                ..*s
            },
            cost(city, &f, limits),
        ),
        (
            State {
                // left
                at: l4,
                dir: l,
                took: 4,
                ..*s
            },
            cost_vec(city, [&l1, &l2, &l3, &l4], limits),
        ),
        (
            State {
                // right
                at: r4,
                dir: r,
                took: 4,
                ..*s
            },
            cost_vec(city, [&r1, &r2, &r3, &r4], limits),
        ),
    ]
    .map(|(s, c)| match c {
        None => None,
        Some(c) => {
            if s.took > 10 {
                None
            } else {
                Some((s, c))
            }
        }
    })
}

fn cost(city: &Vec<Vec<u8>>, at: &Point, limits: &Limits) -> Option<u64> {
    if !at.is_valid(limits.exclusive_x, limits.exclusive_y) {
        None
    } else {
        let (x, y) = at.coords();
        Some(city[x][y] as u64)
    }
}

fn cost_vec(city: &Vec<Vec<u8>>, at: [&Point; 4], limits: &Limits) -> Option<u64> {
    if !at[3].is_valid(limits.exclusive_x, limits.exclusive_y) {
        None
    } else {
        Some(
            at.map(|at| at.coords())
                .map(|(x, y)| city[x][y] as u64)
                .iter()
                .sum(),
        )
    }
}
