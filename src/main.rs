mod day01;
mod aoc;
mod day02;
mod day03;

use cached::proc_macro::io_cached;
use reqwest::cookie::Jar;
use reqwest::Url;
use std::fmt::{Debug, Display};
use std::str;
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;

use crate::aoc::{AocClient, AocDailyPart, AocResponse};
use crate::ExampleError::DiskError;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use regex::Regex;

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
    sync_to_disk_on_cache_change = true
)]
async fn get_input_cached(client: &AocClient, user: &str, day: i64) -> Result<String, ExampleError> {
    println!("fetching {} day {} input", user, day);
    let resp = client.get_day(day).await;
    if resp.starts_with("Please don't") {
        Err(DiskError("too early".into()))
    } else {
        Ok(resp)
    }
}

// store responses as preparation for response parsing
async fn submit_answer_stored<T: Display>(client: &AocClient, user: &str, day: i64, part: aoc::AocDailyPart, answer: T) -> AocResponse {
    let time = SystemTime::now();
    let outfile_name = format!("cache/result_{}_{}_{}_{}_{}.txt", user, day, part, answer, time.duration_since(UNIX_EPOCH).unwrap().as_millis());
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


#[tokio::main]
async fn main() {
    dotenv::from_filename(".env").ok();
    let aoc_session = std::env::var("AOC_SESSION").expect("AOC_SESSION not set");
    let aoc_user = std::env::var("AOC_USER").expect("AOC_USER not set");
    let client = prepare_aoc_client(&aoc_session);


    // day01
    let day01_data = get_input_cached(&client, &aoc_user, 1).await.unwrap();
    let started = Instant::now();
    day01::solve(&day01_data);
    println!("day01 took {} μs", (started.elapsed()).as_micros());


    // day02. kinda slow TODO: one-pass solution?
    //let day02_data = get_input_cached(&client, &aoc_user, 2).await.unwrap();
    //let started = Instant::now();
    //day02::solve(&day02_data);
    //println!("day02 took {} μs", (started.elapsed()).as_micros());

    let day03_data = get_input_cached(&client, &aoc_user, 3).await.unwrap();
    //println!("{}", day03_data);

    let started = Instant::now();
    let (p1,p2) = day03::solve(&day03_data);
    println!("day03 took {} μs", (started.elapsed()).as_micros());
    //let response=submit_answer_stored(&client, &aoc_user, 3, AocDailyPart::Part2, p2).await;
    //let response=submit_answer_stored(&client, &aoc_user, 3, AocDailyPart::Part2, p2).await;
    //println!("{:?}",response);
}
