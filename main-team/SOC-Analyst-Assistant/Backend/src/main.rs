#[allow(dead_code,u)]
mod analysis;
mod cli;
mod email_msg;
mod file_type;
mod response;
mod utils;
mod logger;
mod virustotal;// added by Plamen
mod iris;

use std::string::String;
use std::collections::HashMap;
#[allow(unused_imports)]
use log::{info, warn, error};
use actix_cors::Cors;
use crate::cli::{banner, Cli};
use crate::response::SocAssistResponse;
use actix_multipart::Multipart;
use actix_web::{http, web, App, HttpResponse, HttpServer, Responder, Result};
use clap::Parser;
use futures_util::StreamExt as _;
use lazy_static::lazy_static;
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;
use std::error::Error;
use std::fmt::format;
use std::path::Path;
// use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use crate::analysis::parse_msg;
use crate::logger::setup_logger;
use std::sync::{Arc, Mutex};
// use serde_json::Value::String;
use sha2::{Sha256, Digest};
use crate::virustotal::{scan_url_with_timeout, AnalysisStats}; //added by Plamen
use crate::iris::{get_risk_score_with_timeout}; //added by Plamen

/// Application state: A thread-safe map
type AppState = Arc<Mutex<HashMap<String, SocAssistResponse>>>;
// Define the static global state using lazy_static
lazy_static! {
    static ref GLOBAL_APP_STATE: AppState = Arc::new(Mutex::new(HashMap::new()));
}

// Function to add an entry to the global state
fn add_entry(key: &str, response: SocAssistResponse) {
    let mut state = GLOBAL_APP_STATE.lock().unwrap();
    state.insert(key.to_string(), response);
}

// Function to get an entry from the global state
fn get_entry(key: &str) -> Option<SocAssistResponse> {
    let state = GLOBAL_APP_STATE.lock().unwrap();
    Some(state.get(key)?.clone())
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger();
    let cli = Cli::parse();
    banner();
    info!("{}", cli.start_message());

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .route("/", web::get().to(index_endpoint))
            .route("/api/emails", web::get().to(get_emails))
            .route("/api/emails", web::post().to(post_emails))
            .route("/api/emails/{id}", web::get().to(get_email_by_id))
            .route("/api/virustotal", web::post().to(virustotal_route)) // added by Plamen
            .route("/api/iris", web::post().to(virustotal_route)) // added by Plamen
    })
        .bind(format!("{}:{}", "0.0.0.0", cli.port))?
        .run()
        .await?;
    Ok(())
}

/// A simple user interface for testing and power users
async fn index_endpoint() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("index.html"))
}

// Handler to return a list of email IDs in JSON format
async fn get_emails() -> impl Responder {
    let state = GLOBAL_APP_STATE.lock().unwrap();
    let mut emails = Vec::new();
    for sar in state.keys() {
        emails.push(format!("available :{}", sar));
    }
    // state.insert(key.to_string(), response);
    //
    // // Define a structure for the response
    // #[derive(Serialize)]
    // struct IDList {
    //     ids: Vec<String>,
    // }
    // let emails = IDList {
    //     ids: vec![
    //         "email_id_123".to_string(),
    //         "email_id_456".to_string(),
    //         "email_id_789".to_string(),
    //     ],
    // };
    HttpResponse::Ok().json(emails)
}

fn get_extension(filename:&str) -> String {
    Path::new(&filename)
        .extension()
        .unwrap()
        .to_str()
        .unwrap_or("")
        .to_string()
}

use base64::Engine;

fn bytes_to_base64(bytes: &[u8]) -> String {
    let engine = base64::engine::general_purpose::STANDARD;
    engine.encode(bytes)
}
async fn post_emails(mut payload: Multipart) -> Result<HttpResponse> {
    let mut hasher = Sha256::new(); // Initialize the hasher

    // Finalize the hash and return it as a hex string
    // let hash_result = hasher.finalize();
    #[allow(clippy::never_loop)]
    while let Some(item) = payload.next().await {
        let mut field = item?;
        if let Some(content_disposition) = field.content_disposition() {
            if let Some(filename) = content_disposition.get_filename().map(String::from) {
                let ext = get_extension(&filename).to_ascii_lowercase();
                match ext.as_str() {
                    "msg" => { },
                    any_other => {
                        let msg = format!( "File extension ({}) not supported.", any_other);
                        return Ok(HttpResponse::NotImplemented()
                            .content_type("text/html; charset=utf-8")
                            .body(msg))
                    },
                };
                info!("Received POST ({})", &filename);
                let filepath = env::temp_dir().join(filename);
                let mut file = File::create(&filepath)?;
                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    hasher.update(&data);
                    file.write_all(&data)?;
                }
                // keep 16 bytes of entropy
                let hash = hasher.finalize()[..32].to_vec();

                // let len = hash.len();
                // assert_eq!(len, 32usize);
                // let hash_array: [u8; 32] = [0; 32];
                // hash.clone_from_slice(&hash_array);

                let hash_string = bytes_to_base64(&hash);
                let response = match get_entry(&hash_string) {
                    None => { // not already analyzed
                        info!("analyzing post...");
                        let filename = filepath.to_string_lossy();
                        let email_msg = parse_msg(filename.parse()?).await?;
                        // todo =========================================================
                        // todo if logging temp code true to log temp output for analysis
                        if false {
                            let value = serde_json::to_value(&email_msg)?;
                            append_json_to_file(&value).await?;
                        }
                        // todo =========================================================
                        let response = SocAssistResponse::from_email(email_msg,hash_string.clone());
                        add_entry(hash_string.as_str(), response.clone());
                        response
                    }
                    Some(sar) => { // already analyzed
                        info!("returning previous analysis...");
                        sar
                    }
                };

                // let response = analyze((filepath.to_string_lossy()).parse()?).await?;

                info!("Returning OK");

                // todo handle multiple files and meta data
                return Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(response.to_json()));
            }
        }
    }
    info!("Returning error");

    // todo do we want to return an err here or is ok ok?
    Ok(HttpResponse::InternalServerError()
        .content_type("text/html")
        .body("no response"))
}

// Define a structure to deserialize the ID from the request path
#[derive(Deserialize)]
struct IdPath {
    id: String,
}

// added by Plamen
#[derive(Deserialize)]
struct UrlPayload {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VirusTotalResponse {
    stats: AnalysisStats,
    vt_link: String,
}

async fn virustotal_route(payload: web::Json<UrlPayload>) -> impl Responder {
    // Use your actual API key.
    let api_key = "48e2b231404bca4d18a3dd93dc3db82a9259340d884c2131cade06bf98cf7c5f";

    // Example: set a 10-second timeout
    let timeout_secs = 10;

    match scan_url_with_timeout(api_key, &payload.url, timeout_secs).await {
        Ok(stats) => {
            // If you still want to generate a “View on VirusTotal” link, do so
            // but keep in mind that if you want a link to a *domain* analysis,
            // you pass the domain itself. If you want a link to the *analysis*,
            // you'd do something like: format!(".../analysis/{}", analysis_id).
            //
            // The generate_virustotal_link function you wrote is for domain pages,
            // so do whatever makes sense for your use case:
            let vt_link = &payload.url;

            let response = VirusTotalResponse { stats, vt_link: vt_link.to_string() };
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// This will be our response payload back to the client.
#[derive(Debug, Serialize, Deserialize)]
struct IrisResponse {
    risk_score: u32,
    dt_link: String,
}

// Example route function for DomainTools Iris Investigate
async fn iris_investigate_route(payload: web::Json<UrlPayload>) -> impl Responder {
    // Provide your DomainTools API username & key
    let api_username = "uscis_dhs";
    let api_key = "X";
    let api_key2 = "84730-f93ba-7bfba-30a0a-ed77d";

    // Example: set a 10-second timeout
    let timeout_secs = 10;

    // Call our new Iris Investigate function
    match get_risk_score_with_timeout(api_username, api_key, &payload.url, timeout_secs).await {
        Ok(risk_score) => {
            // If you want to give a link to DomainTools Iris, you can do so here
            // (Assuming you have a direct link; this is just a guess).
            let dt_link = format!("https://iris.domaintools.com/iris/domain/{}", &payload.url);

            let response = IrisResponse { risk_score, dt_link };
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Handler to retrieve an email by ID
#[allow(unused_variables)]
async fn get_email_by_id(path: web::Path<IdPath>) -> impl Responder {
    let id_path = path.id.to_string();
    info!("Getting email by id: {}", id_path);
    // todo Temporary response until the actual response code is developed
    // todo this also tests the from and to functions
    // let test_response = SocAssistResponse::from_json("response.json");
    // let json_response = SocAssistResponse::to_json(&test_response);
    let json_response = include_str!("response.json");

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json_response)
}


// async fn download_endpoint() -> impl Responder {
//     // todo Temporary response until the actual response code is developed
//     // todo this also tests the from and to functions
//     let test_response = SocAssistResponse::from_json("response.json");
//     let json_response = SocAssistResponse::to_json(&test_response);
//
//     HttpResponse::Ok()
//         .content_type("application/json")
//         .body(json_response)
// }



// Function to assist with logging and saving a snapshot of email messages
#[allow(dead_code)]
async fn append_json_to_file(json_param: &Value) -> Result<(), Box<dyn Error>> {
    lazy_static! {
        static ref STATIC_FILE: File = OpenOptions::new()
            .create(true)   // Create the file if it doesn't exist
            .append(true)   // Append to the file
            .open("output.jsonl")
            .expect("Failed to open the file");
    }

    // Serialize the JSON parameter to a string
    let json_string = serde_json::to_string(json_param)?;

    // Write the JSON string to the file, followed by a newline
    Ok(writeln!(&*STATIC_FILE, "{}", json_string)?)
}

