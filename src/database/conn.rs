use bcrypt::hash;
use dotenv::dotenv;
use rusqlite::{Connection, Result};
use std::env;

/// Starts the database using sqlite
pub fn start_database() -> Result<()> {
    dotenv().ok();

    // Creates a new file for the connection
    let conn = Connection::open("db/congregation_noticeboard_database.db")?;

    // Create the user table
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id  VARCHAR PRIMARY KEY,
            name VARCHAR NOT NULL,
            password VARCHAR NOT NULL,
            congregation VARCHAR NOT NULL
        )
    ",
        [],
    )?;

    // For pdf names
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS storage (
            congregation VARCHAR,
            img_name VARCHAR NOT NULL,
            type VARCHAR NOT NULL
        )
        ",
        [],
    )?;

    // Get passwords for the congregation accounts from .env file
    let moorhouse_pass: String =
        env::var("MOORHOUSE_PASSWORD").expect("Could not get env variable for Moorhouse password.");
    let scotby_pass: String =
        env::var("SCOTBY_PASSWORD").expect("Could not get env variable for Scotby password.");
    let brampton_pass: String =
        env::var("BRAMPTON_PASSWORD").expect("Could not get env variable for Brampton password.");

    // Hash password
    let moorhouse_pass = hash(moorhouse_pass, 10).unwrap();
    let scotby_pass = hash(scotby_pass, 10).unwrap();
    let brampton_pass = hash(brampton_pass, 10).unwrap();

    // Convert to &str cause Connection does not take String value
    let moorhouse_pass = &moorhouse_pass;
    let scotby_pass = &scotby_pass;
    let brampton_pass = &brampton_pass;

    // Add congregation accounts
    conn.execute(
        "
        INSERT OR IGNORE INTO users (id, name, password, congregation) VALUES (?, ?, ?, ?)
    ",
        ["1", "Moorhouse", moorhouse_pass, "moorhouse"],
    )?;
    conn.execute(
        "
        INSERT OR IGNORE INTO users (id, name, password, congregation) VALUES (?, ?, ?, ?)
    ",
        ["2", "Scotby", scotby_pass, "scotby"],
    )?;
    conn.execute(
        "
        INSERT OR IGNORE INTO users (id, name, password, congregation) VALUES (?, ?, ?, ?)
    ",
        ["3", "Brampton", brampton_pass, "brampton"],
    )?;

    println!("Database started.");

    Ok(())
}

/// Returns database connection  
pub fn conn_to_database() -> Result<Connection> {
    let conn = Connection::open("db/congregation_noticeboard_database.db")?;

    Ok(conn)
}
