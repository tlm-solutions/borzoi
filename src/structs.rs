extern crate clap;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "borzoi")]
#[clap(author = "contact@tlm.solutions")]
#[clap(version = "0.1.0")]
#[clap(about = "data collection server with authentication and statistics", long_about = None)]
pub struct Args {
    #[clap(short, long, default_value_t = String::from("127.0.0.1"))]
    pub host: String,

    #[clap(long, default_value_t = String::from("127.0.0.1"))]
    pub prometheus_host: String,

    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,
}
