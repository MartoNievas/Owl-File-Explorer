use crate::app::OwlApplication;
use crate::widgets::navbar::OwlNavBar;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;

mod imp {
    use super::*;
    use std::cell::OnceCell;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(file = "../data/window.ui")]
    pub struct OwlWindow {
        #[template_child]
        pub navbar_container: TemplateChild<gtk::Box>,

        // Guardamos referencia al navbar
        pub navbar: OnceCell<OwlNavBar>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for OwlWindow {
        const NAME: &'static str = "OwlWindow";
        type Type = super::OwlWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for OwlWindow {
        fn constructed(&self) {
            self.parent_constructed();

            // Creamos el navbar y lo insertamos en el contenedor
            let navbar = OwlNavBar::new();
            self.navbar_container.append(&navbar);
            self.navbar.set(navbar).unwrap();
        }
    }

    impl WidgetImpl for OwlWindow {}
    impl WindowImpl for OwlWindow {}
    impl ApplicationWindowImpl for OwlWindow {}
}

glib::wrapper! {
    pub struct OwlWindow(ObjectSubclass<imp::OwlWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
                    gtk::Native, gtk::Root, gtk::ShortcutManager,
                    gio::ActionGroup, gio::ActionMap;
}

impl OwlWindow {
    pub fn new(app: &OwlApplication) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn navbar(&self) -> &OwlNavBar {
        self.imp().navbar.get().unwrap()
    }
}
