use crate::str_map::StrMap;
use crate::v::V;
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::VecDeque;
use std::sync::Mutex;

const RIGHT: V = V(0, 1);
const LEFT: V = V(0, -1);


fn calculate_step(m: &StrMap,
                  from_: &mut VecDeque<V>,
                  to_: &mut VecDeque<V>,
                  to_move: &mut Vec<V>,
                  dv: V) -> bool {
    let is_vertical = dv.0 != 0;
    while !from_.is_empty() {
        let curr_check = from_.pop_front().unwrap();
        to_move.push(curr_check);
        let np = curr_check + dv;
        let curr_sym = m.get(np.0, np.1) as char;
        match curr_sym {
            '#' => {
                return false;
            }
            '.' => {}
            'O' => {
                //if !to_.contains(&np) {
                to_.push_back(np);
                //}
            }
            '[' => {
                to_.push_back(np);
                if is_vertical {
                    let npp = np + RIGHT;
                    to_.push_back(npp);
                }
            }
            ']' => {
                if is_vertical {
                    let npp = np + LEFT;
                    if to_.len() == 0 || to_[to_.len() - 1] != np {
                        to_.push_back(npp);
                        to_.push_back(np);
                    }
                } else {
                    to_.push_back(np);
                }
            }
            _ => unreachable!(),
        }
    }
    true
}

fn calculate_move(m: &StrMap,
                  tc1: &mut VecDeque<V>,
                  tc2: &mut VecDeque<V>,
                  to_move: &mut Vec<V>
                  , robot: V, dv: V) -> bool {
    tc1.clear();
    tc2.clear();
    tc1.push_back(robot);
    to_move.clear();
    let mut i = 0;
    while !(tc1.is_empty() && tc2.is_empty()) {
        // assert!(i < 20);
        if i % 2 == 0 {
            if !calculate_step(m, tc1, tc2, to_move, dv) {
                return false;
            }
        } else {
            if !calculate_step(m, tc2, tc1, to_move, dv) {
                return false;
            }
        }
        i += 1;
    }
    true
}

fn simulate_move(m: &mut StrMap,
                 tc1: &mut VecDeque<V>,
                 tc2: &mut VecDeque<V>,
                 to_move: &mut Vec<V>, robot: &mut V, dv: V) {
    let can_move = calculate_move(m, tc1, tc2, to_move, *robot, dv);
    if can_move {
        for &v in to_move.iter().rev() {
            let nv = v + dv;
            //assert_eq!(m.get(nv.0, nv.1) as char, '.');
            m.swap(v.0, v.1, nv.0, nv.1);
        }
        *robot = *robot + dv;
    }
}

fn solve_map(m: &mut StrMap, moves: &[u8]) -> i64 {
    let robot = m.find('@' as u8).unwrap();
    let mut robot = V(robot.0, robot.1);
    let mut to_move = Vec::new();

    let mut tc1 = VecDeque::new();

    let mut tc2 = VecDeque::new();
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
        simulate_move(m, &mut tc1, &mut tc2, &mut to_move, &mut robot, dv);
    }
    let width = (m.w + 1) as i64;
    m.data
        .iter()
        .positions(|&x| (x == ('O' as u8)) || (x == ('[' as u8)))
        .map(|p| {
            let x = (p as i64) / width;
            let y = (p as i64) % width;
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

    let moves = &data[split + 1..];
    let maps = [Mutex::new(p1_data), Mutex::new(p2_data)];
    let res = maps.par_iter().map(|data| {
        let mut v = data.lock().unwrap();
        let width = (*v).iter().position(|&x| x == '\n' as u8).unwrap();
        let mut m = StrMap {
            data: v.as_mut_slice(),
            h: height as i32,
            w: width as i32,
        };
        solve_map(&mut m, moves.as_ref())
    }).collect::<Vec<_>>();
    (res[0], res[1])
}
