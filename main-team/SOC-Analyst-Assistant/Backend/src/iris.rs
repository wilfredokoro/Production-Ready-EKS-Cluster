use reqwest::{Client, Error};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct IrisApiResponse {
    response: ResponseBody,
}

#[derive(Debug, Deserialize)]
struct ResponseBody {
    results: Vec<IrisResult>,
}

#[derive(Debug, Deserialize)]
struct IrisResult {
    domain_risk: DomainRisk,
}

#[derive(Debug, Deserialize)]
struct DomainRisk {
    risk_score: u32,
}

pub async fn get_risk_score_with_timeout(
    api_username: &str,
    api_key: &str,
    domain: &str,
    timeout_secs: u64
) -> Result<u32, Error> {
    // Build a client with custom timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()?;

    // Construct the URL for Iris Investigate
    let url = format!(
        "https://api.domaintools.com/v1/iris-investigate/?domain={domain}&api_username={user}&api_key={key}",
        domain = domain,
        user = api_username,
        key = api_key
    );

    // Perform the GET request
    let resp = client.get(&url).send().await?;

    // Parse the JSON into our structs
    let iris_response: IrisApiResponse = resp.json().await?;

    // Extract the risk_score from the first result (assuming there is at least one)
    let risk_score = iris_response
        .response
        .results
        .get(0)
        .map(|result| result.domain_risk.risk_score)
        .unwrap_or(0); // fallback if no results

    Ok(risk_score)
}
