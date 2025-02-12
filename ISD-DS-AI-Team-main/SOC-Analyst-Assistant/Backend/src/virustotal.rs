use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::time::Duration;
// use base64::{engine::general_purpose, Engine as _};

// ----------------------------------------------
// Structs for the final analysis stats
// ----------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisStats {
    pub malicious: u32,
    pub suspicious: u32,
    pub undetected: u32,
    pub harmless: u32,
}

// ----------------------------------------------
// This is how the "POST /urls" response looks
// ----------------------------------------------
#[derive(Debug, Deserialize)]
struct PostUrlResponse {
    data: PostUrlData,
}

#[derive(Debug, Deserialize)]
struct PostUrlData {
    id: String,
}

// ----------------------------------------------
// This is how the "GET /analyses/{id}" response looks
// ----------------------------------------------
#[derive(Debug, Deserialize)]
struct AnalysisResponse {
    data: AnalysisData,
}

#[derive(Debug, Deserialize)]
struct AnalysisData {
    attributes: AnalysisAttributes,
}

#[derive(Debug, Deserialize)]
struct AnalysisAttributes {
    stats: AnalysisStats,
}

// ----------------------------------------------
// Here’s the new function that does the 2-step process
// ----------------------------------------------
pub async fn scan_url_with_timeout(
    api_key: &str,
    url: &str,
    timeout_secs: u64
) -> Result<AnalysisStats, Error> {
    // Build a client with a custom timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()?;

    // 1) POST the URL for a fresh scan
    let post_response = client
        .post("https://www.virustotal.com/api/v3/urls")
        .header("x-apikey", api_key)
        // VirusTotal wants the "url" in the POST form data
        .form(&[("url", url)])
        .send()
        .await?;

    // Parse the analysis ID from the POST response
    let post_url_res: PostUrlResponse = post_response.json().await?;
    let analysis_id = post_url_res.data.id;

    // 2) GET the result from /analyses/{analysis_id}
    let analysis_url = format!("https://www.virustotal.com/api/v3/analyses/{}", analysis_id);
    let get_response = client
        .get(analysis_url)
        .header("x-apikey", api_key)
        .send()
        .await?;

    let analysis_res: AnalysisResponse = get_response.json().await?;

    // Return the stats
    Ok(analysis_res.data.attributes.stats)
}

