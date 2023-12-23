use crate::module;

pub(crate) fn solve(path: &str) -> u64 {
    let mut modules = module::scan(path).unwrap();
    let flops = module::flops(&modules);

    let mut states: Vec<(u64, u64, u64)> = Vec::new(); // state, low, high
    states.push((module::state(&modules, &flops), 0, 0));
    let mut low = 0u64;
    let mut high = 0u64;

    const CYCLES: usize = 1000;
    for i in 1..=CYCLES {
        let (l, h, _) = module::send(
            &mut modules,
            ("button".to_string(), false, "broadcaster".to_string()),
        );
        low += l;
        high += h;
        let state = module::state(&modules, &flops);
        match states.iter().enumerate().find(|(_, (s, _, _))| s == &state) {
            None => states.push((state, l, h)),
            Some((idx, _)) => {
                // there's a cycle from idx to i-1
                let (cl, ch) =
                    states
                        .iter()
                        .skip(idx + 1)
                        .take(i - idx - 1)
                        .fold((0u64, 0u64), |mut s, x| {
                            s.0 += x.1;
                            s.1 += x.2;
                            s
                        });
                let (cl, ch) = (cl + l, ch + h); // as the cycle can start at 0
                let len = i - idx;
                let left = ((CYCLES - i) / len) as u64;
                let remains = (CYCLES - i) % len;
                assert!(idx + remains < i);
                low += left * cl;
                high += left * ch;
                if remains > 0 {
                    states[idx + 1..idx + remains].iter().for_each(|(_, l, h)| {
                        low += l;
                        high += h;
                    });
                    low += l; // as the cycle can start at 0
                    high += h; // as the cycle can start at 0
                }
                return low * high;
            }
        }
    }

    low * high
}
