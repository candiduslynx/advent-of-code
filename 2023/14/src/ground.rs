use std::fs::read;
use std::io::BufRead;

pub(crate) fn scan(path: &str) -> Vec<Vec<u8>> {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.into_bytes())
        .collect()
}

pub(crate) fn load(ground: &Vec<Vec<u8>>) -> u64 {
    let len = ground.len();
    ground
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&g| g == b'O').count() as u64 * (len - i) as u64)
        .sum()
}

pub(crate) fn cycle(g: &mut Vec<Vec<u8>>) {
    tilt_north(g);
    tilt_west(g);
    tilt_south(g);
    tilt_east(g);
}

pub(crate) fn tilt_north(g: &mut Vec<Vec<u8>>) {
    let (rows, cols) = (g.len(), g[0].len());
    (0..cols).for_each(|col| {
        (0..rows).fold(None, |none, row| tilt(g, row, |x| (x, col), none, false));
    });
}

fn tilt_south(g: &mut Vec<Vec<u8>>) {
    let (rows, cols) = (g.len(), g[0].len());
    (0..cols).for_each(|col| {
        (0..rows)
            .rev()
            .fold(None, |none, row| tilt(g, row, |x| (x, col), none, true));
    });
}

fn tilt_west(g: &mut Vec<Vec<u8>>) {
    let (rows, cols) = (g.len(), g[0].len());
    (0..rows).for_each(|row| {
        (0..cols).fold(None, |none, col| tilt(g, col, |y| (row, y), none, false));
    });
}

fn tilt_east(g: &mut Vec<Vec<u8>>) {
    let (rows, cols) = (g.len(), g[0].len());
    (0..rows).for_each(|row| {
        (0..cols)
            .rev()
            .fold(None, |none, col| tilt(g, col, |y| (row, y), none, true));
    });
}

fn tilt<T>(
    g: &mut Vec<Vec<u8>>,
    at: usize,
    coord: T,
    none: Option<usize>,
    forward: bool,
) -> Option<usize>
where
    T: Fn(usize) -> (usize, usize),
{
    let (x, y) = coord(at);
    match g[x][y] {
        b'#' => return None,
        b'.' => return none.or(Some(at)),
        b'O' => match none {
            Some(to) => {
                let (to_x, to_y) = coord(to);
                (g[x][y], g[to_x][to_y]) = (g[to_x][to_y], g[x][y]);
                return if forward { Some(to - 1) } else { Some(to + 1) };
            }
            _ => none,
        },
        _b => panic!("bad byte {_b}"),
    }
}

/// hash the state to u128
/// each row is encoded as u128, then we do a xor
pub(crate) fn to_u128(v: &Vec<Vec<u8>>) -> u128 {
    v.iter()
        .map(|row| {
            row.iter().fold(0u128, |mut s, &c| {
                s <<= 1;
                if c == b'O' {
                    s += 1;
                }
                s
            })
        })
        .fold(0u128, |s, c| s.rotate_left(1) ^ c)
}
