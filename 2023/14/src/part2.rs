use crate::ground;

pub(crate) fn solve(path: &str) -> u64 {
    let mut g = ground::scan(path);

    // let mut memo: HashMap<u64, u64> = HashMap::new(); // key = stringified field, val = cycles after we have this
    let l = ground::load(&g);
    let mut load: Vec<u64> = vec![l];

    const CYCLES: usize = 1000000000;
    for i in 1..=CYCLES {
        ground::cycle(&mut g);
        let l = ground::load(&g);

        match load
            .iter()
            .enumerate()
            .find_map(|(i, &v)| if v == l { Some(i) } else { None })
        {
            None => load.push(l),
            Some(val) => {
                let extra = (CYCLES - i) % (i - val);
                // the result is at val + extra
                return load[val + extra];
            }
        }
    }

    println!("we had to cycle through all {CYCLES} of the variants!");
    *load.last().unwrap()
}
