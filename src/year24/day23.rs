use bnum::BUint;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use rand::prelude::{SliceRandom, StdRng};
use rand::SeedableRng;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::sync::{Arc, Mutex};

type UINodeMask = BUint<11>; // 26*26 =676 < 64*11=724

const NUM_LETTERS: usize = 26;

fn bron_kerbosch(
    neighbors: &Vec<UINodeMask>,
    r: UINodeMask,
    p: UINodeMask,
    x: UINodeMask,
    res: &mut UINodeMask,
    collision: &mut bool,
) {
    let mut x = x;
    let mut p = p;
    if p.is_zero() && x.is_zero() {
        if r.count_ones() == res.count_ones() {
            *collision = true;
        }
        if r.count_ones() > res.count_ones() {
            *res = r;
            *collision = false;
        }
    };
    let mut cands = (p | x);
    // bitset makes pivot selection difficult. just pick the lowest one
    let mut cand = cands.trailing_zeros();
    let mut q = p;
    if cand < UINodeMask::BITS {
        let nu = neighbors[cand as usize];
        q = p & (!nu);
    }
    while !q.is_zero() {
        let v = q.trailing_zeros();
        let nv = neighbors[v as usize];
        let vm = UINodeMask::power_of_two(v);
        bron_kerbosch(neighbors, r | vm, p & nv, x & nv, res, collision);
        p = p ^ vm;
        q = q ^ vm;
        x = x | vm;
    }
}

fn bron_kerbosch_top(
    neighbors: &Vec<UINodeMask>,
    deg_ordered: &Vec<usize>,
    p: UINodeMask,
    res: &mut UINodeMask,
    collision: &mut bool,
) {
    let mut ps = Arc::new(Mutex::new(p));
    let mut xs = Arc::new(Mutex::new(UINodeMask::ZERO));
    let mut verts = (0..NUM_LETTERS * NUM_LETTERS).collect_vec();
    let mut rng = StdRng::from_os_rng();
    verts.shuffle(&mut rng);
    let sols = deg_ordered
        .par_iter()
        .map(|&v| {
            let p = ps.lock().unwrap().clone();
            let x = xs.lock().unwrap().clone();
            let vm = UINodeMask::power_of_two(v as u32);
            let nv = neighbors[v];
            let mut res = UINodeMask::ZERO;
            let mut collision = false;
            bron_kerbosch(neighbors, vm, p & nv, x, &mut res, &mut collision);
            let mut p = ps.lock().unwrap();
            *p = *p ^ vm;
            let mut x = xs.lock().unwrap();
            *x = *x | vm;
            (res, collision)
        })
        .collect::<Vec<_>>();
    sols.iter().for_each(|&(r, c)| {
        if r.count_ones() == res.count_ones() {
            if r != *res {
                *collision = true;
            }
        }
        if r.count_ones() > res.count_ones() {
            *res = r;
            *collision = c;
        }
    })
}

fn deg_order(neighbors: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut low_degrees: PriorityQueue<usize, _> = PriorityQueue::new();
    let mut remaining_degrees = vec![0; neighbors.len()];
    neighbors.iter().enumerate().for_each(|(i, v)| {
        if !v.is_empty() {
            let deg = v.len();
            low_degrees.push(i, deg);
            remaining_degrees[i] = deg;
        }
    });
    let mut res = Vec::new();
    while !low_degrees.is_empty() {
        let (curr_vert, _) = low_degrees.pop().unwrap();
        res.push(curr_vert);
        remaining_degrees[curr_vert] = 0;
        for &x in neighbors[curr_vert].iter() {
            let rem_deg = remaining_degrees[x];
            if rem_deg > 0 {
                low_degrees.push_decrease(x, rem_deg - 1);
            }
        }
    }
    //assert_eq!(remaining_degrees.iter().sum::<usize>(),0);
    res
}

pub(crate) fn solve(data: &str) -> (String, String) {
    let mut neighbor_mask: Vec<UINodeMask> = vec![UINodeMask::ZERO; 1000];
    let mut neighbors: Vec<Vec<usize>> = vec![Vec::new(); 1000];
    let mut P = UINodeMask::ZERO;

    for line in data.lines() {
        let b = line.bytes().collect_vec();
        let n1 = ((b[0] - b'a') as usize) * 26 + (b[1] - b'a') as usize;
        let n1_mask = UINodeMask::power_of_two(n1 as u32);
        P |= n1_mask;

        let n2 = ((b[3] - b'a') as usize) * 26 + (b[4] - b'a') as usize;
        let n2_mask = UINodeMask::power_of_two(n2 as u32);
        P |= n2_mask;

        neighbor_mask[n1] |= n2_mask;
        neighbor_mask[n2] |= n1_mask;
        if n1 < n2 {
            //assert!(!neighbors[n1].contains(&n2));
            neighbors[n1].push(n2);
        } else {
            //assert!(!neighbors[n2].contains(&n1));
            neighbors[n2].push(n1);
        }
    }

    let solve_p1 = || {
        let mut p1 = 0;
        for i in 0..26 * 26 {
            for &j in neighbors[i].iter() {
                for &k in neighbors[j].iter() {
                    if neighbors[i].contains(&k) {
                        // found triple
                        assert!((i < j) && (j < k));
                        let t = (b't' - b'a') as usize;
                        let any_starts_with_t = (i / 26) == t || (j / 26) == t || (k / 26) == t;
                        if any_starts_with_t {
                            p1 += 1;
                        }
                    }
                }
            }
        }
        p1.to_string()
    };

    let solve_p2 = || {
        let mut res = UINodeMask::ZERO;
        let mut collision = false;
        let parallel = false;

        //// parallel version (is not worth it for regular input)
        if parallel {
            let deg_ordered = deg_order(&neighbors);
            assert_eq!(
                deg_ordered.len(),
                neighbors.iter().filter(|x| x.len() > 0).count()
            );
            bron_kerbosch_top(&neighbor_mask, &deg_ordered, P, &mut res, &mut collision);
            println!("collision {}", collision);
        } else {
            bron_kerbosch(
                &neighbor_mask,
                UINodeMask::ZERO,
                P,
                UINodeMask::ZERO,
                &mut res,
                &mut collision,
            );
        }

        let mut parts = vec![];
        while !res.is_zero() {
            let v = res.trailing_zeros() as usize;
            let (c1, c2) = (v / 26, v % 26);
            parts.push(format!(
                "{}{}",
                (c1 as u8 + b'a') as char,
                (c2 as u8 + b'a') as char
            ));
            res ^= UINodeMask::power_of_two(v as u32);
        }
        parts.join(",")
    };
    let parts = vec![1, 2];
    // par iter is not worth it
    let res = parts
        .iter()
        .map(|&p| {
            let res = if p == 1 {
                solve_p1()
                //"XXX".to_string()
            } else {
                solve_p2()
            };
            println!("{}", res);
            res
        })
        .collect::<Vec<_>>();
    (res[0].clone(), res[1].clone())
}
