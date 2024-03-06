use chrono::{Datelike, Local, NaiveDate};
use error_chain::error_chain;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
        SerdeJsonError(serde_json::Error);
    }
}

const CINESALA_ADDRESS: &str =
    "https://www.veloxtickets.com/Portal/Local/Cinema/Sao-Paulo/CINESALA/CSL/";
const REFRESH_TIME_IN_SECONDS: u64 = 2;

#[tokio::main]
async fn main() -> Result<()> {
    let mut initial_sessions_count = 0;
    loop {
        let updated_sessions_count = fetch_and_process_response().await?;
        if initial_sessions_count == 0 {
            initial_sessions_count = updated_sessions_count;
        } else {
            if initial_sessions_count < updated_sessions_count {
                println!("NOVAS SESSÕES DISPONÍVEIS! Corre: {}", CINESALA_ADDRESS);
            }
        }
        tokio::time::sleep(Duration::from_secs(REFRESH_TIME_IN_SECONDS)).await;
    }
}

async fn fetch_and_process_response() -> Result<usize> {
    let response_text = fetch_response().await?;
    let session_count = process_response(&response_text)?;
    Ok(session_count)
}

async fn fetch_response() -> Result<String> {
    let client = Client::new();
    let response = client
        .get(CINESALA_ADDRESS)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0")
        .send()
        .await?;
    let response_text = response.text().await?;
    Ok(response_text)
}

fn process_response(response_text: &str) -> Result<usize> {
    let mut new_count = 0;
    if let Some(event_dates_data) = extract_event_dates_data(response_text) {
        for segment in event_dates_data.split(';') {
            let trimmed_segment = segment.trim();
            if !trimmed_segment.is_empty() {
                if let Ok(formatted_dates) = process_segment(trimmed_segment) {
                    new_count += formatted_dates.len();
                }
            }
        }
    }
    Ok(new_count)
}

fn extract_event_dates_data(response_text: &str) -> Option<&str> {
    let start_index = response_text.find("var eventDatesData = ")?;
    let start_index = start_index + "var eventDatesData = ".len();
    let end_index = response_text[start_index..].find("];")?;
    let end_index = start_index + end_index + 2;
    Some(&response_text[start_index..end_index])
}

fn process_segment(segment: &str) -> Result<Vec<NaiveDate>> {
    let json_data: Value = serde_json::from_str(segment)?;
    if let Some(array) = json_data.as_array() {
        let dates: Vec<_> = array
            .iter()
            .filter_map(|entry| entry.get("date").and_then(Value::as_str))
            .filter_map(|date_str| parse_date(date_str))
            .collect();
        if !dates.is_empty() {
            let formatted_dates: Vec<String> = dates
                .iter()
                .map(|date| format!("{:02}/{:02}/{}", date.day(), date.month(), date.year()))
                .collect();
            let now = Local::now().format("%H:%M:%S");

            println!(
                "Sessões disponíveis: {}     |    Scraped at: {}",
                formatted_dates.join(", "),
                now
            );

            return Ok(dates);
        }
    }
    Ok(Vec::new())
}

fn parse_date(date_str: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
}
