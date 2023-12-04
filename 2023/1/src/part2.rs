use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve(path: &str) -> u32 {
    read(path).unwrap().lines().
        map(|s| s.unwrap()).
        map(|s| s.replace("zero", "0o")). // _o_ne
        map(|s| s.replace("one", "o1e")). // zer_o_, tw_o_, _e_ight
        map(|s| s.replace("two", "t2o")). // _o_ne, eigh_t_
        map(|s| s.replace("three", "t3e")). // _e_igh_t_
        map(|s| s.replace("four", "4")).
        map(|s| s.replace("five", "5e")). // _e_ight
        map(|s| s.replace("six", "6")).
        map(|s| s.replace("seven", "7n")). // _n_ine
        map(|s| s.replace("eight", "e8t")). // on_e_, _t_wo, _t_hre_e_, fiv_e_
        map(|s| s.replace("nine", "n9e")). // seve_n_, _e_ight
        map(|s| s.chars().filter_map(|c|c.to_digit(10)).collect::<Vec<u32>>()). // digits from the line
        filter(|digits| !digits.is_empty()).
        map(|digits|digits.first().unwrap() * 10 + digits.last().unwrap()). // take first & last
        sum()
}
