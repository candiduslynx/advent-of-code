pub fn winning_numbers(string: String) -> usize {
    let nums: Vec<Vec<u32>> = string.split(": ").last().unwrap().split(" | ").
        map(|s| s.split_whitespace().filter_map(|s| s.parse().ok()).collect()).collect();
    assert_eq!(nums.len(), 2);

    // using HashSet isn't justified by the len of the numbers, so use vectors instead
    nums[0].iter().filter(|v|nums[1].contains(v)).count()
}
