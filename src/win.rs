use crate::app::OwlApplication;
use crate::types::{SortBy, SortOrder, ViewMode};
use crate::widgets::content_panel::OwlContentPanel;
use crate::widgets::navbar::OwlNavBar;
use crate::widgets::side_panel::OwlSidePanel;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;
use std::cell::RefCell;
use std::path::PathBuf;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(file = "../data/window.ui")]
    pub struct OwlWindow {
        #[template_child]
        pub content_panel: TemplateChild<OwlContentPanel>,
        #[template_child]
        pub navbar: TemplateChild<OwlNavBar>,
        #[template_child]
        pub side_panel: TemplateChild<OwlSidePanel>,
        #[template_child]
        pub content_container: TemplateChild<gtk::Box>,

        pub current_path: RefCell<PathBuf>,
        pub forward_stack: RefCell<Vec<PathBuf>>,
        pub history: RefCell<Vec<PathBuf>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for OwlWindow {
        const NAME: &'static str = "OwlWindow";
        type Type = super::OwlWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            OwlNavBar::ensure_type();
            OwlSidePanel::ensure_type();
            OwlContentPanel::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for OwlWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_actions();
            obj.setup_signals();

            let current_dir = std::env::current_dir().unwrap_or_else(|_| {
                PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/".to_string()))
            });
            obj.navigate_to(current_dir, false);
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

    pub fn navigate_to(&self, path: PathBuf, push_history: bool) {
        let imp = self.imp();
        let current = imp.current_path.borrow().clone();

        if push_history && current != PathBuf::new() && current != path {
            imp.history.borrow_mut().push(current);
            imp.forward_stack.borrow_mut().clear();
        }

        *imp.current_path.borrow_mut() = path.clone();
        imp.navbar.set_path(&path);
        imp.content_panel.load_directory(&path);
        self.update_nav_actions();
    }

    fn update_nav_actions(&self) {
        let imp = self.imp();
        let can_back = !imp.history.borrow().is_empty();
        let can_forward = !imp.forward_stack.borrow().is_empty();

        if let Some(a) = self.lookup_action("go-back") {
            a.downcast::<gio::SimpleAction>()
                .unwrap()
                .set_enabled(can_back);
        }
        if let Some(a) = self.lookup_action("go-forward") {
            a.downcast::<gio::SimpleAction>()
                .unwrap()
                .set_enabled(can_forward);
        }
    }

    fn setup_signals(&self) {
        let imp = self.imp();

        imp.navbar.connect_search(glib::clone!(
            #[weak(rename_to = win)]
            self,
            move |text| {
                let path = PathBuf::from(text);
                if path.is_dir() {
                    win.navigate_to(path, true);
                }
            }
        ));

        Self::connect_list_navigation(&imp.content_panel.imp().file_list, self);
        Self::connect_list_navigation(&imp.content_panel.imp().compact_list, self);
        Self::connect_grid_navigation(&imp.content_panel.imp().flow_box, self);
    }

    fn connect_list_navigation(list_box: &gtk::ListBox, win: &OwlWindow) {
        list_box.connect_row_activated(glib::clone!(
            #[weak]
            win,
            move |_, row| {
                let entries = win.imp().content_panel.imp().entries.borrow();
                if let Some(entry) = entries.get(row.index() as usize) {
                    if entry.is_dir {
                        let path = entry.path.clone();
                        drop(entries);
                        win.navigate_to(path, true);
                    }
                }
            }
        ));
    }

    fn connect_grid_navigation(flow_box: &gtk::FlowBox, win: &OwlWindow) {
        flow_box.connect_child_activated(glib::clone!(
            #[weak]
            win,
            move |_, child| {
                let entries = win.imp().content_panel.imp().entries.borrow();
                if let Some(entry) = entries.get(child.index() as usize) {
                    if entry.is_dir {
                        let path = entry.path.clone();
                        drop(entries);
                        win.navigate_to(path, true);
                    }
                }
            }
        ));
    }

    pub fn setup_actions(&self) {
        self.add_action_entries([
            gio::ActionEntry::builder("go-back")
                .activate(|win: &OwlWindow, _, _| {
                    let imp = win.imp();
                    let prev = { imp.history.borrow_mut().pop() };
                    if let Some(path) = prev {
                        let current = { imp.current_path.borrow().clone() };
                        {
                            imp.forward_stack.borrow_mut().push(current);
                        }
                        win.navigate_to(path, false);
                    }
                })
                .build(),
            gio::ActionEntry::builder("go-forward")
                .activate(|win: &OwlWindow, _, _| {
                    let imp = win.imp();
                    let next = { imp.forward_stack.borrow_mut().pop() };
                    if let Some(path) = next {
                        let current = { imp.current_path.borrow().clone() };
                        {
                            imp.history.borrow_mut().push(current);
                        }
                        win.navigate_to(path, false);
                    }
                })
                .build(),
            gio::ActionEntry::builder("go-parent")
                .activate(|win: &OwlWindow, _, _| {
                    let current = win.imp().current_path.borrow().clone();
                    if let Some(parent) = current.parent() {
                        win.navigate_to(parent.to_path_buf(), true);
                    }
                })
                .build(),
            gio::ActionEntry::builder("home")
                .activate(|win: &OwlWindow, _, _| {
                    win.navigate_to(Self::home_dir(), true);
                })
                .build(),
            gio::ActionEntry::builder("go-root")
                .activate(|win: &OwlWindow, _, _| {
                    win.navigate_to(PathBuf::from("/"), true);
                })
                .build(),
            gio::ActionEntry::builder("go-desktop")
                .activate(|win: &OwlWindow, _, _| {
                    win.navigate_to(Self::home_dir().join("Escritorio"), true);
                })
                .build(),
            gio::ActionEntry::builder("go-templates")
                .activate(|win: &OwlWindow, _, _| {
                    win.navigate_to(Self::home_dir().join("Plantillas"), true);
                })
                .build(),
            gio::ActionEntry::builder("refresh")
                .activate(|win: &OwlWindow, _, _| {
                    let current = win.imp().current_path.borrow().clone();
                    win.navigate_to(current, false);
                })
                .build(),
            gio::ActionEntry::builder("navigate")
                .parameter_type(Some(glib::VariantTy::STRING))
                .activate(|win: &OwlWindow, _, param| {
                    if let Some(s) = param.and_then(|p| p.get::<String>()) {
                        let path = PathBuf::from(s);
                        if path.exists() {
                            win.navigate_to(path, true);
                        }
                    }
                })
                .build(),
            gio::ActionEntry::builder("new-window")
                .activate(|win: &OwlWindow, _, _| {
                    if let Some(app) = win.application() {
                        OwlWindow::new(app.downcast_ref::<OwlApplication>().unwrap()).present();
                    }
                })
                .build(),
            gio::ActionEntry::builder("open-terminal")
                .activate(|win: &OwlWindow, _, _| {
                    let current = win.imp().current_path.borrow().clone();
                    let _ = std::process::Command::new("st")
                        .current_dir(current)
                        .spawn();
                })
                .build(),
            gio::ActionEntry::builder("close-window")
                .activate(|win: &OwlWindow, _, _| win.close())
                .build(),
            gio::ActionEntry::builder("sort")
                .parameter_type(Some(glib::VariantTy::STRING))
                .state("name".to_variant())
                .activate(|win: &OwlWindow, action, param| {
                    if let Some(s) = param.and_then(|p| p.get::<String>()) {
                        action.set_state(&s.to_variant());
                        let sort_by = match s.as_str() {
                            "size" => SortBy::Size,
                            "type" => SortBy::Type,
                            "date" => SortBy::Date,
                            _ => SortBy::Name,
                        };
                        win.imp().content_panel.set_sort_menu(sort_by);
                    }
                })
                .build(),
            gio::ActionEntry::builder("order")
                .parameter_type(Some(glib::VariantTy::STRING))
                .state("ascending".to_variant())
                .activate(|win: &OwlWindow, action, param| {
                    if let Some(s) = param.and_then(|p| p.get::<String>()) {
                        action.set_state(&s.to_variant());
                        let order = match s.as_str() {
                            "descending" => SortOrder::Descending,
                            _ => SortOrder::Ascending,
                        };
                        win.imp().content_panel.set_order(order);
                    }
                })
                .build(),
            gio::ActionEntry::builder("view")
                .parameter_type(Some(glib::VariantTy::STRING))
                .state("'list'".to_variant())
                .activate(|win: &OwlWindow, action, param| {
                    println!("view action fired, param: {:?}", param);
                    if let Some(s) = param.and_then(|p| p.get::<String>()) {
                        println!("view mode: {}", s);
                        action.set_state(&s.to_variant());
                        let mode = match s.as_str() {
                            "'grid'" => ViewMode::Grid,
                            "'compact'" => ViewMode::Compact,
                            _ => ViewMode::List,
                        };
                        win.imp().content_panel.set_view_mode(mode);
                    }
                })
                .build(),
            gio::ActionEntry::builder("show-hidden")
                .state(false.to_variant())
                .activate(|win: &OwlWindow, action, _| {
                    let new_state = !action.state().unwrap().get::<bool>().unwrap();
                    action.set_state(&new_state.to_variant());
                    *win.imp().content_panel.imp().show_hidden_files.borrow_mut() = new_state;
                    let current = win.imp().current_path.borrow().clone();
                    win.imp().content_panel.load_directory(&current);
                })
                .build(),
            gio::ActionEntry::builder("sort")
                .parameter_type(Some(glib::VariantTy::STRING))
                .state("name".to_variant())
                .activate(|win: &OwlWindow, action, param| {
                    if let Some(s) = param.and_then(|p| p.get::<String>()) {
                        action.set_state(&s.to_variant());
                        let sort_by = match s.as_str() {
                            "size1" => SortBy::Size,
                            "type1" => SortBy::Type,
                            "date1" => SortBy::Date,
                            _ => SortBy::Name,
                        };
                        win.imp().content_panel.set_sort(sort_by);
                    }
                })
                .build(),
        ]);
    }

    fn home_dir() -> PathBuf {
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/".to_string()))
    }
}
