use crate::ground;

pub(crate) fn solve(path: &str) -> u64 {
    let mut g = ground::scan(path);

    ground::tilt_north(&mut g);
    ground::load(&g)
}
