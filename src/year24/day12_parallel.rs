use itertools::iproduct;
use rayon::iter::ParallelIterator;
use std::cmp::min;
use rayon::prelude::IntoParallelRefIterator;
use hashbrown::HashSet;
use crate::str_map::{StrMap, DIRECTIONS};

const BLOCK_SIZE: usize = 50;


pub(crate) fn solve(data: &str) -> (String, String) {
    let h = data.lines().count() as i32;
    let w = data.lines().next().unwrap().len() as i32;

    let mut data = String::from(data).into_bytes();
    let m = StrMap {
        data: data.as_mut_slice(),
        h,
        w,
    };

    let h_splits = (0..h).step_by(BLOCK_SIZE).collect::<Vec<i32>>();
    let w_splits = (0..w).step_by(BLOCK_SIZE).collect::<Vec<i32>>();

    let block_starts: Vec<_> = iproduct!(h_splits, w_splits).collect();
    let tups = block_starts
        .par_iter()
        .map(|&(h_start, w_start)| {
            let h_end = min(h_start + (BLOCK_SIZE as i32), h);
            let w_end = min(w_start + (BLOCK_SIZE as i32), w);
            let mut visited = HashSet::with_capacity(BLOCK_SIZE * BLOCK_SIZE);
            let mut to_visit = Vec::new();

            let mut res1 = 0;
            let mut res2 = 0;
            for (x, y) in iproduct!(h_start..h_end, w_start..w_end) {
                if visited.contains(&(x, y)) {
                    continue;
                }
                let mut valid = true;
                let mut boundary = 0;
                let mut inline = 0;
                let mut area = 0;
                let sym = m.get(x, y);
                to_visit.clear();
                to_visit.push((x, y));
                while to_visit.len() > 0 {
                    let (cx, cy) = to_visit.pop().unwrap();
                    if visited.contains(&(cx, cy)) {
                        continue;
                    }
                    visited.insert((cx, cy));
                    area += 1;
                    for (dx, dy) in DIRECTIONS {
                        let (nx, ny) = (cx + dx, cy + dy);
                        let new_sym = m.get(nx, ny);
                        if new_sym == sym {
                            // only count regions if their leftmost-upmost point is within our block
                            if (ny < w_start)
                                || ((nx < h_start) && (ny < w_start + (BLOCK_SIZE as i32)))
                            {
                                valid = false;
                            }
                            // rotate 90
                            let (xd, yd) = (cx - dy, cy + dx);
                            let (nxd, nyd) = (nx - dy, ny + dx);
                            if (m.get(xd, yd) != sym) && (m.get(nxd, nyd) != sym) {
                                inline += 1;
                            }
                            // rotate -90
                            let (xd, yd) = (cx + dy, cy - dx);
                            let (nxd, nyd) = (nx + dy, ny - dx);
                            if (m.get(xd, yd) != sym) && (m.get(nxd, nyd) != sym) {
                                inline += 1;
                            }
                            to_visit.push((nx, ny));
                        } else {
                            boundary += 1;
                        }
                    }
                }
                if valid {
                    res1 += boundary * area;
                    res2 += (boundary - inline / 2) * area;
                }
            }
            (res1, res2)
        })
        .collect::<Vec<_>>();
    let (r1, r2) = tups.iter()
        .fold((0, 0), |(p11, p21), (p12, p22)| (p11 + p12, p21 + p22));
    (r1.to_string(), r2.to_string())
}
