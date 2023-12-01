use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve() ->u32{
    return  read("./input.txt").unwrap().lines().
        map(|s| {
            let d = digits(s.unwrap());
            return d.first().unwrap() * 10 + d.last().unwrap();
        }).
        fold(0, |sum:u32, x| sum + (x as u32) );
}

fn digits(src: String) -> Vec<u8> {
    return src.char_indices().map( |(from, _)|src.split_at(from).1).filter_map(
        |s|{
            if s.starts_with("zero") || s.starts_with("0") {
                return Some(0);
            }
            if s.starts_with("one") || s.starts_with("1") {
                return Some(1);
            }
            if s.starts_with("two") || s.starts_with("2") {
                return Some(2);
            }
            if s.starts_with("three") || s.starts_with("3") {
                return Some(3);
            }
            if s.starts_with("four") || s.starts_with("4") {
                return Some(4);
            }
            if s.starts_with("five") || s.starts_with("5") {
                return Some(5);
            }
            if s.starts_with("six") || s.starts_with("6") {
                return Some(6);
            }
            if s.starts_with("seven") || s.starts_with("7") {
                return Some(7);
            }
            if s.starts_with("eight") || s.starts_with("8") {
                return Some(8);
            }
            if s.starts_with("nine") || s.starts_with("9") {
                return Some(9);
            }
            return None;
        }
    ).collect();
}
