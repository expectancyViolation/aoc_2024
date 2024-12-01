use counter::Counter;
use itertools::Itertools;

pub fn parse(raw_input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();
    raw_input.lines().for_each(|line| {
        let (x, y) = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple().unwrap();
        left_list.push(x);
        right_list.push(y);
    });
    (left_list, right_list)
}

pub fn part1(left_list: &Vec<i64>, right_list: &Vec<i64>) -> i64 {
    let mut ll = left_list.clone();
    ll.sort();
    let mut rl = right_list.clone();
    rl.sort();
    ll.iter().zip(rl.iter()).map(|(l, &r)| (l - r).abs()).sum::<i64>()
}

pub fn part2(left_list: &Vec<i64>, right_list: &Vec<i64>) -> i64 {
    let counts = right_list.iter().collect::<Counter<_>>();
    left_list.iter().map(|x| x * (counts.get(x).cloned().unwrap_or(0)) as i64).sum::<i64>()
}

