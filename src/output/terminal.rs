use owo_colors::OwoColorize;

/// Print an error message to stderr with a help message.
pub fn error<E: std::error::Error>(err: E, help: &str) {
    eprintln!("{} {}", "Error:".red().bold(), err);
    eprintln!("{} {}", "Help:".cyan(), help);
}
