use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Card {
    pub id: u32,
    pub winning: HashSet<u32>,
    pub present: HashSet<u32>,
}

impl Card {
    pub fn from_string(string:String) -> Self {
        let parts:Vec<&str> = string.split(": ").collect();
        assert_eq!(parts.len(), 2);

        let nums:Vec<&str> = parts[1].split(" | ").collect();
        assert_eq!(nums.len(), 2);

        Card{
            id: parts[0].strip_prefix("Card ").unwrap().trim().parse().unwrap(),
            winning: nums[0].split_whitespace().
                map(|s| s.trim()).filter(|s| !s.is_empty()).
                map(|s| s.parse().unwrap()).collect(),
            present: nums[1].split_whitespace().
                map(|s| s.trim()).filter(|s| !s.is_empty()).
                map(|s| s.parse().unwrap()).collect(),
        }
    }

    pub fn winning_numbers(&self) -> Vec<&u32> {
        self.winning.intersection(&self.present).collect()
    }
}
