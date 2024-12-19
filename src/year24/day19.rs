use itertools::Itertools;

fn match_pattern(towels: &Vec<String>, pattern: &str) -> i128 {
    let mut counts = vec![0; pattern.len() + 1];

    // this could be sped up by precalculating match table
    // (the smart way, without full compare at each pos)
    counts[0] = 1;
    for i in 0..pattern.len() {
        for towel in towels {
            let new_ind = i + towel.len();
            if new_ind > pattern.len() {
                continue;
            }
            if pattern[i..].starts_with(towel) {
                counts[new_ind] += counts[i];
            }
        }
    }
    counts[pattern.len()]
}


pub(crate) fn solve(data: &str) -> (String, String) {
    let towels = data
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();


    let counts = data
        .lines()
        .skip(2)
        .map(|x| match_pattern(&towels, &x.trim())
        )
        .collect_vec();

    let p1 = counts.iter().filter(|&&c| c > 0).count();

    let p2 = counts.iter().sum::<i128>();

    (p1.to_string(), p2.to_string())
}
