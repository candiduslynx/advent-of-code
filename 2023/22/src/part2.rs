use std::collections::VecDeque;

use crate::sand;
use crate::sand::FallenBrick;

pub(crate) fn solve(path: &str) -> u64 {
    let bricks = sand::scan(path);
    let fallen = sand::fall(&bricks);
    let can_remove = sand::only_support(&fallen); // we only need to consider those
    can_remove
        .iter()
        .enumerate()
        .filter_map(|(id, ok)| if *ok { None } else { Some(id) })
        .map(|id| will_fall(&fallen, id))
        .sum()
}

fn will_fall(fallen: &Vec<FallenBrick>, remove: usize) -> u64 {
    let mut fallen = fallen.clone();
    let mut result = 0u64;
    let mut open: VecDeque<FallenBrick> = may_fall_next(&mut fallen, remove);
    let mut processed = vec![false; fallen.len() + 1];

    // shorthand to writing len > 0 + pop front
    while let Some(f) = open.pop_front() {
        if f.supported_by.len() > 0 {
            continue;
        }
        // f is indeed falling now
        if processed[f.id] {
            continue;
        }
        processed[f.id] = true;
        result += 1;

        may_fall_next(&mut fallen, f.id)
            .into_iter()
            .for_each(|f| open.push_back(f));
    }
    result
}

fn may_fall_next(fallen: &mut Vec<FallenBrick>, remove: usize) -> VecDeque<FallenBrick> {
    fallen
        .iter_mut()
        .filter_map(|f| {
            match f.supported_by.iter().enumerate().find_map(|(idx, v)| {
                if v == &remove {
                    Some(idx)
                } else {
                    None
                }
            }) {
                Some(idx) => {
                    f.supported_by.swap_remove(idx);
                    Some(f.clone())
                }
                None => None,
            }
        })
        .collect()
}
