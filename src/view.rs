use crate::model::{Logs, Displayable};

pub fn load_data(
    webview: &mut web_view::WebView<()>,
    data: Box<dyn Displayable>
) {
    let eval_str = data.get_setter_string();
    match webview.eval(&eval_str) {
        Ok(()) => {
            //println!("the eval command did not return an error");
        }
        Err(err) => {
            println!("error while executing js code: {}", err.to_string());
        }
    };
}
