use std::cmp::Reverse;
use hashbrown::{HashMap, HashSet};
use itertools::iproduct;
use priority_queue::PriorityQueue;
use crate::str_map::StrMap;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

type D16State = (i32, i32, usize);

fn get_best_pred_counts(
    end_state: D16State,
    predecessors: &HashMap<D16State, Vec<(D16State, i32, i32, usize)>>,
    distances: &HashMap<D16State, i32>,
) -> i64 {
    let mut best_preds = HashSet::new();
    let mut frontier = HashSet::new();
    frontier.insert(end_state);
    let mut res = 0;
    let mut cnts = HashMap::new();
    while !frontier.is_empty() {
        let mut nf = HashSet::new();
        for x in frontier.iter() {
            predecessors.get(x).map(|v| {
                for &(y, d, cnt, rot) in v {
                    if d == distances[x] {
                        nf.insert(y);
                        let ny = (y.0, y.1, (y.2 + rot) % 4);
                        best_preds.insert(ny);
                        cnts.insert(ny, cnt);
                    }
                }
            });
        }
        frontier = nf;
    }
    best_preds.iter().for_each(|&(y)| {
        res += cnts.get(&y).unwrap();
    });

    let rs = best_preds.iter().map(|y| (y.0, y.1)).collect::<HashSet<_>>().len() as i64;
    res as i64 + rs + 1
}

const DELTAS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];
pub(crate) fn solve(data: &str) -> (i64, i64) {
    let w = data.lines().next().unwrap().len() as i32;
    let h = data.lines().count() as i32;
    let start = data.find('S').unwrap() as i32;
    let end = data.find('E').unwrap() as i32;
    let end_pos = (end / (w + 1), end % (w + 1));
    let start_pos = (start / (w + 1), start % (w + 1));
    let mut data = String::from(data).into_bytes();
    let m = StrMap { data: data.as_mut_slice(), w, h };

    let crossings: Vec<_> = iproduct!(0..h, 0..w).filter_map(|(x, y)| {
        if m.get(x, y) == b'#' {
            return None;
        };
        let mut nbs = 0;
        for (dx, dy) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
            if m.get(x + dx, y + dy) != b'#' {
                nbs += 1;
            }
        };
        if nbs > 2 || (x, y) == start_pos || (x, y) == end_pos {
            Some((x, y))
        } else { None }
    }).collect();

    let step_to_next_crossing = |x: i32, y: i32, i: usize| -> Option<((i32, i32, usize), i32, i32)> {
        let mut curr = (x, y, i);
        let mut cost = 0;
        let mut dist = 0;
        loop {
            let mut nb: Option<(i32, i32, usize)> = None;
            if m.get(curr.0, curr.1) == b'E' {
                return Some((curr, cost, dist));
            }
            for j in 0..4 {
                // do not turn around
                if (4 + j - curr.2) % 4 == 2 {
                    continue;
                }
                let (ddx, ddy) = DELTAS[j];
                let (nnx, nny) = (curr.0 + ddx, curr.1 + ddy);
                if m.get(nnx, nny) != b'#' {
                    if nb.is_some() {
                        return Some((curr, cost, dist));
                    } else { nb = Some((nnx, nny, j)) }
                }
            }
            if nb.is_none() {
                return None;
            }
            let nb = nb.unwrap();
            cost += if nb.2 != curr.2 {
                1001
            } else { 1 };
            dist += 1;
            curr = nb;
        }
    };

    let mut jumps = HashMap::new();
    crossings.iter().for_each(|(x, y)| {
        for i in 0..4 {
            let (nx, ny) = (x + DELTAS[i].0, y + DELTAS[i].1);
            if m.get(nx, ny) == b'#' {
                continue;
            }
            step_to_next_crossing(nx, ny, i).map(|(s, cost, dist)| {
                jumps.insert((*x, *y, i), (s, cost, dist));
            });
        }
    });

    let mut distances = HashMap::new();
    let mut predecessors = HashMap::new();

    let mut frontier = PriorityQueue::new();
    frontier.push((start_pos.0, start_pos.1, 0usize), Reverse(0));
    loop {
        let (cs, dist) = frontier.pop().unwrap();
        distances.insert(cs, dist.0);
        if (cs.0, cs.1) == end_pos {
            return (dist.0 as i64, get_best_pred_counts(cs, &predecessors, &distances));
        }
        for rot in [0, 1, 3] {
            let np = (cs.0, cs.1, (cs.2 + rot) % 4);
            jumps.get(&np).iter().for_each(|&&(s2, cost, counts)| {
                if !distances.contains_key(&s2) {
                    let new_dist = 1 + dist.0 + cost + (rot % 2) as i32 * 1000;
                    frontier.push_increase(s2, Reverse(new_dist));
                    predecessors
                        .entry(s2)
                        .or_insert_with(Vec::new)
                        .push((cs, new_dist, counts, rot));
                }
            })
        }
    }
}