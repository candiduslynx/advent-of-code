use crate::workflow;

pub(crate) fn solve(path: &str) -> u64 {
    let (w, p) = workflow::scan(path).unwrap();
    workflow::sort(&p, &w)
}
