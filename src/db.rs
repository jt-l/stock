extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use rusqlite::types::ToSql;

use crate::Config;

use crate::Command;

use crate::api;

use crate::formatter;

// enum of available queries
pub enum Queries {
    GetStocks,
    InsertStock,
    RemoveStock,
}

// Stock
#[derive(Debug)]
struct Stock {
    id: i32,
    symbol: String,
}

pub fn create_tables() -> Result<()> {
    let conn = Connection::open("stocks.db")?;

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

// execute a given query given a db connection
pub fn execute(config: Config, query: Queries) -> Result<()> {

    let conn = Connection::open("stocks.db")?;

    match query {
        Queries::InsertStock   => {
            if let Command::InsertStock{arg} = config.command {
                insert_stock(conn, &arg)?;
                println!("{} successfully added", &arg);
            }
        },
        Queries::RemoveStock   => {
            if let Command::RemoveStock{arg} = config.command {
                remove_stock(conn, &arg)?;
                println!("{} successfully removed", &arg);
            }
        },
        Queries::GetStocks     => {
            if let Command::GetStocks = config.command {
                formatter::print(get_stocks(conn, config.alpha_vantage_key)?);
            }
        },
     }

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

fn remove_stock<'a>(conn: Connection, symbol: &'a String) -> Result<()> {

    conn.execute(
        "DELETE FROM STOCKS WHERE symbol = (?1)",
        &[&symbol],
    )?;

    Ok(())
}

fn get_stocks(conn: Connection, alpha_vantage_key: String) -> Result<std::vec::Vec<api::Response>> {

    let mut stmt = conn
        .prepare("SELECT * from STOCKS;")?;

    let stocks = stmt
        .query_map(NO_PARAMS, |row| Ok(Stock {
            id: row.get(0)?,
            symbol: row.get(1)?,
        }))?;

    let mut responses = vec![];

    for stock in stocks {
        let symbol = stock.unwrap().symbol;
        responses.push(api::get_stock(symbol, &alpha_vantage_key));
    }

    Ok(responses)
}

