extern crate config as cfglib;

mod app;
mod components;

use app::App;

fn main() {
    let app = App::init();
    let a = app.clone();
    ctrlc::set_handler(move || a.stop(1)).expect("Error setting Ctrl-C handler");
    app.start();
    app.stop(0);
}
