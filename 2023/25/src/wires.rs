use std::collections::HashSet;
use std::fs::read;
use std::io::BufRead;

pub(crate) type Wire = (String, String);

pub(crate) fn scan(path: &str) -> HashSet<Wire> {
    let mut res = HashSet::new();
    for s in read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
    {
        let (from, to) = s.split_once(": ").unwrap();
        to.split_whitespace().for_each(|to| {
            res.insert((from.to_string(), to.to_string()));
            res.insert((to.to_string(), from.to_string()));
        });
    }
    res
}
