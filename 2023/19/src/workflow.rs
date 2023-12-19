use std::collections::HashMap;
use std::fs::read;
use std::iter::once;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct ParseError;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Scale {
    X,
    M,
    A,
    S,
}
impl FromStr for Scale {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            match s.as_bytes()[0] {
                b'x' => Ok(Scale::X),
                b'm' => Ok(Scale::M),
                b'a' => Ok(Scale::A),
                b's' => Ok(Scale::S),
                _ => Err(ParseError),
            }
        } else {
            Err(ParseError)
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Decision {
    scale: Option<Scale>,
    term: Option<bool>, // termination rule (A or R)
    next: Option<String>,
    gt: Option<bool>, // None -> no cmp, Some(true) => ? > val
    val: Option<u64>,
}

impl Decision {
    fn parse_next(s: &str) -> (Option<bool>, Option<String>) {
        match s {
            "A" => (Some(true), None),
            "R" => (Some(false), None),
            _ => (None, Some(String::from(s))),
        }
    }

    fn apply(&self, p: &Part) -> (Option<bool>, Option<String>) {
        match self.scale {
            None => (self.term, self.next.clone()),
            Some(s) => {
                let v = match s {
                    Scale::X => p.x,
                    Scale::M => p.m,
                    Scale::A => p.a,
                    Scale::S => p.s,
                };
                if self.gt.unwrap() && v > self.val.unwrap() {
                    (self.term, self.next.clone())
                } else if !self.gt.unwrap() && v < self.val.unwrap() {
                    (self.term, self.next.clone())
                } else {
                    (None, None)
                }
            }
        }
    }
}

impl FromStr for Decision {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(":") {
            None => {
                let (term, next) = Decision::parse_next(s);
                Ok(Self {
                    scale: None,
                    term,
                    next,
                    gt: None,
                    val: None,
                })
            }
            Some((cond, destination)) => {
                let (term, next) = Decision::parse_next(destination);
                if cond.len() < 3 {
                    return Err(ParseError);
                }
                let scale = Some(Scale::from_str(&cond[..1])?);
                let gt = Some(match cond.as_bytes()[1] {
                    b'>' => Ok(true),
                    b'<' => Ok(false),
                    _ => Err(ParseError),
                }?);
                let val = Some(cond[2..].parse::<u64>().map_err(|_| ParseError)?);

                Ok(Self {
                    scale,
                    term,
                    next,
                    gt,
                    val,
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Workflow {
    decisions: Vec<Decision>,
    name: String,
}

impl Workflow {
    fn apply(&self, p: &Part) -> (Option<bool>, Option<String>) {
        let d: Vec<(Option<bool>, Option<String>)> =
            self.decisions.iter().map(|s| s.apply(p)).collect();

        d.into_iter()
            .find(|(term, next)| term.is_some() || next.is_some())
            .unwrap()
    }
}

impl FromStr for Workflow {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((name, rest)) = s.split_once("{") else {
            return Err(ParseError);
        };
        let name = String::from(name);

        let Some(rest) = rest.strip_suffix("}") else {
            return Err(ParseError);
        };

        Ok(Self {
            decisions: rest
                .split(",")
                .map(|s| Decision::from_str(s).unwrap())
                .collect(),
            name,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn new() -> Self {
        Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        }
    }

    pub(crate) fn rating(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s.as_bytes()[0] != b'{' || s.as_bytes()[s.len() - 1] != b'}' {
            return Err(ParseError);
        }

        Ok(s[1..s.len() - 1]
            .split(",")
            .map(|s| {
                let Some((scale, val)) = s.split_once("=") else {
                    return Err(ParseError);
                };
                let scale = Scale::from_str(scale)?;
                let val = val.parse::<u64>().map_err(|_| ParseError)?;
                Ok((scale, val))
            })
            .fold(Part::new(), |mut p, el| {
                let (s, v) = el.unwrap();
                match s {
                    Scale::X => p.x = v,
                    Scale::M => p.m = v,
                    Scale::A => p.a = v,
                    Scale::S => p.s = v,
                }
                p
            }))
    }
}

pub(crate) fn scan(path: &str) -> Result<(HashMap<String, Workflow>, Vec<Part>), ParseError> {
    let data = String::from_utf8(read(path).unwrap()).map_err(|_| ParseError)?;
    let parts: Vec<&str> = data.split("\n\n").collect();
    if parts.len() != 2 {
        return Err(ParseError);
    }

    let w = parts[0]
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| Workflow::from_str(s).unwrap())
        .fold(HashMap::new(), |mut h, w| {
            h.insert(w.name.clone(), w);
            h
        });

    let parts = parts[1]
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| Part::from_str(s).unwrap())
        .collect();
    Ok((w, parts))
}

pub(crate) fn next(
    p: &Part,
    w: &Workflow,
    workflows: &HashMap<String, Workflow>,
) -> (Option<bool>, Option<Workflow>) {
    let r = w.apply(p);
    match r {
        (Some(term), None) => (Some(term), None),
        (None, Some(next)) => match workflows.get(&next) {
            Some(w) => (None, Some(w.clone())),
            None => panic!("no workflow named {next}"),
        },
        o => panic!("{:?}", o),
    }
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

#[derive(Debug, Clone)]
struct PartRange {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>,
}

impl PartRange {
    fn workflow(&self, w: &Workflow) -> (Vec<(PartRange, String)>, u64) {
        let mut state = self.clone();
        let mut result = 0u64;
        let mut next = Vec::new();
        for d in w.decisions.iter() {
            let (matched, skipped) = state.decision(d);
            match d.term {
                Some(false) => {}
                Some(true) => match matched {
                    Some(m) => result += m.possibilities(),
                    _ => {}
                },
                None => match matched {
                    Some(m) => next.push((m, d.next.clone().unwrap())),
                    _ => {}
                },
            }
            match skipped {
                None => break,
                Some(skipped) => state = skipped,
            }
        }
        (next, result)
    }

    /// split range in 2 parts: the one that will be affected by the decision & the one that will skip that
    fn decision(&self, d: &Decision) -> (Option<PartRange>, Option<PartRange>) {
        match d.scale {
            None => (Some(self.clone()), None),
            Some(scale) => {
                let val = d.val.unwrap();
                let r = match scale {
                    Scale::X => &self.x,
                    Scale::M => &self.m,
                    Scale::A => &self.a,
                    Scale::S => &self.s,
                };
                let (mut l, val, mut r) = PartRange::split(r, val);
                let (matched, skipped) = match d.gt.unwrap() {
                    true => {
                        // ? > val -> the matching are (val..r.end), skipped = [l.start, val]
                        l.end = val + 1; // we work on exclusive range
                        (r, l)
                    }
                    false => {
                        // ? < val -> the matching are (l.start, val), skipped = [val..r.end)
                        r.start = val;
                        (l, r)
                    }
                };
                let matched = if matched.is_empty() {
                    None
                } else {
                    Some(self.with_range_at_scale(matched, scale))
                };
                let skipped = if skipped.is_empty() {
                    None
                } else {
                    Some(self.with_range_at_scale(skipped, scale))
                };
                (matched, skipped)
            }
        }
    }

    fn with_range_at_scale(&self, r: Range<u64>, scale: Scale) -> Self {
        match scale {
            Scale::X => Self {
                x: r,
                ..self.clone()
            },
            Scale::M => Self {
                m: r,
                ..self.clone()
            },
            Scale::A => Self {
                a: r,
                ..self.clone()
            },
            Scale::S => Self {
                s: r,
                ..self.clone()
            },
        }
    }

    /// (start..at,at, at+1..end)
    fn split(r: &Range<u64>, at: u64) -> (Range<u64>, u64, Range<u64>) {
        if at < r.start {
            (at..at, at, r.clone())
        } else if at >= r.end {
            (r.clone(), at, at..at)
        } else {
            (r.start..at, at, at + 1..r.end)
        }
    }

    fn possibilities(self) -> u64 {
        let x = self.x.count() as u64;
        let m = self.m.count() as u64;
        let a = self.a.count() as u64;
        let s = self.s.count() as u64;
        x * m * a * s
    }
}

pub(crate) fn possibilities(workflows: &HashMap<String, Workflow>) -> u64 {
    let mut result = 0u64;
    let mut open = vec![(
        PartRange {
            x: 1..4001u64, // [1..4000)
            m: 1..4001u64, // [1..4000)
            a: 1..4001u64, // [1..4000)
            s: 1..4001u64, // [1..4000)
        },
        workflows.get("in").unwrap(),
    )];

    while !open.is_empty() {
        let mut next_open = Vec::new();
        for (r, w) in open.into_iter() {
            let (next, diff) = r.workflow(w);
            result += diff;
            next.into_iter()
                .map(|(r, s)| (r, workflows.get(&s).unwrap()))
                .for_each(|p| next_open.push(p));
        }
        open = next_open;
    }
    result
}
