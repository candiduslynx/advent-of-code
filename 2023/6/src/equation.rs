// solving:
// x*y >= Distance+1 ; x+y = Time
//
// x^2 - Time*x + Distance+1 <= 0
//
// discriminant: D = T^2 - 4*(Distance+1) (has to be >=0. 0 -> 1 solution only)
// roots: (Time +/- sqrt(D))/2 - need to find all integers between
pub(crate) fn solutions(time: i64, distance: i64) -> u64 {
    let roots = solve_sq_eq(1.0, -(time as f64), distance as f64 +1.0);

    if roots.0.is_none() {
        return 0;
    }
    let r0 = roots.0.unwrap();

    if roots.1.is_none() {
        // check that the only root is actually an integer
        if r0.fract() == 0.0 { return 1; } else { return 0; }
    }
    let r1 = roots.1.unwrap();

    let solutions = (r1.floor() as i32) - (r0.floor() as i32) + if r0.fract() == 0.0 || r1.fract() == 0.0 { 1 } else { 0 };
    if solutions < 0 { 0 } else { solutions as u64}
}

fn solve_sq_eq(a:f64, b:f64, c:f64) -> (Option<f64>,Option<f64>) {
    assert!(a != 0.0);

    let d:f64 = b*b - 4.0*a*c;
    if d < 0.0 {
        return (None, None);
    }

    let x = -0.5*b/a;
    if d == 0.0 {
        return (Some(x), None);
    }
    let d1 = 0.5*d.sqrt()/a;
    (Some(x-d1),Some(x+d1))
}