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
    saw: &mut Vec<Vec<bool>>,
    curr: usize,
    to: &Point,
    at: Point,
    best: usize,
) -> Option<usize> {
    let (max_x, max_y) = (field.len() as u64, field[0].len() as u64);
    if &at == to {
        if best < curr {
            println!("saw {curr}, prev best was {best}")
        }
        return Some(curr);
    }
    let mut best = best;
    for (x, y) in at
        .neighbors_straight()
        .into_iter()
        .filter(|p| p.is_valid(max_x, max_y))
        .map(|p| p.coords())
        .filter(|&(x, y)| field[x][y] != b'#')
    {
        match saw[x][y] {
            true => continue,
            false => {
                saw[x][y] = true;
                let new = dfs(field, saw, curr + 1, to, Point::from_coords(x, y), best);
                saw[x][y] = false;
                if new.is_some() {
                    best = best.max(new.unwrap());
                }
            }
        }
    }
    Some(best)
}
