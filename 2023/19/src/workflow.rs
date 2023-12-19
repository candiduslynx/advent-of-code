use std::collections::HashMap;
use std::fs::read;
use std::iter::once;
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
