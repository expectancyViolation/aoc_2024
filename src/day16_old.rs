use crate::v::{Facing, V};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

type D16State = (V, Facing);
type PredMap = HashMap<D16State, Vec<(D16State, i64)>>;
type DistMap = HashMap<D16State, i64>;
fn get_best_pred_count(cs: (V, Facing), predecessors: &PredMap, distances: &DistMap, pred_count: &mut HashMap<D16State, i32>) -> usize {
    let mut best_preds = HashSet::new();
    best_preds.insert(cs);
    let mut frontier = HashSet::new();
    frontier.insert(cs);
    let mut tot = 0;
    while !frontier.is_empty() {
        let mut nf = HashSet::new();
        for x in frontier.iter() {
            predecessors.get(x).map(|v| {
                for &(y, d) in v {
                    if d == distances[x] {
                        nf.insert(y);
                        best_preds.insert(y);
                    }
                }
            });
        }
        frontier = nf;
    }
    best_preds.iter().for_each(|x| {
        pred_count.get(x).iter().for_each(|&n| {
            tot += n;
            if *n > 0 {
                tot -= 1;
            }
        });
    });
    let l = best_preds.into_iter().map(|p| p.0).collect::<HashSet<_>>().len();
    l + tot as usize
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

    let get_tile = |v: V| data.as_bytes()[(v.0 as usize) * width + (v.1 as usize)];

    // let is_crossing = |v: V| {
    //     [Facing::WEST, Facing::SOUTH, Facing::EAST, Facing::NORTH].map(|f| get_tile(v + f) != b'#').count() > 2
    // };

    let get_nbs = |(v, facing): (V, Facing)| {
        let tl = facing.turn_left();
        let tr = facing.turn_right();
        [
            ((v + tl, tl), 1001),
            ((v + tr, tr), 1001),
            ((v + facing, facing), 1)
        ].into_iter().filter(|&((v, _), _)| get_tile(v) != b'#').collect_vec()
    };

    let step_to_crossing = |s: D16State, pred_count: &mut HashMap<D16State, i32>| -> Option<_> {
        let mut cd = 0;
        let mut cs = s;
        let mut cnt = 0;
        loop {
            let nbs = get_nbs(cs);
            if nbs.len() > 1 || (get_tile(cs.0) != b'.') {
                pred_count.insert(s, cnt);
                return Some((cs, cd));
            };
            if nbs.len() == 0 {
                return None;
            }
            let (ns, d) = nbs[0];
            cs = ns;
            cd += d;
            cnt += 1;
        }
    };

    let mut distances = HashMap::new();
    let mut predecessors: PredMap = HashMap::new();
    let mut pred_count: HashMap<D16State, i32> = HashMap::new();
    distances.insert(start_pos, 0);
    let mut frontier = PriorityQueue::new();
    frontier.push(start_pos, Reverse(0));

    loop {
        let (cs, dist) = frontier.pop().unwrap();
        distances.insert(cs, dist.0);
        if cs.0 == end_pos {
            return (
                dist.0,
                get_best_pred_count(cs, &predecessors, &distances, &mut pred_count) as i64,
            );
        }


        step_to_crossing(cs, &mut pred_count).map(|(at_crossing, diff)| {
            if !distances.contains_key(&at_crossing) {
                let new_prio = dist.0 + diff;
                frontier.push_increase(at_crossing, Reverse(new_prio));
                predecessors
                    .entry(at_crossing)
                    .or_insert_with(Vec::new)
                    .push((cs, new_prio));
            }

            for (ns, diff_) in get_nbs(at_crossing)
            {
                if !distances.contains_key(&ns) {
                    let new_prio = dist.0 + diff + diff_;
                    frontier.push_increase(ns, Reverse(new_prio));
                    predecessors
                        .entry(ns)
                        .or_insert_with(Vec::new)
                        .push((cs, new_prio));
                }
            }
        });
    }
}
