use std::collections::HashMap;
use itertools::iproduct;
use crate::day10::{StrMap, DIRECTIONS};
use union_find::{UnionFind, UnionBySize, QuickUnionUf};

pub(crate) fn solve(data: &str) -> (i64, i64) {
    let h = data.lines().count() as i32;
    let w = data.lines().next().unwrap().len() as i32;

    let mut data = String::from(data).into_bytes();
    let m = StrMap { data: data.as_mut_slice(), h, w };
    let mut uf = QuickUnionUf::<UnionBySize>::new((w * h) as usize);
    iproduct!(0..h,0..w).for_each(|(x, y)| {
        let curr_sym = m.get(x, y) as usize;
        DIRECTIONS.iter().for_each(|(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            let nb_sym = m.get(nx, ny) as usize;
            if curr_sym == nb_sym {
                let (k0, k1) = ((x * w + y) as usize, (nx * w + ny) as usize);
                uf.union(k0, k1);
            }
        }
        )
    });

    let mut perimeters: HashMap<usize, i32> = HashMap::new();

    let mut connected_perimeters: HashMap<usize, i32> = HashMap::new();

    let mut areas: HashMap<usize, i32> = HashMap::new();

    iproduct!(0..h,0..w).for_each(|(x, y)| {
        let curr_sym = m.get(x, y) as usize;
        let cluster = uf.find((x * w + y) as usize);
        *areas.entry(cluster).or_insert(0) += 1;
        DIRECTIONS.iter().for_each(|(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            let nb_sym = m.get(nx, ny) as usize;
            if nb_sym != curr_sym {
                *perimeters.entry(cluster).or_insert(0) += 1;
            } else {
                // same cluster check=> check if edge connected
                DIRECTIONS.iter().for_each(|(ddx, ddy)| {
                    // only orthogonal
                    if dx * ddx + dy * ddy != 0 {
                        return;
                    }
                    let (xd, yd) = (x + ddx, y + ddy);
                    let xd_sym = m.get(xd, yd) as usize;
                    let (nxd, nyd) = (nx + ddx, ny + ddy);
                    let nxd_sym = m.get(nxd, nyd) as usize;
                    if xd_sym != curr_sym && nxd_sym != curr_sym {
                        // edge is connected
                        *connected_perimeters.entry(cluster).or_insert(0) += 1;
                    }
                })
            }
        });
    });
    let p1 = areas.iter().map(|(p, c)| {
        let peri = perimeters.get(p).unwrap_or(&0);
        (c * peri) as i64
    }).sum();
    let p2 = areas.iter().map(|(&p, &c)| {
        let &peri = perimeters.get(&p).unwrap_or(&0);
        let &peri2 = connected_perimeters.get(&p).unwrap_or(&0);
        // subtract connected perimeters (counted twice)
        (c * (peri - (peri2 / 2))) as i64
    }).sum();
    (p1, p2)
}