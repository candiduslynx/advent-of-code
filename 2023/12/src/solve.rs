pub(crate) fn solve(s: &str, repetitions: usize) -> u64 {
    let parts: Vec<&str> = s.split_whitespace().collect();
    assert_eq!(parts.len(), 2);

    let pattern = &vec![parts[0]; repetitions].join("?");
    let broken: Vec<usize> = vec![parts[1]; repetitions]
        .join(",")
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{pattern} | {broken:?}");
    possibilities(pattern, &broken)
}

fn possibilities(s: &str, broken: &Vec<usize>) -> u64 {
    remaining(&partition(s), &broken).unwrap()
}

fn remaining(partitions: &[&str], broken: &[usize]) -> Option<u64> {
    // println!("{partitions:?} | {broken:?}");

    if partitions.is_empty() {
        return match broken.len() {
            0 => Some(1),
            _ => None,
        };
    }

    let curr = partitions[0];
    let &max = broken.iter().max().unwrap();
    if max < curr.len() {
        // curr has to have at least 1 dot
    }
    let res = partition_possibilities(curr, broken)
        .iter()
        .map(|(amount, p)| {
            let remaining = remaining(&partitions[1..], &broken[*amount..]);
            // println!("{curr:?} took {amount} part(s) x {p} time(s) from {broken:?}, tail possibilities: {remaining:?}");
            if remaining.is_none() {
                0
            } else {
                p * remaining.unwrap()
            }
        })
        .sum();
    if res == 0 {
        None
    } else {
        Some(res)
    }
}

pub(crate) fn partition(s: &str) -> Vec<&str> {
    s.split(".").filter(|&s| !s.is_empty()).collect()
}

// look at the partition (without any dots)
// and tell how many pieces of the broken parts fit
// the results can be used to skip the corresponding amounts of broken parts
// the result elem is amount & how many ways to achieve it are there
fn partition_possibilities(s: &str, broken: &[usize]) -> Vec<(usize, u64)> {
    if s.as_bytes().iter().find(|&&b| b == b'#').is_none() {
        return max_possibilities(s.len(), broken);
    }
    let zero = take_zero_parts(s);
    if broken.len() == 0 {
        return zero;
    }

    let expected = broken[0];
    if s.len() < expected {
        return zero;
    } else if s.len() == expected {
        // we can take 1 element from the broken parts at hand, as the partition doesn't have dots
        return dedup(vec![zero, vec![(1usize, 1u64)]], 1);
    }

    let first_hash: Vec<(usize, u64)> = if s.as_bytes()[expected] == b'?' {
        // are able to take the part
        partition_possibilities(&s[expected + 1..], &broken[1..])
            .into_iter()
            .map(|(amount, p)| (amount + 1, p))
            .collect()
    } else {
        vec![]
    };

    let first_dot: Vec<(usize, u64)> = if s.as_bytes()[0] == b'?' {
        partition_possibilities(&s[1..], broken)
    } else {
        vec![]
    };

    let result = dedup(vec![first_hash, first_dot], s.len());
    // println!("{s} | {broken:?} -> {result:?}");
    result
}

fn take_zero_parts(s: &str) -> Vec<(usize, u64)> {
    match s.as_bytes().iter().find(|&&b| b == b'#') {
        None => vec![(0, 1)],
        Some(_) => vec![],
    }
}

fn dedup(a: Vec<Vec<(usize, u64)>>, max: usize) -> Vec<(usize, u64)> {
    let mut result: Vec<u64> = vec![0u64; max + 1];
    a.into_iter()
        .flatten()
        .for_each(|(amount, times)| result[amount] += times);
    result
        .into_iter()
        .enumerate()
        .filter(|&(_, x)| x > 0u64)
        .collect()
}

// solve for ??????? (len = n)
fn max_possibilities(len: usize, broken: &[usize]) -> Vec<(usize, u64)> {
    if broken.is_empty() {
        return vec![(0, 1)];
    }
    let first = broken[0];
    if len < first {
        return vec![(0, 1)];
    } else if broken.len() == 1 {
        return vec![(0, 1), (1, (len - first + 1) as u64)];
    }

    let mut calc: Vec<Vec<u64>> = vec![vec![1u64; len + 1]]; // [amount][len] -> ways to achieve
    let mut one: Vec<u64> = vec![0u64; first];
    for i in first..=len {
        one.push((i - first + 1) as u64);
    }
    calc.push(one);

    let mut at_least = first;
    for idx in 1usize..broken.len() {
        let next = broken[idx];
        at_least += next + 1;
        if at_least > len {
            break;
        }

        let mut row = vec![0u64; at_least];
        for ll in at_least..=len {
            // calc the possibilities to grab (idx) elems in len = l
            row.push(
                // idx shows the prev amount, use that
                calc[idx]
                    .iter()
                    .enumerate()
                    .take(ll - next) // limit len to a plausible one
                    .filter(|(_, &n)| n > 0)
                    .map(|(l, &n)| {
                        // we have n ways to get idx-1 items in len l
                        // remaining are ours to grab
                        // N = (len - l - 1) = how much place we have to take next elems
                        // ways = N-next+1
                        // (len - l - 1) -next + 1 = len - l - next
                        let ways = (len - l - next) as u64;
                        ways * n // don't forget to factor the prev items
                    })
                    .sum(),
            )
        }
        calc.push(row);
    }

    let res = calc
        .into_iter()
        .enumerate()
        .map(|(i, r)| (i, r[len]))
        .collect();
    res
}

// if we have the ways = [amount][len]->possibilities
// placing next is simple
// say, we have up to amount = A
// to calc for amount = A+1:
// ways[A+1][len: 0..(sum_len + curr + A (dots between))] = 0
// for others: we take ways of putting in first M elems
// then we take the remaining pattern & check the amount of ways to put next part there (=N)
// if it's impossible, we should account for 0
// if possible, store N * ways we used

// the result should be res[total][total len]
// we also don't actually require to store the whole array, just the prev row
// for 0 elems we store 1
