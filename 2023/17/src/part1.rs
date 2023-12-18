use crate::city;
use crate::city::{Limits, State};

pub(crate) fn solve(path: &str) -> u64 {
    city::solve(path, next_moves)
}

// after each step we have 3 possibilities: forward, left or right
// we also need to take the validity into account
fn next_moves(city: &Vec<Vec<u8>>, s: &State, limits: &Limits) -> [Option<(State, u64)>; 3] {
    // next state + cost incurred
    let (l, r) = (s.dir.turn_counterclockwise(), s.dir.turn_clockwise());
    [
        State {
            // forward
            at: s.at.neighbour(&s.dir),
            took: s.took + 1,
            ..*s
        },
        State {
            // left
            at: s.at.neighbour(&l),
            dir: l,
            took: 1,
            ..*s
        },
        State {
            // right
            at: s.at.neighbour(&r),
            dir: r,
            took: 1,
            ..*s
        },
    ]
    .map(|s| {
        if !s.at.is_valid(limits.exclusive_x, limits.exclusive_y) || s.took > 3 {
            None
        } else {
            let (x, y) = s.at.coords();
            Some((s, city[x][y] as u64))
        }
    })
}
