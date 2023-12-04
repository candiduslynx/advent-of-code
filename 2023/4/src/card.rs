use std::collections::HashSet;

pub fn winning_numbers(string: String) -> usize {
    let parts: Vec<&str> = string.split(": ").collect();
    assert_eq!(parts.len(), 2);

    let nums: Vec<&str> = parts[1].split(" | ").collect();
    assert_eq!(nums.len(), 2);

    nums[0].split_whitespace().
        map(|s| s.trim()).filter(|s| !s.is_empty()).
        map(|s| s.parse().unwrap()).collect::<HashSet<u32>>().
        intersection(
            &nums[1].split_whitespace().
                map(|s| s.trim()).filter(|s| !s.is_empty()).
                map(|s| s.parse().unwrap()).collect::<HashSet<u32>>()
        ).count()
}
