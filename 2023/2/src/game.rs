#[derive(Debug, PartialEq)]
struct Mov {
    r: u32,
    g: u32,
    b: u32,
}

impl Mov {
    fn possible(&self, max_r: u32, max_g: u32, max_b: u32) -> bool {
        self.r <= max_r && self.g <= max_g && self.b <= max_b
    }

    fn add_color(mut self, balls: &str) -> Self {
        let parts: Vec<&str> = balls.split_whitespace().collect();
        assert_eq!(parts.len(), 2);

        let amount: u32 = parts[0].parse().unwrap();
        match parts[1] {
            "red" => self.r += amount,
            "green" => self.g += amount,
            "blue" => self.b += amount,
            _ => panic!("{}", balls)
        }

        self
    }

    fn power(self) -> u32 {
        self.r * self.g * self.b
    }

    fn from_str(balls: &str) -> Self {
        balls.split(", ").
            fold(Mov { r: 0, g: 0, b: 0 }, |sum, x| sum.add_color(x))
    }
}

#[derive(Debug)]
pub(crate) struct Game {
    moves: Vec<Mov>,
    pub(crate) id: u32,
}

impl Game {
    pub(crate) fn possible(&self, max_r: u32, max_g: u32, max_b: u32) -> bool {
        self.moves.iter().all(|m| m.possible(max_r, max_g, max_b))
    }

    fn minimal(&self) -> Mov {
        self.moves.iter().
            fold(Mov { r: 0, g: 0, b: 0 },
                 |mut sum, x| {
                     if sum.r < x.r { sum.r = x.r }
                     if sum.g < x.g { sum.g = x.g }
                     if sum.b < x.b { sum.b = x.b }
                     sum
                 })
    }

    pub(crate) fn power(self) -> u32 {
        self.minimal().power()
    }


    pub(crate) fn from_str(game: String) -> Self {
        if game.is_empty() {
            return Game { moves: vec![], id: 0 };
        }

        let parts: Vec<&str> = game.split(": ").collect();
        assert_eq!(parts.len(), 2);

        Game {
            moves: parts[1].split("; ").map(|m| Mov::from_str(m)).collect(),
            id: parts[0].strip_prefix("Game ").unwrap().parse().unwrap(),
        }
    }
}
