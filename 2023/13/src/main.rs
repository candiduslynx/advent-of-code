use lib::time_solution::*;

mod solve;

fn main() {
    solve_part(|path| solve::solve(path, false), "part 1", None);
    solve_part(|path| solve::solve(path, true), "part 2", None);
}
