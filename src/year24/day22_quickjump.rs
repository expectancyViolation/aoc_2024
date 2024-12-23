//use std::time::Instant;
use galois_field_2pm::{gf2, GaloisField};
use itertools::Itertools;
use lazy_static::lazy_static;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use crate::year24::gf2_mod::GF2PolyDiv;

const P: u128 = 0b1000000101010100000011101;

const P_N: u128 = 24;
pub type GF = gf2::GFu64<P>;

/* coefficient seems to be wrong, removing it should only very slightly increase runtime. TODO: resolve poly matrix*/
const IRREDUCIBLES: [u128; 40] = [
    2, 3, 7, 11, 13, 19, 25, 31, 37, 41, 47, 55, 59, 61, 67, 73, 87, 91, 97, 103, 109, 115, 117,
    131, /*137,*/ 143, 145, 157, 167, 171, 185, 191, 193, 203, 211, 213, 229, 239, 241, 247, 253,
];

// const IRREDUCIBLES: [u128; 23] = [
//     2, 3, 7, 11, 13, 19, 25, 31, 37, 41, 47, 55, 59, 61, 67, 73, 87, 91, 97, 103, 109, 115, 117 //,131,137,143
// ];

const LOGS: [u128; 40] = [
    1, 1092464, 10342409, 593193, 7771580, 13057661, 5024933, 3687479, 6499903,
    6535514, 261301, 7863997, 3031511, 3841894, 16052237, 6569839, 3032996,
    4462432,
    12460360, 11445647, 8956400, 1407117, 8374710, 7558465, /*12938356,*/ 12422597,
    8377239, 12938356, 3384608, 10553014, 14000550, 13442123, 6188829, 14916954,
    7758384, 3601046, 7820348, 16446499, 8349006, 11425387, 13171618
];

const B_INV: [u128; 24] = [
    572075,
    10269278,
    6765888,
    3808448,
    16695544,
    2036426,
    8703246,
    6212462,
    2352836,
    13056686,
    9069450,
    12759402,
    11765572,
    7211408,
    3337392,
    16680012,
    15759044,
    4100298,
    1145504,
    7544602,
    15527094,
    11157142,
    15337732,
    2077452];

const M2000: [u128; 24] = [
    11593647,
    115592,
    9650831,
    11935952,
    464073,
    1874787,
    8943425,
    6826032,
    5526809,
    5714711,
    9844545,
    10663580,
    9219426,
    7597282,
    491792,
    9818029,
    7423510,
    8679750,
    6442813,
    10841274,
    900302,
    10889424,
    14239094,
    2959297,
];

lazy_static! {
    pub(crate) static ref IRREDUCIBLES_GF: Vec<GF> =
        IRREDUCIBLES.map(|i| GF::new(i as u64)).into_iter().collect_vec();
}

fn factorize_smooth(p: u128) -> Option<Vec<usize>> {
    // why does this even work?
    //let (_, mut p) = u128::gf2_poly_div_poly(p, P);
    let mut p = p;
    //println!("{}", p);
    let mut i = 0;
    let mut res = Vec::new();
    while (p != 1) && i < IRREDUCIBLES.len() {
        let (quot, rem) = u128::gf2_poly_div_poly(p, IRREDUCIBLES[i]);
        if rem == 0 {
            res.push(i);
            p = quot;
            // println!("{} {}", i, p);
        } else {
            i += 1;
        }
    }
    if p != 1 {
        None
    } else { Some(res) }
}

fn quick_pow(n: u128) -> GF {
    let mut res = GF::ONE;
    let mut curr = GF::new(2);
    let mut n = n;
    while n > 0 {
        if (n % 2) == 1 {
            res *= curr;
        }
        curr *= curr;
        n /= 2;
    }
    res
}

pub(crate) fn discrete_log(p: GF) -> u128 {
    //let mut rng = StdRng::seed_from_u64(228);

    let mut rng = StdRng::from_os_rng();
    loop {
        let i: u128 = rng.random_range(100..(1 << 17));
        let power = quick_pow(i);
        let to_test = (power * p).value;
        let factors = factorize_smooth(to_test as u128);
        if factors.is_some() {
            let factors = factors.unwrap();
            //println!("{:?}",factors);
            let total_power = factors.iter().map(|i| LOGS[*i]).sum::<u128>();
            assert!(total_power > i);
            return (total_power - i + (1 << P_N) - 1) % ((1 << P_N) - 1);
        }
    }
}

const M: u64 = 1 << 24;
fn evolve(num: u64) -> u64 {
    let num = (num ^ (num << 6)) % M;
    let num = (num ^ (num >> 5)) % M;
    (num ^ (num << 11)) % M
}

fn get_cycle(n: u64) -> u64 {
    let mut curr = 1;
    for i in 0.. {
        if curr == n {
            return i;
        }
        curr = evolve(curr);
    };
    unreachable!()
}

pub(crate) fn predict_cycle(n: u128) -> u128 {
    let mut k = 0;
    B_INV.iter().rev().for_each(|&i| {
        let bit = (i & n).count_ones() % 2;
        k = 2 * k + bit;
    });
    // println!("k is {}", k);
    discrete_log(GF::new(k as u64))
}


pub(crate) fn solve(data: &str) -> (String, String) {
    // let data = "10041208";
    let nums = data.lines().map(|l| l.parse::<u128>().unwrap()).collect_vec();
    println!("nums {:?}", nums.len());
    let mut p1: u128 = 0;
    for num in nums {
        let mut k = 0;
        M2000.iter().rev().for_each(|&i| {
            let bit = (i & num).count_ones() % 2;
            k = 2 * k + bit;
        });
        p1 += (k as u128);
        // println!("{} {}", num, k);
        // let started = Instant::now();
        // let predicted = predict_cycle(num);
        // println!("predicted {} ({} micros)", predicted, started.elapsed().as_micros());
        // let started = Instant::now();
        // let real_cycle = get_cycle(num as u64);
        // println!("real_cycle {} ({} micros)", real_cycle, started.elapsed().as_micros());
        // if real_cycle as u128 != predicted {
        //     println!("--------------------------------------");
        // }
        // assert_eq!(real_cycle as u128, predicted);
    }

    (p1.to_string(), "XXX".to_string())
}

#[cfg(test)]
mod tests {
    use galois_field_2pm::GaloisField;
    use crate::year24::day22_quickjump::{discrete_log, factorize_smooth, GF};

    #[test]
    fn can_factorize() {
        let test_val: u128 = 22003881;
        let res = factorize_smooth(test_val);
        println!("{:?}", res.unwrap());
    }

    #[test]
    fn can_log() {
        let test_val: u64 = 16171214;
        let res = discrete_log(GF::new(test_val));
        println!("{:?}", res);
    }
}
