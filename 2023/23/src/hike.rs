use std::collections::{HashMap, VecDeque};
use std::fs::read;
use std::io::BufRead;

use lib::point::Point;

pub(crate) fn scan(path: &str) -> (Vec<Vec<u8>>, Point, Point) {
    let field: Vec<Vec<u8>> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.as_bytes().to_vec())
        .collect();
    let start = Point::from_coords(
        0,
        field[0]
            .iter()
            .enumerate()
            .find_map(|(y, b)| if b == &b'.' { Some(y) } else { None })
            .unwrap(),
    );
    let end = Point::from_coords(
        field.len() - 1,
        field
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .find_map(|(y, b)| if b == &b'.' { Some(y) } else { None })
            .unwrap(),
    );
    (field, start, end)
}

pub(crate) fn longest(field: &Vec<Vec<u8>>, start: &Point, to: &Point) -> usize {
    let (max_x, max_y) = (field.len() as u64, field[0].len() as u64);

    let mut l: HashMap<Point, usize> = HashMap::new();
    let mut open: VecDeque<(Option<Point>, Point, usize)> = VecDeque::from([(None, *start, 0)]);

    while let Some((prev, p, dst)) = open.pop_front() {
        if !p.is_valid(max_x, max_y) {
            continue;
        }
        let (x, y) = p.coords();
        if field[x][y] == b'#' {
            //wall
            continue;
        }

        // p is where we are going to
        match l.get_mut(&p) {
            None => {
                l.insert(p, dst);
            }
            Some(val) => {
                if *val > dst {
                    continue;
                } else {
                    *val = dst;
                }
            }
        }
        // if we're here we're looking at a new place or a better way
        let next = match field[x][y] {
            b'.' => p.neighbors_straight().to_vec(),
            b'>' => vec![p.right()],
            b'<' => vec![p.left()],
            b'^' => vec![p.above()],
            b'v' => vec![p.below()],
            _b => panic!("unexpected val {_b}"),
        };
        // no backtracking
        match prev {
            None => next
                .into_iter()
                .for_each(|next| open.push_back((Some(p), next, dst + 1))),
            Some(prev) => next
                .into_iter()
                .filter(|&next| next != prev)
                .for_each(|next| open.push_back((Some(p), next, dst + 1))),
        }
    }
    *l.get(to).unwrap()
}

pub(crate) fn dfs(
    field: &Vec<Vec<u8>>,
    path: &mut VecDeque<Point>,
    to: &Point,
    at: Point,
    best: usize,
) -> usize {
    let mut at = at;
    let mut pushed = 0usize;
    let (max_x, max_y) = (field.len() as u64, field[0].len() as u64);
    let res = loop {
        let mut res = best;
        if &at == to {
            if best < path.len() {
                println!("found a path with {} len", path.len());
            }
            break path.len();
        }
        let next: Vec<Point> = at
            .neighbors_straight()
            .into_iter()
            .filter(|p| p.is_valid(max_x, max_y))
            .filter(|p| !path.contains(p))
            .filter(|p| field[p.ux()][p.uy()] != b'#')
            .collect();

        match next.len() {
            0 => break res, // no path available next
            1 => {
                pushed += 1; // push next
                path.push_back(next[0]);
                at = next[0];
                continue;
            }
            _ => {
                next.into_iter().for_each(|p| {
                    path.push_back(p);
                    let new = dfs(field, path, to, p, res);
                    path.pop_back();
                    res = res.max(new);
                });
                break res;
            }
        }
    };

    // cleanup
    (0..pushed).for_each(|_| {
        path.pop_back();
    });
    res
}
