use crate::day10::DIRECTIONS;
use hashbrown::HashMap;
use itertools::{iproduct, Itertools};
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
    fn jump(&self, guard: &Day06Guard) -> Option<Day06Guard> {
        //println!("jumping {:?}", guard);
        match guard.facing_index {
            0 | 2 => {
                if let Some(x_coords) = self.x_blocks.get(&guard.y) {
                    //println!("{:?}", x_coords);
                    let idx = x_coords.partition_point(|x| x <= &guard.x);
                    //println!("partition x {}", idx);
                    match guard.facing_index {
                        0 => {
                            // moving up
                            if idx == 0 {
                                None
                            } else {
                                Some(Day06Guard { x: x_coords[idx - 1] + 1, y: guard.y, facing_index: (guard.facing_index + 1) % 4 })
                            }
                        }
                        2 => {
                            // moving down
                            if idx == x_coords.len() {
                                None
                            } else {
                                Some(Day06Guard { x: x_coords[idx] - 1, y: guard.y, facing_index: (guard.facing_index + 1) % 4 })
                            }
                        }
                        _ => unreachable!()
                    }
                } else {
                    None
                }
            }
            1 | 3 => {
                if let Some(y_coords) = self.y_blocks.get(&guard.x) {
                    //println!("{:?}", y_coords);
                    let idy = y_coords.partition_point(|y| y <= &guard.y);
                    //println!("partition y {}", idy);
                    match guard.facing_index {
                        3 => {
                            // moving left
                            if idy == 0 {
                                None
                            } else {
                                Some(Day06Guard { x: guard.x, y: y_coords[idy - 1] + 1, facing_index: (guard.facing_index + 1) % 4 })
                            }
                        }
                        1 => {
                            // moving down
                            if idy == y_coords.len() {
                                None
                            } else {
                                Some(Day06Guard { x: guard.x, y: y_coords[idy] - 1, facing_index: (guard.facing_index + 1) % 4 })
                            }
                        }
                        _ => unreachable!()
                    }
                } else {
                    None
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Day06Guard {
    fn step(
        &self,
        occupied: &OccMap,
        L: i32,
        l: i32,
        blocked: Option<(i32, i32)>,
    ) -> Option<Day06Guard> {
        if blocked.is_none() || blocked.is_some_and(|t| {
            self.x == t.0 || self.y == t.1
        }) {
            // go slow
            let (dx, dy) = FACINGS[self.facing_index];
            let (nx, ny) = (self.x + dx, self.y + dy);
            if blocked.is_some_and(|t| t == (nx, ny))
                || occupied.all_occupied.contains(&(nx, ny))
            {
                let new_facing = (self.facing_index + 1) % 4;
                Some(Day06Guard { x: self.x, y: self.y, facing_index: new_facing })
            } else {
                if (0 <= nx) && (nx < L) && (0 <= ny) && (ny < l) {
                    Some(Day06Guard { x: nx, y: ny, facing_index: self.facing_index })
                } else {
                    None
                }
            }
        } else {
            // go fast
            let res = occupied.jump(&self);
            //println!("jumped {:?}", res);
            res
        }
    }

    // fn is_in_bounds(x: i32, y: i32, occupied: &Vec<Vec<bool>>) -> bool {
    //     (0 <= x) && (x < occupied.len() as i32) && (0 <= y) && (y < occupied[0].len() as i32)
    // }

    fn simulate_jump(
        &self,
        occupied: &OccMap,
        L: i32,
        l: i32,
        blocked: (i32, i32),
    ) -> bool {
        // TODO this can be reduced
        let max_iter = 4 * (L+l)+4*(occupied.all_occupied.len() as i32);
        let mut guard = Some(self.clone());
        for _ in (0..max_iter) {
            match guard {
                None => break,
                Some(g) => {
                    guard =
                        g.step(&occupied, L, l, Some(blocked));
                }
            }
        }
        guard.is_some()
    }

    fn simulate(
        &self,
        occupied: &OccMap,
        L: i32,
        l: i32,
    ) -> (HashSet<Day06Guard>, HashMap<(i32, i32), Day06Guard>) {
        let mut guard = Some(self.clone());
        let mut visited = HashSet::new();
        let mut first_entered = HashMap::new();
        while guard.is_some_and(|g| !visited.contains(&g))
        {
            let curr_state = guard.unwrap().clone();
            visited.insert(curr_state);
            guard = curr_state.step(&occupied, L, l, None);
            if let Some(g) = guard {
                let curr_pos = (g.x, g.y);
                if !first_entered.contains_key(&curr_pos) {
                    first_entered.insert(curr_pos, curr_state);
                }
            }
        }
        (visited, first_entered)
    }
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
    //     let data = "....#.....
    // .........#
    // ..........
    // ..#.......
    // .......#..
    // ..........
    // .#..^.....
    // ........#.
    // #.........
    // ......#...";
    let L = data.lines().count() as i32;
    let l = data.lines().next().unwrap().len() as i32;
    let (state, occupied) = parse(&data);
    let occupied = OccMap::from_hashset(occupied);
    //println!("x blocks {:?}", occupied.x_blocks);
    //println!("y blocks{:?}", occupied.y_blocks);
    let start_pos = (state.x, state.y);
    let (p1_visited, first_entry) = state.clone().simulate(&occupied, L, l);
    let p1_positions = p1_visited
        .iter()
        .map(|g| (g.x, g.y))
        .collect::<HashSet<_>>();
    //println!("{:?}", p1_positions);
    let p2 = p1_positions
        .par_iter()
        .filter(|&&(bx, by)| {
            if (bx == start_pos.0) && (by == start_pos.1) {
                return false;
            }
            let mut prev_state = first_entry.get(&(bx, by)).unwrap().clone();
            prev_state.simulate_jump(&occupied, L, l, (bx, by))
        })
        .count();
    (p1_positions.len() as i64, p2 as i64)
}
