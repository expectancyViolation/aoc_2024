use crate::str_map::DIRECTIONS;
use hashbrown::HashSet;
use itertools::{iproduct, Itertools};
use std::ops::ControlFlow;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

pub(crate) fn solve(data: &str) -> (i64, i64) {
    let mut p1 = 0;
    const W: i32 = 71;
    let mut blocked = [[false; W as usize]; W as usize];
    let mut comps: QuickUnionUf<UnionBySize> = QuickUnionUf::new(((W + 2) * (W + 2)) as usize);
    let (mut cx, mut cy) = (0, -1);
    let (mut dx, mut dy) = (1, 0);
    let (mut px, mut py) = (cx, cy);
    let k = |x: i32, y: i32| ((x + 1) * (W + 2) + (y + 1)) as usize;
    // walk boundary and form 2 unions of walls (lower left and upper right)
    while (cx, cy) != (-1, -1) {
        if (cx, cy) != (W, W) {
            comps.union(k(cx, cy), k(px, py));
        }
        if (cx + dx == W + 1) || (cx + dx == -2) || (cy + dy == W + 1) || (cy + dy == -2) {
            (dx, dy) = (-dy, dx);
        }
        (px, py) = (cx, cy);
        cx += dx;
        cy += dy;
    }
    let left_key = k(-1, 0);
    let right_key = k(0, -1);
    assert_ne!(comps.find(left_key), comps.find(right_key));

    let res = data.lines().enumerate().try_for_each(|(j, line)| {
        let (x, y) = line
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        blocked[x][y] = true;
        let x = x as i32;
        let y = y as i32;

        // merge walls (even across diagonals)
        for (dx, dy) in iproduct!((-1..2), (-1..2)) {
            let (nx, ny) = (x + dx, y + dy);
            if !(0 <= nx && nx < W && 0 <= ny && ny < W) || blocked[nx as usize][ny as usize] {
                comps.union(k(x, y), k(nx, ny));
            }
        }
        // blocked from lower left to upper right => no path possible anymore
        if comps.find(right_key) == comps.find(left_key) {
            return ControlFlow::Break((x, y));
        }
        if j == 1024 {
            let mut frontier = HashSet::new();
            frontier.insert((0, 0));
            let mut distances = [[-1; W]; W];
            distances[0][0] = 0;
            for i in 1.. {
                if frontier.is_empty() {
                    break;
                }
                let mut nf = HashSet::new();
                for (x, y) in frontier.iter() {
                    for (dx, dy) in DIRECTIONS {
                        let (nx, ny) = (x + dx, y + dy);
                        if 0 <= nx && nx < W && 0 <= ny && ny < W {
                            if !blocked[nx as usize][ny as usize]
                                && distances[nx as usize][ny as usize] == -1
                            {
                                distances[nx as usize][ny as usize] = i;
                                nf.insert((nx, ny));
                            }
                        }
                    }
                }
                frontier = nf;
            }
            p1 = distances[70][70];
        }
        ControlFlow::Continue(())
    });
    let p2 = res.break_value().unwrap();
    (p1, (p2.0 * 100 + p2.1) as i64)
}
