use std::collections::HashSet;

pub fn winning_numbers(string: String) -> usize {
    let nums: Vec<&str> = string.split(": ").last().unwrap().split(" | ").collect();
    assert_eq!(nums.len(), 2);

    u32_set(nums[0]).intersection(&u32_set(nums[1])).count()
}

fn u32_set(string: &str) -> HashSet<u32> {
    string.split_whitespace().
        map(|s| s.trim()).
        filter_map(|s| s.parse().ok()).
        collect::<HashSet<u32>>()
}
