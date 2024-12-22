use hashbrown::{HashMap, HashSet};
use itertools::{iterate, Itertools};
use std::iter;

const M: i64 = 1 << 24;
fn evolve(num: i64) -> i64 {
    let num = (num ^ (num << 6)) % M;
    let num = (num ^ (num >> 5)) % M;
    (num ^ (num << 11)) % M
}

const DT_LEN: usize = 20 * 20 * 20 * 20;
const D1: usize = 20 * 20 * 20;
const D2: usize = 20 * 20;
const D3: usize = 20;

const D1234: usize = D1 + D2 + D3 + 1;
type BananaArr = [i32; DT_LEN];
fn solve_bananas(num: i64, hm: &mut BananaArr) -> i64 {
    let mut seen: [bool; DT_LEN] = [false; DT_LEN];
    let mut res = 0;
    let m = iterate(num, |num: &i64| evolve(*num))
        .inspect(|x| res = *x)
        .map(|x| (x % 10) as usize);
    m.take(2001)
        .tuple_windows::<(_, _, _, _, _)>()
        .for_each(|(x0, x1, x2, x3, x4)| {
            let difftup =
                D1 * (10 + x1 - x0) + D2 * (10 + x2 - x1) + D3 * (10 + x3 - x2) + (10 + x4 - x3);
            // let difftup = D1 * x1 + D2 * x2 + D3 * x3 + x4 + D1234 * (10 - x0);
            if !seen[difftup] {
                seen[difftup] = true;
                hm[difftup] += x4 as i32;
            }
        });
    res
}

pub(crate) fn solve(data: &str) -> (String, String) {
    let nums = data
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    println!("nums: {:?}", nums.len());

    let mut total_bmh: BananaArr = [0; DT_LEN];
    let p1 = nums
        .iter()
        .map(|&n| solve_bananas(n, &mut total_bmh))
        .sum::<i64>();

    (p1.to_string(), total_bmh.iter().max().unwrap().to_string())
}
