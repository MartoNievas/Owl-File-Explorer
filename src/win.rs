use crate::app::OwlApplication;
use crate::widgets::navbar::OwlNavBar;
use crate::widgets::side_panel::OwlSidePanel;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(file = "../data/window.ui")]
    pub struct OwlWindow {
        #[template_child]
        pub navbar: TemplateChild<OwlNavBar>,
        #[template_child]
        pub side_panel: TemplateChild<OwlSidePanel>,
        #[template_child]
        pub content_container: TemplateChild<gtk::Box>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for OwlWindow {
        const NAME: &'static str = "OwlWindow";
        type Type = super::OwlWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            OwlNavBar::ensure_type();
            OwlSidePanel::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for OwlWindow {
        fn constructed(&self) {
            self.parent_constructed();
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
        &self.imp().navbar
    }

    pub fn side_panel(&self) -> &OwlSidePanel {
        &self.imp().side_panel
    }

    pub fn content_container(&self) -> &gtk::Box {
        &self.imp().content_container
    }
}
