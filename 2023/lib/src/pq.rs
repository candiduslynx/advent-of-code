use std::collections::VecDeque;

pub trait PriorityPosition<T>
where
    T: Ord,
{
    fn p_pos(&self, v: &T) -> usize;
}

impl<T> PriorityPosition<T> for VecDeque<T>
where
    T: Ord,
{
    fn p_pos(&self, v: &T) -> usize {
        if self.is_empty() || v < self.get(0).unwrap() {
            // insert at the beginning
            return 0;
        }

        let (mut s, mut e) = (0, self.len() - 1);
        loop {
            // v <= self.get(s)
            if s == e {
                if self.get(s).unwrap() < v {
                    break s + 1;
                } else {
                    break s;
                }
            }

            let m = (s + e + 1) >> 1;
            let mv = self.get(m).unwrap();

            if mv == v {
                break m;
            }

            if mv < v {
                s = m;
            } else if m == e {
                // can only be if interval has two elements
                break e;
            } else {
                e = m;
            }
        }
    }
}

/// trait for making a pq from dq

pub trait PriorityInsert<T>
where
    T: Ord,
{
    fn p_insert(&mut self, v: T);
}

impl<T> PriorityInsert<T> for VecDeque<T>
where
    T: Ord,
{
    fn p_insert(&mut self, v: T) {
        self.insert(self.p_pos(&v), v);
    }
}
