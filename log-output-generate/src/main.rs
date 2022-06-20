use uuid::Uuid;

use chrono::prelude::*;

use std::time::Duration;
use std::io::prelude::*;
use std::fs::File;

#[tokio::main]
async fn main() {
    let string = Uuid::new_v4().hyphenated().to_string();

    loop {
        let message_with_timestamp = return_with_timestamp(&string);
        println!("{}", message_with_timestamp);

        let mut file = File::create("/shared/log.txt").expect("Failed to open file!");

        file.write_all(message_with_timestamp.as_bytes()).expect("Failed to update log file!");

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}


fn return_with_timestamp(value: &str) -> String {
    let time = Utc::now();

    format!("{}: {}", time, value)
}
