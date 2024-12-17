use std::cmp::Reverse;
use hashbrown::{HashMap, HashSet};
use itertools::iproduct;
use priority_queue::PriorityQueue;
use crate::str_map::StrMap;

type D16State = (i32, i32, usize);

fn get_best_pred_counts(
    end_state: D16State,
    m: &StrMap,
    distances: &HashMap<D16State, i32>,
) -> i64 {
    let mut frontier: PriorityQueue<D16State, i32> = PriorityQueue::new();
    let mut visited: HashSet<D16State> = HashSet::new();
    frontier.push(end_state, distances[&end_state]);
    let mut res = 0;
    while !frontier.is_empty() {
        let (x, dist) = frontier.pop().unwrap();
        visited.insert(x);
        let np = (x.0, x.1, (x.2 + 2) % 4);
        let (nx, ny) = (np.0 + DELTAS[np.2].0, np.1 + DELTAS[np.2].1);
        if m.get(nx, ny) == b'#' {
            continue;
        }
        step_to_next_crossing(&m, nx, ny, np.2).map(|(s, cost, counts)| {
            // prevent recount if already found
            let mut found = false;
            for rot in [1, 2, 3] {
                let mut expected_cost = distances[&x] - cost - 1;
                if rot != 2 {
                    expected_cost -= 1000;
                }
                let nnp = (s.0, s.1, (s.2 + rot) % 4);
                if distances.contains_key(&nnp) {
                    if distances[&nnp] == expected_cost {
                        if !visited.contains(&nnp) {
                            frontier.push_increase(nnp, distances[&nnp]);
                        }
                        if !found {
                            res += counts;
                            found = true;
                        }
                    }
                }
            }
        });
    }
    let rs = visited.iter().map(|y| (y.0, y.1)).collect::<HashSet<_>>().len() as i64;
    res as i64 + rs
}

fn step_to_next_crossing(m: &StrMap, x: i32, y: i32, i: usize) -> Option<((i32, i32, usize), i32, i32)> {
    let mut curr = (x, y, i);
    let mut cost = 0;
    let mut dist = 0;
    loop {
        let mut nb: Option<(i32, i32, usize)> = None;
        let curr_sym = m.get(curr.0, curr.1);
        if curr_sym == b'E' || curr_sym == b'S' {
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


    let mut distances = HashMap::new();
    let mut frontier = PriorityQueue::new();
    frontier.push((start_pos.0, start_pos.1, 0usize), Reverse(0));
    loop {
        let (cs, dist) = frontier.pop().unwrap();
        distances.insert(cs, dist.0);
        if (cs.0, cs.1) == end_pos {
            return (dist.0 as i64, get_best_pred_counts(cs, &m, &distances));
        }
        for rot in [0, 1, 3] {
            let np = (cs.0, cs.1, (cs.2 + rot) % 4);
            let (nx, ny) = (np.0 + DELTAS[np.2].0, np.1 + DELTAS[np.2].1);
            if m.get(nx, ny) == b'#' {
                continue;
            }
            step_to_next_crossing(&m, nx, ny, np.2).map(|(s, cost, counts)| {
                if !distances.contains_key(&s) {
                    let new_dist = 1 + dist.0 + cost + (rot % 2) as i32 * 1000;
                    frontier.push_increase(s, Reverse(new_dist));
                }
            });
        }
    }
}