use hashbrown::HashMap;
use itertools::Itertools;
use tokio::time::Instant;
use union_find::UnionFind;

pub(crate) fn solve(data: &str) -> (String, String) {
    let mut connections: Vec<Vec<u16>> = vec![Vec::new(); 1000];
    let mut node_index: HashMap<(char, char), u16> = HashMap::new();
    let mut node_lookup = Vec::new();

    let mut starts_with_t: Vec<bool> = vec![false; data.len()];
    let mut counts = HashMap::new();
    for line in data.lines() {
        let b = line.chars().collect_vec();
        let n1 = (b[0], b[1]);
        let n2 = (b[3], b[4]);
        *counts.entry(n1).or_insert(0) += 1;
        *counts.entry(n2).or_insert(0) += 1;
    }
    /*
    not even worth it, since they all have degree 13?
    counts.iter().sorted_by_key(|&(x, &c)| -c).for_each(|(&x, _)| {
        let _ = node_index.entry(x).or_insert_with(|| {
            node_lookup.push(x);
            (node_lookup.len() - 1) as u16
        });
    }
    );*/

    for line in data.lines() {
        let b = line.chars().collect_vec();
        let n1 = (b[0], b[1]);
        let node1 = node_index[&n1];
        let n2 = (b[3], b[4]);
        let node2 = node_index[&n2];
        if node1 < node2 {
            connections[node1 as usize].push(node2);
        } else {
            connections[node2 as usize].push(node1);
        }
        //comps.union(node1 as usize, node2 as usize);
        if b[0] == 't' {
            starts_with_t[node1 as usize] = true;
        }
        if b[3] == 't' {
            starts_with_t[node2 as usize] = true;
        }
    }
    let l = node_index.len();
    let mut res = 0;
    let mut triples = Vec::new();
    for i in 0..l {
        for &j in connections[i].iter() {
            let j = j as usize;
            for &k in connections[j].iter() {
                let k = k as usize;
                if connections[i].contains(&(k as u16)) {
                    // found triple
                    let any_starts_with_t = starts_with_t[k] || starts_with_t[j] || starts_with_t[i];
                    if any_starts_with_t {
                        res += 1;
                    }
                    triples.push(vec![i, j, k]);
                }
            }
        }
    }

    let started = Instant::now();

    // let mut candidates = (0..node_lookup.len()).map(|x|vec![x]).collect_vec();
    let mut candidates = triples;
    let mut password = String::new();

    while candidates.len() > 0 {
        let mut new_candidates = Vec::new();
        for candidate in candidates.iter() {
            let curr_ind = candidate[candidate.len() - 1];
            // let mut newly_connected = Vec::new();
            for &j in connections[curr_ind].iter() {
                let connected = candidate.iter().all(|&c| connections[c].contains(&j));
                if connected {
                    let mut nc = candidate.clone();
                    nc.push(j as usize);
                    new_candidates.push(nc);
                }
            }
        }
        if new_candidates.len() == 0 {
            assert_eq!(candidates.len(), 1);
            let mut parts = candidates.first().unwrap().iter().map(|&c| {
                let (c1, c2) = node_lookup[c as usize];
                format!("{}{}", c1, c2)
            }).collect_vec();
            parts.sort();
            password = parts.join(",");
        }
        candidates = new_candidates;
    }

    (res.to_string(), password)
}