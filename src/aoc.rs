use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub(crate) type Solution = fn(&str) -> (String, String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Copy)]
pub enum AocDailyPart {
    Part1,
    Part2,
}

impl Display for AocDailyPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AocDailyPart::Part1 => write!(f, "1"),
            AocDailyPart::Part2 => write!(f, "2"),
        }
    }
}

pub(crate) struct AocSolveResult {
    pub(crate) day: i64,
    pub(crate) p1: String,
    pub(crate) p2: String,
    pub(crate) elapsed_micros: u128,
}

impl Display for AocSolveResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "day {}\n {} {}\n took\t{} μs",
            self.day, self.p1, self.p2, self.elapsed_micros
        )
    }
}

pub struct AocClient {
    req: Client,
    pub year: i32,
}

impl AocClient {
    pub fn new(client: Client) -> AocClient {
        AocClient {
            req: client,
            year: 2024,
        }
    }

    pub fn for_year(client: Client, year: i32) -> AocClient {
        AocClient { req: client, year }
    }

    pub async fn get_day_input(&self, day: i64) -> String {
        let input_url = format!("https://adventofcode.com/{}/day/{}/input", self.year, day);
        let res = self.req.get(input_url).send().await.unwrap();
        assert!(res.status().is_success());
        res.text().await.unwrap()
    }

    pub async fn get_day(&self, day: i64) -> String {
        let input_url = format!("https://adventofcode.com/{}/day/{}", self.year, day);
        let res = self.req.get(input_url).send().await.unwrap();
        assert!(res.status().is_success());
        res.text().await.unwrap()
    }

    pub async fn submit_day<T: Display>(
        &self,
        day: i64,
        part: AocDailyPart,
        val: T,
    ) -> AocResponse {
        let submit_url = format!("https://adventofcode.com/2024/day/{}/answer", day);
        let result = format!("{}", val);
        let params = [("answer", result), ("level", part.to_string())];
        let raw_response = self
            .req
            .post(submit_url)
            .form(&params)
            .send()
            .await
            .unwrap();
        let response_data = raw_response.text().await.unwrap();
        let response = AocResponse::from_raw_response(response_data.as_str());
        response
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AocResponse {
    text: String,
}

impl AocResponse {
    pub fn from_raw_response(raw_response: &str) -> AocResponse {
        let doc = Html::parse_document(raw_response);
        let selector = Selector::parse("article").unwrap();
        let article = doc.select(&selector).next().unwrap();
        let text = article.text().collect::<Vec<_>>();
        AocResponse {
            text: text.join(""),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AocSubmission {
    day: i64,
    part: AocDailyPart,
    answer: String,
    response: AocResponse,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AocStatus {
    part1_result: Option<String>,
    part2_result: Option<String>,
}


impl AocStatus {
    fn validate_part(site_result: &str, provided_result: &str) {
        let provided_result = provided_result.to_string();
        let matches = provided_result == site_result;
        if matches {
            print!("✅");
        } else {
            print!("❌ should be: {} is: {}", site_result, provided_result);
        }
    }
    pub(crate) fn validate_result(&self, res: &AocSolveResult) {
        self.part1_result.as_ref().map(|site_result| AocStatus::validate_part(&site_result, &*res.p1));
        self.part2_result.as_ref().map(|site_result| AocStatus::validate_part(&site_result, &*res.p2));
    }
}

impl AocStatus {
    pub fn from_raw_response(raw_response: &str) -> AocStatus {
        let doc = Html::parse_document(raw_response);
        let selector = Selector::parse("main>p>code").unwrap();
        let results = doc.select(&selector);
        let results = results
            .map(|res| {
                let text = res.text().next().unwrap();
                text.to_string()
            })
            .collect::<Vec<_>>();
        AocStatus {
            part1_result: results.get(0).cloned(),
            part2_result: results.get(1).cloned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::{AocClient, AocStatus};
    use crate::prepare_client;
    use tokio::runtime::Runtime;

    #[test]
    fn can_parse_progress() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            dotenv::from_filename(".env").ok();
            let aoc_session = std::env::var("AOC_SESSION").expect("AOC_SESSION not set");
            let aoc_user = std::env::var("AOC_USER").expect("AOC_USER not set");
            let client = AocClient::for_year(prepare_client(&aoc_session), 2024);
            let res = client.get_day(1).await;
            let status = AocStatus::from_raw_response(res.as_str());
            println!("{:#?}", status);
        });
    }
}
