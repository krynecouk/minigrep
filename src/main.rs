use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from(&args).unwrap_or_else(|err| {
        eprintln!("Unable to parse config: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
