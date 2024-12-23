use itertools::{iterate, Itertools};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::sync::{Arc, Mutex};

const M: i64 = 1 << 24;
fn evolve(num: i64) -> i64 {
    let num = (num ^ (num << 6)) % M;
    let num = (num ^ (num >> 5)) % M;
    (num ^ (num << 11)) % M
}

const N_DIGS: usize = 19;
const OFFSET: usize = 9;
const D3: usize = N_DIGS;
const D2: usize = N_DIGS * D3;
const D1: usize = N_DIGS * D2;
const DT_LEN: usize = N_DIGS * D1;
const D1234: usize = D1 + D2 + D3 + 1;
fn solve_bananas(num: i64, seq_counts: &mut Vec<u16>) -> i64 {
    let mut seen: [bool; DT_LEN] = [false; DT_LEN];
    let mut res = 0;
    let m = iterate(num, |num: &i64| evolve(*num))
        .inspect(|x| res = *x)
        .map(|x| (x % 10) as usize);
    m.take(2001)
        .tuple_windows::<(_, _, _, _, _)>()
        .for_each(|(x0, x1, x2, x3, x4)| {
            let difftup =
                D1 * (OFFSET + x1 - x0) + D2 * (OFFSET + x2 - x1) + D3 * (OFFSET + x3 - x2) + (OFFSET + x4 - x3);
            if !seen[difftup] {
                seen[difftup] = true;
                seq_counts[difftup] += x4 as u16;
            }
        });
    res
}

pub(crate) fn solve(data: &str) -> (String, String) {
    let l = data.lines().count();
    let nums = data
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .chunks(l / 20)
        .into_iter()
        .map(|c| c.collect_vec())
        .collect::<Vec<_>>();

    let total_total = Arc::new(Mutex::new([0; DT_LEN]));
    let p1 = nums
        .par_iter()
        .map(|ns| {
            let mut chunk_total = vec![0; DT_LEN];
            let mut res = 0;
            for n in ns.iter() {
                res += solve_bananas(*n, &mut chunk_total);
            }
            let mut data = total_total.lock().unwrap();
            for i in 0..chunk_total.len() {
                data[i] += chunk_total[i];
            }
            res
        })
        .sum::<i64>();

    let tt = total_total.lock().unwrap();
    let p2 = tt.iter().max().unwrap();
    (p1.to_string(), p2.to_string())
}
