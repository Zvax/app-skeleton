use serde::Deserialize;
use tinyfiledialogs::{MessageBoxIcon, message_box_ok};
use rusqlite::ErrorCode::ConstraintViolation;
use rusqlite::Error::SqliteFailure;

use Cmd::*;
use crate::{view, model};
use rusqlite::Error;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
#[derive(Debug)]
pub enum Cmd {
    LoadLogs {},
    SaveLog { msg: String },
    ErrorNotif { msg: String },
}

pub fn execute_command(webview: &mut web_view::WebView<()>, conn: &rusqlite::Connection, command: Cmd) {
    println!("we are handling {:?}", command);
    match command {
        LoadLogs {} => {
            load_logs(conn, webview);
        }

        SaveLog { msg } => {
            model::insert(conn, msg);
        }

        ErrorNotif { msg } => {
            message_box_ok("Error", &msg, MessageBoxIcon::Error);
        }
    }
}

fn load_logs(conn: &rusqlite::Connection, webview: &mut web_view::WebView<()>) {
    let logs = model::get_logs(conn);
    view::load_data(webview, Box::new(logs));
}
