use std::error::Error;
use std::env;
use std::process;

mod db;
mod api;

use db::Queries;

// valid commands
pub enum Command {
    InsertStock {arg: String},
    RemoveStock {arg: String},
    GetStocks,
}

trait FromStr {
    fn from_str(command: &str, arg: &str) -> Result<Command, (&'static str)>;
}

// FromStr is used to parse command line arg into enum
impl FromStr for Command {

    fn from_str(command: &str, arg: &str) -> Result<Command, (&'static str)> {
        let arg = arg.to_string();

        match command {
            "insert_stock" => Ok(Command::InsertStock {arg: arg}),
            "remove_stock" => Ok(Command::RemoveStock {arg: arg}),
            "get_stocks" => Ok(Command::GetStocks),
            _ => Err("Invalid command"),
        }
    }
}

pub struct Config {
    pub command: Command, 
    pub alpha_vantage_key: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // create tables if they do not exist
        db::create_tables().unwrap_or_else(|err| {
            eprintln!("Problem creating db tables: {}", err);   
            process::exit(1);               
        });

        let command = Command::from_str(&args[1], &args[2]).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);   
            process::exit(1);
        });

        let alpha_vantage_key = env::var("ALPHA_VANTAGE_API_KEY").unwrap_or_else(|err| {
            eprintln!("ALPHA_VANTAGE_API_KEY is not set: {}", err);   
            process::exit(1);
        });

        Ok(Config {command, alpha_vantage_key})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    match config.command {
        Command::InsertStock{arg: _ }   => { db::execute(config, Queries::InsertStock)?; },
        Command::RemoveStock{arg: _ }   => { db::execute(config, Queries::RemoveStock)?; },
        Command::GetStocks              => { db::execute(config, Queries::GetStocks)?; },
    }

    Ok(())
}
