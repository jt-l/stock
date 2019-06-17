use std::env;
use std::process;

use stocks::Config;

mod db;

fn main() {
    let args: Vec<String> = env::args().collect();

    db::create_tables();


    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);   
        process::exit(1);
    });

    if let Err(e) = stocks::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
