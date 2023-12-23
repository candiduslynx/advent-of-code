use crate::field;

pub(crate) fn solve(path: &str) -> u64 {
    let (f, start) = field::scan(path);
    let d = field::distances(&f, &start);

    const STEPS: usize = 64;
    (STEPS & 1..=STEPS.min(d.len() - 1))
        .step_by(2)
        .map(|i| d[i])
        .sum()
}
