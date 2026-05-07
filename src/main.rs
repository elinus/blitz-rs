mod bench;
mod cli;
mod config;
mod error;
mod http;
mod output;
mod source;

use crate::cli::CliArgs;
use crate::error::AppError;

#[tokio::main]
async fn main() {
    let args = match CliArgs::parse() {
        Ok(a) => a,
        Err(e) => {
            if let AppError::Cli(clap_err) = &e {
                clap_err.exit();
            }
            output::terminal::error(&e, e.help_message());
            std::process::exit(2);
        }
    };

    if let Err(e) = args.run().await {
        output::terminal::error(&e, e.help_message());
        std::process::exit(1);
    }
}
