use crate::ground;

pub(crate) fn solve(path: &str) -> u64 {
    let mut g = ground::scan(path);

    let mut load: Vec<(u128, u64)> = vec![(ground::to_u128(&g), ground::load(&g))];
    const CYCLES: usize = 1000000000;
    for i in 1..=CYCLES {
        ground::cycle(&mut g);
        let t = (ground::to_u128(&g), ground::load(&g));
        match load
            .iter()
            .enumerate()
            .find_map(|(i, &v)| if v == t { Some(i) } else { None })
        {
            None => load.push(t),
            Some(val) => return load[val + (CYCLES - i) % (i - val)].1, // the result is at val + extra
        }
    }

    println!("we had to cycle through all {CYCLES} of the variants!");
    load.last().unwrap().1
}
