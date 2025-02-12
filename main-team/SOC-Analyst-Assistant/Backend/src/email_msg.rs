#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]
use crate::analysis::clean_html;
use crate::response::{HeaderDetails, Sender};
use msg_parser::{Attachment, Outlook, TransportHeaders};
use std::env::args;
use std::fmt;
use std::fs::{self, File};
use std::io::Write;

pub fn parse_msg(msg_path: PathBuf) -> Result<EmailMessage, Box<dyn std::error::Error>> {
    let args = args().skip(1).collect::<Vec<_>>();
    // Path to the .msg file
    // let msg_path = &args[0];    // Path to the .msg file

    // Output directory for attachments
    let output_dir = "attachments";
    fs::create_dir_all(output_dir)?;

    let email = EmailMessage::from_outlook_msg_file(msg_path.to_str().unwrap())?;
    println!("{}", email);
    let display = [format!("{}", email)];
    Ok(email)
    // // std::process::exit(42);
    // // Parse the .msg file
    // let msg = Outlook::from_path(msg_path)?;
    // let from = msg.sender;
    // let to = msg.to;
    // let subject = msg.subject;
    // let body = msg.body;
    //
    // println!("Subject: {}", subject);
    // // Iterate over attachments
    // for (i, attachment) in msg.attachments.iter().enumerate() {
    //     // Determine the filename or use a default name
    //     let filename = &attachment
    //         .file_name;
    //
    //     println!("Attaching {} to {}", filename, output_dir);
    //         // .or_else(|| attachment.short_filename())
    //         // .unwrap_or_else(|| format!("attachment_{}", i + 1));
    //
    //     let attachment_path = format!("{}/{}", output_dir, filename.as_str());
    //
    //     // Extract and decode attachment data
    //     let data = &attachment.payload;
    //     println!("payload len {}", data.len());
    //
    //     // // Check if the data is Base64 encoded
    //     // if is_base64_encoded(data.as_bytes()) {
    //     //     // Decode the Base64 data
    //     //     let decoded_data = base64::decode(&data)?;
    //     //     fs::write(&attachment_path, decoded_data)?;
    //     //     println!("Base64-decoded attachment saved to: {}", attachment_path);
    //     // } else {
    //         // Save raw binary data
    //         // fs::write(&attachment_path, data)?;
    //     // Convert hex to bytes
    //     let bytes = hex::decode(data)?;
    //     let file_type = FileType::from_magic_number(bytes.as_slice());
    //
    //     eprintln!("attachment file type: {}", file_type.description());
    //
    //
    //     // Write to a PDF file
    //     let mut file = File::create(attachment_path)?;
    //     file.write_all(&bytes)?;
    //
    //         // println!("{}",data);
    //         // println!("Attachment saved to: {}", attachment_path);
    //     // }
    // }

    // Ok(())
}

/// Helper function to check if data is Base64 encoded
fn is_base64_encoded(data: &[u8]) -> bool {
    // Check if the data contains valid Base64 characters
    data.iter()
        .all(|&b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/' || b == b'=')
}
// use std::fs::File;
use std::io::{self, Read};

use crate::analysis::get_converted_urls;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
// use tokio::sync::watch::Sender;

/// Struct to represent the parts of an email message
#[derive(Serialize, Deserialize, Debug)]
pub struct EmailMessage {
    pub(crate) msg: Outlook,
    pub(crate) text: String,
    pub(crate) links: Vec<String>,
    pub(crate) attachments: Vec<String>,
    // pub(crate) id: String,
    // pub(crate) sender: Sender,
    // pub(crate) recipients: Vec<String>,
    // pub(crate) subject: Option<String>,
    // pub(crate) body: Option<String>,
    // pub(crate) links: Vec<String>,
    // pub(crate) attachments: Vec<String>,
    // pub(crate) attachments: HashMap<String, Vec<u8>>, // Filename to binary data
}

/// Populate the EmailMessage from an Outlook .msg file
impl EmailMessage {
    pub fn from_outlook_msg_file(
        file_path: &str,
    ) -> Result<EmailMessage, Box<dyn std::error::Error>> {
        let mut msg = Outlook::from_path(file_path)?;
        msg.rtf_compressed = "".to_string(); // todo do we need to keep this
        for attachment in &mut msg.attachments {
            attachment.payload = String::from(""); // todo do we need to keep this
        }
        let text = clean_html(&msg.body.to_string());
        let links = get_converted_urls(msg.body.to_owned())?;
        let attachments = get_filenames_from_attachments(&msg.attachments);
        Ok(EmailMessage {
            msg,
            text,
            links,
            attachments,
        })
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
    }
}
fn get_filenames_from_attachments(attachments: &Vec<Attachment>) -> Vec<String> {
    let mut filenames = vec![];
    for attachment in attachments {
        filenames.push(attachment.file_name.clone());
    }
    filenames
}
impl fmt::Display for EmailMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_string = serde_json::to_string_pretty(self).unwrap();
        writeln!(f, "{}", display_string)?;
        Ok(())
    }
}
