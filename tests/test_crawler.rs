use furafila_cinesala::crawler::*;

const MOCK_URL: &str = "https://example.com";
const MOCK_RESPONSE_TEXT: &str = r#"
        var eventDatesData = [
            { "date": "2024-03-06" },
            { "date": "2024-03-07" }
        ];
    "#;

#[tokio::test]
async fn test_fetch_response() {
    let response_text = fetch_response(MOCK_URL).await.unwrap();
    assert!(!response_text.is_empty());
}

#[test]
fn test_process_response() {
    let session_count = process_response(MOCK_RESPONSE_TEXT).unwrap();
    assert_eq!(session_count, 2);
}

#[test]
fn test_extract_event_dates_data() {
    let extracted_data = extract_event_dates_data(MOCK_RESPONSE_TEXT).unwrap();
    let expected_data = "[\n            { \"date\": \"2024-03-06\" },\n            { \"date\": \"2024-03-07\" }\n        ]";
    assert_eq!(extracted_data, expected_data);
}

#[test]
fn test_process_segment() {
    let segment = r#"[
        { "date": "2024-03-06" },
        { "date": "2024-03-07" }
    ]"#;

    let dates = process_segment(segment).unwrap();
    assert_eq!(dates.len(), 2);
}

#[test]
fn test_parse_date() {
    let date_str = "2024-03-06";
    let parsed_date = parse_date(date_str).unwrap();
    let formatted_date = parsed_date.format("%Y-%m-%d").to_string();
    assert_eq!(formatted_date, date_str);
}
