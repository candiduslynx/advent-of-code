use lib::time_solution::*;

mod part1;
mod part2;
mod card;

fn main() {
    solve_part(part1::solve, "part 1");
    solve_part(part2::solve, "part 2");
}
