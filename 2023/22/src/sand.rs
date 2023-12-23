use std::fmt::{Debug, Formatter};
use std::fs::read;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Coord {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<usize> = s.split(",").filter_map(|s| s.parse().ok()).collect();
        assert_eq!(s.len(), 3);
        Ok(Self {
            x: s[0],
            y: s[1],
            z: s[2],
        })
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Brick {
    id: usize,
    start: Coord,
    end: Coord,
}

#[derive(Debug)]
pub(crate) struct ParseError;
impl FromStr for Brick {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("~").unwrap();
        let (start, end) = (Coord::from_str(start)?, Coord::from_str(end)?);
        assert!(start.x <= end.x);
        assert!(start.y <= end.y);
        assert!(start.z <= end.z);
        Ok(Self { id: 0, start, end })
    }
}

impl Brick {
    fn with_id(&mut self, id: usize) -> &Self {
        self.id = id;
        self
    }

    fn xy(&self) -> Vec<(usize, usize)> {
        if self.start.x < self.end.x && self.start.y < self.end.y {
            println!("non-trivial {:?}~{:?}", self.start, self.end)
        }
        (self.start.x..=self.end.x)
            .into_iter()
            .flat_map(|x| (self.start.y..=self.end.y).into_iter().map(move |y| (x, y)))
            .collect()
    }

    fn fall_to(&self, min_height: usize) -> Self {
        let mut res = *self;
        if min_height > res.start.z {
            println!(
                "odd move {} to {min_height}: {:?}~{:?}",
                self.id, self.start, self.end
            );
        }
        assert!(min_height <= res.start.z);
        (res.start.z, res.end.z) = (min_height, (res.end.z - res.start.z) + min_height);
        res
    }

    fn height(&self) -> usize {
        self.end.z - self.start.z + 1
    }
}

pub(crate) fn scan(path: &str) -> Vec<Brick> {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(i, s)| *Brick::from_str(&s).unwrap().with_id(i + 1))
        .collect()
}

pub(crate) struct FallenBrick {
    supported_by: Vec<usize>,
}

impl FallenBrick {
    pub(crate) fn only_support(&self) -> Option<usize> {
        match self.supported_by.len() {
            1 => Some(self.supported_by[0]),
            _ => None,
        }
    }
}

pub(crate) fn fall(bricks: &Vec<Brick>) -> Vec<FallenBrick> {
    let mut bricks = bricks.clone();
    bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z)); // the ones lowest to the ground fall first
    let mut space = [[0usize; 10]; 10]; // last seen id. 0 = ground
    let mut result = Vec::new();
    let mut top_level = vec![0usize; bricks.len() + 1];

    for b in bricks {
        let support: Vec<(usize, usize)> = b
            .xy()
            .iter()
            .map(|&(x, y)| {
                let id = space[x][y];
                (id, top_level[id])
            })
            .collect();

        let max = support.iter().map(|&(_, h)| h).max().unwrap();
        let mut supported_by: Vec<usize> = support
            .into_iter()
            .filter_map(|(id, x)| if id > 0 && x == max { Some(id) } else { None })
            .collect();
        // this is needed as we can get support in more than a single point
        supported_by.sort();
        supported_by.dedup();

        result.push(FallenBrick { supported_by });
        b.xy().iter().for_each(|&(x, y)| space[x][y] = b.id);
        top_level[b.id] = max + b.height();
    }
    result
}
