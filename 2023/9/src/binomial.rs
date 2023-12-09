// pre-calc binomial coefficients
pub(crate) fn cnk(max: usize) -> Vec<Vec<i64>> {
    if max == 0 {
        return vec![vec![1]];
    }

    let mut c = cnk(max - 1);
    let last = c.last().unwrap();
    let mut cn: Vec<i64> = last.iter().skip(1).zip(last.iter())
        .map(|(x, y)| x+y).fold(vec![1], |mut c, x| {
        c.push(x);
        c
    });
    cn.push(1);
    c.push(cn);
    c
}
