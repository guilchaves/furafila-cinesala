use chrono::{Datelike, Local, NaiveDate};
use error_chain::error_chain;
use reqwest::Client;
use serde_json::Value;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
        SerdeJsonError(serde_json::Error);
    }
}

pub async fn fetch_and_process_response(fetch_address: &str) -> Result<usize> {
    let response_text = fetch_response(fetch_address).await?;
    let session_count = process_response(&response_text)?;
    Ok(session_count)
}

pub async fn fetch_response(fetch_address: &str) -> Result<String> {
    let client = Client::new();
    let response = client
        .get(fetch_address)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0")
        .send()
        .await?;
    let response_text = response.text().await?;
    Ok(response_text)
}

pub fn process_response(response_text: &str) -> Result<usize> {
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

pub fn extract_event_dates_data(response_text: &str) -> Option<&str> {
    let start_index = response_text.find("var eventDatesData = ")?;
    let start_index = start_index + "var eventDatesData = ".len();
    let end_index = response_text[start_index..].find("];")?;
    let end_index = start_index + end_index + 1;
    Some(&response_text[start_index..end_index])
}

pub fn process_segment(segment: &str) -> Result<Vec<NaiveDate>> {
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

pub fn parse_date(date_str: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
}
