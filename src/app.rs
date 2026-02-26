use crate::win::OwlWindow;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;
mod imp {
    use super::*;

    #[derive(Default)]
    pub struct OwlApplication;

    #[glib::object_subclass]
    impl ObjectSubclass for OwlApplication {
        const NAME: &'static str = "OwlApplication";
        type Type = super::OwlApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for OwlApplication {}

    impl ApplicationImpl for OwlApplication {
        fn activate(&self) {
            let settings = gtk::Settings::default().unwrap();
            settings.set_gtk_theme_name(Some("Adwaita"));

            let app = self.obj();
            let app_ref = app.downcast_ref::<super::OwlApplication>().unwrap();
            let window = OwlWindow::new(app_ref);

            window.present();
        }
    }

    impl GtkApplicationImpl for OwlApplication {}
}

glib::wrapper! {
    pub struct OwlApplication(ObjectSubclass<imp::OwlApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl OwlApplication {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", "com.owl.app")
            .build()
    }
}
