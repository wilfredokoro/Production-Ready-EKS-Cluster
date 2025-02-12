use crate::email_msg;
use email_msg::EmailMessage;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Serialize;
use serde_json::Value;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::string::String;
use std::time::Duration;
use log::error;
use tokio::time::sleep;

pub fn get_converted_urls(text: String) -> Result<Vec<String>, Box<dyn Error>> {
    let original_urls: Vec<String> = get_urls(text)?;
    let mut safelink_urls = Vec::new();
    for url in original_urls {
        if let Some(converted_url) = convert_safelink(url.as_str()) {
            safelink_urls.push(converted_url);
        } else {
            safelink_urls.push(url.clone());
        }
    }
    let mut converted_urls: Vec<String> = Vec::new();
    for url in safelink_urls {
        if let Some(converted_url) = convert_safelink(url.as_str()) {
            converted_urls.push(converted_url);
        } else {
            converted_urls.push(url.clone());
        }
    }
    Ok(converted_urls)
}

pub fn get_urls(text: String) -> Result<Vec<String>, Box<dyn Error>> {
    // Define a static regex using lazy_static
    lazy_static! {
        static ref URL_REGEX: Regex = Regex::new(r"https?://[^\s]+").unwrap();
    }
    let mut result = Vec::new();
    let urls: Vec<String> = URL_REGEX
        .find_iter(&text)
        .map(|mat| mat.as_str().to_string())
        .collect();
    for url in urls {
        result.push(url);
    }
    Ok(result)
}

/// Removes HTML markup, non-printable characters, and normalizes linefeeds in a string.
///
/// # Arguments
/// - `input`: The HTML string to process.
///
/// # Returns
/// A cleaned string with no HTML tags, no non-printable characters, and normalized linefeeds.
pub(crate) fn clean_html(input: &str) -> String {
    // Regex to remove HTML tags
    let re_tags = Regex::new(r"<[^>]*>").expect("Failed to compile regex for HTML tags");
    let without_tags = re_tags.replace_all(input, "");
    // Remove non-printable characters (ASCII control chars except for whitespace)
    let without_nonprintable: String = without_tags
        .chars()
        .filter(|&c| c.is_ascii_graphic() || c.is_whitespace())
        .collect();
    // Replace literal '\r\n' with '\n'
    without_nonprintable.replace("\\r\\n", "\n")
}

pub async fn parse_msg(filename: PathBuf) -> Result<EmailMessage, Box<dyn Error>> {
    // todo is there any information wanted about the actual file?
    // todo ... maybe yes, we can use the detected type to confirm the extension
    // let _file_info = FileInfo::from_path(filename.clone())?;
    let file_extension = Path::new(&filename)
        .extension()
        .unwrap()
        .to_str()
        .unwrap_or("");

    match file_extension {
        "msg" => Ok(EmailMessage::from_outlook_msg_file(
            filename.clone().to_str().unwrap(),
        )?),
        ext => {
            error!("File extension {ext} is not supported");
            Err(Box::from("Not implemented".to_string()))
        },
    }
}

#[derive(Serialize)]
struct ScanRequest<'a> {
    url: &'a str,
    visibility: &'a str,
}

#[allow(dead_code)]
pub async fn url_scan(url: String) -> Result<String, Box<dyn Error>> {
    // todo development
    let developing = true;
    if developing {
        // skip the whole submission for now
        Ok("url submission paused for developent".to_string())
    } else {
        // Define the API endpoint and your API key
        let api_url = "https://urlscan.io/api/v1/scan/";
        let api_key = "fb053818-05bc-4c32-a2b3-bf681d3d635a";

        // Set up the headers
        let mut headers = HeaderMap::new();
        headers.insert("API-Key", HeaderValue::from_str(api_key)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Define the JSON body
        let scan_request = ScanRequest {
            url: url.as_str(),
            visibility: "public",
        };

        // Create the HTTP client
        let client = reqwest::Client::new();

        // Send the POST request
        let response = client
            .post(api_url)
            .headers(headers.clone())
            .json(&scan_request)
            .send()
            .await?;

        // Check if the response status is success
        if !response.status().is_success() {
            eprintln!(
                "Failed to submit URL for scanning. Status: {}",
                response.status()
            );
            return Ok(format!(
                "Failed to submit URL for scanning. Status: {}",
                response.status()
            )); // Exit if submission fails
        }

        // Parse the response as JSON
        let json_response: Value = response.json().await?;
        println!(
            "Initial Response: {}",
            serde_json::to_string_pretty(&json_response)?
        );

        // Get the API result URL (if available)
        let result_api_url = match json_response.get("api") {
            Some(api_url) => api_url.as_str().unwrap_or(""),
            None => {
                // eprintln!("API field not found in the initial response.");
                return Ok("API field not found in the initial response.".to_string());
            }
        };

        let initial_secs = 10;
        let polling_secs = 5;

        // Wait before polling
        sleep(Duration::from_secs(initial_secs)).await;

        // Poll the API result URL until a valid response is received
        let mut result_strings = Vec::new();
        loop {
            println!("Checking API result...");

            let check_response = client
                .get(result_api_url)
                .headers(headers.clone())
                .send()
                .await?;

            if check_response.status().is_success() {
                let result_json: Value = check_response.json().await?;
                result_strings.push(serde_json::to_string_pretty(&result_json)?);
                // println!(
                //     "Scan Result: {}",
                //     serde_json::to_string_pretty(&result_json)?
                // );
                break;
            } else if check_response.status().as_u16() == 404 {
                println!("Result not ready yet (404). Retrying in {polling_secs} seconds...");
                sleep(Duration::from_secs(polling_secs)).await;
            } else {
                result_strings.push(format!("Unexpected status: {}", check_response.status()));
                // eprintln!("Unexpected status: {}", check_response.status());
                break;
            }
        }
        Ok(result_strings.join("\n"))
    }
}

fn convert_safelink(safelink: &str) -> Option<String> {
    safelink
        .split('?')
        .nth(1) // Get the query string part
        .and_then(|query| {
            query
                .split('&') // Split into key-value pairs
                .find(|pair| pair.starts_with("url=")) // Find the "url=" parameter
                .map(|pair| &pair[4..]) // Extract the value after "url="
                .map(percent_decode) // Decode the URL
        })
}

/// Decodes percent-encoded values (like %3A to :)
fn percent_decode(encoded: &str) -> String {
    encoded.replace("%3A", ":").replace("%2F", "/") // Add more replacements as needed
}
