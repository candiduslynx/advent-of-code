// solving:
// x*y >= Distance+1 ; x+y = Time
//
// x^2 - Time*x + Distance+1 <= 0
//
// discriminant: D = T^2 - 4*(Distance+1) (has to be >=0. 0 -> 1 solution only)
// roots: (Time +/- sqrt(D))/2 - need to find all integers between
pub(crate) fn solutions(time: i64, distance: i64) -> u64 {
    let roots = solve_sq_eq(1.0, -(time as f64), distance as f64 + 1.0);

    if roots.is_none() {
        return 0;
    }
    let (r0, r1) = roots.unwrap();

    let solutions = (r1.floor() as i32) - (r0.ceil() as i32) + 1;

    // if we have a single root that's not integer we'll get solutions=0 above
    solutions as u64
}

fn solve_sq_eq(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    assert_ne!(a, 0.0);

    let d: f64 = b * b - 4.0 * a * c;
    if d < 0.0 {
        return None;
    }

    let x = -0.5 * b / a;
    let ds = 0.5 * d.sqrt() / a;
    Some((x-ds, x+ds))
}
