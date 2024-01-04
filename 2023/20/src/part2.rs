use std::collections::HashMap;

use crate::module;

pub(crate) fn solve(path: &str) -> u64 {
    let mut modules = module::scan(path).unwrap();

    // only single node sends to rx, so just be OK with that being up
    let pre_rx = modules
        .values()
        .find(|m| m.sends_to("rx"))
        .unwrap()
        .name()
        .to_string();

    let mut pre_tg: HashMap<String, u64> = modules
        .get(&pre_rx)
        .unwrap()
        .src()
        .into_iter()
        .map(|s| (s, 0u64))
        .collect();

    let mut idx = 0u64;
    loop {
        idx += 1;
        let (_, _, signals) = module::send(
            &mut modules,
            ("button".to_string(), false, "broadcaster".to_string()),
        );
        for s in signals.iter().filter(|(_, what, _)| *what) {
            match pre_tg.get_mut(&s.0) {
                None => {}
                Some(v) => match *v {
                    0u64 => *v = idx,
                    _ => {}
                },
            }
        }

        if pre_tg.values().all(|&v| v > 0) {
            break pre_tg
                .into_values()
                .reduce(|s, x| lib::lcm::lcm(s, x))
                .unwrap();
        }
    }
}
