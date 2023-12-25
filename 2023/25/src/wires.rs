use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read;

pub(crate) type Graph = HashMap<String, HashSet<String>>;
fn parse(input: String) -> Graph {
    let mut links: Graph = HashMap::new();
    for line in input.lines() {
        let (n, os) = line.split_once(": ").unwrap();
        let n = n.to_string();
        for o in os.split_whitespace() {
            let o = o.to_string();
            links.entry(n.clone()).or_default().insert(o.clone());
            links.entry(o.clone()).or_default().insert(n.clone());
        }
    }
    links
}

pub(crate) fn scan(path: &str) -> Graph {
    let s = String::from_utf8(read(path).unwrap().to_vec()).unwrap();
    (move |s: String| parse(s))(s)
}

pub(crate) fn find_bridge(graph: &Graph) -> (String, String) {
    let mut paths: HashMap<(String, String), usize> = HashMap::new();
    for start in graph.keys().cloned() {
        let mut to_see = VecDeque::new();
        to_see.push_back(start.clone());
        let mut seen = HashSet::new();
        seen.insert(start.clone());
        while let Some(node) = to_see.pop_front() {
            for n in graph[&node].iter().cloned() {
                if !seen.contains(&n) {
                    to_see.push_back(n.clone());
                    seen.insert(n.clone());
                    let edge = if n < node {
                        (n.clone(), node.clone())
                    } else {
                        (node.clone(), n.clone())
                    };
                    *paths.entry(edge).or_default() += 1;
                }
            }
        }
    }
    paths.into_iter().max_by_key(|&(_, v)| v).unwrap().0
}

pub(crate) fn bfs_reach(graph: &Graph, node: &String) -> usize {
    let mut seen: HashSet<&str> = HashSet::new();
    let mut open: VecDeque<&str> = VecDeque::from([node.as_str()]);

    while let Some(node) = open.pop_front() {
        if seen.contains(node) {
            continue;
        }
        graph
            .get(node)
            .unwrap()
            .iter()
            .map(|s| s.as_str())
            .filter(|s| !seen.contains(s))
            .for_each(|s| open.push_back(s));
        seen.insert(node);
    }
    seen.len()
}
