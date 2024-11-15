use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};
use serde_json::json;

/// Function to generate a token based on a username and a given timestamp.
fn generate_token(username: &str, timestamp: i64) -> String {
    let data: String = format!("{}{}", username, timestamp);
    
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    // Convert the resulting hash to a hexadecimal string
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Utility function to convert a date string to a Unix timestamp.
fn get_unix_timestamp(date_str: &str) -> i64 {
    let datetime = DateTime::parse_from_rfc2822(date_str).expect("Invalid date format");
    datetime.timestamp()
}

fn main() {
    // Provided UTC time: Fri, 15 Nov 2024 20:25:25 GMT
    let reference_timestamp = get_unix_timestamp("Fri, 15 Nov 2024 20:25:25 GMT");
    let username = "test1";

    println!("Tokens for username '{}' in the range of ±5 seconds:", username);

    // Generate tokens for all timestamps within ±5 seconds
    for offset in -5..=5 {
        let current_timestamp = reference_timestamp + offset;
        let token = generate_token(username, current_timestamp);

        // Print the token along with its corresponding timestamp
        let timestamp_str = DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp_opt(current_timestamp, 0).unwrap(), Utc);
        println!("Time: {}, Token: {}", timestamp_str, token);
    }
}
