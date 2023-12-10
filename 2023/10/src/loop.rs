use std::collections::HashSet;

use lib::point::Point;

pub(crate) fn get_loop(
    lines: &Vec<Vec<char>>,
    start: Point,
    treat: &char,
) -> Option<HashSet<Point>> {
    let mut pos = start;
    let mut pipe = treat;
    let mut prev: Option<Point> = None;
    let (max_x, max_y) = (lines.len(), lines[0].len());
    let mut result: HashSet<Point> = HashSet::new();

    while prev != Some(start) {
        let next = match prev {
            Some(prev) => {
                match pipe {
                    '|' => Point {
                        x: (pos.x << 1) - prev.x,
                        y: pos.y,
                    },
                    '-' => Point {
                        x: pos.x,
                        y: (pos.y << 1) - prev.y,
                    },
                    'F' => Point {
                        x: (pos.x << 1) - prev.x + 1, // below => prev.x = pos.x + 1, right => pos.x = prev.x
                        y: (pos.y << 1) - prev.y + 1, // below => prev.y = pos.y, right => pos.y = prev.y + 1
                    },
                    '7' => Point {
                        x: (pos.x << 1) - prev.x + 1, // below => prev.x = pos.x + 1, left => pos.x = prev.x
                        y: (pos.y << 1) - prev.y - 1, // below => prev.y = pos.y, left => pos.y = prev.y - 1
                    },
                    'J' => Point {
                        x: (pos.x << 1) - prev.x - 1, // above => prev.x = pos.x - 1, left => pos.x = prev.x
                        y: (pos.y << 1) - prev.y - 1, // above => prev.y = pos.y, left => pos.y = prev.y - 1
                    },
                    'L' => Point {
                        x: (pos.x << 1) - prev.x - 1, // above => prev.x = pos.x - 1, left => pos.x = prev.x
                        y: (pos.y << 1) - prev.y + 1, // below => prev.y = pos.y, right => pos.y = prev.y + 1
                    },
                    _ => return None,
                }
            }
            None => match pipe {
                '|' | 'F' | '7' => pos.below(),
                '-' | 'J' => pos.left(),
                'L' => pos.right(),
                _ => return None,
            },
        };
        if !next.is_valid(max_x, max_y) {
            return None;
        }

        if pos != start {
            pipe = &lines[next.x as usize][next.y as usize];
        } else {
            pipe = treat
        }

        // check that we can actually connect by matching pipe
        if pos.x < next.x {
            match pipe {
                '|' | 'L' | 'J' => {}
                _ => return None,
            }
        } else if pos.x > next.x {
            match pipe {
                '|' | 'F' | '7' => {}
                _ => return None,
            }
        } else if pos.y < next.y {
            match pipe {
                '-' | 'J' | '7' => {}
                _ => return None,
            }
        } else if pos.y > next.y {
            // pos left to prev
            match pipe {
                '-' | 'L' | 'F' => {}
                _ => return None,
            }
        }

        result.insert(pos);
        (prev, pos) = (Some(pos), next);
    }
    Some(result)
}
