use itertools::Itertools;
use std::collections::VecDeque;
use crate::str_map::StrMap;
use crate::v::V;

const RIGHT: V = V(0, 1);
const LEFT: V = V(0, -1);
fn determine_move_poss(m: &StrMap, to_check: &mut VecDeque<V>, to_move: &mut Vec<V>, robot: V, dv: V) -> bool {
    to_check.clear();
    to_check.push_back(robot);
    to_move.clear();
    while !to_check.is_empty() {
        let curr_check = to_check.pop_front().unwrap();
        // this is O(N) but still fast, since vecs are small
        if to_move.contains(&curr_check) {
            continue;
        }
        to_move.push(curr_check);
        let np = curr_check + dv;
        let curr_sym = m.get(np.0, np.1) as char;
        match curr_sym {
            '#' => {
                return false;
            }
            '.' => {}
            'O' => {
                to_check.push_back(np);
            }
            '[' => {
                to_check.push_back(np);
                if dv.0 != 0 {
                    to_check.push_back(np + RIGHT);
                }
            }
            ']' => {
                to_check.push_back(np);
                if dv.0 != 0 {
                    to_check.push_back(np + LEFT);
                }
            }
            _ => unreachable!(),
        }
    }
    true
}

fn simulate_move(m: &mut StrMap, to_check: &mut VecDeque<V>, to_move: &mut Vec<V>, robot: &mut V, dv: V) {
    let can_move = determine_move_poss(m, to_check, to_move, *robot, dv);
    if can_move {
        for &v in to_move.iter().rev() {
            let curr_sym = m.get(v.0, v.1);
            let nv = v + dv;
            //assert_eq!(m.get(nv.0, nv.1) as char, '.');
            m.set(nv.0, nv.1, curr_sym);
            m.set(v.0, v.1, '.' as u8);
        }
        *robot = *robot + dv;
    }
}

fn solve_map(m: &mut StrMap, moves: &[u8]) -> i64 {
    let robot = m.find('@' as u8).unwrap();
    let mut robot = V(robot.0, robot.1);
    let mut to_move = Vec::new();
    let mut to_check = VecDeque::new();
    for &mov in moves {
        let dv = match (mov as char) {
            '\n' => {
                continue;
            }
            '>' => V(0, 1),
            '^' => V(-1, 0),
            '<' => V(0, -1),
            'v' => V(1, 0),
            _ => {
                continue;
            }
        };
        simulate_move(m, &mut to_check, &mut to_move, &mut robot, dv);
    }
    let width = m.w as i64;
    m.data
        .iter()
        .positions(|&x| (x == ('O' as u8)) || (x == ('[' as u8)))
        .map(|p| {
            let x = (p as i64) / (width + 1);
            let y = (p as i64) % (width + 1);
            (x * 100 + y)
        })
        .sum::<i64>()
}

pub(crate) fn solve(data: &str) -> (i64, i64) {
    let split = data.find("\n\n").unwrap();
    let width = data.lines().next().unwrap().len();
    let height = (split + 1) / width;
    let mut p1_data = data[..split].to_owned().into_bytes();
    let mut p2_data = p1_data
        .iter()
        .flat_map(|&x| {
            let res = match x as char {
                '.' => "..",
                'O' => "[]",
                '#' => "##",
                '@' => "@.",
                '\n' => "\n",
                _ => "",
            };
            res.as_bytes().iter().cloned()
        })
        .collect::<Vec<u8>>();
    let mut m1 = StrMap {
        data: p1_data.as_mut_slice(),
        h: height as i32,
        w: width as i32,
    };
    let moves = &data[split + 1..];
    let p1 = solve_map(&mut m1, moves.as_ref());

    let mut m2 = StrMap {
        data: p2_data.as_mut_slice(),
        h: height as i32,
        w: 2 * width as i32,
    };
    let p2 = solve_map(&mut m2, moves.as_ref());

    (p1, p2)
}
