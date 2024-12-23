use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use tokio::time::Instant;
use union_find::{QuickUnionUf, UnionByRank, UnionFind};

pub(crate) fn solve(data: &str) -> (String, String) {
//     let data="kh-tc
// qp-kh
// de-cg
// ka-co
// yn-aq
// qp-ub
// cg-tb
// vc-aq
// tb-ka
// wh-tc
// yn-cg
// kh-ub
// ta-co
// de-co
// tc-td
// tb-wq
// wh-td
// ta-ka
// td-qp
// aq-cg
// wq-ub
// ub-vc
// de-ta
// wq-aq
// wq-vc
// wh-yn
// ka-de
// kh-ta
// co-tc
// wh-qp
// tb-vc
// td-yn";
    let mut connections: Vec<Vec<usize>> = vec![Vec::new(); 1000];
    let mut node_index: HashMap<(char, char), usize> = HashMap::new();
    let mut node_lookup = Vec::new();
    // let mut starts_with_t: HashSet<u16> = HashSet::new();

    // let mut comps: QuickUnionUf<UnionByRank> = QuickUnionUf::new(data.len());
    let mut starts_with_t: Vec<bool> = vec![false; data.len()];
    let mut ind: usize = 0;
    for line in data.lines() {
        let b = line.chars().collect_vec();
        let n1 = (b[0], b[1]);
        let node1 = *node_index.entry(n1).or_insert_with(|| {
            node_lookup.push(n1);
            (node_lookup.len() - 1)
        });
        let n2 = (b[3], b[4]);
        let node2 = *node_index.entry(n2).or_insert_with(|| {
            node_lookup.push(n2);
            (node_lookup.len() - 1)
        });
        connections[node1 as usize].push(node2);
        connections[node2 as usize].push(node1);
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
    let started = Instant::now();
    for i in 0..l {
        for &j in connections[i].iter() {
            let j = j as usize;
            if j < i {
                continue;
            }
            for &k in connections[j].iter() {
                let k = k as usize;
                if k < j {
                    continue;
                }
                if connections[i].contains(&k) {
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
    println!("part1 {}", started.elapsed().as_millis());

    let started = Instant::now();

    let mut password = String::new();
    let get_cands = |state: &Vec<usize>| {
        let mut candidates = vec![];
        for &t in state.iter() {
            for &c in connections[t as usize].iter() {
                if !candidates.contains(&c) && (c> *state.last().unwrap()) {
                    candidates.push(c);
                }
            }
        }
        candidates
    };
    let mut best = 0;
    let mut best_clique = Vec::new();
    for triple in triples.into_iter() {
        let candidates = get_cands(&triple);
        let mut cand_stack = vec![candidates];
        let mut state_stack = vec![triple];
        let mut cand_inds = vec![0];
        while !cand_stack.is_empty() {
            let stack_pos=cand_stack.len()-1;
            let candidates = &cand_stack[stack_pos];
            let cand_ind = cand_inds[stack_pos];
            if cand_ind >= candidates.len() {
                cand_stack.pop();
                cand_inds.pop();
                state_stack.pop();
                continue;
            }
            let candidate = candidates[cand_ind];
            let state = &state_stack[stack_pos];
            // if cand_ind==0{
            //     println!("candidate: {:?}, state: {:?}", candidate, state);
            // }
            cand_inds[cand_stack.len() - 1] += 1;
            if state.len() > best {
                best = state.len();
                println!("best is {}",best);
                best_clique = state.clone();
            }
            let connected = state.iter().all(|&c| (connections[c].contains(&candidate)));
            if connected {
                let new_cands = get_cands(state);
                cand_stack.push(new_cands);
                cand_inds.push(0);
                let mut new_state = state.clone();
                new_state.push(candidate);
                state_stack.push(new_state);
            }
        }
    }

    let mut parts = best_clique.iter().map(|&c| {
        let (c1, c2) = node_lookup[c as usize];
        format!("{}{}", c1, c2)
    }).collect_vec();
    parts.sort();
    password = parts.join(",");

    println!("part1 {}", started.elapsed().as_millis());

    (res.to_string(), password)
}