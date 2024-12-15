use itertools::sorted;
use rayon::prelude::IntoParallelRefIterator;

use rayon::iter::ParallelIterator;

fn parse(day02_data: &str) -> Vec<Vec<i64>> {
    day02_data
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| str::parse(x).unwrap())
                .collect()
        })
        .collect()
}

fn part1_is_safe(report: &Vec<i64>) -> bool {
    let is_increasing = sorted(report).eq(report);
    let is_decreasing = sorted(report).rev().eq(report);
    let diffs_ok = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(x, y)| {
            let diff = (y - x).abs();
            (1 <= diff) && (diff <= 3)
        })
        .all(|x| x);
    (is_increasing || is_decreasing) && diffs_ok
}
fn part1(data: &Vec<Vec<i64>>) -> i64 {
    data.iter().filter(|&x| part1_is_safe(x)).count() as i64
}

fn part2_is_safe(report: &Vec<i64>) -> bool {
    (0..report.len())
        .map(|i| {
            report
                .iter()
                .enumerate()
                // remove i-th entry
                .filter_map(|(j, &x)| if j != i { Some(x) } else { None })
                .collect::<Vec<i64>>()
        })
        .any(|x| part1_is_safe(&x))
}

fn part2(data: &Vec<Vec<i64>>) -> i64 {
    data.par_iter().filter(|&x| part2_is_safe(x)).count() as i64
}

pub fn solve(input: &str) -> (i64, i64) {
    let data = parse(&input);
    let part1_result = part1(&data);
    let part2_result = part2(&data);
    (part1_result, part2_result)
}
