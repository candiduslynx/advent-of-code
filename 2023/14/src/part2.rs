use std::collections::HashMap;

use crate::ground;

pub(crate) fn solve(path: &str) -> u64 {
    let mut g = ground::scan(path);

    let mut memo: HashMap<String, u64> = HashMap::new(); // key = stringified field, val = cycles after we have this
    memo.insert(ground::to_str(&g), 0);
    let mut load: Vec<u64> = vec![ground::load(&g)];

    const CYCLES: u64 = 1000000000;
    for i in 1..=CYCLES {
        ground::cycle(&mut g);
        let s = ground::to_str(&g);
        let l = ground::load(&g);
        match memo.get(&s) {
            None => {
                memo.insert(s, i);
                load.push(l)
            }
            Some(val) => {
                let extra = (CYCLES - i) % (i - val);
                // the result is at val + extra
                return load[(val + extra) as usize];
            }
        }
    }

    println!("we had to cycle through all of the variants!");
    *load.last().unwrap()
}
