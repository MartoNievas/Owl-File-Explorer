use crate::win::FileExplorerWindow;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;
use std::path::PathBuf;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct FileExplorerApplicationImp;

    #[glib::object_subclass]
    impl ObjectSubclass for FileExplorerApplicationImp {
        const NAME: &'static str = "FileExplorerApplication";
        type Type = super::FileExplorerApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for FileExplorerApplicationImp {}

    impl ApplicationImpl for FileExplorerApplicationImp {
        fn activate(&self) {
            self.parent_activate();
            let app = self.obj();
            let window = FileExplorerWindow::new(
                app.downcast_ref::<super::FileExplorerApplication>()
                    .unwrap(),
            );

            // Use navigate_to so the initial directory is recorded in history
            let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
            window.navigate_to(&PathBuf::from(home));

            window.present();
        }
    }

    impl GtkApplicationImpl for FileExplorerApplicationImp {}
}

glib::wrapper! {
    pub struct FileExplorerApplication(ObjectSubclass<imp::FileExplorerApplicationImp>)
        @extends gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl FileExplorerApplication {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", "com.example.FileUI")
            .build()
    }
}
