pub fn winning_numbers(string: String) -> usize {
    let nums: Vec<Vec<u32>> = string.split(": ").last().unwrap().split(" | ").
        map(|s| s.split_whitespace().filter_map(|s| s.parse().ok()).collect()).collect();
    assert_eq!(nums.len(), 2);

    // using HashSet isn't justified by the len of the numbers, so use vectors instead
    intersection(&nums[0], &nums[1]).len()
}

fn intersection(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
    a.into_iter().filter(|v|b.contains(v)).map(|v|*v).collect::<Vec<u32>>()
}
