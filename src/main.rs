use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(args.as_slice()).unwrap_or_else(|err| {
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        process::exit(1);
    }
}

