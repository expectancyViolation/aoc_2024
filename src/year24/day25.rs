use itertools::{iproduct, Itertools};

pub(crate) fn solve(data: &str) -> (String, String) {

    let mut keys = Vec::new();
    let mut locks = Vec::new();
    data.split("\n\n").for_each(|block| {
        let mut counts = [0; 5];
        let first_line = block.lines().next().unwrap();
        let is_key = first_line.chars().next().unwrap() == '#';

        for line in block.lines() {
            for (i, x) in line.chars().enumerate() {
                if x == '#' {
                    counts[i] += 1;
                }
            }
        }
        if is_key {
            keys.push(counts);
        } else {
            locks.push(counts);
        }
    });
    let mut res = 0;
    for v in iproduct!(keys, locks) {
        let matches = v.0.iter().zip(v.1).all(|(a, b)| (a + b) <= 7);
        if matches {
            res += 1;
        }
    }
    (res.to_string(), String::new())
}