use crate::workflow;
use crate::workflow::{next, Part, Workflow};
use std::collections::HashMap;
use std::iter::once;

pub(crate) fn solve(path: &str) -> u64 {
    let (w, p) = workflow::scan(path).unwrap();
    sort(&p, &w)
}

pub(crate) fn sort(parts: &Vec<Part>, workflows: &HashMap<String, Workflow>) -> u64 {
    let start = workflows.get("in").unwrap();
    let mut open: Vec<(&Part, Workflow)> = parts.iter().zip(once(start.clone()).cycle()).collect();
    let mut result = 0u64;

    while !open.is_empty() {
        let mut next_open = Vec::new();
        for (p, at) in open.into_iter() {
            let r = next(p, &at, workflows);
            match r {
                (Some(term), None) => {
                    if term {
                        result += p.rating()
                    }
                }
                (None, Some(next)) => next_open.push((p, next)),
                _o => panic!("odd state {_o:?}"),
            }
        }
        open = next_open;
    }
    result
}
