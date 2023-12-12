use std::iter;

pub(crate) fn solve(s: &str, repetitions: usize) -> u64 {
    let parts: Vec<&str> = s.split_whitespace().collect();
    assert_eq!(parts.len(), 2);

    possibilities(
        &vec![parts[0]; repetitions].join("?"),
        iter::repeat(parts[1].split(",").map(|s| s.parse().unwrap()))
            .take(repetitions)
            .flatten(),
    )
}

fn possibilities<T>(s: &str, mut broken: T) -> u64
where
    T: Iterator<Item = usize>,
{
    let len = s.len();
    let mut ways: Vec<u64> = place_first(s, broken.next().unwrap());
    for next in broken {
        let at_least = ways
            .iter()
            .enumerate()
            .find(|(_, &n)| n > 0)
            .map(|(i, _)| i)
            .unwrap()
            + next
            + 1;

        let mut row = vec![0u64; at_least];
        for ll in at_least..=len {
            // calc the possibilities to grab (idx) elems in len = l
            row.push(
                ways.iter()
                    .enumerate()
                    // take up to len = ll-next
                    .take(ll - next)
                    // only parts where we can actually spread idx-1 elements
                    .filter(|(_, &n)| n > 0)
                    // we have to be able to split
                    .filter(|(l, _)| s.as_bytes()[*l] != b'#')
                    // to dedup we'll be placing only at the beginning of the slice
                    .filter(|(l, _)| can_place_at_start(&s[l + 1..ll], next))
                    // we have n ways to get idx items in len=l
                    .map(|(_, n)| n)
                    .sum(),
            );
        }
        ways = row;
    }

    ways[s.len()]
}

fn place_first(s: &str, part: usize) -> Vec<u64> {
    (0..=s.len())
        .into_iter()
        .map(|l| place_single(&s[..l], part))
        .collect()
}

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

fn can_place_at_start(s: &str, len: usize) -> bool {
    s[..len].as_bytes().iter().find(|&&b| b == b'.').is_none() && can_place_zero(&s[len..])
}

fn can_place_zero(s: &str) -> bool {
    s.as_bytes().iter().find(|&&b| b == b'#').is_none()
}
