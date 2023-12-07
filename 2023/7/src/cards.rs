use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(Copy, Clone, Debug, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Face { Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace }

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

    pub(crate) fn cmp_with_joker(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            return Ordering::Equal;
        }

        if Face::Jack.eq(self) {
            return Ordering::Less;
        }

        if Face::Jack.eq(other) {
            return Ordering::Greater;
        }

        self.cmp(other)
    }
}

#[derive(Copy, Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum HandStrength { HC, P, TP, T, FH, F4, F5 }

impl HandStrength {
    fn from_cards_with_joker(cards: &[Face; 5]) -> Self {
        let jokers = cards.into_iter().filter(|&f| Face::Jack.eq(f)).count();
        let base = HandStrength::from_cards(cards);

        match jokers {
            0 => base,
            1 => { // only 1 joker, so we account for that in the transformations
                match base {
                    HandStrength::HC => HandStrength::P,
                    HandStrength::P => HandStrength::T,
                    HandStrength::TP => HandStrength::FH,
                    HandStrength::T => HandStrength::F4,
                    HandStrength::F4 => HandStrength::F5,
                    _ => { panic!("impossible value {base:?} for hand with 1 joker") }
                }
            }
            2 => { // 1 pair is caused by jokers
                match base {
                    HandStrength::P => HandStrength::T,
                    HandStrength::TP => HandStrength::F4,
                    HandStrength::FH => HandStrength::F5,
                    _ => { panic!("impossible value {base:?} for hand with 2 jokers") }
                }
            }
            3 => { // 1 three is caused by jokers
                match base {
                    HandStrength::T => HandStrength::F4,
                    HandStrength::FH => HandStrength::F5,
                    _ => { panic!("impossible value {base:?} for hand with 3 jokers") }
                }
            }
            4 | 5 => HandStrength::F5,
            _ => { panic!("impossible value {base:?} of jokers") }
        }
    }
    fn from_cards(cards: &[Face; 5]) -> Self {
        let values: Vec<u8> = cards.into_iter().
            fold(HashMap::<Face, u8>::new(),
                 |mut hm, &face| {
                     let entry = hm.entry(face).or_insert(0);
                     entry.add_assign(1);
                     hm
                 }).values().map(|v| *v).collect();

        match values.len() {
            1 => HandStrength::F5,
            2 => {
                match values[0] {
                    1 | 4 => HandStrength::F4,
                    2 | 3 => HandStrength::FH,
                    _ => panic!("2 faces with {values:?} amounts")
                }
            }
            3 => {
                match values[0] {
                    1 => {
                        match values[1] {
                            1 | 3 => HandStrength::T,
                            2 => HandStrength::TP,
                            _ => panic!("3 faces with {values:?} amounts")
                        }
                    }
                    2 => {
                        match values[1] {
                            1 | 2 => HandStrength::TP,
                            _ => panic!("3 faces with {values:?} amounts")
                        }
                    }
                    3 => HandStrength::T,
                    _ => panic!("2 faces with {values:?} amounts")
                }
            }
            4 => HandStrength::P,
            5 => HandStrength::HC,
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

    pub(crate) fn from_str_with_jokers(s: &str) -> Self {
        let parts: Vec<Face> = s.trim().chars().map(|c| Face::from_char(c)).collect();
        assert_eq!(parts.len(), 5);

        let cards: [Face; 5] = [parts[0], parts[1], parts[2], parts[3], parts[4]];
        let strength = HandStrength::from_cards_with_joker(&cards);
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

    pub(crate) fn cmp_with_joker(&self, other: &Self) -> Ordering {
        match self.strength.cmp(&other.strength) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                self.cards.iter().zip(other.cards.iter()).
                    find(|(l, r)| l.ne(r)).
                    map_or(Ordering::Equal, |(l, r)| l.cmp_with_joker(r))
            }
        }
    }
}
