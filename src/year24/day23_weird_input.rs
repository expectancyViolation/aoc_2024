use itertools::Itertools;
use union_find::UnionFind;

// input graph vertices all have degree 13
// solution clique has size 13
// => constructing a maximal clique for each vertex using the greedy strategy
//  has a very high chance of never hitting the "wrong" neighbor  (almost 1-(1/13)^13)
// => no backtracking necessary
pub(crate) fn solve(data: &str) -> (String, String) {
    let mut connections: Vec<Vec<usize>> = vec![Vec::new(); 26 * 26];

    for line in data.lines() {
        let b = line.bytes().collect_vec();
        let n1 = ((b[0] - b'a') as usize) * 26 + (b[1] - b'a') as usize;

        let n2 = ((b[3] - b'a') as usize) * 26 + (b[4] - b'a') as usize;

        if n1 < n2 {
            connections[n1].push(n2);
        } else {
            connections[n2].push(n1);
        }
    }
    let mut res = 0;
    for i in 0..26 * 26 {
        for &j in connections[i].iter() {
            for &k in connections[j].iter() {
                if connections[i].contains(&k) {
                    // found triple
                    let t = (b't' - b'a') as usize;
                    let any_starts_with_t = (i / 26) == t || (j / 26) == t || (k / 26) == t;
                    if any_starts_with_t {
                        res += 1;
                    }
                }
            }
        }
    }

    let mut password_clique = Vec::new();
    for v in 0..26 * 26 {
        let mut maximal_clique = vec![v];
        for &cand in connections[v].iter() {
            let connected = maximal_clique.iter().all(|&c| (connections[c].contains(&cand) || connections[cand].contains(&c)));
            if connected {
                maximal_clique.push(cand);
            }
        }
        if maximal_clique.len() > password_clique.len() {
            password_clique = maximal_clique;
        }
    }

    let mut parts = password_clique.iter().map(|&c| {
        let (c1, c2) = (c / 26, c % 26);
        format!("{}{}", (c1 as u8 + b'a') as char, (c2 as u8 + b'a') as char)
    }).collect_vec();
    parts.sort();
    let password = parts.join(",");

    (res.to_string(), password)
}