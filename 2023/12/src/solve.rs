use std::collections::HashMap;
use std::iter;

pub(crate) fn solve(s: &str, repetitions: usize) -> u64 {
    let parts: Vec<&str> = s.split_whitespace().collect();
    assert_eq!(parts.len(), 2);

    into_partitions(
        &partition(&vec![parts[0]; repetitions].join("?")),
        &iter::repeat(parts[1].split(",").map(|s| s.parse().unwrap()))
            .take(repetitions)
            .flatten()
            .collect::<Vec<usize>>(),
        &mut HashMap::new(),
    )
}
fn partition(s: &str) -> Vec<&str> {
    s.split(".").filter(|s| !s.is_empty()).collect()
}

/// Returns the amount of ways to put all remaining parts into the partitions.
/// ```
/// result = ways to put ALL parts into ALL the partitions
/// ```

fn into_partitions(
    partitions: &[&str],
    parts: &[usize],
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let len = parts.len();
    if partitions.is_empty() {
        return match len {
            0 => 1,
            _ => 0,
        };
    }

    let remaining = &partitions[1..];
    let left = remaining.len();
    let single = into_single_partition(partitions[0], parts);
    single
        .iter()
        .enumerate()
        .filter(|(_, &n)| n > 0)
        .map(|(i, &n)| {
            let mut memorized: Option<u64> = memo.get(&(left, len - i)).map(|s| *s);
            if memorized.is_none() {
                let ways = into_partitions(remaining, &parts[i..], memo);
                memo.insert((left, len - i), ways);
                memorized = Some(ways);
            }
            match memorized.unwrap() {
                0 => 0,
                val => n * val,
            }
        })
        .sum()
}

/// Returns the vector of ways to put `L` parts into the partition.
/// The resulting vector len is always `parts.len() + 1`:
/// ```
/// result[L] = ways to put L parts into the whole partition
/// ```
fn into_single_partition(s: &str, parts: &[usize]) -> Vec<u64> {
    let mut result = vec![0u64; parts.len() + 1];
    result[0] = place_zero(s);
    if parts.len() == 0 {
        return result;
    }

    let len = s.len();
    let mut ways = place_first(s, parts[0]);
    result[1] = ways[len];

    for (idx, &next) in parts.iter().enumerate().skip(1) {
        let first = ways
            .iter()
            .enumerate()
            .find(|(_, &n)| n > 0)
            .map(|(i, _)| i);
        if first.is_none() {
            return result;
        }
        let at_least = first.unwrap() + next + 1;

        if at_least > len {
            // we won't be able to place anymore
            return result;
        }
        let mut row = vec![0u64; at_least];
        for ll in at_least..=len {
            let s = &s[..ll];
            // calc the possibilities to grab (idx) elems in len = l
            row.push(
                ways.iter()
                    .enumerate()
                    // take up to len = ll-next-1 (idx in vector = prefix len)
                    .take(ll - next)
                    // only parts where we can actually spread idx-1 elements
                    .filter(|(_, &n)| n > 0)
                    // we have to be able to split
                    .filter(|(l, _)| s.as_bytes()[*l] != b'#')
                    // to dedup we'll be placing only at the beginning of the slice
                    .filter(|(l, _)| can_place_at_start(&s[l + 1..], next))
                    // we have n ways to get idx items in len=l
                    .map(|(_, n)| n)
                    .sum(),
            );
        }
        result[idx + 1] = row[len];
        ways = row;
    }
    result
}

fn place_zero(s: &str) -> u64 {
    if can_place_zero(s) {
        1u64
    } else {
        0u64
    }
}

/// Returns the vector of ways to put a single part into the partition prefix.
/// The resulting vector len is always `s.len() + 1`:
/// ```
/// result[L]= ways to put the part into &s[..L].
/// ```
fn place_first(s: &str, part: usize) -> Vec<u64> {
    (0..=s.len())
        .into_iter()
        .map(|l| place_single(&s[..l], part))
        .collect()
}

/// ```
/// result = ways to put single part of length=len into the whole s
/// ```
fn place_single(s: &str, len: usize) -> u64 {
    if s.len() < len {
        return 0;
    } else if s.len() == len {
        return placing_at_start(s, len);
    }

    match s.as_bytes()[0] {
        b'.' => place_single(&s[1..], len),
        b'#' => placing_at_start(s, len),
        b'?' => placing_at_start(s, len) + place_single(&s[1..], len),
        b => panic!("bad byte {b}"),
    }
}

fn placing_at_start(s: &str, len: usize) -> u64 {
    if can_place_at_start(s, len) {
        1u64
    } else {
        0u64
    }
}

/// s is assumed to be a partition with no dots
fn can_place_at_start(s: &str, len: usize) -> bool {
    can_place_zero(&s[len..])
}

fn can_place_zero(s: &str) -> bool {
    s.as_bytes().iter().find(|&&b| b == b'#').is_none()
}
