use crate::file_entry::FileEntry;
use crate::types::{SortBy, SortOrder, ViewMode};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;
use std::cell::RefCell;
use std::path::PathBuf;

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
        pub file_list: TemplateChild<gtk::ListBox>, // list with fields view
        #[template_child]
        pub compact_list: TemplateChild<gtk::ListBox>, // compact list without fields view
        #[template_child]
        pub flow_box: TemplateChild<gtk::FlowBox>, // grid view
        pub sort_by: RefCell<SortBy>,
        pub sort_order: RefCell<SortOrder>,
        pub view_mode: RefCell<ViewMode>,
        pub entries: RefCell<Vec<FileEntry>>,

        pub show_hidden_files: RefCell<bool>,
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

    pub fn load_directory(&self, path: &PathBuf) {
        let imp = self.imp();
        let mut entries = FileEntry::list_directory(path);
        if !*imp.show_hidden_files.borrow() {
            entries.retain(|f| *imp.show_hidden_files.borrow() || !f.name.starts_with('.'));
        }

        self.sort_entries(&mut entries);
        *imp.entries.borrow_mut() = entries;
        self.refresh_view();
        self.update_sort_headers();
    }

    fn update_sort_headers(&self) {
        let imp = self.imp();
        let sort_by = imp.sort_by.borrow().clone();
        let sort_order = imp.sort_order.borrow().clone();

        let arrow = match sort_order {
            SortOrder::Ascending => " ↑",
            SortOrder::Descending => " ↓",
        };

        let labels = ["Name", "Size", "Type", "Date"];
        let active = match sort_by {
            SortBy::Name => 0,
            SortBy::Size => 1,
            SortBy::Type => 2,
            SortBy::Date => 3,
        };

        let header = imp.column_header.get();
        let mut child_opt = header.first_child();
        let mut i = 0usize;

        while let Some(widget) = child_opt {
            let next = widget.next_sibling();
            if let Ok(btn) = widget.downcast::<gtk::Button>() {
                if i < labels.len() {
                    if i == active {
                        btn.set_label(&format!("{}{}", labels[i], arrow));
                        btn.add_css_class("accent");
                    } else {
                        btn.set_label(labels[i]);
                        btn.remove_css_class("accent");
                    }
                    i += 1;
                }
            }
            child_opt = next;
        }
    }

    pub fn set_view_mode(&self, mode: ViewMode) {
        *self.imp().view_mode.borrow_mut() = mode;
        self.refresh_view();
    }

    pub fn set_sort_menu(&self, sort_by: SortBy) {
        let imp = self.imp();

        *imp.sort_by.borrow_mut() = sort_by;
        *imp.sort_order.borrow_mut() = SortOrder::Ascending;

        let mut entries = imp.entries.borrow().clone();
        self.sort_entries(&mut entries);
        *imp.entries.borrow_mut() = entries;
        self.refresh_view();
    }

    //Change sort field and refresh view
    pub fn set_sort(&self, sort_by: SortBy) {
        let imp = self.imp();
        let current = imp.sort_by.borrow().clone();
        if current == sort_by {
            let current_order = imp.sort_order.borrow().clone();
            *imp.sort_order.borrow_mut() = match current_order {
                SortOrder::Ascending => SortOrder::Descending,
                SortOrder::Descending => SortOrder::Ascending,
            };
        } else {
            *imp.sort_by.borrow_mut() = sort_by;
            *imp.sort_order.borrow_mut() = SortOrder::Ascending;
        }
        let mut entries = imp.entries.borrow().clone();
        self.sort_entries(&mut entries);
        *imp.entries.borrow_mut() = entries;
        self.refresh_view();
        self.update_sort_headers();
    }

    // Private methods
    fn sort_entries(&self, entries: &mut Vec<FileEntry>) {
        let imp = self.imp();
        let sort_by = imp.sort_by.borrow().clone();
        let sort_order = imp.sort_order.borrow().clone();

        entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => {
                let ord = match sort_by {
                    SortBy::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                    SortBy::Size => a.size.cmp(&b.size),
                    SortBy::Type => a.kind_display().cmp(&b.kind_display()),
                    SortBy::Date => a.modified.cmp(&b.modified),
                };

                match sort_order {
                    SortOrder::Ascending => ord,
                    SortOrder::Descending => ord.reverse(),
                }
            }
        });
    }

    pub fn set_visible_files(&self) {
        let flag = self.imp().show_hidden_files.borrow().clone();
        *self.imp().show_hidden_files.borrow_mut() = !flag;

        self.refresh_view();
    }

    pub fn set_order(&self, sort_order: SortOrder) {
        let imp = self.imp();

        *imp.sort_order.borrow_mut() = sort_order;

        let mut entries = imp.entries.borrow().clone();
        self.sort_entries(&mut entries);
        *imp.entries.borrow_mut() = entries;
        self.refresh_view();
        self.update_sort_headers();
    }

    fn refresh_view(&self) {
        let imp = self.imp();
        let mode = imp.view_mode.borrow().clone();

        match mode {
            ViewMode::List => {
                self.populate_list_view();
                imp.stack.set_visible_child_name("list");
            }
            ViewMode::Grid => {
                self.populate_grid_view();
                imp.stack.set_visible_child_name("grid");
            }
            ViewMode::Compact => {
                self.populate_compact_view();
                imp.stack.set_visible_child_name("compact");
            }
        }
    }

    // Detail list view

    fn populate_list_view(&self) {
        let list_box = self.imp().file_list.get();
        Self::clear_list_box(&list_box);

        for entry in self.imp().entries.borrow().iter() {
            list_box.append(&self.make_list_row(entry));
        }
    }

    fn make_list_row(&self, entry: &FileEntry) -> gtk::ListBoxRow {
        let row = gtk::ListBoxRow::new();
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        hbox.set_margin_top(4);
        hbox.set_margin_bottom(4);
        hbox.set_margin_start(6);
        hbox.set_margin_end(6);

        // Icon
        let icon = gtk::Image::from_icon_name(entry.icon_name());
        icon.set_margin_end(8);
        hbox.append(&icon);

        // Name
        let name = gtk::Label::new(Some(&entry.name));
        name.set_halign(gtk::Align::Start);
        name.set_hexpand(true);
        name.set_ellipsize(gtk::pango::EllipsizeMode::End);
        hbox.append(&name);

        // Size
        let size = gtk::Label::new(Some(&entry.size_display()));
        size.set_halign(gtk::Align::Start);
        size.set_width_request(80);
        size.add_css_class("dim-label");
        hbox.append(&size);

        // Type
        let kind = gtk::Label::new(Some(&entry.kind_display()));
        kind.set_halign(gtk::Align::Start);
        kind.set_width_request(100);
        kind.add_css_class("dim-label");
        hbox.append(&kind);

        // Date
        let date = gtk::Label::new(Some(&entry.date_display()));
        date.set_halign(gtk::Align::Start);
        date.set_width_request(150);
        date.add_css_class("dim-label");
        hbox.append(&date);

        row.set_child(Some(&hbox));
        row
    }

    // Grid view

    fn populate_grid_view(&self) {
        let flow_box = self.imp().flow_box.get();

        while let Some(child) = flow_box.first_child() {
            flow_box.remove(&child);
        }

        for entry in self.imp().entries.borrow().iter() {
            let item = self.make_grid_item(entry);
            flow_box.insert(&item, -1);

            // Ajustar el FlowBoxChild wrapper que GTK crea automáticamente
            if let Some(child) = item.parent() {
                child.set_valign(gtk::Align::Start);
                child.set_vexpand(false);
            }
        }
    }

    fn make_grid_item(&self, entry: &FileEntry) -> gtk::Box {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 4);
        vbox.set_halign(gtk::Align::Center);
        vbox.set_valign(gtk::Align::Start); // clave
        vbox.set_vexpand(false); // clave
        vbox.set_width_request(90);
        vbox.set_margin_start(6);
        vbox.set_margin_end(6);
        vbox.set_margin_top(4);
        vbox.set_margin_bottom(4);

        let icon = gtk::Image::from_icon_name(entry.icon_name());
        icon.set_pixel_size(48);
        icon.set_halign(gtk::Align::Center);
        icon.set_valign(gtk::Align::Center);

        let name = gtk::Label::new(Some(&entry.name));
        name.set_halign(gtk::Align::Center);
        name.set_valign(gtk::Align::Start);
        name.set_max_width_chars(10);
        name.set_ellipsize(gtk::pango::EllipsizeMode::End);
        name.set_wrap(true);
        name.set_lines(2);
        name.set_justify(gtk::Justification::Center);

        vbox.append(&icon);
        vbox.append(&name);
        vbox
    }

    // Compact view

    fn populate_compact_view(&self) {
        let compact_list = self.imp().compact_list.get();

        Self::clear_list_box(&compact_list);
        for entry in self.imp().entries.borrow().iter() {
            compact_list.append(&self.make_compact_row(entry));
        }
    }

    fn make_compact_row(&self, entry: &FileEntry) -> gtk::ListBoxRow {
        let row = gtk::ListBoxRow::new();
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        hbox.set_margin_top(2);
        hbox.set_margin_bottom(2);
        hbox.set_margin_start(6);

        let icon = gtk::Image::from_icon_name(entry.icon_name());
        let name = gtk::Label::new(Some(&entry.name));
        name.set_halign(gtk::Align::Start);
        name.set_ellipsize(gtk::pango::EllipsizeMode::End);

        hbox.append(&icon);
        hbox.append(&name);
        row.set_child(Some(&hbox));
        row
    }

    // Utilities

    fn clear_list_box(list_box: &gtk::ListBox) {
        while let Some(child) = list_box.first_child() {
            list_box.remove(&child);
        }
    }
}
