use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(file = "../../data/navbar.ui")]
    pub struct OwlNavBar {
        #[template_child]
        pub btn_back: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_forward: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_up: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_home: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_refresh: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_search: TemplateChild<gtk::Button>,
        #[template_child]
        pub search: TemplateChild<gtk::SearchEntry>,
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

    // Métodos públicos para conectar señales desde la ventana
    pub fn connect_back<F: Fn() + 'static>(&self, f: F) {
        self.imp().btn_back.connect_clicked(move |_| f());
    }

    pub fn connect_forward<F: Fn() + 'static>(&self, f: F) {
        self.imp().btn_forward.connect_clicked(move |_| f());
    }

    pub fn connect_up<F: Fn() + 'static>(&self, f: F) {
        self.imp().btn_up.connect_clicked(move |_| f());
    }

    pub fn connect_home<F: Fn() + 'static>(&self, f: F) {
        self.imp().btn_home.connect_clicked(move |_| f());
    }

    pub fn connect_refresh<F: Fn() + 'static>(&self, f: F) {
        self.imp().btn_refresh.connect_clicked(move |_| f());
    }

    pub fn connect_search<F: Fn(&str) + 'static>(&self, f: F) {
        self.imp().search.connect_activate(move |entry| {
            f(entry.text().as_str());
        });
    }

    pub fn set_back_enabled(&self, v: bool) {
        self.imp().btn_back.set_sensitive(v);
    }

    pub fn set_forward_enabled(&self, v: bool) {
        self.imp().btn_forward.set_sensitive(v);
    }
}
