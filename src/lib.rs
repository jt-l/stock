use std::error::Error;
use std::env;
use std::process;


// valid commands
pub enum Command {
    AddStock,
    RemoveStock,
    ListStocks,
    AddProfile,
    RemoveProfile,
    ListProfiles
}

trait FromStr {
    fn from_str(s: &str) -> Result<Command, (&'static str)>;
}

// FromStr is used to parse command line arg into enum
impl FromStr for Command {

    fn from_str(s: &str) -> Result<Command, (&'static str)> {
        match s {
            "add_stock" => Ok(Command::AddStock),
            "remove_stock" => Ok(Command::RemoveStock),
            "list_stocks" => Ok(Command::ListStocks),
            "add_profile" => Ok(Command::AddProfile),
            "remove_profile" => Ok(Command::RemoveProfile),
            "list_profiles" => Ok(Command::ListProfiles),
            _ => Err("Invalid command"),
        }
    }
}

pub struct Config {
    pub command: Command, 
    pub profile: String,
    pub alpha_vantage_key: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let command = Command::from_str(&args[1]).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);   
            process::exit(1);
        });

        let profile = args[2].clone();

        let alpha_vantage_key = env::var("ALPHA_VANTAGE_API_KEY").unwrap_or_else(|err| {
            eprintln!("ALPHA_VANTAGE_API_KEY is not set: {}", err);   
            process::exit(1);
        });

        Ok(Config {command, profile, alpha_vantage_key})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    match config.command {
        Command::AddStock => println!("AddStock"),
        Command::RemoveStock => println!("RemoveStock"), 
        Command::ListStocks => println!("ListStocks"),
        Command::AddProfile => println!("AddProfile"),
        Command::RemoveProfile => println!("RemoveProfile"), 
        Command::ListProfiles => println!("ListProfiles"),
    }

    Ok(())
}
