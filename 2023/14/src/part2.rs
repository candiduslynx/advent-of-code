use crate::ground;

pub(crate) fn solve(path: &str) -> u64 {
    let mut g = ground::scan(path);

    let mut load: Vec<(Vec<u128>, u64)> = vec![(ground::to_u128_vec(&g), ground::load(&g))];

    const CYCLES: usize = 1000000000;
    for i in 1..=CYCLES {
        ground::cycle(&mut g);
        let h = ground::to_u128_vec(&g);
        let l = ground::load(&g);
        match load.iter().enumerate().find_map(|(i, (v, l_saved))| {
            if *l_saved == l && h.eq(v) {
                Some(i)
            } else {
                None
            }
        }) {
            None => {
                load.push((h, l));
            }
            Some(val) => {
                let extra = (CYCLES - i) % (i - val);
                // the result is at val + extra
                return load[(val + extra) as usize].1;
            }
        }
    }

    println!("we had to cycle through all {CYCLES} of the variants!");
    load.last().unwrap().1
}
