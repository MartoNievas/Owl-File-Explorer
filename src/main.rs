mod app;
mod entry;
mod types;
mod widgets;
mod win;
use app::OwlApplication;
use gtk::prelude::*;
use gtk4 as gtk;
fn main() -> gtk::glib::ExitCode {
    let app = OwlApplication::new();
    app.run()
}
