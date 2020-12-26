pub mod service;
pub mod view;
pub mod model;

use web_view::*;
use std::process::{Command, Output, ExitStatus, exit};
use std::io::Result;
use std::fs;
use std::env;
use serde::Serialize;

fn main() {
    /// build the svelte bundle
    /// then create the html content necessary for the web_view

    // TODO the path should not be hardcoded here
    // but I don't know how to dynamically get the current file working directory at compile time as a &str
    match execute_cli_command(
        "cmd.exe",
        &["/c", "npm", "run", "build"],
        Some("E:\\projects\\rust\\app-skeleton\\client"),
    ) {
        Ok(something) => {
            println!("we executed a cli command: {}", something);
        }
        Err(err) => {
            println!("we could not execute a cli command: {:?}", err);
            exit(exitcode::IOERR);
        }
    }

    let html_bundle = make_content();
    let rusqlite_conn = rusqlite::Connection::open("./db.sqlite")
        .expect("couldn't connect to sqlite db");

    match make_webview(html_bundle, &rusqlite_conn) {
        Ok(_) => println!("we could make webview"),
        Err(e) => println!("we couldn't make webview: {}", e.to_string()),
    }
}

fn make_content() -> web_view::Content<String> {
    println!("generating Html Content from built files");
    /// here at runtime it seems we are at crate level for get file as str
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset='utf-8'>
            <meta name='viewport' content='width=device-width,initial-scale=1'>
            {global_css}
            {bundle_css}

        </head>

        <body>
        <script type="application/javascript">
            invoke = (arg) => {{ window.external.invoke(JSON.stringify(arg)); }};
        </script>
        {bundle_js}
        </body>
        </html>
        "#,
        global_css = inline_style(&get_file_as_str("client/src/global.css")),
        bundle_css = inline_style(&get_file_as_str("client/build/bundle.css")),
        bundle_js = inline_script(&get_file_as_str("client/build/bundle.js")),
    );
    Content::Html(html)
}

pub fn get_file_as_str(path: &str) -> String {
    /// I really like to have a file returned to me as a string I don't care for performance leave me alone
    /// also it needs to be used by prepending a & to it I'm not quite sure why like
    /// ```
    /// let file_content = &get_file_as_str("gui/public/global.css");
    /// ```
    fs::read_to_string(path).unwrap()
}

fn execute_cli_command(exe: &str, args: &[&str], working_directory: Option<&str>) -> Result<ExitStatus> {
    match working_directory {
        None => Command::new(exe)
            .args(args)
            .spawn()?
            .wait(),
        Some(str) => Command::new(exe)
            .current_dir(str)
            .args(args)
            .spawn()?
            .wait(),
    }
}

fn make_webview<'a>(content: Content<String>, rusqlite_conn: &rusqlite::Connection) -> WVResult<()> {
    web_view::builder()
        .title("Application Skeleton")
        .size(1200, 800)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match serde_json::from_str(arg) {
                Ok(cmd) => {
                    service::execute_command(webview, &rusqlite_conn, cmd);
                }
                Err(err) => {
                    println!("unknown command: {}", err.to_string())
                }
            }

            Ok(())
        })
        .content(content)
        .run()
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="application/javascript">{}</script>"#, s)
}
