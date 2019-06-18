extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use rusqlite::types::ToSql;

use crate::Config;

// enum of available queries
pub enum Queries {
    GetProfiles,
    GetStocks,
    InsertProfile,
    InsertStock,
    RemoveProfile,
    RemoveStock,
}

// Profile
#[derive(Debug)]
struct Profile {
    id: i32,
    name: String,
}

// Stock
#[derive(Debug)]
struct Stock {
    id: i32,
    profile: String,
    symbol: String
}

pub fn create_tables() -> Result<()> {
    let conn = Connection::open("stocks.db")?;

    // Profiles table
    conn.execute(
        "create table if not exists profiles (
             id integer primary key,
             name text not null unique
         )",
        NO_PARAMS,
    )?;

    // stocks table
    conn.execute(
        "create table if not exists stocks (
             id integer primary key,
             name text not null
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

// execute a given query given a db connection
pub fn execute(config: Config, query: Queries) -> Result<()> {

    let conn = Connection::open("stocks.db")?;

    match query {
        Queries::InsertProfile => insert_profile(conn, config.profile),
        Queries::RemoveProfile => insert_profile(conn, config.profile),
        Queries::GetProfiles   => get_profiles(conn),
        Queries::InsertStock   => insert_profile(conn, config.profile),
        Queries::RemoveStock   => insert_profile(conn, config.profile),
        Queries::GetStocks     => insert_profile(conn, config.profile),
     }


}

// insert a new profile
fn insert_profile(conn: Connection, name: String) -> Result<()> {

    let profile = Profile {
        id: 0,
        name: name,
    };

    conn.execute(
        "INSERT INTO PROFILES (name)
            VALUES (?1)",
        &[&profile.name as &ToSql],
    )?;

    Ok(())
}

// get all profiles
fn get_profiles(conn: Connection) -> Result<()> {

    let mut stmt = conn
        .prepare("SELECT * from PROFILES;")?;

    let profiles = stmt
        .query_map(NO_PARAMS, |row| Ok(Profile {
            id: row.get(0)?,
            name: row.get(1)?,
        }))?;

    for profile in profiles {
        println!("Found profile {:?}", profile.unwrap());
    }

    Ok(())
}

