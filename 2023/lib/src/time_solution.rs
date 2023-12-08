use std::time::{Duration, Instant};

pub fn solve_part<T: std::fmt::Display>(f: fn(&str) -> T, name: &str, test_path: Option<&str>) {
    let test_path = if test_path.is_some() {test_path.unwrap()} else {"./test.txt"};
    let (result, duration) = solve_and_time(f, test_path);
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