
use hashbrown::HashMap;
use itertools::Itertools;
use rayon::prelude::*;
use std::cell::RefCell;
use std::collections::HashSet;

const FACINGS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];


#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Day06Guard {
    x: i32,
    y: i32,
    facing_index: usize,
}

#[derive(Debug, Clone)]
struct HitMap {
    // facing x  map y coord to sorted x
    pub x_blocks: HashMap<i32, Vec<i32>>,

    // facing y  map x coord to sorted y
    pub y_blocks: HashMap<i32, Vec<i32>>,
}

impl HitMap {
    fn add_entry(&mut self, x: i32, y: i32) {
        let x_row = self.x_blocks.entry(y).or_insert(Vec::new());
        let y_row = self.y_blocks.entry(x).or_insert(Vec::new());
        x_row.push(x);
        y_row.push(y);
        x_row.sort();
        y_row.sort();
    }
    fn remove_entry(&mut self, x: i32, y: i32) {
        let x_row = self.x_blocks.entry(y).or_insert(Vec::new());
        let y_row = self.y_blocks.entry(x).or_insert(Vec::new());
        let (x_ind, _) = x_row.iter().find_position(|&&xx| xx == x).unwrap();
        let (y_ind, _) = y_row.iter().find_position(|&&yy| yy == y).unwrap();
        x_row.remove(x_ind);
        y_row.remove(y_ind);
    }
    fn from_hashset(occupied: &HashSet<(i32, i32)>) -> HitMap {
        let mut x_blocks = HashMap::new();
        let mut y_blocks = HashMap::new();
        occupied.iter().for_each(|&(x, y)| {
            let x_row = x_blocks.entry(y).or_insert(Vec::new());
            let y_row = y_blocks.entry(x).or_insert(Vec::new());
            x_row.push(x);
            y_row.push(y);
        });

        x_blocks.iter_mut().for_each(|(_, v)| {
            v.sort_unstable();
        });

        y_blocks.iter_mut().for_each(|(_, v)| {
            v.sort_unstable();
        });

        HitMap { x_blocks, y_blocks }
    }
    fn jump(&self, guard: &mut Day06Guard) -> bool {
        match guard.facing_index {
            0 => {
                // moving up
                self.x_blocks.get(&guard.y)
                    .and_then(|x_co| {
                        let x_part = x_co.partition_point(|x| x <= &guard.x);
                        x_co.get(x_part - 1).map(|nx| {
                            guard.x = nx + 1;
                            guard.facing_index = 1;
                        })
                    })
                    .is_some()
            }
            2 => {
                // moving down
                self.x_blocks.get(&guard.y)
                    .and_then(|x_co| {
                        let x_part = x_co.partition_point(|x| x <= &guard.x);
                        x_co.get(x_part).map(|nx| {
                            guard.x = nx - 1;
                            guard.facing_index = 3;
                        })
                    })
                    .is_some()
            }
            3 => {
                // moving left
                self.y_blocks.get(&guard.x)
                    .and_then(|y_co| {
                        let y_part = y_co.partition_point(|y| y <= &guard.y);
                        y_co.get(y_part - 1).map(|ny| {
                            guard.y = ny + 1;
                            guard.facing_index = 0;
                        })
                    })
                    .is_some()
            }
            1 => {
                // moving down
                self.y_blocks.get(&guard.x)
                    .and_then(|y_co| {
                        let y_part = y_co.partition_point(|y| y <= &guard.y);
                        y_co.get(y_part).map(|ny| {
                            guard.y = ny - 1;
                            guard.facing_index = 2;
                        })
                    })
                    .is_some()
            }
            _ => unreachable!(),
        }
    }
}

fn step(
    guard: &mut Day06Guard,
    all_occupied: &HashSet<(i32, i32)>,
    height: i32,
    width: i32,
) -> bool {
    let (dx, dy) = FACINGS[guard.facing_index];
    let (nx, ny) = (guard.x + dx, guard.y + dy);
    if all_occupied.contains(&(nx, ny)) {
        guard.facing_index = (guard.facing_index + 1) % 4;
        true
    } else {
        if (0 <= nx) && (nx < height) && (0 <= ny) && (ny < width) {
            guard.x = nx;
            guard.y = ny;
            true
        } else {
            false
        }
    }
}


fn simulate_jump(
    guard: &mut Day06Guard,
    hit_map: &HitMap,
) -> bool {
    let mut visited = HashSet::with_capacity(200);
    loop {
        if !hit_map.jump(guard) {
            return false;
        }
        if visited.contains(guard) {
            return true;
        }
        visited.insert(guard.clone());
    }
}

fn simulate(
    guard: &mut Day06Guard,
    all_occupied: &HashSet<(i32, i32)>,
    height: i32,
    width: i32,
) -> (HashSet<Day06Guard>, HashMap<(i32, i32), Day06Guard>) {
    let mut visited = HashSet::new();
    let mut first_entered = HashMap::new();
    while !visited.contains(guard) {
        let prev_state = guard.clone();
        visited.insert(prev_state);
        if !step(guard, all_occupied, height, width) {
            break;
        }
        let curr_pos = (guard.x, guard.y);
        if !first_entered.contains_key(&curr_pos) {
            first_entered.insert(curr_pos, prev_state);
        }
    }
    (visited, first_entered)
}

fn parse(data: &str) -> (Day06Guard, HashSet<(i32, i32)>) {
    let mut start_pos = (0, 0);
    let mut occupied = HashSet::new();
    data.lines().enumerate().for_each(|(x, line)| {
        for (y, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    occupied.insert((x as i32, y as i32));
                }
                '^' => start_pos = (x as i32, y as i32),
                _ => {}
            }
        }
    });
    (
        Day06Guard {
            x: start_pos.0,
            y: start_pos.1,
            facing_index: 0,
        },
        occupied,
    )
}

pub fn solve(data: &str) -> (i64, i64) {
    let height = data.lines().count() as i32;
    let width = data.lines().next().unwrap().len() as i32;
    let (mut state, occupied) = parse(&data);
    let hit_map = HitMap::from_hashset(&occupied);
    let start_pos = (state.x, state.y);
    let (p1_visited, first_entry) = simulate(&mut state, &occupied, height, width);
    let p1_positions = p1_visited
        .iter()
        .map(|g| (g.x, g.y))
        .collect::<HashSet<_>>();

    thread_local! {
        static T_HITMAP: RefCell<Option<HitMap >> = RefCell::new(None);
    }
    let p2 = p1_positions
        .par_iter()
        .filter(|&&(bx, by)| {
            if (bx == start_pos.0) && (by == start_pos.1) {
                return false;
            }
            let mut prev_state = first_entry.get(&(bx, by)).unwrap().clone();
            let mut loops = false;
            T_HITMAP.with(|cell| {
                let mut hm_ref = cell.borrow_mut();
                if hm_ref.is_none() {
                    *hm_ref = Some(hit_map.clone());
                }
                let hm = hm_ref.as_mut().unwrap();
                hm.add_entry(bx, by);
                loops = simulate_jump(&mut prev_state, &hm);
                hm.remove_entry(bx, by);
            });
            loops
        })
        .count();
    (p1_positions.len() as i64, p2 as i64)
}
