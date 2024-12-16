mod aoc;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day07_mitm;
mod y16_day24;
mod y16_day12;
mod y16_assembunny;
mod y16_day23;
mod y16_day25;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day11_exploration;
mod day12_parallel;
mod day13;
mod day14;
mod util;
mod day15;
mod str_map;
mod v;
mod day15_bits;
mod day16;

use cached::proc_macro::io_cached;
use itertools::Itertools;
use reqwest::cookie::Jar;
use reqwest::{Client, Url};
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::{fs, io, str};
use thiserror::Error;

use crate::aoc::{AocClient, AocDailyPart, AocResponse};
use crate::ExampleError::DiskError;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::runtime::Runtime;

fn prepare_client(aoc_session: &str) -> Client {
    let cookie_store = Arc::new(Jar::default());
    let cookie = format!("session={}", aoc_session);
    let aoc_url = "https://adventofcode.com".parse::<Url>().unwrap();
    cookie_store.add_cookie_str(&cookie, &aoc_url);
    Client::builder()
        .cookie_provider(cookie_store)
        .build()
        .unwrap()
}

#[derive(Error, Debug, PartialEq, Clone)]
enum ExampleError {
    #[error("error with disk cache `{0}`")]
    DiskError(String),
}

// "sync_to_disk_on_cache_change"=true is necessary!! why?
#[io_cached(
    disk = true,
    key = "String",
    convert = r#"{ format!("{}_{}_{}",user,client.year,day) }"#,
    map_error = r##"|e| ExampleError::DiskError(format!("{:?}", e))"##,
    sync_to_disk_on_cache_change = true
)]
async fn get_input_cached(
    client: &AocClient,
    user: &str,
    day: i64,
) -> Result<String, ExampleError> {
    println!("fetching {} day {} input", user, day);
    let resp = client.get_day(day).await;
    if resp.starts_with("Please don't") {
        Err(DiskError("too early".into()))
    } else {
        Ok(resp)
    }
}

// store responses as preparation for response parsing
async fn submit_answer_stored<T: Display>(
    client: &AocClient,
    user: &str,
    day: i64,
    part: AocDailyPart,
    answer: T,
) -> AocResponse {
    let time = SystemTime::now();
    let outfile_name = format!(
        "cache/result_{}_{}_{}_{}_{}.txt",
        user,
        day,
        part,
        answer,
        time.duration_since(UNIX_EPOCH).unwrap().as_millis()
    );
    //let outfile_name = format!("result_{}_{}_{}_.txt", day, part, answer);
    //if let Ok(f) = fs::read(&outfile_name).await {
    //    let deserialized: AocResponse = serde_json::from_slice(f.as_slice()).unwrap();
    //    deserialized
    //} else {
    println!("submitting {} for day {} part {}", answer, day, part);
    let response = client.submit_day(day, part, answer).await;
    let serialized = serde_json::to_string(&response).unwrap();
    let mut file = File::create(&outfile_name).await.unwrap();
    file.write_all(serialized.as_ref()).await.unwrap();
    response
    //}
}

struct SolveResult {
    day: i64,
    p1: i64,
    p2: i64,
    elapsed_micros: u128,
}

impl Display for SolveResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "day {}\n {} {}\n took\t{} μs\n", self.day, self.p1, self.p2, self.elapsed_micros)
    }
}

async fn run_solve<F>(client: &AocClient, user: &str, day: i64, solve: F) -> SolveResult
where
    F: Fn(&str) -> (i64, i64),
{
    let data = get_input_cached(client, user, day).await.unwrap();
    let started = Instant::now();
    let (p1, p2) = solve(&data);
    let elapsed_micros = started.elapsed().as_micros();
    SolveResult { day, p1, p2, elapsed_micros }
}

fn main_2024() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        dotenv::from_filename(".env").ok();
        let aoc_session = std::env::var("AOC_SESSION").expect("AOC_SESSION not set");
        let aoc_user = std::env::var("AOC_USER").expect("AOC_USER not set");
        let client = AocClient::new(prepare_client(&aoc_session));

        let solves: Vec<(i64, fn(&str) -> (i64, i64))> = vec![
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
            (16,day16::solve)
        ];

        let mut results = vec![];

        for (day, solver) in solves {
            let res = run_solve(&client, &aoc_user, day, solver).await;
            results.push(res);
        }

        let mut total_micros = 0;
        for res in results {
            println!("{}", res);
            total_micros += res.elapsed_micros;
        }

        println!("Total solve time: {} μs", total_micros);
        // let res = run_solve(&client, &aoc_user, 16, day16::solve).await;
        // println!("{}", res);
    })
}


fn main_2016() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        dotenv::from_filename(".env").ok();
        let aoc_session = std::env::var("AOC_SESSION").expect("AOC_SESSION not set");
        let aoc_user = std::env::var("AOC_USER").expect("AOC_USER not set");
        let client = AocClient::for_year(prepare_client(&aoc_session), 2016);


        run_solve(&client, &aoc_user, 12, y16_day12::solve).await;
        run_solve(&client, &aoc_user, 24, y16_day24::solve).await;
        run_solve(&client, &aoc_user, 25, y16_day25::solve).await;
    });
}

fn main() {
    main_2024();
    //main_2016();
    //let stdin = io::read_to_string(io::stdin()).unwrap();
    // let started = Instant::now();
    //let res = day16::solve(stdin.as_str());
    // let elapsed_micros = started.elapsed().as_micros();
    //println!("day16 result:{:?}", res);
}
