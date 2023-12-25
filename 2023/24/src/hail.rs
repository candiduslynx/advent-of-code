use std::fs::read;
use std::io::BufRead;
use std::ops::{Add, Div, Mul, RangeInclusive, Sub};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Coord {
    x: f64,
    y: f64,
    z: f64,
}

impl From<&str> for Coord {
    fn from(s: &str) -> Self {
        let s: Vec<f64> = s
            .split(", ")
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        assert_eq!(s.len(), 3);
        Self {
            x: s[0],
            y: s[1],
            z: s[2],
        }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Coord {
    type Output = Coord;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Coord {
    type Output = Coord;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

/// vector mul
impl Mul for Coord {
    type Output = Coord;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Coord {
    fn collinear(&self, rhs: Self) -> bool {
        let m = *self * rhs;
        m.x == 0.0 && m.y == 0.0 && m.z == 0.0
    }

    fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn rounded(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
        }
    }

    pub(crate) fn sum(&self) -> f64 {
        self.x + self.y + self.z
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Hailstone {
    coord: Coord,
    velocity: Coord,
}

impl From<&str> for Hailstone {
    fn from(s: &str) -> Self {
        let (coord, velocity) = s.split_once(" @ ").unwrap();
        let (coord, velocity) = (Coord::from(coord), Coord::from(velocity));
        Self { coord, velocity }
    }
}

impl Hailstone {
    fn at(&self, time: f64) -> Coord {
        self.coord + self.velocity * time
    }

    /// return (x,vx)
    fn x(&self) -> (f64, f64) {
        (self.coord.x, self.velocity.x)
    }

    /// return (y,vy)
    fn y(&self) -> (f64, f64) {
        (self.coord.y, self.velocity.y)
    }

    fn intersect_xy(&self, rhs: &Self) -> Option<Coord> {
        let ((x1, vx1), (y1, vy1)) = (self.x(), self.y());
        let ((x2, vx2), (y2, vy2)) = (rhs.x(), rhs.y());

        if let Some((x, y)) = solve_linear_2_variables((vx1, -vx2, x1 - x2), (vy1, -vy2, y1 - y2)) {
            if x >= 0.0 && y >= 0.0 {
                return Some(self.at(x));
            }
        }
        None
    }

    fn plane(&self, rhs: Self) -> (Coord, f64) {
        let p12 = self.coord - rhs.coord;
        let v12 = self.velocity - rhs.velocity;
        let vv = self.velocity * rhs.velocity;
        return (p12 * v12, p12.dot(vv));
    }

    fn rock(nc: [Self; 3]) -> (Coord, f64) {
        let a = nc[0].plane(nc[1]);
        let b = nc[0].plane(nc[2]);
        let c = nc[1].plane(nc[2]);

        let w = ((b.0 * c.0) * a.1 + (c.0 * a.0) * b.1 + (a.0 * b.0) * c.1) / a.0.dot(b.0 * c.0);
        let w = w.rounded(); // less carry

        let w1 = nc[0].velocity - w;
        let w2 = nc[1].velocity - w;
        let ww = w1 * w2;
        let s = ww.dot(ww);

        (
            w1 * ww.dot(nc[1].coord * w2) - w2 * ww.dot(nc[0].coord * w1)
                + ww * nc[0].coord.dot(ww),
            s,
        )
    }
}

fn solve_linear_2_variables(
    (a1, b1, c1): (f64, f64, f64),
    (a2, b2, c2): (f64, f64, f64),
) -> Option<(f64, f64)> {
    let d = b1 * a2 - b2 * a1;
    if d == 0.0 {
        None
    } else {
        Some(((b2 * c1 - b1 * c2) / d, (c2 * a1 - c1 * a2) / d))
    }
}

pub(crate) fn scan(path: &str) -> Vec<Hailstone> {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| Hailstone::from(s.as_str()))
        .collect()
}

pub(crate) fn intersect_xy(stones: &Vec<Hailstone>, range: RangeInclusive<f64>) -> u64 {
    (0..stones.len())
        .flat_map(|i| (i + 1..stones.len()).map(move |j| (i, j)))
        .map(|(i, j)| (&stones[i], &stones[j]))
        .filter_map(|(a, b)| a.intersect_xy(b))
        .filter(|c| range.start() <= &c.x && &c.x <= range.end())
        .filter(|c| range.start() <= &c.y && &c.y <= range.end())
        .count() as u64
}

pub(crate) fn rock(stones: &Vec<Hailstone>) -> (Coord, f64) {
    for i in 0..stones.len() {
        let a = stones[i];
        for j in i + 1..stones.len() {
            let b = stones[j];
            if a.velocity.collinear(b.velocity) {
                continue;
            }
            for k in j + 1..stones.len() {
                let c = stones[k];
                if a.velocity.collinear(c.velocity) || b.velocity.collinear(c.velocity) {
                    continue;
                }
                return Hailstone::rock([a, b, c]);
            }
        }
    }
    panic!("no solution")
}
