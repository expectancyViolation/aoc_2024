use hashbrown::HashMap;
use itertools::Itertools;
use rayon::prelude::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub(crate) fn n_digs(x: i64) -> i64 {
    (x.ilog10() + 1) as i64
}

const N_ELS: usize = 4000;

#[derive(Debug)]
struct IndexLookup {
    pub indices: Vec<i64>,
    lookup: HashMap<i64, usize>,
    nbs: Vec<(usize, usize)>,
}

impl Default for IndexLookup {
    fn default() -> Self {
        IndexLookup {
            indices: vec![0], // seed with dummy val
            lookup: HashMap::new(),
            nbs: vec![(0, 0); N_ELS],
        }
    }
}

impl IndexLookup {
    fn get_index(&mut self, x: i64) -> usize {
        if self.lookup.contains_key(&x) {
            self.lookup[&x]
        } else {
            let i = self.indices.len();
            self.indices.push(x);
            self.lookup.insert(x, i);
            i
        }
    }
    fn get_val(&self, ind: usize) -> i64 {
        self.indices[ind]
    }

    fn get_neighbors(&mut self, i: usize) -> (usize, usize) {
        if self.nbs[i].0 == 0 {
            let x = self.get_val(i);
            if x == 0 {
                self.nbs[i] = (self.get_index(1), 0);
            } else {
                let digs = n_digs(x);
                match digs % 2 {
                    0 => {
                        let ten_pow = 10_i64.pow((digs / 2) as u32);
                        let x1 = x / ten_pow;
                        let x2 = x % ten_pow;

                        self.nbs[i] = (self.get_index(x1), self.get_index(x2));
                    }
                    _ => {
                        self.nbs[i] = (self.get_index(x * 2024), 0);
                    }
                }
            }
        }
        self.nbs[i]
    }
}

fn count(x: i64, ns: Vec<i64>) -> Vec<i64> {
    let mut curr: Vec<i64> = vec![0; N_ELS];
    let mut buff: Vec<i64> = vec![0; N_ELS];

    let mut indices = IndexLookup::default();
    let mut res = vec![0; ns.len()];
    curr[indices.get_index(x)] = 1;
    for i in 0..*ns.iter().max().unwrap() {
        let (from_, to_) = if i % 2 == 0 {
            (&mut curr, &mut buff)
        } else {
            (&mut buff, &mut curr)
        };
        to_.fill(0);
        from_
            .iter()
            .enumerate()
            .take(indices.indices.len())
            .skip(1)
            .for_each(|(y_ind, cnt)| {
                let nb = indices.get_neighbors(y_ind);
                to_[nb.0] += cnt;
                to_[nb.1] += cnt;
            });
        ns.iter().positions(|&x| x == i + 1).for_each(|j| {
            res[j] =
                to_.iter().skip(1).sum()
        });
    }
    res
}

pub(crate) fn solve(data: &str) -> (String, String) {
    let nums = data
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();
    let mut res = [0, 0];
    let partial_results: Vec<Vec<i64>> = nums.par_iter().map(|&x| count(x, vec![25, 75])).collect();
    let _ = partial_results.into_iter().for_each(|x| {
        res[0] += x[0];
        res[1] += x[1];
    });

    (res[0].to_string(), res[1].to_string())
}
