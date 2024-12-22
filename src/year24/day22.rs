use hashbrown::HashMap;
use itertools::Itertools;

const M: i64 = 1 << 24;
fn evolve(num: i64) -> i64 {
    let num = (num ^ (num << 6)) % M;
    let num = (num ^ (num >> 5)) % M;
    (num ^ (num << 11)) % M
}

fn solve_bananas(num: i64) -> HashMap<(i64, i64, i64, i64), i64> {
    let mut num = num;
    let mut res = HashMap::new();
    let mut diffs: Vec<i64> = vec![0; 4];
    for i in 0..2000 {
        let next_num = evolve(num);
        let curr_val = num % 10;
        let next_val = next_num % 10;
        let mut diffs_ = diffs[1..].into_iter().cloned().collect_vec();
        diffs_.push((next_val - curr_val));
        let difftup = (diffs_[0], diffs_[1], diffs_[2], diffs_[3]);
        if i >= 3 {
            if !res.contains_key(&difftup) {
                res.insert(difftup, next_val);
            }
        }
        diffs = diffs_;
        num = next_num;
    };
    res
}

pub(crate) fn solve(data: &str) -> (String, String) {
    let nums = data.lines().map(|x| x.parse::<i64>().unwrap()).collect_vec();

    let p1 = nums.iter().map(|&n| {
        let mut num = n;
        for _ in 0..2000 {
            num = evolve(num);
        }
        num
    }).sum::<i64>();

    let mut total_bmh: HashMap<_, i64> = HashMap::new();
    nums.iter().for_each(|&n| {
        let mut num = n;
        let bmh = solve_bananas(num);
        for (k, v) in bmh {
            let e = total_bmh.entry(k).or_insert(0);
            *e += v;
        }
    });

    (p1.to_string(), total_bmh.values().max().unwrap().to_string())
}