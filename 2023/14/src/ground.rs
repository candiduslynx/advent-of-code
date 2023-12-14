#[derive(Debug, Clone, Copy)]
pub(crate) enum Ground {
    None,
    Round,
    Cube,
}

impl Ground {
    pub(crate) fn from_char(c: &char) -> Self {
        match c {
            '.' => Ground::None,
            'O' => Ground::Round,
            '#' => Ground::Cube,
            _ => panic!("bad char {c}"),
        }
    }

    pub(crate) fn to_char(&self) -> char {
        match self {
            Ground::None => '.',
            Ground::Round => 'O',
            Ground::Cube => '#',
        }
    }

    pub(crate) fn from_str(s: &str) -> Vec<Self> {
        s.chars().map(|c| Ground::from_char(&c)).collect()
    }

    fn load(v: &Vec<Self>) -> u64 {
        let l = v.len();
        v.iter()
            .enumerate()
            .map(|(i, v)| match v {
                Ground::Round => (l - i) as u64,
                _ => 0,
            })
            .sum()
    }
}

pub(crate) fn ground_load(ground: &Vec<Vec<Ground>>) -> u64 {
    transpose(ground).iter().map(|row| Ground::load(row)).sum()
}

pub(crate) fn cycle(ground: &Vec<Vec<Ground>>) -> Vec<Vec<Ground>> {
    tilt_east(&tilt_south(&tilt_west(&tilt_north(ground))))
}

pub(crate) fn tilt_north(ground: &Vec<Vec<Ground>>) -> Vec<Vec<Ground>> {
    transpose(
        &transpose(ground)
            .iter()
            .map(|row| tilt_to_start(row))
            .collect(),
    )
}

fn tilt_south(ground: &Vec<Vec<Ground>>) -> Vec<Vec<Ground>> {
    transpose(
        &transpose(ground)
            .iter()
            .map(|row| tilt_to_end(row))
            .collect(),
    )
}

fn tilt_west(ground: &Vec<Vec<Ground>>) -> Vec<Vec<Ground>> {
    ground.iter().map(|row| tilt_to_start(row)).collect()
}

fn tilt_east(ground: &Vec<Vec<Ground>>) -> Vec<Vec<Ground>> {
    ground.iter().map(|row| tilt_to_end(row)).collect()
}

fn tilt_to_end(v: &Vec<Ground>) -> Vec<Ground> {
    let mut v = v.clone();
    v.reverse();
    v = tilt_to_start(&v);
    v.reverse();
    v
}

fn tilt_to_start(v: &Vec<Ground>) -> Vec<Ground> {
    let mut result: Vec<Ground> = Vec::new();
    let mut none: Option<usize> = None; // first available idx of none
    for i in 0..v.len() {
        result.push(v[i]);
        match v[i] {
            Ground::Cube => none = None,
            Ground::None => none = none.or(Some(i)),
            Ground::Round => {
                match none {
                    Some(val) => {
                        // use empty space
                        (result[val], result[i], none) = (result[i], result[val], Some(val + 1));
                    }
                    None => {}
                }
            }
        }
    }
    result
}

fn transpose(v: &Vec<Vec<Ground>>) -> Vec<Vec<Ground>> {
    if v.len() == 0 {
        return v.to_vec();
    }

    let mut result: Vec<Vec<Ground>> = Vec::with_capacity(v[0].len());
    for i in 0..v[0].len() {
        result.push(v.iter().map(|v| v[i]).collect());
    }

    result
}

pub(crate) fn print(v: &Vec<Vec<Ground>>) {
    let len = v.len();
    to_str(v)
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % len == 0 {
                Some('\n')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .for_each(|c| print!("{c}",));
}

pub(crate) fn to_str(v: &Vec<Vec<Ground>>) -> String {
    v.iter()
        .flat_map(|row| row.iter().map(|g| g.to_char()))
        .collect()
}
