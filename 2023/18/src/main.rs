use lib::time_solution::*;

mod dig;
mod part1;
mod part2;

fn main() {
    solve_part(part1::solve, "part 1", None);
    solve_part(part2::solve, "part 2", None);
}
