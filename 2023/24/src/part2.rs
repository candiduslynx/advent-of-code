use crate::hail;

/// mostly copied from https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kersplf
pub(crate) fn solve(path: &str) -> f64 {
    let stones = hail::scan(path);
    let (rock, s) = hail::rock(&stones);
    rock.sum() / s
}
