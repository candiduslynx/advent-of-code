use crate::sand;

pub(crate) fn solve(path: &str) -> usize {
    let bricks = sand::scan(path);
    let fallen = sand::fall(&bricks);
    let mut can_remove = vec![true; bricks.len() + 1];
    can_remove[0] = false; // ground is out of scope
    fallen
        .iter()
        .filter_map(|fb| fb.only_support())
        .for_each(|id| can_remove[id] = false);
    can_remove.into_iter().filter(|b| *b).count()
}
