extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

pub fn create_tables() {
    let conn = Connection::open("stocks.db");
}
