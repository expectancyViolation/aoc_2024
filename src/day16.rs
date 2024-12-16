use crate::v::{Facing, V};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

type D16State = (V, Facing);
type PredMap = HashMap<D16State, Vec<(D16State, i64)>>;
type DistMap = HashMap<D16State, i64>;
fn get_best_pred_count(cs: (V, Facing), predecessors: &PredMap, distances: &DistMap) -> usize {
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
    best_preds.len()
}

pub(crate) fn solve(data: &str) -> (i64, i64) {
    let width = data.lines().next().unwrap().len() + 1;
    let start = data.find('S').unwrap();
    let end = data.find('E').unwrap();
    let end_pos = V((end / width) as i32, (end % width) as i32);
    let start_pos = (
        V((start / width) as i32, (start % width) as i32),
        Facing::EAST,
    );

    let tile_is_free = |v: V| data.as_bytes()[(v.0 as usize) * width + (v.1 as usize)] != b'#';

    let get_nbs = |(v, facing): (V, Facing)| {
        let tl = facing.turn_left();
        let tr = facing.turn_right();
        [
            ((v + tl, tl), 1001),
            ((v + tr, tr), 1001),
            ((v + facing, facing), 1),
        ]
    };

    let mut distances = HashMap::new();
    let mut predecessors: PredMap = HashMap::new();
    distances.insert(start_pos, 0);
    let mut frontier = PriorityQueue::new();
    frontier.push(start_pos, Reverse(0));

    loop {
        let (cs, dist) = frontier.pop().unwrap();
        distances.insert(cs, dist.0);
        if cs.0 == end_pos {
            return (
                dist.0,
                get_best_pred_count(cs, &predecessors, &distances) as i64,
            );
        }

        for (ns, diff) in get_nbs(cs)
            .into_iter()
            .filter(|&((pos, _f), _)| tile_is_free(pos))
        {
            if !distances.contains_key(&ns) {
                let new_prio = dist.0 + diff;
                frontier.push_increase(ns, Reverse(new_prio));
                predecessors
                    .entry(ns)
                    .or_insert_with(Vec::new)
                    .push((cs, new_prio));
            }
        }
    }
}
