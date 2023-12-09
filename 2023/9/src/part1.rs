use std::fs::read;
use std::io::BufRead;

use crate::binomial;

// to find X = x_(n+1) we're solving the eq:
// sum(i=0..=(n+1))( (-1.pow(n+1-i))*c(n+1,i)*x_i) = 0
// => X = sum(i=0..=n)( (-1.pow(n-i))*c(n+1,i)*x_i)
//
// where c(n,k) = (n!)/((n-k)!*k!)
pub(crate) fn solve(path: &str) -> i64 {
    let nums: Vec<Vec<i64>> = read(path).unwrap().lines()
        .map(|s| s.unwrap()).filter(|s| !s.is_empty())
        .map(|s| s.split_whitespace().filter_map(|s| s.parse::<i64>().ok()).collect())
        .collect();

    let max_len = nums.iter().map(|nums| nums.len()).max().unwrap();
    let c = binomial::cnk(max_len + 1);

    nums.iter().map(|nums| {
        let n = nums.len();
        let coef = |i| { if (n - i) & 1 == 0 { -1i64 } else { 1i64 } };
        nums.iter().enumerate().map(|(i, x)| coef(i) * c[n][i] * x).sum::<i64>()
    }).sum()
}
