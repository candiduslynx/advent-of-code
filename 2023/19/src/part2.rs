use crate::workflow;

pub(crate) fn solve(path: &str) -> u64 {
    let (w, _) = workflow::scan(path).unwrap();
    workflow::possibilities(&w)
}
