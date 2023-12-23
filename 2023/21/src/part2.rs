use crate::field;

pub(crate) fn solve(path: &str) -> u64 {
    let (f, start) = field::scan(path);
    let d = field::distances(&f, &start);

    let lim = f.len() / 2;
    // how much even corners are there (the ones that have even parity & not fully covered
    let even_corners: u64 = d
        .iter()
        .enumerate()
        .filter_map(|(i, x)| if i % 2 == 0 && i > lim { Some(x) } else { None })
        .sum();

    // how much even corners are there (the ones that have odd parity & not fully covered
    let odd_corners: u64 = d
        .iter()
        .enumerate()
        .filter_map(|(i, x)| if i % 2 == 1 && i > lim { Some(x) } else { None })
        .sum();

    // total amount od nodes designated by even amount of steps
    let even_full: u64 = (0..d.len()).step_by(2).map(|i| d[i]).sum();

    // total amount od nodes designated by odd amount of steps
    let odd_full: u64 = (1..d.len()).step_by(2).map(|i| d[i]).sum();
    const STEPS: usize = 26501365;
    // total wraps required in each direction to cover fully
    let n = ((STEPS - (f.len() / 2)) / f.len()) as u64;
    let even = n * n;
    let odd = (n + 1) * (n + 1);

    odd * odd_full + even * even_full - ((n + 1) * odd_corners) + (n * even_corners)
}
