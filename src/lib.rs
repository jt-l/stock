extern crate clap;

use clap::{App, Arg, SubCommand};

use std::env;
use std::error::Error;
use std::process;

mod api;
mod db;
mod formatter;

use db::Queries;

// valid commands
#[derive(Clone)]
pub enum Command {
    InsertStock { arg: String },
    RemoveStock { arg: String },
    GetStocks,
}

trait FromStr {
    fn from_str(args: &[String]) -> Result<Command, (&'static str)>;
}

// FromStr is used to parse command line arg into enum
impl FromStr for Command {
    fn from_str(args: &[String]) -> Result<Command, (&'static str)> {
        let command = &args[0];

        match command.as_ref() {
            "add" => Ok(Command::InsertStock {
                arg: args[1].clone(),
            }),
            "rm" => Ok(Command::RemoveStock {
                arg: args[1].clone(),
            }),
            "ls" => Ok(Command::GetStocks),
            _ => Err("Invalid command"),
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub command: Command,
    pub api_key: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let mut args: Vec<String> = Vec::new();

        let matches = App::new("stocks")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .subcommand(
                SubCommand::with_name("add")
                    .about("Add a stock to the db")
                    .arg(
                        Arg::with_name("symbol")
                            .index(1)
                            .required(true)
                            .help("The symbol that you want to add"),
                    ),
            )
            .subcommand(
                SubCommand::with_name("rm")
                    .about("Remove a stock from the db")
                    .arg(
                        Arg::with_name("symbol")
                            .index(1)
                            .required(true)
                            .help("The symbol that you want to remove"),
                    ),
            )
            .subcommand(SubCommand::with_name("ls").about("List the info for stocks in the db"))
            .get_matches();

        // match the command
        if let Some(matches) = matches.subcommand_matches("add") {
            if let Some(val) = matches.value_of("symbol") {
                args.push("add".to_string());
                args.push(val.to_string().to_lowercase());
            }
        } else if let Some(matches) = matches.subcommand_matches("rm") {
            if let Some(val) = matches.value_of("symbol") {
                args.push("rm".to_string());
                args.push(val.to_string().to_lowercase());
            }
        } else if matches.is_present("ls") {
            args.push("ls".to_string());
        }

        let command = Command::from_str(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

        let api_key = env::var("WORLD_TRADING_DATA_API_KEY").unwrap_or_else(|err| {
            eprintln!("WORLD_TRADING_DATA_API_KEY is not set: {}", err);
            process::exit(1);
        });

        Ok(Config { command, api_key })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // create tables if they do not exist
    db::execute(config.clone(), Queries::CreateTables)?;

    // execute command
    match config.command {
        Command::InsertStock { .. } => {
            db::execute(config, Queries::InsertStock)?;
        }
        Command::RemoveStock { .. } => {
            db::execute(config, Queries::RemoveStock)?;
        }
        Command::GetStocks => {
            db::execute(config, Queries::GetStocks)?;
        }
    }

    Ok(())
}
