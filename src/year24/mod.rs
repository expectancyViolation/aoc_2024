use lazy_static::lazy_static;
use crate::aoc::Solution;

pub(crate) mod day01;
pub(crate) mod day02;
pub(crate) mod day03;
pub(crate) mod day04;
pub(crate) mod day05;
pub(crate) mod day06;
pub(crate) mod day07;
pub(crate) mod day07_mitm;
pub(crate) mod day08;
pub(crate) mod day09;
pub(crate) mod day10;
pub(crate) mod day11;
pub(crate) mod day11_exploration;
pub(crate) mod day12;
pub(crate) mod day12_parallel;
pub(crate) mod day13;
pub(crate) mod day14;
pub(crate) mod day15;
pub(crate) mod day15_bits;
pub(crate) mod day16;
pub(crate) mod day16_old;
pub(crate) mod day17;

pub(crate) mod day18;

pub(crate) mod day18_bfs;

pub(crate) mod day18_weird_input;
mod day19;


lazy_static! {
 pub(crate) static ref SOLVES: Vec<(i64,Solution )> = vec![
    (1, day01::solve),
    (2, day02::solve),
    (3, day03::solve),
    (4, day04::solve),
    (5, day05::solve),
    (6, day06::solve),
    (7, day07::solve),
    (8, day08::solve),
    (9, day09::solve),
    (10, day10::solve),
    (11, day11::solve),
    //(12, day12::solve),
    (12, day12_parallel::solve),
    (13, day13::solve),
    (14, day14::solve),
    (15, day15::solve),
    (15, day15_bits::solve),
    (16, day16::solve),
    (17, day17::solve),
    (18, day18::solve),
    (18, day18_bfs::solve),
    (18, day18_weird_input::solve),
];
}