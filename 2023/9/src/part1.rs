use std::fs::read;
use std::io::BufRead;

use crate::binomial;

// to find X = x_(n+1) we're solving the eq:
// sum(i=0..=(n+1))( (-1.pow(n+1-i))*c(n+1,i)*x_i) = 0
// => X = sum(i=0..=n)( (-1.pow(n-i))*c(n+1,i)*x_i)
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
            // based on the len we want -1,1,... or 1,-1,... as coefficients
            (if n & 1 == 0 {
                vec![-1i64, 1i64]
            } else {
                vec![1i64, -1i64]
            })
            .iter()
            .cycle()
            .zip(nums.iter())
            .enumerate()
            .map(|(i, (k, x))| k * c[n][i] * x)
            .sum::<i64>()
        })
        .sum()
}
