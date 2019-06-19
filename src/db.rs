extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use rusqlite::types::ToSql;

use crate::Config;

use crate::Command;

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
    symbol: String
}

pub fn create_tables() -> Result<()> {
    let conn = Connection::open("stocks.db")?;

    // stocks table
    conn.execute(
        "create table if not exists STOCKS (
             id integer primary key,
             symbol text not null
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
                insert_stock(conn, arg);
            }
        },
        Queries::RemoveStock   => {
            if let Command::RemoveStock{arg} = config.command {
                remove_stock(conn, arg);
            }
        },
        Queries::GetStocks     => {
            if let Command::GetStocks = config.command {
                get_stocks(conn);
            }
        },
     }

    Ok(())
}

// insert a new profile
fn insert_stock(conn: Connection, symbol: String) -> Result<()> {

    let stock = Stock {
        id: 0,
        symbol: symbol,
    };

    conn.execute(
        "INSERT INTO STOCKS (symbol)
            VALUES (?1)",
        &[&stock.symbol as &ToSql],
    )?;

    Ok(())
}

fn remove_stock(conn: Connection, symbol: String) -> Result<()> {

    conn.execute(
        "DELETE FROM STOCKS WHERE symbol = (?1)",
        &[&symbol],
    )?;

    Ok(())
}

// get all profiles
fn get_stocks(conn: Connection) -> Result<()> {

    let mut stmt = conn
        .prepare("SELECT * from STOCKS;")?;

    let stocks = stmt
        .query_map(NO_PARAMS, |row| Ok(Stock {
            id: row.get(0)?,
            symbol: row.get(1)?,
        }))?;

    for stock in stocks {
        println!("Found stock {:?}", stock.unwrap());
    }

    Ok(())
}

