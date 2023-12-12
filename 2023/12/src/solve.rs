pub(crate) fn remaining(s: &str, broken: &[usize]) -> u64 {
    if s.len() == 0 {
        return match broken.len() {
            0 => 1,
            _ => 0,
        };
    }

    if broken.len() == 0 {
        return match s.as_bytes().iter().find(|&&b| b == b'#') {
            None => 1,
            Some(_) => 0,
        };
    }
    let expect = broken[0];

    let idx = s
        .as_bytes()
        .iter()
        .enumerate()
        .find(|(_, &b)| b == b'#' || b == b'?');
    if idx.is_none() {
        return 0;
    }
    let (idx, &b) = idx.unwrap();

    // we have possibility to either map the ? or examine #
    let as_dot = if b == b'?' {
        remaining(&s[idx + 1..], broken)
    } else {
        0u64
    };

    let as_hash: u64 = if s.len() - idx < expect {
        0
    } else if s.len() - idx == expect {
        // end of the pattern
        match broken.len() {
            1 => match (&s[idx + 1..]).as_bytes().iter().find(|&&b| b == b'.') {
                Some(_) => 0,
                None => 1,
            },
            _ => 0,
        }
    } else {
        match (&s[idx + 1..idx + expect])
            .as_bytes()
            .iter()
            .find(|&&b| b == b'.')
        {
            Some(_) => 0,
            None => match s.as_bytes()[idx + expect] {
                b'#' => 0, // the next broken is glued together with current, impossible
                _ => remaining(&s[idx + expect + 1..], &broken[1..]),
            },
        }
    };

    as_hash + as_dot
}
