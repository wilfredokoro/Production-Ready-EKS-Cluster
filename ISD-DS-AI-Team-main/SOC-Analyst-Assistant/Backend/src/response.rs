#![allow(dead_code)]
// #![allow(private_interfaces)]

use log::info;
use crate::analysis::clean_html;
use crate::email_msg::EmailMessage;
use msg_parser::Person;
use serde::{Deserialize, Serialize};
use crate::utils;
use crate::utils::get_current_timestamp;

#[derive(Serialize, Deserialize, Clone)]
pub struct SocAssistResponse {
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "incidentId")]
    pub incident_id: String,
    #[serde(rename = "detectionTimestamp")]
    pub detection_timestamp: String,
    pub status: String,
    #[serde(rename = "emailSubject")]
    pub email_subject: String,
    pub sender: Sender,
    #[serde(rename = "riskRating")]
    pub risk_rating: String,
    pub recipients: Vec<String>,
    pub observations: Vec<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
    #[serde(rename = "headerDetails")]
    pub header_details: HeaderDetails,
    #[serde(rename = "emailContent")]
    email_content: EmailContent,
    #[serde(rename = "linksAndAttachments")]
    links_and_attachments: LinksAndAttachments,
    #[serde(rename = "remediationActions")]
    remediation_actions: Vec<String>,
    #[serde(rename = "supplementalDetails")]
    supplemental_details: SupplementalDetails,
    #[serde(rename = "auditLog")]
    audit_log: Vec<AuditRecord>,
}

// todo are there any defaults for any parts of a response?
impl Default for SocAssistResponse {
    fn default() -> SocAssistResponse {
        SocAssistResponse {
            request_id: Default::default(),
            incident_id: "*** incident_id ***".to_string(), // todo
            detection_timestamp: "***  detection_timestamp ***".to_string(), // todo
            status: "***  status ***".to_string(),          // todo
            email_subject: "*** email_subject ***".to_string(),
            sender: Default::default(),
            risk_rating: "*** risk_rating ***".to_string(),
            recipients: vec!["*** recipients ***".to_string()],
            observations: vec![],
            last_updated: "*** last_updated ***".to_string(),
            header_details: Default::default(),
            email_content: Default::default(),
            links_and_attachments: Default::default(),
            remediation_actions: vec![],
            supplemental_details: Default::default(),
            audit_log: vec![],
        }
    }
}

#[allow(clippy::field_reassign_with_default)]
impl SocAssistResponse {
    pub fn from_email(email: EmailMessage, id:String) -> SocAssistResponse {
        let mut response = SocAssistResponse::default();
        response.audit_log.push(AuditRecord::new("Parsing".to_string(), "Submitted for Parsing".to_string()));
        response.request_id = id.clone();
        // todo for the time being we can use request_id as incident_id
        response.incident_id = id.clone();
        response.status = "Submitted".to_string();
        response.risk_rating = "in progress".to_string();
        response.detection_timestamp = get_current_timestamp();
        info!("Setting incident id: {}", response.incident_id);
        response.email_subject = email.msg.subject.clone();
        response.sender = Sender::from_person(&email.msg.sender);

        response.recipients = email
            .msg
            .to
            .iter()
            .map(|person| format!("{:?}", person))
            .collect(); // todo duplicate field
        response.sender = Sender::from_person(&email.msg.sender);
        response.recipients = email.msg.to.iter().map(|s| s.email.clone()).collect();
        // todo to clean html content or not to clean
        // response.email_content = EmailContent {
        //     html: email.msg.body.clone(),
        //     suspicious_elements: vec![],
        // };
        response.email_content = EmailContent {
            html: clean_html(&email.msg.body),
            suspicious_elements: vec![],
        };
        response.links_and_attachments = LinksAndAttachments::from_email(&email);
        response.header_details.cc = format!("{:?}", email.msg.cc);
        response.header_details.to = email
            .msg
            .to
            .iter()
            .map(|person| format!("{:?}", person))
            .collect(); // todo duplicate field
        response.header_details.date = email.msg.headers.date.clone();
        response.header_details.message_id = email.msg.headers.message_id;
        response.header_details.from = Sender::from_person(&email.msg.sender);
        response.header_details.received = email.msg.headers.date;
        response.header_details.reply_to = email.msg.headers.reply_to.clone();
        response.header_details.subject = email.msg.subject.clone();
        let _ = response.make_observations();
        response
    }
    pub fn from_json(json: &str) -> SocAssistResponse {
        // todo return result?
        serde_json::from_str(json).unwrap_or_else(|_e| SocAssistResponse::default())
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
    }
    fn make_observations(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut has_tracking_link = false;
        for link in &self.links_and_attachments.links {
            let url = link.url.clone();
            if utils::contains_tracking_link(url.as_str()) {
                has_tracking_link = true;
            }
        }
        if has_tracking_link {
            self.observations.push("Message contains tracking links".to_string())
        } else {
            self.observations.push("No tracking links detected".to_string())
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct AuditRecord {
    pub timestamp: String,
    pub user: String,
    pub action: String,
    pub notes: String,
}
impl AuditRecord {
    fn new(action: String, notes: String) -> AuditRecord {
        let timestamp = get_current_timestamp();
        let user = "SOC Analyst Assistant".to_string();
        AuditRecord {
            timestamp,
            user,
            action,
            notes,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Mitre {
    pub technique: String,
    pub tactic: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct SupplementalDetails {
    #[serde(rename = "userInteractions")]
    pub user_interactions: Vec<String>,
    #[serde(rename = "threatIntelligence")]
    pub threat_intelligence: Vec<String>,
    #[serde(rename = "MITRE")]
    pub mitre: Mitre,
}
impl Default for SupplementalDetails {
    fn default() -> Self {
        SupplementalDetails {
            user_interactions: vec![],
            threat_intelligence: vec![],
            mitre: Mitre {
                technique: "".to_string(),
                tactic: "".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Link {
    pub url: String,
    pub status: String,
    pub analysis: String,
    pub action: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct LinksAndAttachments {
    pub links: Vec<Link>,
    pub attachments: Vec<String>, // todo how to return? attachments?
}
impl LinksAndAttachments {
    fn from_email(email: &EmailMessage) -> Self {
        let mut links = Vec::<Link>::new();
        for link in &email.links {
            links.push(Link {
                url: link.to_string(),
                status: "unknown".to_string(),
                analysis: "unknown".to_string(),
                action: "unknown".to_string(),
            })
        }
        let mut attachments = Vec::<String>::new();
        for attachment in &email.attachments {
            attachments.push(attachment.to_string());
        }
        Self { links, attachments }
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct EmailContent {
    pub html: String,
    #[serde(rename = "suspiciousElements")]
    pub suspicious_elements: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AuthenticationResults {
    #[serde(rename = "SPF")]
    pub spf: String,
    #[serde(rename = "DKIM")]
    pub dkim: String,
    #[serde(rename = "DMARC")]
    pub dmarc: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct HeaderDetails {
    pub from: Sender,
    pub to: Vec<String>,
    pub cc: String,
    pub subject: String,
    pub date: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "replyTo")]
    pub reply_to: String,
    #[serde(rename = "returnPath")]
    pub return_path: String,
    pub received: String,
    #[serde(rename = "authenticationResults")]
    pub authentication_results: AuthenticationResults,
    #[serde(rename = "observations")]
    pub observations: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Sender {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}
impl Sender {
    fn from_person(person: &Person) -> Sender {
        Sender {
            display_name: person.name.clone(),
            email_address: person.email.to_string(),
        }
    }
}

// struct Observations {
//     observations: Vec<String>,
// }
// impl Observations {
//     fn from_email(email: &EmailMessage) -> Observations {
//         let mut observations = Vec::<String>::new();
//         for link in email.links {
//
//         }
//     }
// }