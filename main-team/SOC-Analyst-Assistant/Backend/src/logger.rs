#![allow(dead_code)]
use std::fmt::Arguments;
use chrono::Utc;
use env_logger::Builder;
use log::Level::{Debug, Error, Info, Trace, Warn};
use log::{Level, LevelFilter, Record};
use std::io::Write;
use clap::builder::Str;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

pub fn setup_logger() {
    fn colorize_msg(level: log::Level, msg: &str) -> String {
        match level {
            Error => { format!("\x1b[91{}\x1b[0m", &msg ) }
            Warn  => { format!("\x1b[31{}\x1b[0m", &msg ) }
            Info  => { format!("\x1b[32{}\x1b[0m", &msg ) }
            Debug => { format!("\x1b[34{}\x1b[0m", &msg ) }
            Trace => { format!("\x1b[33{}\x1b[0m", &msg ) }
        }
    }
    fn normal() -> &'static str {
        "\x1b[0m"
    }
    fn color(record:&Record) -> &'static str {
        match record.level() {
            Error => {"\x1b[91"}
            Warn => {"\x1b[31"}
            Info => {"\x1b[32"}
            Debug => {"\x1b[34"}
            Trace => {"\x1b[33"}
        }
    }
    // fn colorize(record:Record, msg:&str) -> String {
    //     match record.level() {
    //         Error => format!("\x1b[91{}\x1b[0m",msg), // bright red
    //         Warn  => format!("\x1b[31m{} \x1b[0m",msg),   // red
    //         Info  => format!("\x1b[32m{} \x1b[0m",msg),   // green
    //         Debug => format!("\x1b[34m{}\x1b[0m",msg), // blue
    //         Trace => format!("\x1b[33m{}\x1b[0m",msg), // yellow
    //     }
    // }
    // fn xcolorize(level: log::Level) -> &'static str {
    //     match level {
    //         Error => "\x1b[91merror\x1b[0m", // bright red
    //         Warn  => "\x1b[31mwarn \x1b[0m",   // red
    //         Info  => "\x1b[32minfo \x1b[0m",   // green
    //         Debug => "\x1b[34mdebug\x1b[0m", // blue
    //         Trace => "\x1b[33mtrace\x1b[0m", // yellow
    //     }
    // }
    #[allow(dead_code)]
    fn from_string(level_string: String) -> LevelFilter {
        match level_string.to_lowercase().as_str() {
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        }
    }
    Builder::from_default_env()
        .format(|buf, record| {
            // let fmt = format!("[{}]:{} {}", record.level().as_str(), Utc::now(), record.args());
            // let msg = colorize_msg( record.level(), fmt.as_str());
            writeln!(buf,"[{}] :{} {}",
                     record.level(),
                     Utc::now(),
                     record.args(),
            )
        })
        .init();
}

#[allow(dead_code)]
fn main() {
    setup_logger();
    log::error!("This is an error");
    log::warn!("This is an warning");
    log::info!("This is information");
    log::debug!("This is a debugging message");
    log::trace!("This is as trace message");
}
