use std::cmp::max;
use memoize::memoize;
use crate::day07::ten_pow;

pub(crate) fn n_digs(x: i128) -> i128 {
    (x.checked_ilog10().unwrap_or(0) + 1) as i128
}


#[memoize]
fn count(n: i128, steps: i128) -> i64 {
    match (steps, n) {
        (0, _) => 1,
        (s, 0) => count(1, s - 1),
        (s, x) => {
            let digs = n_digs(x);
            match digs % 2 {
                0 => {
                    let ten_pow = 10_i128.pow((digs / 2) as u32);
                    count(n / ten_pow, s - 1) + count(n % ten_pow, s - 1)
                }
                _ => count(n.checked_mul(2024).unwrap(), s - 1)
            }
        }
    }
}

pub(crate) fn solve(data: &str) -> (i64, i64) {
    let nums = data.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
    let p1 = nums.iter().map(|&x| count(x as i128, 25)).sum();
    let p2 = nums.iter().map(|&x| count(x as i128, 75)).sum();
    (p1, p2)
}