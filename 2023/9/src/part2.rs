use std::fs::read;
use std::io::BufRead;

use crate::binomial;

// part 2 is to find x_(-1), which is the same as to solve part 1 for reversed sequence
pub(crate) fn solve(path: &str) -> i64 {
    let c = binomial::cnk(25);
    read(path).unwrap().lines()
        .map(|s| s.unwrap()).filter(|s| !s.is_empty())
        .map(|s| {
            let nums: Vec<i64> = s.split_whitespace().filter_map(|s| s.parse::<i64>().ok()).collect();
            let n = nums.len();
            // based on the len we want -1,1,... or 1,-1,... as coefficients
            (if n & 1 == 0 { vec![-1i64, 1i64] } else { vec![1i64, -1i64] }).iter().cycle()
                .zip(nums.iter().rev())
                .enumerate()
                .map(|(i, (k, x))| k * c[n][i] * x)
                .sum::<i64>()
        }).sum()
}
