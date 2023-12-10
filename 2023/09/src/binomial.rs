// pre-calc binomial coefficients
pub(crate) fn cnk(max: usize) -> Vec<Vec<i64>> {
    let mut res: Vec<Vec<i64>> = Vec::new();
    let mut last: Vec<i64> = vec![1];
    res.push(last.clone());

    for _n in 1..=max {
        last = last
            .iter()
            .skip(1)
            .zip(last.iter())
            .map(|(x, y)| x + y)
            .fold(vec![1], |mut c, x| {
                c.push(x);
                c
            });
        last.push(1);
        res.push(last.clone());
    }
    res
}
