extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

// enum of available queries
pub enum Queries {
    GetProfiles,
    GetStocks,
    InsertProfile,
    InsertStock,
}

// Profile
struct Profile {
    id: i32,
    name: String,
}

// Stock
struct Stock {
    id i32,
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

pub fn execute(connection: &Connection, query: Queries,) -> impl Future<Item = Vec<WeatherAgg>, Error = AWError> {
    // duplicate the pool
    let conn = pool.clone();

    // match and execute a query
    // web::block executes a blocking function on a thread pool, returns a future that resolves to the result of the function execution
    // pool.get() gets a connection from the connection pool, it will wait at most for the configured connection timeout before returning an error
    web::block(move || match query {
        Queries::GetTopTenHottestYears => get_hottest_years(pool.get()?),
        Queries::GetTopTenColdestYears => get_coldest_years(pool.get()?),
        Queries::GetTopTenHottestMonths => get_hottest_months(pool.get()?),
        Queries::GetTopTenColdestMonths => get_coldest_months(pool.get()?),
    })
    .from_err()
}

fn get_profiles(conn: Connection) -> Result<Vec<WeatherAgg>, Error> {
    let stmt = "
    SELECT cast(strftime('%Y', date) as int) as theyear,
            sum(tmax) as total
        FROM nyc_weather
        WHERE tmax <> 'TMAX'
        GROUP BY theyear
        ORDER BY total DESC LIMIT 10;";

    let mut prep_stmt = conn.prepare(stmt)?;
    let annuals = prep_stmt
        .query_map(NO_PARAMS, |row| WeatherAgg::AnnualAgg {
            year: row.get(0),
            total: row.get(1),
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<WeatherAgg>>())
        })?;

    sleep(Duration::from_secs(2));

    Ok(annuals)
}

