#[derive(Debug, Clone, Copy, PartialEq)]
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
}

pub(crate) fn ground_load(ground: &Vec<Vec<Ground>>) -> u64 {
    let len = ground.len();
    ground
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().filter(|&&g| g == Ground::Round).count() as u64 * (len - i) as u64
        })
        .sum()
}

pub(crate) fn cycle(g: &mut Vec<Vec<Ground>>) {
    tilt_north(g);
    tilt_west(g);
    tilt_south(g);
    tilt_east(g);
}

pub(crate) fn tilt_north(g: &mut Vec<Vec<Ground>>) {
    let (rows, cols) = (g.len(), g[0].len());
    for col in 0..cols {
        let mut none: Option<usize> = None; // first available idx of none
        for row in 0..rows {
            let v = g[row][col];
            match v {
                Ground::Cube => none = None,
                Ground::None => none = none.or(Some(row)),
                Ground::Round => match none {
                    Some(val) => {
                        (g[val][col], g[row][col], none) = (g[row][col], g[val][col], Some(val + 1))
                    }
                    None => {}
                },
            }
        }
    }
}

fn tilt_south(g: &mut Vec<Vec<Ground>>) {
    let (rows, cols) = (g.len(), g[0].len());
    for col in 0..cols {
        let mut none: Option<usize> = None; // first available idx of none
        for row in (0..rows).rev() {
            let v = g[row][col];
            match v {
                Ground::Cube => none = None,
                Ground::None => none = none.or(Some(row)),
                Ground::Round => match none {
                    Some(val) => {
                        (g[val][col], g[row][col], none) = (g[row][col], g[val][col], Some(val - 1))
                    }
                    None => {}
                },
            }
        }
    }
}

fn tilt_west(g: &mut Vec<Vec<Ground>>) {
    g.iter_mut().for_each(|row| {
        let mut none: Option<usize> = None; // first available idx of none
        for col in 0..row.len() {
            let v = row[col];
            match v {
                Ground::Cube => none = None,
                Ground::None => none = none.or(Some(col)),
                Ground::Round => match none {
                    Some(val) => (row[val], row[col], none) = (row[col], row[val], Some(val + 1)),
                    None => {}
                },
            }
        }
    });
}

fn tilt_east(g: &mut Vec<Vec<Ground>>) {
    g.iter_mut().for_each(|row| {
        let mut none: Option<usize> = None; // first available idx of none
        for col in (0..row.len()).rev() {
            let v = row[col];
            match v {
                Ground::Cube => none = None,
                Ground::None => none = none.or(Some(col)),
                Ground::Round => match none {
                    Some(val) => (row[val], row[col], none) = (row[col], row[val], Some(val - 1)),
                    None => {}
                },
            }
        }
    });
}

pub(crate) fn to_str(v: &Vec<Vec<Ground>>) -> String {
    v.iter()
        .flat_map(|row| row.iter().map(|g| g.to_char()))
        .collect()
}
