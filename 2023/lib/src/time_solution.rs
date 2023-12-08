use std::time::{Duration, Instant};

pub fn solve_part<T: std::fmt::Display>(f: fn(&str) -> T, name: &str) {
    let (result, duration) = solve_and_time(f, "./test.txt");
    println!("{name} test: result is {result:20} took {duration:?}");

    let (result, duration) = solve_and_time(f, "./input.txt");
    println!("{name}:      result is {result:20} took {duration:?}");
}

pub fn solve_and_time<T>(f: fn(&str) -> T, s: &str) -> (T, Duration) {
    let start = Instant::now();
    let result = f(s);
    let elapsed = start.elapsed();
    (result, elapsed)
}