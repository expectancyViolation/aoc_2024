mod day01;
mod aoc;

use cached::proc_macro::io_cached;
use reqwest::cookie::Jar;
use reqwest::Url;
use std::fmt::{format, Debug, Display};
use std::str;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

use crate::aoc::{AocClient, AocResponse};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::ExampleError::DiskError;

fn prepare_aoc_client(aoc_session: &str) -> AocClient {
    let cookie_store = Arc::new(Jar::default());
    let cookie = format!("session={}", aoc_session);
    let aoc_url = "https://adventofcode.com".parse::<Url>().unwrap();
    cookie_store.add_cookie_str(&cookie, &aoc_url);
    let c = reqwest::Client::builder()
        .cookie_provider(cookie_store).build().unwrap();
    AocClient::new(c)
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
    convert = r#"{ format!("{}_{}",user,day) }"#,
    map_error = r##"|e| ExampleError::DiskError(format!("{:?}", e))"##,
    sync_to_disk_on_cache_change =true
)]
async fn get_input_cached(client: &AocClient,user:&str, day: i64) -> Result<String, ExampleError> {
    let res = format(format_args!("{0}_{1}", user, day));
    println!("{:#?}", res);

    println!("fetching {} day {} input",user,day);
    let resp = client.get_day(day).await;
    if resp.starts_with("Please don't") {
        Err(DiskError("too early".into()))
    } else {
        Ok(resp)
    }
}

// store responses as preparation for response parsing
async fn submit_answer_stored<T: Display>(client: &AocClient,user:&str, day: i64, part: aoc::AocDailyPart, answer: T) -> AocResponse {
    let time = SystemTime::now();
    let outfile_name = format!("cache/result_{}_{}_{}_{}_{}.txt",user, day, part, answer, time.duration_since(UNIX_EPOCH).unwrap().as_millis());
    //let outfile_name = format!("result_{}_{}_{}_.txt", day, part, answer);
    //if let Ok(f) = fs::read(&outfile_name).await {
    //    let deserialized: AocResponse = serde_json::from_slice(f.as_slice()).unwrap();
    //    deserialized
    //} else {
        println!("submitting {} for day {} part {}", answer, day, part);
        let response = client.submit_day(&day, &part, &answer).await;
        let serialized = serde_json::to_string(&response).unwrap();
        let mut file = File::create(&outfile_name).await.unwrap();
        file.write_all(serialized.as_ref()).await.unwrap();
        response
    //}
}

async fn solve_day01(client: &AocClient,user:&str) {
    let day01_data = get_input_cached(&client,user, 1).await.unwrap();
    let (left_list, right_list) = day01::parse(&day01_data);
    let part1_result = day01::part1(&left_list, &right_list);
    println!("{}", part1_result);
    let r1 = submit_answer_stored(&client,user, 1, aoc::AocDailyPart::Part1, part1_result).await;
    println!("{:?}", r1);
    let part2_result = day01::part2(&left_list, &right_list);
    println!("{}", part2_result);

    let r2 = submit_answer_stored(&client,user, 1, aoc::AocDailyPart::Part2, part2_result).await;
    println!("{:?}", r2);
}

#[tokio::main]
async fn main() {
    dotenv::from_filename(".env_gunther").ok();
    let aoc_session = std::env::var("AOC_SESSION").expect("AOC_SESSION not set");
    let aoc_user = std::env::var("AOC_USER").expect("AOC_USER not set");
    let client = prepare_aoc_client(&aoc_session);

    // day01
    solve_day01(&client,&aoc_user).await;

    //println!("{}", resp);
}
