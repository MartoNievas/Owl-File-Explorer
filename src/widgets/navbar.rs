use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;
use std::path::PathBuf;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(file = "../../data/navbar.ui")]
    pub struct OwlNavBar {
        #[template_child]
        pub search: TemplateChild<gtk::Entry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for OwlNavBar {
        const NAME: &'static str = "OwlNavBar";
        type Type = super::OwlNavBar;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for OwlNavBar {}
    impl WidgetImpl for OwlNavBar {}
    impl BoxImpl for OwlNavBar {}
}

glib::wrapper! {
    pub struct OwlNavBar(ObjectSubclass<imp::OwlNavBar>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
                    gtk::Orientable;
}

impl OwlNavBar {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn connect_search<F: Fn(&str) + 'static>(&self, f: F) {
        self.imp().search.connect_activate(move |entry| {
            f(entry.text().as_str());
        });
    }

    pub fn set_path(&self, path: &PathBuf) {
        self.imp().search.set_text(&path.to_string_lossy());
    }
}
