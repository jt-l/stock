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
        let command = &args[1];

        match command.as_ref() {
            "add" => {
                if args.len() < 3 {
                    return Err("Not enough arguments");
                };
                Ok(Command::InsertStock {
                    arg: args[2].clone(),
                })
            }
            "rm" => {
                if args.len() < 3 {
                    return Err("Not enough arguments");
                };
                Ok(Command::RemoveStock {
                    arg: args[2].clone(),
                })
            }
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
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        let matches = App::new("stocks")
            .version("0.0.1")
            .author("James Leahy. <jamesleahy314@gmail.com>")
            .about("A CLI application to veiw information about stocks")
            .subcommand(
                SubCommand::with_name("add")
                    .about("add a stock to the db")
                    .arg(
                        Arg::with_name("symbol")
                            .index(1)
                            .required(true)
                            .help("the symbol that you want to add"),
                    )
                    .arg(
                        Arg::with_name("num_stocks")
                            .short("num")
                            .takes_value(true)
                            .requires("symbol")
                            .help("flag to pass the number of stocks for a given symbol")
                            .long("num_stocks")

                        ),
            )
            .get_matches();

        if let Some(c) = matches.value_of("config") {
            println!("Value for config: {}", c);
        }

        // You can check for the existence of subcommands, and if found use their
        // matches just as you would the top level app
        if let Some(matches) = matches.subcommand_matches("add") {

            if let Some(val) = matches.value_of("symbol") {
                println!("{}", val);

                if let Some(num) = matches.value_of("num_stocks") {
                    println!("{}", num);
                } 
            }


        }

        let command = Command::from_str(args).unwrap_or_else(|err| {
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
        Command::InsertStock { arg: _ } => {
            db::execute(config, Queries::InsertStock)?;
        }
        Command::RemoveStock { arg: _ } => {
            db::execute(config, Queries::RemoveStock)?;
        }
        Command::GetStocks => {
            db::execute(config, Queries::GetStocks)?;
        }
    }

    Ok(())
}
