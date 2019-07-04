extern crate rusqlite;

use std::env;

use rusqlite::types::ToSql;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

use crate::Config;

use crate::Command;

use crate::api;

use crate::formatter;

// enum of available queries
pub enum Queries {
    GetStocks,
    InsertStock,
    RemoveStock,
    CreateTables,
}

// Stock
#[derive(Debug)]
struct Stock {
    id: i32,
    symbol: String,
}

// execute a given query given a db connection
pub fn execute(config: Config, query: Queries) -> Result<()> {
    match env::current_exe() {
        Ok(exe_path) => {
            // convert path to a str
            match exe_path.to_str() {
                Some(path) => {
                    // build path to db
                    let db_extension = ".db";
                    let mut db_path = path.to_string();
                    db_path.push_str(db_extension);

                    // open db connection
                    let conn = Connection::open(db_path)?;

                    match query {
                        Queries::InsertStock => {
                            if let Command::InsertStock { arg } = config.command {
                                insert_stock(conn, &arg)?;
                                println!("{} successfully added", &arg);
                            }
                        }
                        Queries::RemoveStock => {
                            if let Command::RemoveStock { arg } = config.command {
                                remove_stock(conn, &arg)?;
                                println!("{} successfully removed", &arg);
                            }
                        }
                        Queries::GetStocks => {
                            if let Command::GetStocks = config.command {
                                formatter::print(get_stocks(conn, config.api_key)?);
                            }
                        }
                        Queries::CreateTables => {
                            create_tables(conn)?;
                        }
                    }

                    Ok(())
                }

                // panic if exe_path fails to convert to str
                None => panic!(),
            }
        }
        // panic if env::current_exe() fails
        _ => panic!(),
    }
}

pub fn create_tables(conn: Connection) -> Result<()> {
    // stocks table
    conn.execute(
        "create table if not exists STOCKS (
             id integer primary key,
             symbol text unique not null
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

fn insert_stock<'a>(conn: Connection, symbol: &'a String) -> Result<()> {
    conn.execute(
        "INSERT INTO STOCKS (symbol)
            VALUES (?1)",
        &[symbol as &ToSql],
    )?;

    Ok(())
}

fn remove_stock<'a>(conn: Connection, symbol: &str) -> Result<()> {
    conn.execute("DELETE FROM STOCKS WHERE symbol = (?1)", &[&symbol])?;

    Ok(())
}

fn get_stocks(conn: Connection, api_key: String) -> Result<std::vec::Vec<api::Response>> {
    let mut stmt = conn.prepare("SELECT * from STOCKS;")?;

    let stocks = stmt.query_map(NO_PARAMS, |row| {
        Ok(Stock {
            id: row.get(0)?,
            symbol: row.get(1)?,
        })
    })?;

    let mut responses = vec![];

    for stock in stocks {
        let symbol = stock.unwrap().symbol;
        responses.push(api::get_stock(symbol, &api_key));
    }

    Ok(responses)
}
