use std::time::Duration;
use tokio::time::sleep;

use furafila_cinesala::crawler;

const CINESALA_ADDRESS: &str = "https://www.veloxtickets.com/Portal/Local/Cinema/Sao-Paulo/CINESALA/CSL/";
const REFRESH_TIME_IN_SECONDS: u64 = 2;

#[tokio::main]
async fn main() {
    let mut initial_sessions_count = 0;
    loop {
        if let Ok(updated_sessions_count) = crawler::fetch_and_process_response(CINESALA_ADDRESS).await {
            if initial_sessions_count == 0 {
                initial_sessions_count = updated_sessions_count;
            } else {
                if initial_sessions_count < updated_sessions_count {
                    println!("NOVAS SESSÕES DISPONÍVEIS! Corre: {}", CINESALA_ADDRESS);
                }
            }
        }
        sleep(Duration::from_secs(REFRESH_TIME_IN_SECONDS)).await;
    }
}
