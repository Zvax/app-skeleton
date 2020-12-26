//! At first I used Diesel and I was having problems when trying to get the framework to generate
//! aggregated types or one/many-to-many relationships. So I've just copied the migration up <version>
//! api because I like, but all of this is shaky. Everything must be done manually and
//! you gotta remember which version you're using and run subsequently various up or down commands with
//! the right version numbers.

use std::env;
use std::fs;
use std::process::exit;
use std::path::Path;
use exitcode::*;
use rusqlite::{NO_PARAMS, Connection};
use std::env::Args;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let conn = rusqlite::Connection::open("db.sqlite").unwrap();
    let command = args[1].as_str();
    match command {
        "run" => {
            if args.len() < 3 {
                println!("you need to specify which version to run");
                exit(USAGE);
            }
            let version = &args[2];
            let migration_file_name = format!("src/sql/{}_up.sql", version);
            println!("opening sqlite database at {}", migration_file_name);
            if !Path::new(&migration_file_name).exists() {
                println!("the version {} does not seem to have a corresponding migration file", version);
                exit(NOINPUT);
            }
            match conn.execute_batch(fs::read_to_string(migration_file_name).unwrap().as_str()) {
                Ok(_) => println!("successfully migrated version {}", version),
                Err(err) => println!("{}", err.to_string()),
            }
        },
        "down" => {
            down(&conn, args);
        },
        _ => {
            println!("unknown command {}", command);
        },
    }
}

fn down(conn: &Connection, args: Vec<String>) {
    if args.len() < 3 {
        println!("you need to specify which version to run");
        exit(USAGE);
    }
    let version = &args[2];
    let migration_file_name = format!("sql/db_schema_version/version_{}_down.sql", version);
    if !Path::new(&migration_file_name).exists() {
        println!("the version {} does not seem to have a corresponding migration file", version);
        exit(NOINPUT);
    }
    match conn.execute_batch(fs::read_to_string(migration_file_name).unwrap().as_str()) {
        Ok(_) => println!("successfully downgraded version {}", version),
        Err(err) => println!("{}", err.to_string()),
    }
}
