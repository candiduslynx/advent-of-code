use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve(path: &str, smudge: bool) -> u64 {
    let fields = read(path).unwrap().lines().map(|s| s.unwrap()).fold(
        vec![Vec::<Vec<u8>>::new()],
        |mut state, s| {
            match s.len() {
                0 => state.push(vec![]),
                _ => state.last_mut().unwrap().push(s.as_bytes().to_vec()),
            }
            state
        },
    );

    let (x, y) = fields
        .iter()
        .map(|f| patterns(f, smudge))
        .reduce(|(x, y), (a, b)| (x + a, y + b))
        .unwrap();

    (x as u64) * 100 + y as u64
}

/// Returns the (rows above the reflection line, columns left to the reflection line)
fn patterns(field: &Vec<Vec<u8>>, smudge: bool) -> (usize, usize) {
    let rows: Vec<u32> = field.iter().map(|r| as_num(r)).collect();
    let cols: Vec<u32> = (0..field[0].len())
        .map(|i| field.iter().map(|r| r[i]).collect::<Vec<u8>>())
        .map(|r| as_num(&r))
        .collect();

    return (mirror_after(&rows, smudge), mirror_after(&cols, smudge));
}

/// there's at most 17 cols or rows in the input, so we can represent a whole string via a single num
fn as_num(data: &[u8]) -> u32 {
    data.iter()
        .map(|&b| match b {
            b'.' => 0u32,
            b'#' => 1u32,
            _b => panic!("unsupported byte {_b}"),
        })
        .fold(0u32, |mut res, x| {
            res <<= 1;
            res += x;
            res
        })
}

/// Returns the elements before or after split
fn mirror_after(d: &Vec<u32>, smudge: bool) -> usize {
    match reflection_point(d, smudge) {
        0 => match reflection_point(&d.into_iter().rev().map(|&i| i).collect(), smudge) {
            0 => 0,
            v => d.len() - v,
        },
        v => v,
    }
}

/// Returns the amount before the mirror or 0, if none is there
/// if we request a smudge to be removed, we will account for an exactly single smudge used
fn reflection_point(d: &Vec<u32>, smudge: bool) -> usize {
    // we know we have to have an even number in the palindrome
    ((1..d.len())
        .step_by(2)
        .find(|j| {
            let mut ok = !smudge;

            for k in 0..(j + 1) / 2 {
                let (dk, djk) = (d[k], d[j - k]);
                if dk != djk {
                    if ok {
                        // either no smudges allowed or already used
                        return false;
                    }

                    let possible_smudge = dk ^ djk;
                    if possible_smudge & (possible_smudge - 1) != 0 {
                        return false;
                    }

                    ok = true;
                }
            }

            ok
        })
        .unwrap_or(0)
        + 1)
        >> 1
}
