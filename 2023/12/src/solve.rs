pub(crate) fn solve(s: &str, repetitions: usize) -> u64 {
    let parts: Vec<&str> = s.split_whitespace().collect();
    assert_eq!(parts.len(), 2);

    let pattern = &vec![parts[0]; repetitions].join("?");
    let broken: Vec<usize> = vec![parts[1]; repetitions]
        .join(",")
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    possibilities(pattern, &broken)
}

fn possibilities(s: &str, broken: &[usize]) -> u64 {
    let len = s.len();
    let mut ways: Vec<u64> = place_first(s, broken[0]);
    // println!("placements {:?} for {s}: {:?}", &broken[..1], &ways[1..]);
    for &next in broken.iter().skip(1) {
        let first_non_zero = ways
            .iter()
            .enumerate()
            .find(|(_, &n)| n > 0)
            .map(|(i, _)| i)
            .unwrap();
        // println!(
        //     "at least {first_non_zero:?} len is required for {:?}",
        //     &broken[..idx]
        // );
        let at_least = first_non_zero + next + 1;

        let mut row = vec![0u64; at_least];

        for ll in at_least..=len {
            // println!(">>> {:?} in {} start", &broken[..idx + 1], &s[..ll]);
            // calc the possibilities to grab (idx) elems in len = l
            let w = ways
                .iter()
                .enumerate()
                .take(ll - next) // take up to len = ll-next
                .filter(|(_, &n)| n > 0)
                .map(|(l, &n)| {
                    // println!(
                    //     "we have {n} way(s) to place {:?} in \"{}\"",
                    //     &broken[..idx],
                    //     &s[..l]
                    // );
                    // we have n ways to get idx items in len=l
                    // remaining are ours to grab
                    // we have len = l, so last taken char is at &s[l-1]
                    if s.as_bytes()[l] == b'#' {
                        // println!("s.as_bytes()[{l}] = #");
                        // we can't split here (as the broken have to be spread by at least a single dot)
                        return 0u64;
                    }

                    // now look at he remaining part: &s[l+1..ll]
                    // to dedup we'll be placing only at the beginning of the slice
                    let part = &s[l + 1..ll];
                    if can_place_at_start(part, next) {
                        // println!("we can place {next} at {part} start, take {n}");
                        n
                    } else {
                        // println!("we can't place {next} at {part} start, take 0");
                        0u64
                    }
                    // let single = place_single(part, next);
                    // println!(
                    //     "(have={idx} parts, taking {next}): &s[{}..{ll}]={part} -> {single}*{n}",
                    //     l + 1
                    // );
                    // n * single
                })
                .sum();
            // println!(
            //     "<<< {w} ways to place {:?} in {}",
            //     &broken[..idx + 1],
            //     &s[..ll]
            // );
            row.push(w);
        }
        // println!("placing {:?} for {s}: {:?}", &broken[..=idx], &row[1..]);
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

fn can_place_zero(s: &str) -> bool {
    s.as_bytes().iter().find(|&&b| b == b'#').is_none()
}

fn can_place_at_start(s: &str, len: usize) -> bool {
    s[..len].as_bytes().iter().find(|&&b| b == b'.').is_none() && can_place_zero(&s[len..])
}

fn place_single(s: &str, broken: usize) -> u64 {
    if s.len() < broken {
        return 0;
    } else if s.len() == broken {
        // we only can consume this as a whole
        return match s.as_bytes().iter().find(|&&b| b == b'.') {
            None => 1,
            _ => 0,
        };
    }

    let as_dot = if s.as_bytes()[0] == b'#' {
        0u64
    } else {
        place_single(&s[1..], broken)
    };

    let as_hash = if s.as_bytes()[0] == b'.' {
        0u64
    } else if !can_place_zero(&s[broken..]) {
        0u64
    } else {
        match s.as_bytes()[1..broken].iter().find(|&&b| b == b'.') {
            None => 1,
            _ => 0,
        }
    };
    as_dot + as_hash
}
