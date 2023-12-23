use crate::module;

pub(crate) fn solve(path: &str) -> u64 {
    let mut modules = module::scan(path).unwrap();
    let mut low = 0u64;
    let mut high = 0u64;

    const CYCLES: usize = 1000;
    (0..CYCLES).for_each(|_| {
        let (l, h, _) = module::send(
            &mut modules,
            ("button".to_string(), false, "broadcaster".to_string()),
        );
        low += l;
        high += h;
    });

    low * high
}
