use std::collections::HashMap;

pub(crate) fn to_nodes(lines: Vec<String>) -> HashMap<String, (String, String)> {
    lines.iter().
        map(|s| s.trim()).filter(|s| !s.is_empty()).
        fold(HashMap::<String, (String, String)>::new(), |mut sum, s| {
            let parts: Vec<&str> = s.split(" = ").collect();
            assert_eq!(parts.len(), 2);

            let lr: Vec<&str> = parts[1]
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(", ")
                .collect();
            assert_eq!(lr.len(), 2);
            sum.insert(parts[0].to_string(), (lr[0].to_string(), lr[1].to_string()));
            sum
        })
}

pub(crate) enum Dir { L, R }

impl Dir {
    pub(crate) fn from_char(c: char) -> Option<Dir> {
        match c {
            'L' => Some(Dir::L),
            'R' => Some(Dir::R),
            _ => None
        }
    }
}

pub(crate) fn path<C>(nodes: &HashMap<String, (String, String)>,
                      dirs: &Vec<Dir>,
                      start: &String,
                      done: C,
) -> u64
    where C: Fn(&String) -> bool
{
    let mut pos = start;
    dirs.iter().cycle()
        .enumerate()
        .find(|(_, next)| {
            match next {
                Dir::L => pos = &nodes.get(pos).unwrap().0,
                Dir::R => pos = &nodes.get(pos).unwrap().1,
            }
            done(pos)
        })
        .unwrap()
        .0 as u64 + 1
}
