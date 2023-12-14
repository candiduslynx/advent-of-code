use lib::time_solution::*;

mod solve;

fn main() {
    solve_part(|path| solve::solve(path, 2), "part 1", None);
    solve_part(|path| solve::solve(path, 1000000), "part 2", None);
}
