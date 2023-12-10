use std::collections::HashMap;

pub(crate) enum Dir {
    L,
    R,
}

impl Dir {
    pub(crate) fn from_char(c: char) -> Option<Dir> {
        match c {
            'L' => Some(Dir::L),
            'R' => Some(Dir::R),
            _ => None,
        }
    }
}

pub(crate) struct Node<'a> {
    pub(crate) label: &'a str,
    left: &'a str,
    right: &'a str,
    left_index: usize,
    right_index: usize,
}

impl<'a> Node<'a> {
    pub(crate) fn from_lines(lines: &'a Vec<String>) -> Vec<Node<'a>> {
        let mut idx: HashMap<&str, usize> = HashMap::new();
        let mut nodes: Vec<Node> = Vec::new();

        for (i, s) in lines
            .iter()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .enumerate()
        {
            nodes.push(Node {
                label: &s[0..=2],
                left: &s[7..=9],
                right: &s[12..=14],
                left_index: 0,
                right_index: 0,
            });
            idx.insert(&s[0..=2], i);
        }

        nodes.iter_mut().for_each(|node| {
            node.left_index = *idx.get(node.left).unwrap();
            node.right_index = *idx.get(node.right).unwrap();
        });

        nodes
    }

    fn get_next(&self, dir: &Dir, nodes: &'a Vec<Node<'a>>) -> &'a Node {
        match dir {
            Dir::L => &nodes[self.left_index],
            Dir::R => &nodes[self.right_index],
        }
    }

    pub(crate) fn path_len<CHK>(start: &Node, dirs: &Vec<Dir>, nodes: &Vec<Node>, done: CHK) -> u64
    where
        CHK: Fn(&Node) -> bool,
    {
        let mut curr = start;
        dirs.iter()
            .cycle()
            .enumerate()
            .find(|(_, dir)| {
                curr = curr.get_next(dir, nodes);
                done(curr)
            })
            .unwrap()
            .0 as u64
            + 1
    }

    pub(crate) fn paths_len<CHK>(
        starts: &Vec<&Node>,
        dirs: &Vec<Dir>,
        nodes: &Vec<Node>,
        done: CHK,
    ) -> u64
    where
        CHK: Fn(&Node) -> bool + Clone,
    {
        starts
            .iter()
            .map(|node| Node::path_len(node, dirs, nodes, done.clone()))
            .fold(1u64, |s, x| lcm(s, x))
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a, b);
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
