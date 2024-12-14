use hashbrown::HashMap;
use rayon::prelude::*;
use std::collections::HashSet;

const FACINGS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];


#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Day06Guard {
    x: i32,
    y: i32,
    facing_index: usize,
}


#[derive(Debug)]
struct OccMap {
    // facing x  map y coord to sorted x
    pub x_blocks: HashMap<i32, Vec<i32>>,

    // facing y  map x coord to sorted y
    pub y_blocks: HashMap<i32, Vec<i32>>,

    pub all_occupied: HashSet<(i32, i32)>,
}

impl OccMap {
    fn from_hashset(occupied: HashSet<(i32, i32)>) -> OccMap {
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

        OccMap {
            x_blocks,
            y_blocks,
            all_occupied: occupied,
        }
    }
    fn jump(&self, guard: &mut Day06Guard) -> bool {
        let x_coords = self.x_blocks.get(&guard.y);
        let x_part = x_coords.map(|x_coords| x_coords.partition_point(|x| x <= &guard.x));
        let y_coords = self.y_blocks.get(&guard.x);
        let y_part = y_coords.map(|y_coords| y_coords.partition_point(|y| y <= &guard.y));
        match guard.facing_index {
            0 => {
                // moving up
                x_coords.and_then(|x_co| x_co.get(x_part.unwrap() - 1).map(|nx| {
                    guard.x = nx + 1;
                    guard.facing_index = 1;
                }
                )).is_some()
            }
            2 => {
                // moving down
                x_coords.and_then(|x_co| x_co.get(x_part.unwrap()).map(|nx| {
                    guard.x = nx - 1;
                    guard.facing_index = 3;
                }
                )).is_some()
            }
            3 => {
                // moving left
                y_coords.and_then(|y_co| y_co.get(y_part.unwrap() - 1).map(|ny| {
                    guard.y = ny + 1;
                    guard.facing_index = 0;
                }
                )).is_some()
            }
            1 => {
                // moving down
                y_coords.and_then(|y_co| y_co.get(y_part.unwrap()).map(|ny| {
                    guard.y = ny - 1;
                    guard.facing_index = 2;
                }
                )).is_some()
            }
            _ => unreachable!()
        }
    }
}

fn step(
    guard: &mut Day06Guard,
    occupied: &OccMap,
    height: i32,
    width: i32,
    blocked: Option<(i32, i32)>,
) -> bool {
    if blocked.is_some_and(|t| {
        guard.x == t.0 || guard.y == t.1
    }) || blocked.is_none() {
        // go slow
        let (dx, dy) = FACINGS[guard.facing_index];
        let (nx, ny) = (guard.x + dx, guard.y + dy);
        if blocked.is_some_and(|t| t == (nx, ny))
            || occupied.all_occupied.contains(&(nx, ny))
        {
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
    } else {
        // go fast
        occupied.jump(guard)
    }
}

// fn is_in_bounds(x: i32, y: i32, occupied: &Vec<Vec<bool>>) -> bool {
//     (0 <= x) && (x < occupied.len() as i32) && (0 <= y) && (y < occupied[0].len() as i32)
// }

fn simulate_jump(
    guard: &mut Day06Guard,
    occupied: &OccMap,
    height: i32,
    width: i32,
    blocked: (i32, i32),
) -> bool {
    // TODO can this be reduced safely?
    let max_iter = 4 * (height + width) + 4 * (occupied.all_occupied.len() as i32) + 1;
    for _ in 0..max_iter {
        if !step(guard, &occupied, height, width, Some(blocked)) {
            return false;
        }
    }
    true
}

fn simulate(
    guard: &mut Day06Guard,
    occupied: &OccMap,
    height: i32,
    width: i32,
) -> (HashSet<Day06Guard>, HashMap<(i32, i32), Day06Guard>) {
    let mut visited = HashSet::new();
    let mut first_entered = HashMap::new();
    while !visited.contains(guard)
    {
        let prev_state = guard.clone();
        visited.insert(prev_state);
        if !step(guard, &occupied, height, width, None) {
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
    let occupied = OccMap::from_hashset(occupied);
    let start_pos = (state.x, state.y);
    let (p1_visited, first_entry) = simulate(&mut state, &occupied, height, width);
    let p1_positions = p1_visited
        .iter()
        .map(|g| (g.x, g.y))
        .collect::<HashSet<_>>();
    let p2 = p1_positions
        .par_iter()
        .filter(|&&(bx, by)| {
            if (bx == start_pos.0) && (by == start_pos.1) {
                return false;
            }
            let mut prev_state = first_entry.get(&(bx, by)).unwrap().clone();
            simulate_jump(&mut prev_state, &occupied, height, width, (bx, by))
        })
        .count();
    (p1_positions.len() as i64, p2 as i64)
}
