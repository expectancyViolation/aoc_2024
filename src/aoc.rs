use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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

pub struct AocClient {
    req: Client,
    pub year: i32,
}

impl AocClient {
    pub fn new(client: Client) -> AocClient {
        AocClient { req: client, year: 2024 }
    }

    pub fn for_year(client: Client, year: i32) -> AocClient {
        AocClient { req: client, year }
    }

    pub async fn get_day(&self, day: i64) -> String {
        let input_url = format!("https://adventofcode.com/{}/day/{}/input", self.year, day);
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
