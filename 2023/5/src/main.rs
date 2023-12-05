mod part1;
mod part2;
mod interval;
mod almanac;
mod range;

fn main() {
    println!("part 1 test result: {}", part1::solve("./test.txt"));
    println!("part 1 result: {}", part1::solve("./input.txt"));
    println!("part 2 test result: {}", part2::solve("./test.txt"));
    println!("part 2 result: {}", part2::solve("./input.txt"));
}
