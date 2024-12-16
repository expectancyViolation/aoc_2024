use std::cmp::Reverse;
use std::ops::Add;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use priority_queue::PriorityQueue;
use crate::v::{Facing, V};


pub(crate) fn solve(data: &str) -> (i64, i64) {
    let width = data.lines().next().unwrap().len() + 1;
    let start = data.find('S').unwrap();
    let end = data.find('E').unwrap();
    let end_pos = V((end / width) as i32, (end % width) as i32);
    let start_pos = (V((start / width) as i32, (start % width) as i32), Facing::EAST);

    let mut get_nbs = |(v, facing): (V, Facing)| {
        let mut nbs = Vec::with_capacity(3);
        nbs.push(((v, facing.turn_left()), 1000));
        nbs.push(((v, facing.turn_right()), 1000));
        let nv = v + facing;
        if data.as_bytes()[(nv.0 as usize) * width + (nv.1 as usize)] != b'#' {
            nbs.push(((nv, facing), 1));
        }
        nbs
    };

    let mut distances = HashMap::new();
    let mut predecessors: HashMap<(V, Facing), Vec<((V, Facing), i64)>> = HashMap::new();
    distances.insert(start_pos, 0);
    let mut frontier = PriorityQueue::new();
    frontier.push(start_pos, Reverse(0));
    loop {
        let (cs, dist) = frontier.pop().unwrap();
        distances.insert(cs, dist.0);
        if cs.0 == end_pos {
            let mut best_preds = HashSet::new();
            best_preds.insert(cs.0);
            let mut frontier = HashSet::new();
            frontier.insert(cs);
            while !frontier.is_empty() {
                let mut nf = HashSet::new();
                for x in frontier.iter() {
                    predecessors.get(x).map(|v| {
                        for &(y, d) in v {
                            if d == distances[x] {
                                nf.insert(y);
                                best_preds.insert(y.0);
                            }
                        }
                    });
                }
                frontier = nf;
            }
            return (dist.0, best_preds.len() as i64);
        }

        for &(ns, diff) in get_nbs(cs).iter() {
            if !distances.contains_key(&ns) {
                let new_prio = dist.0 + diff;
                frontier.push_increase(ns, Reverse(new_prio));
                predecessors.entry(ns).or_insert_with(Vec::new).push((cs, new_prio));
            }
        }
    }
}