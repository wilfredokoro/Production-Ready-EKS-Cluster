#![allow(dead_code)]
use pdf_extract::extract_text;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::ZipArchive;
use chrono::Local;


/// A .docx file is essentially a ZIP archive containing XML files.
/// We can extract the text content from the word/document.xml file,
/// which contains the main body of the document.
fn extract_text_from_docx(filename: &Path) -> Result<String, Box<dyn std::error::Error>> {
    // Open the .docx file as a ZIP archive
    let file = File::open(filename)?;
    let mut archive = ZipArchive::new(file)?;

    // Find and extract `word/document.xml`
    let mut document_xml = String::new();
    if let Ok(mut file) = archive.by_name("word/document.xml") {
        file.read_to_string(&mut document_xml)?;
    } else {
        return Err("word/document.xml not found in .docx".into());
    }

    // Extract text by stripping XML tags
    let text = strip_xml_tags(&document_xml);
    Ok(text)
}

fn save_pdf_from_hex(hex_data: &str, output_path: &str) -> Result<(), String> {
    // Decode the hexadecimal string into raw binary data
    let binary_data = hex::decode(hex_data).map_err(|e| format!("Failed to decode hex: {}", e))?;

    // Create and write to the output file
    let mut file =
        File::create(output_path).map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(&binary_data)
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(())
}

fn strip_xml_tags(xml_content: &str) -> String {
    // Simple regex to strip XML tags and retain text
    use regex::Regex;
    let re = Regex::new(r"<[^>]+>").unwrap();
    re.replace_all(xml_content, "\n").to_string()
}

fn extract_text_from_pdf(filename: &Path) -> Result<String, Box<dyn Error>> {
    let text = extract_text(filename)?;
    Ok(text)
}


pub fn get_current_timestamp() -> String {
    // Get the current time
    let now = Local::now();

    // Format the timestamp as a string
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub(crate) fn contains_tracking_link(url: &str) -> bool {
    // List of known tracking links or their parts
    let tracking_links = [
        "click.mailchimpapp.com",
        "mailchi.mp",
        "r20.rs6.net",
        "visitor.constantcontact.com",
        "t.sidekickopen.com",
        "app.hubspot.com",
        "mkto-",
        "cmail1.com",
        "cmail20.com",
        "acems.com",
        "trackcmp.net",
        "doubleclick.net",
        "googlesyndication.com",
        "qksrv.net",
        "jdoqocy.com",
        "shareasale.com",
        "linksynergy.com",
        "clickbank.net",
        "bit.ly",
        "tinyurl.com",
        "rebrand.ly",
        "t.ly",
        "snip.ly",
        "tr1.brz.io",
        "links.iterable.com",
        "go.pardot.com",
        "sg-links.com",
        "clicks.sendgrid.net",
        "trk.klaviyomail.com",
        "gr8.com",
        "getresponse.click",
        "clickfunnels.com",
        "outbrain.com",
        "trc.taboola.com",
    ];

    // Check if the URL contains any of the tracking links
    tracking_links.iter().any(|&link| url.contains(link))
}
