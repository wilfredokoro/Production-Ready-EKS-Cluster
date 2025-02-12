use clap::Parser;
/// CLI application with local, serve, and scan functionalities
#[derive(Parser)]
#[command(name = "SOC Analyst Assistant")]
#[command(about = "SOC Assistant serving a REST API.")]
#[command(author, version, long_about = None)]
pub(crate) struct Cli {
    #[arg(short, long, default_value_t = String::from("0.0.0.0"))]
    pub(crate) url: String,

    /// Port to run the server on
    #[arg(short, long, default_value_t = 5000)]
    pub(crate) port: u16,

    #[arg(short, long, default_value_t = true)]
    pub(crate) log: bool,
}
impl Cli {
    pub(crate) fn start_message(&self) -> String {
        format!("Starting server {}:{}", &self.url, &self.port)
    }
}

#[allow(clippy::vec_init_then_push)]
pub fn banner()  {
    // TODO dynamic version in banner
    const _NAME: &str = env!("CARGO_PKG_NAME");
    const _VERSION: &str = env!("CARGO_PKG_VERSION");
    eprintln!("\x1b[31m--------------------------------------------------------\x1b[0m");
    eprintln!("\x1b[31m  U.S. Citizenship and Immigration Services    (USCIS)  \x1b[0m");
    eprintln!("\x1b[37m  Cybersecurity Artificial Intelligence Support (CAIS)  \x1b[0m");
    eprintln!("\x1b[34m  SOC Analyst Assistant version 0.42                    \x1b[0m");
    eprintln!("\x1b[34m--------------------------------------------------------\x1b[0m");
}


