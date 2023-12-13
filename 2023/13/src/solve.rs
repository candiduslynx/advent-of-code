/// Returns the (rows above the reflection line, columns left to the reflection line)
pub(crate) fn patterns(field: &Vec<Vec<u8>>) -> (usize, usize) {
    let rows: Vec<u32> = field.iter().map(|r| as_num(r)).collect();
    let cols: Vec<u32> = (0..field[0].len())
        .map(|i| field.iter().map(|r| r[i]).collect::<Vec<u8>>())
        .map(|r| as_num(&r))
        .collect();

    let r = mirror_after(&rows);
    let c = mirror_after(&cols);
    visualize(field, r, c);
    (r, c)
}

fn visualize(field: &Vec<Vec<u8>>, rows: usize, cols: usize) {
    let f = reconstruct(field);
    let s: Vec<&str> = f.split_whitespace().collect();
    if rows != 0 {
        println!();
        println!("have vertical:");
        for i in 0..rows {
            println!("{}", &s[i]);
        }
        println!("{}", "-".repeat(s[0].len()));
        for i in rows..s.len() {
            println!("{}", &s[i]);
        }
        println!();
    }
    if cols != 0 {
        println!();
        println!("have horizontal:");
        for s in s {
            println!("{}|{}", &s[..cols], &s[cols..]);
        }
        println!();
    }
}

fn reconstruct(field: &Vec<Vec<u8>>) -> String {
    field
        .iter()
        .map(|row| reconstruct_row(row))
        .collect::<Vec<&str>>()
        .join("\n")
}

fn reconstruct_row(row: &Vec<u8>) -> &str {
    std::str::from_utf8(row).unwrap()
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
fn mirror_after(d: &Vec<u32>) -> usize {
    match longest_even_palindrome_from_start(d) {
        0 => match longest_even_palindrome_from_start(&d.into_iter().rev().map(|&i| i).collect()) {
            0 => 0,
            v => d.len() - v,
        },
        v => v,
    }
}

/// Returns the amount before the mirror or 0, if none is there
fn longest_even_palindrome_from_start(d: &Vec<u32>) -> usize {
    let mut max_l = 0usize;

    for j in 0..d.len() {
        let possible = j + 1;
        if possible & 1 == 1 {
            // we know we have to have an even number in the palindrome
            continue;
        }
        match (0..possible / 2).find(|&k| d[k] != d[j - k]) {
            None => max_l = possible,
            _ => {}
        };
    }

    max_l >> 1 // ~ max_l / 2
}
