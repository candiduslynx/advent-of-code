use crate::hike;

pub(crate) fn solve(path: &str) -> u64 {
    let field = hike::scan(path);
    let start_y = field[0]
        .iter()
        .enumerate()
        .find_map(|(y, b)| if b == &b'.' { Some(y) } else { None })
        .unwrap();
    let end_y = field
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .find_map(|(y, b)| if b == &b'.' { Some(y) } else { None })
        .unwrap();

    println!(
        "walking from (0,{start_y}) to ({},{end_y}) on {}x{} map",
        field.len(),
        field.len(),
        field[0].len()
    );
    field.len() as u64
}
