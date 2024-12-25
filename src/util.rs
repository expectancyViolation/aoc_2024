use num::PrimInt;

// from https://raw.githubusercontent.com/TheAlgorithms/Rust/refs/heads/master/src/math/extended_euclidean_algorithm.rs
fn update_step(a: &mut i32, old_a: &mut i32, quotient: i32) {
    let temp = *a;
    *a = *old_a - quotient * temp;
    *old_a = temp;
}

pub fn extended_euclidean_algorithm(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut old_r, mut rem) = (a, b);
    let (mut old_s, mut coeff_s) = (1, 0);
    let (mut old_t, mut coeff_t) = (0, 1);

    while rem != 0 {
        let quotient = old_r / rem;

        update_step(&mut rem, &mut old_r, quotient);
        update_step(&mut coeff_s, &mut old_s, quotient);
        update_step(&mut coeff_t, &mut old_t, quotient);
    }

    (old_r, old_s, old_t)
}

// from https://raw.githubusercontent.com/TheAlgorithms/Rust/refs/heads/master/src/math/chinese_remainder_theorem.rs

fn mod_inv(x: i32, n: i32) -> Option<i32> {
    let (g, x, _) = extended_euclidean_algorithm(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

pub fn chinese_remainder_theorem(residues: &[i32], modulli: &[i32]) -> Option<i32> {
    let prod = modulli.iter().product::<i32>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulli) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}


pub struct FenwickTree {
    pub data: Vec<i64>,
}

impl FenwickTree {
    pub fn new(size: usize) -> FenwickTree {
        FenwickTree { data: vec![0; size] }
    }

    pub fn query(&self, index: usize) -> i64 {
        let mut index = index;
        let mut res = 0;
        while index > 0 {
            res += self.data[index];
            index -= 1 << index.trailing_zeros();
        }
        res
    }

    pub fn update(&mut self, index: usize, value: i64) {
        let mut index = index;
        while index < self.data.len() {
            self.data[index] += value;
            index += 1 << index.trailing_zeros();
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::util::FenwickTree;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    #[test]
    fn fenwick_works() {
        let mut rng = StdRng::seed_from_u64(222);
        let ts: usize = 99;
        let mut fw = FenwickTree::new(ts);
        let mut test_vec = vec![0; ts];
        let mut test_data: Vec<(usize, i64)> = Vec::new();
        for _ in 0..20 {
            test_data.push((rng.random_range(0..ts), rng.random_range(0..10000)))
        }
        for &(i, x) in test_data.iter() {
            fw.update(i, x);
            test_vec[i] += x;
            for k in 0..ts-1 {
                let fw_sum = fw.query(k);
                let slow_sum = test_vec[..k+1].iter().sum::<i64>();
                //println!("{:?} {:?} ",fw_sum, slow_sum);
                //println!("{:?}",test_vec);
                //println!("{:?}",fw.data);
                assert_eq!(fw_sum, slow_sum);
            }
        }
    }
}



