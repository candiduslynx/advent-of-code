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
    for col in 0..cols {
        let mut none: Option<usize> = None; // first available idx of none
        for row in 0..rows {
            match g[row][col] {
                b'#' => none = None,
                b'.' => none = none.or(Some(row)),
                b'O' => match none {
                    Some(val) => {
                        (g[val][col], g[row][col], none) = (g[row][col], g[val][col], Some(val + 1))
                    }
                    _ => {}
                },
                _b => panic!("bad byte {_b}"),
            }
        }
    }
}

fn tilt_south(g: &mut Vec<Vec<u8>>) {
    let (rows, cols) = (g.len(), g[0].len());
    for col in 0..cols {
        let mut none: Option<usize> = None; // first available idx of none
        for row in (0..rows).rev() {
            match g[row][col] {
                b'#' => none = None,
                b'.' => none = none.or(Some(row)),
                b'O' => match none {
                    Some(val) => {
                        (g[val][col], g[row][col], none) = (g[row][col], g[val][col], Some(val - 1))
                    }
                    _ => {}
                },
                _b => panic!("bad byte {_b}"),
            }
        }
    }
}

fn tilt_west(g: &mut Vec<Vec<u8>>) {
    g.iter_mut().for_each(|row| {
        let mut none: Option<usize> = None; // first available idx of none
        for col in 0..row.len() {
            match row[col] {
                b'#' => none = None,
                b'.' => none = none.or(Some(col)),
                b'O' => match none {
                    Some(val) => (row[val], row[col], none) = (row[col], row[val], Some(val + 1)),
                    _ => {}
                },
                _b => panic!("bad byte {_b}"),
            }
        }
    });
}

fn tilt_east(g: &mut Vec<Vec<u8>>) {
    g.iter_mut().for_each(|row| {
        let mut none: Option<usize> = None; // first available idx of none
        for col in (0..row.len()).rev() {
            match row[col] {
                b'#' => none = None,
                b'.' => none = none.or(Some(col)),
                b'O' => match none {
                    Some(val) => (row[val], row[col], none) = (row[col], row[val], Some(val - 1)),
                    _ => {}
                },
                _b => panic!("bad byte {_b}"),
            }
        }
    });
}

pub(crate) fn to_str(v: &Vec<Vec<u8>>) -> String {
    v.iter()
        .map(|row| std::str::from_utf8(row.as_slice()).unwrap())
        .collect()
}
