use std::collections::{HashMap, VecDeque};
use std::fs::read;
use std::io::BufRead;
use std::str::FromStr;

pub(crate) type Signal = (String, bool, String); // from, what, to

/// Simple send-all logic
#[derive(Clone, Debug)]
pub(crate) struct Sender {
    name: String,
    dst: Vec<String>,
}

impl Sender {
    fn new(name: String, dst: Vec<String>) -> Self {
        Self { name, dst }
    }

    fn send(&self, s: bool) -> Vec<Signal> {
        self.dst
            .iter()
            .map(|to| (self.name.clone(), s, to.clone()))
            .collect()
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Module {
    Broadcast { sender: Sender },
    FlipFlop { sender: Sender, state: bool },
    Conjunction { sender: Sender, src: HashMap<String, bool> },
}

impl Module {
    fn receive(&mut self, from: &str, signal: bool) -> Option<Vec<Signal>> {
        match self {
            Module::Broadcast { sender } => Some(sender.send(signal)),
            Module::FlipFlop { sender, state } => match signal {
                true => None,
                false => {
                    *state = !*state;
                    Some(sender.send(*state))
                }
            },
            Module::Conjunction { sender, src } => {
                src.entry(from.to_string()).and_modify(|e| *e = signal);
                match signal {
                    false => Some(sender.send(true)), // don't even iterate as at least 1 input is false
                    true => Some(sender.send(src.values().any(|b| !*b))),
                }
            }
        }
    }

    pub(crate) fn sends_to(&self, to: &str) -> bool {
        let sender = match self {
            Module::Broadcast { sender } => sender,
            Module::FlipFlop { sender, .. } => sender,
            Module::Conjunction { sender, .. } => sender,
        };
        sender.dst.contains(&to.to_string())
    }

    fn add_src(&mut self, from: &str) {
        if let Module::Conjunction { src, .. } = self {
            src.insert(from.to_string(), false);
        }
    }

    pub(crate) fn name(&self) -> &str {
        let sender = match self {
            Module::Broadcast { sender } => sender,
            Module::FlipFlop { sender, .. } => sender,
            Module::Conjunction { sender, .. } => sender,
        };
        &sender.name
    }

    pub(crate) fn src(&self) -> Vec<String> {
        if let Module::Conjunction { src, .. } = self {
            return src.keys().map(|s| s.clone()).collect();
        }
        Vec::new()
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

        match name.as_bytes()[0] {
            b'b' => Ok(Module::Broadcast { sender: Sender::new(name.to_string(), dst) }),
            b'%' => Ok(Module::FlipFlop { sender: Sender::new((&name[1..]).to_string(), dst), state: false }),
            b'&' => Ok(Module::Conjunction { sender: Sender::new((&name[1..]).to_string(), dst), src: HashMap::new() }),
            _ => Err(ParseErr),
        }
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
        let name = m.name();
        let mut m = m.clone();
        modules
            .iter()
            .filter(|&d| d.sends_to(name))
            .for_each(|src| m.add_src(src.name()));
        result.insert(name.to_string(), m);
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
