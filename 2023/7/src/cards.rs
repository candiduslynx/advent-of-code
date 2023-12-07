use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(Copy, Clone, Debug, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Face {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Face {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Face::Two,
            '3' => Face::Three,
            '4' => Face::Four,
            '5' => Face::Five,
            '6' => Face::Six,
            '7' => Face::Seven,
            '8' => Face::Eight,
            '9' => Face::Nine,
            'T' => Face::Ten,
            'J' => Face::Jack,
            'Q' => Face::Queen,
            'K' => Face::King,
            'A' => Face::Ace,
            _ => panic!("{c}"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum HandStrength {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    Three = 3,
    FullHouse = 4,
    Four = 5,
    Five = 6,
}

impl HandStrength {
    fn from_cards(cards: &[Face; 5]) -> Self {
        let values: Vec<u8> = cards.into_iter().
            fold(HashMap::<Face, u8>::new(),
                 |mut hm, &face| {
                     let entry = hm.entry(face).or_insert(0);
                     entry.add_assign(1);
                     hm
                 }).values().map(|v| *v).collect();

        match values.len() {
            1 => HandStrength::Five,
            2 => {
                match values[0] {
                    1 | 4 => HandStrength::Four,
                    2 | 3 => HandStrength::FullHouse,
                    _ => panic!("2 faces with {values:?} amounts")
                }
            }
            3 => {
                match values[0] {
                    1 => {
                        match values[1] {
                            1 | 3 => HandStrength::Three,
                            2 => HandStrength::TwoPair,
                            _ => panic!("3 faces with {values:?} amounts")
                        }
                    }
                    2 => {
                        match values[1] {
                            1 | 2 => HandStrength::TwoPair,
                            _ => panic!("3 faces with {values:?} amounts")
                        }
                    }
                    3 => HandStrength::Three,
                    _ => panic!("2 faces with {values:?} amounts")
                }
            }
            4 => HandStrength::Pair,
            5 => HandStrength::HighCard,
            _ => panic!("{values:?} amounts")
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd)]
pub(crate) struct Hand {
    cards: [Face; 5],
    strength: HandStrength,
}

impl Hand {
    pub(crate) fn from_str(s: &str) -> Self {
        let parts: Vec<Face> = s.trim().chars().map(|c| Face::from_char(c)).collect();
        assert_eq!(parts.len(), 5);

        let cards: [Face; 5] = [parts[0], parts[1], parts[2], parts[3], parts[4]];
        let strength = HandStrength::from_cards(&cards);
        Hand { cards, strength }
    }
    // comparison
    pub(crate) fn cmp(&self, other: &Self) -> Ordering {
        match self.strength.cmp(&other.strength) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                self.cards.iter().zip(other.cards.iter()).
                    find(|(l, r)| l.ne(r)).
                    map_or(Ordering::Equal, |(l, r)| l.cmp(r))
            }
        }
    }
}
