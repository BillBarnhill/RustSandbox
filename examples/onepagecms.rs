use log::info;

use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

extern crate env_logger;
extern crate rusqlite;

/*
fn create_db_mem() -> Result<Connection> {
    let connection = Connection::open(":memory:")?;

    connection.execute("CREATE TABLE users (name TEXT, age INTEGER)", NO_PARAMS)?;
    connection.execute("INSERT INTO users VALUES ('Alice', 42)", NO_PARAMS)?;
    connection.execute("INSERT INTO users VALUES ('Bob', 69)", NO_PARAMS)?;

    Ok(connection)
}
 */

fn list_tables(_conn: Connection) -> Vec<String> {
    let mut vec = Vec::new();
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master 
                      WHERE type IN ('table','view') 
                      AND name NOT LIKE 'sqlite_%'
                      ORDER BY 1",
    )?;
    let tables = stmt.query_map(NO_PARAMS, |row| Ok(row.get(0)?));
    for table in tables {
        vec.push(table)
    }
    vec
}

fn create_db_file() -> Result<Connection> {
    let connection = Connection::open("users.db")?;

    connection.execute("CREATE TABLE users (name TEXT, age INTEGER)", NO_PARAMS)?;
    connection.execute("INSERT INTO users VALUES ('Alice', 42)", NO_PARAMS)?;
    connection.execute("INSERT INTO users VALUES ('Bob', 69)", NO_PARAMS)?;

    Ok(connection)
}

fn main() -> Result<(), String> {
    env_logger::init();

    info!("Hello");

    match create_db_file() {
        Ok(_connection) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}
