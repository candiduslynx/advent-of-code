use std::fs::read;
use std::io::BufRead;

use crate::binomial;

// part 2 is to find x_(-1), which is the same as to solve part 1 for reversed sequence
// it can also be treated as
// to find X = x_(-) we're solving the eq:
// sum(i=-1..=(n))( (-1.pow(i+1))*c(n+1,i+1)*x_i) = 0
// => X = sum(i=0..=n)( (-1.pow(i))*c(n+1,i+1)*x_i)
// where c(n,k) = (n!)/((n-k)!*k!)
pub(crate) fn solve(path: &str) -> i64 {
    let c = binomial::cnk(25);
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let nums: Vec<i64> = s
                .split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect();
            let n = nums.len();
            vec![1i64, -1i64]
                .iter()
                .cycle()
                .zip(nums.iter())
                .enumerate()
                .map(|(i, (k, x))| k * c[n][i + 1] * x)
                .sum::<i64>()
        })
        .sum()
}
