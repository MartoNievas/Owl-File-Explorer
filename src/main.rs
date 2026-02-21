mod app;
mod entry;
mod win;

use app::FileExplorerApplication;
use gtk4 as gtk;

fn main() -> gtk::glib::ExitCode {
    let app = FileExplorerApplication::new();
    gtk::prelude::ApplicationExtManual::run(&app)
}
