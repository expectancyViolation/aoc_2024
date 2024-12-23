use bnum::BUint;
use itertools::Itertools;
//use rayon::iter::IntoParallelRefIterator;
//use rayon::iter::ParallelIterator;

type UINodeMask = BUint<11>; // 26*26 =676 < 64*11=724


fn bron_kerbosch(neighbors: &Vec<UINodeMask>, r: UINodeMask, p: UINodeMask, x: UINodeMask, res: &mut UINodeMask)
{
    let mut x = x;
    let mut p = p;
    if p.is_zero() && x.is_zero() {
        if r.count_ones() > res.count_ones() {
            *res = r;
        }
    };
    let u = (p | x).trailing_zeros();
    let mut q = p;
    if u < UINodeMask::BITS {
        let nu = neighbors[u as usize];
        q = p & (!nu);
    }
    while !q.is_zero() {
        let v = q.trailing_zeros();
        let nv = neighbors[v as usize];
        let vm = UINodeMask::power_of_two(v);
        bron_kerbosch(neighbors, r | vm, p & nv, x & nv, res);
        p = p ^ vm;
        q = q ^ vm;
        x = x | vm;
    }
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
            neighbors[n1].push(n2);
        } else {
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
        bron_kerbosch(&neighbor_mask, UINodeMask::ZERO, P, UINodeMask::ZERO, &mut res);

        let mut parts = vec![];
        while !res.is_zero() {
            let v = res.trailing_zeros() as usize;
            let (c1, c2) = (v / 26, v % 26);
            parts.push(format!("{}{}", (c1 as u8 + b'a') as char, (c2 as u8 + b'a') as char));
            res ^= UINodeMask::power_of_two(v as u32);
        }
        parts.join(",")
    };
    let parts = vec![1, 2];
    // par iter is not worth it
    let res = parts.iter().map(|&p| {
        let res = if p == 1 {
            solve_p1()
        } else {
            solve_p2()
        };
        res
    }).collect::<Vec<_>>();
    (res[0].clone(), res[1].clone())
}