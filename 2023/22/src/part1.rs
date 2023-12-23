use crate::sand;

pub(crate) fn solve(path: &str) -> usize {
    let bricks = sand::scan(path);
    let fallen = sand::fall(&bricks);
    let can_remove = sand::only_support(&fallen);
    can_remove.into_iter().filter(|b| *b).count()
}
