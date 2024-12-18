use crate::str_map::{StrMap, DIRECTIONS};
use itertools::iproduct;
use std::collections::{HashMap, HashSet};

pub(crate) fn solve(data: &str) -> (i64, i64) {
    let h = data.lines().count() as i32;
    let w = data.lines().next().unwrap().len() as i32;
    let mut p1 = 0;
    let mut p2 = 0;
    let mut data = String::from(data).into_bytes();
    iproduct!(0..h, 0..w).for_each(|(x, y)| {
        let m = StrMap { data: data.as_mut_slice(), h, w };
        if m.get(x, y) != '0' as u8 {
            return;
        }
        let mut res: HashMap<(i32, i32), i64> = HashMap::new();
        res.insert((x, y), 1);
        let mut frontier: HashSet<_> = HashSet::new();
        frontier.insert((x, y));
        for i in '1' as u8..='9' as u8 {
            let mut new_frontier: HashSet<_> = HashSet::new();
            frontier.iter().for_each(|&(x, y)| {
                let curr_val = res.get(&(x, y)).unwrap().clone();
                DIRECTIONS.iter().for_each(|&(dx, dy)| {
                    let (nx, ny) = (x + dx, y + dy);
                    if m.get(nx, ny) == i {
                        new_frontier.insert((nx, ny));
                        let entry = res.entry((nx, ny)).or_insert(0);
                        *entry += curr_val;
                    }
                })
            });
            frontier = new_frontier;
        }
        p1 += frontier.len() as i64;
        p2 += frontier.iter().map(|p| { res.get(p).unwrap() }).sum::<i64>();
    });
    (p1, p2)
}
