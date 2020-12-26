use serde::{Serialize, Deserialize};
use rusqlite::{Error, Connection, params};

pub trait Displayable {
    fn get_setter_string(&self) -> String;
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Log {
    pub id: i32,
    pub msg: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Logs {
    data: Vec<Log>
}

pub struct Numbers {
    pub data: Vec<i32>,
}

impl Displayable for Logs {
    fn get_setter_string(&self) -> String {
        format!("set_logs({})", serde_json::json!(&self.data))
    }
}

impl Displayable for Numbers {
    fn get_setter_string(&self) -> String {
        format!("set_numbers({})", serde_json::json!(&self.data))
    }
}

pub fn insert(conn: &Connection, msg: String) -> Result<usize, Error> {
    let mut stmt = conn.prepare_cached("insert into `logs` (`msg`) values (?)")?;
    stmt.execute(&[msg])
}

pub fn get_logs(conn: &Connection) -> Logs {
    let mut items = vec![];
    let mut stmt = conn.prepare_cached("select `id`, `msg` from `logs`").unwrap();
    let rows = stmt.query(params![]).unwrap();
    let iter = rows.mapped(|row| {
        let id = row.get(0).unwrap();
        let msg = row.get(1).unwrap();
        Ok(Log { id, msg })
    });
    for entry in iter {
        items.push(entry.unwrap())
    }
    Logs {
        data: items
    }
}
