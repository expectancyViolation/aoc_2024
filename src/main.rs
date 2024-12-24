#![allow(dead_code)]
mod aoc;
mod str_map;
mod util;
mod v;

mod year24;

mod year16;

use cached::proc_macro::io_cached;
use galois_field_2pm::GaloisField;
use reqwest::cookie::Jar;
use reqwest::{Client, Url};
use std::fmt::{Debug, Display};
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::{io, str};
use thiserror::Error;

use crate::aoc::{AocClient, AocDailyPart, AocResponse, AocSolveResult, AocStatus, Solution};
use crate::year16::{day12,  day25};
use crate::year24::{day23, day23_bron, day23_weird_input, day24, day24_};
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
    let resp = client.get_day_input(day).await;
    if resp.starts_with("Please don't") {
        Err(DiskError("too early".into()))
    } else {
        Ok(resp)
    }
}

#[io_cached(
    disk = true,
    key = "String",
    convert = r#"{ format!("{}_{}_{}",user,client.year,day) }"#,
    map_error = r##"|e| ExampleError::DiskError(format!("{:?}", e))"##,
    sync_to_disk_on_cache_change = true
)]
async fn get_status_cached(
    client: &AocClient,
    user: &str,
    day: i64,
) -> Result<String, ExampleError> {
    println!("fetching {} day {} status", user, day);
    let resp = client.get_day(day).await;
    Ok(resp)
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


async fn run_solve(client: &AocClient, user: &str, day: i64, solve: Solution) -> AocSolveResult
{
    let data = get_input_cached(client, user, day).await.unwrap();
    let started = Instant::now();
    let (p1, p2) = solve(&data);
    let elapsed_micros = started.elapsed().as_micros();
    AocSolveResult {
        day,
        p1,
        p2,
        elapsed_micros,
    }
}

fn benchmark_year(year: i32, solves: &Vec<(i64, Solution)>) {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        dotenv::from_filename(".env").ok();
        let aoc_session = std::env::var("AOC_SESSION").expect("AOC_SESSION not set");
        let aoc_user = std::env::var("AOC_USER").expect("AOC_USER not set");
        let client = AocClient::for_year(prepare_client(&aoc_session), year);


        /*
        let mut results = vec![];

        for &(day, solver) in solves {
            let res = run_solve(&client, &aoc_user, day, solver).await;
            let current_status = get_status_cached(&client, &aoc_session, day).await;
            let current_status = current_status.ok().map(|s| AocStatus::from_raw_response(s.as_str()));
            results.push((res, current_status));
        }

        let mut total_micros = 0;
        for (res, status) in results {
            println!("{}", res);
            if status.is_some() {
                let status = status.unwrap();
                status.validate_result(&res);
            }
            println!("\n---");
            total_micros += res.elapsed_micros;
        }

        println!("Total solve time: {} μs", total_micros);*/

        //let res = run_solve(&client, &aoc_user, 23, day23_weird_input::solve).await;


        //let res = run_solve(&client, &aoc_user, 23, day23::solve).await;

        //println!("{}", res);

        let res = run_solve(&client, &aoc_user, 24, day24_::solve).await;

        println!("{}", res);
    })
}

fn main_2016() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        dotenv::from_filename(".env").ok();
        let aoc_session = std::env::var("AOC_SESSION").expect("AOC_SESSION not set");
        let aoc_user = std::env::var("AOC_USER").expect("AOC_USER not set");
        let client = AocClient::for_year(prepare_client(&aoc_session), 2016);

        run_solve(&client, &aoc_user, 12, day12::solve).await;
        //run_solve(&client, &aoc_user, 24, day24::solve).await;
        run_solve(&client, &aoc_user, 25, day25::solve).await;
    });
}

fn main() {
    let solves: Vec<(i64, Solution)> = year24::SOLVES.to_vec();
    benchmark_year(2024, &solves);

    //main_2016();
    //let stdin = io::read_to_string(io::stdin()).unwrap();
    //let res = day23::solve(stdin.as_str());
    //println!("day23 result:{:?}", res);
    //let res = day23_bron::solve(stdin.as_str());
    //println!("day23 result:{:?}", res);
}
