use crate::field;

pub(crate) fn solve(path: &str) -> u64 {
    let (f, start) = field::scan(path);
    const STEPS: usize = 64; // todo: update after test passes
    let w = field::walk(&f, &start, STEPS);
    // the even/odd stays the same, so we just need to calc properly
    (STEPS % 2..=STEPS)
        .step_by(2)
        .map(|idx| w[idx].len() as u64)
        .sum()
}
