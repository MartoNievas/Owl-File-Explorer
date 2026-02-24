use crate::entry::FileEntry;
use crate::types::{SortBy, SortOrder, ViewMode};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(file = "../../data/content_panel.ui")]
    pub struct OwlContentPanel {
        #[template_child]
        pub column_header: TemplateChild<gtk::Box>,
        #[template_child]
        pub separator: TemplateChild<gtk::Separator>,
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub file_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub flow_box: TemplateChild<gtk::FlowBox>,

        pub sort_by: RefCell<SortBy>,
        pub sort_order: RefCell<SortOrder>,
        pub view_mode: RefCell<ViewMode>,
        pub entries: RefCell<Vec<FileEntry>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for OwlContentPanel {
        const NAME: &'static str = "OwlContentPanel";
        type Type = super::OwlContentPanel;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for OwlContentPanel {}
    impl WidgetImpl for OwlContentPanel {}
    impl BoxImpl for OwlContentPanel {}
}

glib::wrapper! {
    pub struct OwlContentPanel(ObjectSubclass<imp::OwlContentPanel>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Orientable;
}

impl OwlContentPanel {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
