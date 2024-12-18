use itertools::Itertools;

pub(crate) fn solve(data: &str) -> (String, String) {
    let mut nodes: Vec<Vec<_>> = vec![Vec::new(); 256];
    for (i, line) in data.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            match char {
                '.' | '\n' => {}
                _ => {
                    nodes[char as usize].push((i as i64, j as i64));
                }
            }
        }
    };
    let h = data.lines().count() as i64;
    let w = data.lines().next().unwrap().len() as i64;
    let mut part1_anti_nodes = vec![vec![false; w as usize]; h as usize];
    let mut part2_anti_nodes = vec![vec![false; w as usize]; h as usize];
    let mark = |v: &mut Vec<Vec<bool>>, x: i64, y: i64| -> bool{
        if (0 <= x) && (x < h) && (0 <= y) && (y < w) {
            v[x as usize][y as usize] = true;
            true
        } else {
            false
        }
    };

    for node_group in nodes {
        for ns in node_group.iter().combinations(2) {
            let &(x1, y1) = ns[0];
            let &(x2, y2) = ns[1];

            let dx = x2 - x1;
            let dy = y2 - y1;
            mark(&mut part1_anti_nodes, x1 - dx, y1 - dy);
            mark(&mut part1_anti_nodes, x2 + dx, y2 + dy);

            ////this seems to not be necessary for given input
            //let g = euclid_u64(dx.abs() as u64, dy.abs() as u64) as i64;
            //dx = dx / g;
            //dy = dy / g;

            let deltas = [(dx, dy), (-dx, -dy)];
            for &(dx, dy) in deltas.iter() {
                let mut px = x1;
                let mut py = y1;
                while mark(&mut part2_anti_nodes, px, py) {
                    px += dx;
                    py += dy;
                }
            }
        }
    };
    let p1 = part1_anti_nodes.iter().flatten().filter(|&&v| v).count();
    let p2 = part2_anti_nodes.iter().flatten().filter(|&&v| v).count();
    (p1.to_string(), p2.to_string())
}