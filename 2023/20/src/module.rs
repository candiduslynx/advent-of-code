use std::collections::{HashMap, VecDeque};
use std::fs::read;
use std::io::BufRead;
use std::str::FromStr;

pub(crate) type Signal = (String, bool, String); // from, what, to

/// Simple send-all logic
#[derive(Clone, Debug)]
pub(crate) struct Sender {
    pub(crate) dst: Vec<String>,
}

impl Sender {
    fn new(dst: &Vec<String>) -> Self {
        Self {
            dst: dst.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn send(&self, from: &str, s: bool) -> Vec<Signal> {
        self.dst
            .iter()
            .map(|to| (from.to_string(), s, to.clone()))
            .collect()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Type {
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Clone, Debug)]
pub(crate) struct Module {
    pub(crate) name: String,
    typ: Type,
    state: bool,
    pub(crate) src: HashMap<String, bool>,
    send: Sender,
}

impl Module {
    fn receive(&mut self, from: &str, s: bool) -> Option<Vec<Signal>> {
        match self.typ {
            Type::Broadcast => Some(self.send.send(&self.name, s)),
            Type::FlipFlop => match s {
                true => None,
                false => {
                    self.state = !self.state;
                    Some(self.send.send(&self.name, self.state))
                }
            },
            Type::Conjunction => {
                self.src.entry(from.to_string()).and_modify(|e| *e = s);
                match s {
                    false => Some(self.send.send(&self.name, true)), // don't even iterate as at least 1 input is false
                    true => {
                        if self.src.values().all(|b| *b) {
                            Some(self.send.send(&self.name, false))
                        } else {
                            Some(self.send.send(&self.name, true))
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn sends_to(&self, to: &str) -> bool {
        self.send.dst.contains(&to.to_string())
    }
}

#[derive(Debug)]
pub(crate) struct ParseErr;

impl FromStr for Module {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, dst) = s.split_once(" -> ").unwrap();
        let dst: Vec<String> = dst
            .split(", ")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let (typ, name) = match name.as_bytes()[0] {
            b'b' => (Type::Broadcast, name),
            b'%' => (Type::FlipFlop, &name[1..]),
            b'&' => (Type::Conjunction, &name[1..]),
            _ => return Err(ParseErr),
        };

        Ok(Self {
            name: name.to_string(),
            typ,
            state: false,
            src: HashMap::new(),
            send: Sender::new(&dst),
        })
    }
}

pub(crate) fn scan(path: &str) -> Result<HashMap<String, Module>, ParseErr> {
    let modules: Vec<Module> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| Module::from_str(s.as_str()).unwrap())
        .collect();

    let mut result = HashMap::new();
    for m in modules.iter() {
        let mut m = m.clone();
        modules
            .iter()
            .filter(|&d| d.send.dst.contains(&m.name))
            .for_each(|src| {
                m.src.insert(src.name.clone(), false);
            });
        result.insert(m.name.clone(), m);
    }

    Ok(result)
}

/// `send` will return the amount of (low, high) signals that were caused by this press along with all signals that were sent during this iteration.
pub(crate) fn send(modules: &mut HashMap<String, Module>, s: Signal) -> (u64, u64, Vec<Signal>) {
    let mut open: VecDeque<Signal> = VecDeque::from([s]);
    let mut done: Vec<Signal> = Vec::new();

    let mut low = 0u64;
    let mut high = 0u64;

    while !open.is_empty() {
        let s = open.pop_front().unwrap();
        if s.1 {
            high += 1;
        } else {
            low += 1;
        }
        match modules.get_mut(&s.2) {
            None => {}
            Some(m) => match m.receive(&s.0, s.1) {
                None => {}
                Some(signals) => signals.into_iter().for_each(|s| open.push_back(s)),
            },
        }
        done.push(s);
    }
    (low, high, done)
}
