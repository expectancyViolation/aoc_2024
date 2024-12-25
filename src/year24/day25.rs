use itertools::Itertools;

pub(crate) fn solve(data: &str) -> (String, String) {
    let keys = data.split("\n\n").map(|block| {
        let mut counts = [0; 7];
        let first_line = block.lines().next().unwrap();
        let fc = first_line.chars().next().unwrap();
        if fc == '#' {
            counts[5] = 7;
        } else {
            counts[6] = 7;
        };
        for line in block.lines() {
            for (i, x) in line.chars().enumerate() {
                if x == '#' {
                    counts[i] += 1;
                }
            }
        }
        counts
    }).collect::<Vec<_>>();
    let mut res = 0;
    for v in keys.iter().combinations(2) {
        let matches = v[0].iter().zip(v[1]).all(|(a, b)| (a + b) <= 7);
        if matches {
            res += 1;
        }
    }
    (res.to_string(), String::new())
}