use crate::app::FileExplorerApplication;
use crate::entry::FileEntry;
use crate::types::{SortBy, SortOrder};
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;
use std::cell::OnceCell;
use std::cell::RefCell;
use std::path::{Path, PathBuf};

// ─────────────────────────────────────────────
//  BuiltUi
// ─────────────────────────────────────────────
struct BuiltUi {
    root: gtk::Box,
    search: gtk::SearchEntry,
    btn_back: gtk::Button,
    btn_forward: gtk::Button,
    btn_up: gtk::Button,
    btn_home: gtk::Button,
    btn_refresh: gtk::Button,
    btn_search: gtk::Button,
    side_panel: gtk::Box,
    file_list: gtk::ListBox,
    paned: gtk::Paned,
}

// ─────────────────────────────────────────────
//  imp
// ─────────────────────────────────────────────
mod imp {
    use super::*;

    #[derive(Default)]
    pub struct FileExplorerWindowImp {
        pub search: OnceCell<gtk::SearchEntry>,
        pub btn_back: OnceCell<gtk::Button>,
        pub btn_forward: OnceCell<gtk::Button>,
        pub btn_up: OnceCell<gtk::Button>,
        pub btn_home: OnceCell<gtk::Button>,
        pub btn_refresh: OnceCell<gtk::Button>,
        pub btn_search: OnceCell<gtk::Button>,
        pub side_panel: OnceCell<gtk::Box>,
        pub file_list: OnceCell<gtk::ListBox>,
        pub paned: OnceCell<gtk::Paned>,
        // Navigation history
        pub history: RefCell<Vec<PathBuf>>,
        pub history_pos: RefCell<usize>,
        // Currently loaded directory
        pub current_dir: RefCell<PathBuf>,
        pub show_hidden_files: RefCell<bool>,

        //Sort mode
        pub sort_by: RefCell<SortBy>,
        pub sort_order: RefCell<SortOrder>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FileExplorerWindowImp {
        const NAME: &'static str = "FileExplorerWindow";
        type Type = super::FileExplorerWindow;
        type ParentType = gtk::ApplicationWindow;
    }

    impl ObjectImpl for FileExplorerWindowImp {
        fn constructed(&self) {
            self.parent_constructed();
            let window = self.obj();
            window.set_title(Some("File Manager"));
            window.set_default_width(900);
            window.set_default_height(550);

            let built = super::FileExplorerWindow::build_ui();

            self.search.set(built.search).unwrap();
            self.btn_back.set(built.btn_back).unwrap();
            self.btn_forward.set(built.btn_forward).unwrap();
            self.btn_up.set(built.btn_up).unwrap();
            self.btn_home.set(built.btn_home).unwrap();
            self.btn_refresh.set(built.btn_refresh).unwrap();
            self.btn_search.set(built.btn_search).unwrap();
            self.side_panel.set(built.side_panel).unwrap();
            self.file_list.set(built.file_list).unwrap();
            self.paned.set(built.paned).unwrap();

            window.set_child(Some(&built.root));

            // Connect all signals after widgets are initialized
            window.connect_signals();
        }
    }

    impl WidgetImpl for FileExplorerWindowImp {}
    impl WindowImpl for FileExplorerWindowImp {}
    impl ApplicationWindowImpl for FileExplorerWindowImp {}
}

// ─────────────────────────────────────────────
//  Wrapper
// ─────────────────────────────────────────────
glib::wrapper! {
    pub struct FileExplorerWindow(ObjectSubclass<imp::FileExplorerWindowImp>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
                    gtk::Native, gtk::Root, gtk::ShortcutManager,
                    gio::ActionGroup, gio::ActionMap;
}

impl FileExplorerWindow {
    pub fn new(app: &FileExplorerApplication) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn imp(&self) -> &imp::FileExplorerWindowImp {
        imp::FileExplorerWindowImp::from_obj(self)
    }

    // ─────────────────────────────────────────
    //  Signal connections
    // ─────────────────────────────────────────

    fn connect_signals(&self) {
        self.connect_navbar_signals();
        self.connect_filelist_signals();
    }

    // Connect all navbar button and search signals
    fn connect_navbar_signals(&self) {
        let imp = self.imp();

        // Back button: go to previous directory in history
        {
            let win = self.clone();
            imp.btn_back.get().unwrap().connect_clicked(move |_| {
                win.navigate_back();
            });
        }

        // Forward button: go to next directory in history
        {
            let win = self.clone();
            imp.btn_forward.get().unwrap().connect_clicked(move |_| {
                win.navigate_forward();
            });
        }

        // Up button: go to parent directory
        {
            let win = self.clone();
            imp.btn_up.get().unwrap().connect_clicked(move |_| {
                win.navigate_up();
            });
        }

        // Home button: go to user home directory
        {
            let win = self.clone();
            imp.btn_home.get().unwrap().connect_clicked(move |_| {
                let home = std::env::var("HOME").unwrap_or("/".to_string());
                win.navigate_to(&PathBuf::from(home));
            });
        }

        // Refresh button: reload current directory
        {
            let win = self.clone();
            imp.btn_refresh.get().unwrap().connect_clicked(move |_| {
                win.refresh();
            });
        }

        // Search entry: trigger search on Enter key
        {
            let win = self.clone();
            imp.search.get().unwrap().connect_activate(move |entry| {
                let text = entry.text();
                win.on_search_activate(text.as_str());
            });
        }

        // Search button: trigger search on click
        {
            let win = self.clone();
            imp.btn_search.get().unwrap().connect_clicked(move |_| {
                let text = win.imp().search.get().unwrap().text();
                win.on_search_activate(text.as_str());
            });
        }
    }

    // Connect file list signals
    fn connect_filelist_signals(&self) {
        let imp = self.imp();

        // Row activated: open file or navigate into folder on double-click or Enter
        {
            let win = self.clone();
            imp.file_list
                .get()
                .unwrap()
                .connect_row_activated(move |_, row| {
                    win.on_row_activated(row);
                });
        }

        // Selection changed: track how many items are selected
        {
            imp.file_list
                .get()
                .unwrap()
                .connect_selected_rows_changed(move |list| {
                    let count = list.selected_rows().len();
                    println!("{} item(s) selected", count);
                });
        }
    }

    // ─────────────────────────────────────────
    //  Navigation
    // ─────────────────────────────────────────

    // Navigate to a new path and record it in history
    pub fn navigate_to(&self, path: &PathBuf) {
        let imp = self.imp();
        let mut history = imp.history.borrow_mut();
        let mut pos = imp.history_pos.borrow_mut();

        if let Some(current) = history.get(*pos)
            && current == path
        {
            return;
        }

        if history.is_empty() {
            // First navigation: just push, pos stays at 0
            history.push(path.clone());
            *pos = 0;
        } else {
            // Discard any forward history when navigating to a new path
            history.truncate(*pos + 1);
            history.push(path.clone());
            *pos = history.len() - 1;
        }

        drop(history);
        drop(pos);

        self.load_directory(path);
        self.update_nav_buttons();
    }

    //Get the current dir
    pub fn current_dir(&self) -> PathBuf {
        self.imp().current_dir.borrow().clone()
    }

    // Go back one step in history
    fn navigate_back(&self) {
        let imp = self.imp();
        let mut pos = imp.history_pos.borrow_mut();

        if *pos == 0 {
            return; // Already at the oldest entry
        }

        *pos -= 1;
        let path = imp.history.borrow()[*pos].clone();
        drop(pos);

        self.load_directory(&path);
        self.update_nav_buttons();
    }

    // Go forward one step in history
    fn navigate_forward(&self) {
        let imp = self.imp();
        let mut pos = imp.history_pos.borrow_mut();
        let history_len = imp.history.borrow().len();

        if *pos + 1 >= history_len {
            return; // Already at the newest entry
        }

        *pos += 1;
        let path = imp.history.borrow()[*pos].clone();
        drop(pos);

        self.load_directory(&path);
        self.update_nav_buttons();
    }

    // Navigate to the parent of the current directory
    fn navigate_up(&self) {
        let current = self.imp().current_dir.borrow().clone();
        if let Some(parent) = current.parent() {
            self.navigate_to(&parent.to_path_buf());
        }
    }

    // Reload the current directory without modifying history
    pub fn refresh(&self) {
        let current = self.imp().current_dir.borrow().clone();
        self.load_directory(&current);
    }

    // Enable or disable back/forward buttons based on history position
    fn update_nav_buttons(&self) {
        let imp = self.imp();
        let pos = *imp.history_pos.borrow();
        let history_len = imp.history.borrow().len();

        imp.btn_back.get().unwrap().set_sensitive(pos > 0);
        imp.btn_forward
            .get()
            .unwrap()
            .set_sensitive(pos + 1 < history_len);
    }

    // ─────────────────────────────────────────
    //  Event handlers
    // ─────────────────────────────────────────

    // Handle row activation: navigate into folder or open file
    fn on_row_activated(&self, row: &gtk::ListBoxRow) {
        let index = row.index() as usize;
        let current = self.imp().current_dir.borrow().clone();
        let entries = FileEntry::list_directory(&current);

        if let Some(entry) = entries.get(index) {
            if entry.is_dir {
                // Navigate into the selected directory
                self.navigate_to(&entry.path.clone());
            } else {
                // Placeholder: open file with default application
                println!("Open file: {}", entry.path.display());
            }
        }
    }

    // Handle search: navigate if text is a valid directory path
    fn on_search_activate(&self, text: &str) {
        if text.is_empty() {
            return;
        }

        let path = PathBuf::from(text);
        if path.is_dir() {
            // If the typed text is a directory path, navigate there
            self.navigate_to(&path);
        } else {
            println!("Search: {}", text);
        }
    }

    // ─────────────────────────────────────────
    //  Public methods
    // ─────────────────────────────────────────

    // Clear the file list and populate it with the contents of path
    pub fn load_directory(&self, path: &Path) {
        let file_list = self.imp().file_list.get().unwrap();

        // Remove all existing rows
        while let Some(row) = file_list.row_at_index(0) {
            file_list.remove(&row);
        }

        // Build one row per entry
        let mut entries: Vec<FileEntry> = FileEntry::list_directory(path)
            .into_iter()
            .filter(|e| *self.imp().show_hidden_files.borrow() || !e.name.starts_with('.'))
            .collect();

        let sort_by = *self.imp().sort_by.borrow();
        let sort_order = *self.imp().sort_order.borrow();

        //
        entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => {
                let ord = match sort_by {
                    SortBy::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                    SortBy::Size => a.size.cmp(&b.size),
                    SortBy::Type => a
                        .kind_display()
                        .to_lowercase()
                        .cmp(&b.kind_display().to_lowercase()),
                    SortBy::Date => a.modified.cmp(&b.modified),
                };

                match sort_order {
                    SortOrder::Ascending => ord,
                    SortOrder::Descending => ord.reverse(),
                }
            }
        });
        for entry in &entries {
            let size = entry.size_display();
            let kind = entry.kind_display();
            let date = entry.date_display();

            file_list.append(&Self::build_file_row(
                entry.icon_name(),
                &entry.name,
                &size,
                &kind,
                &date,
            ));
        }

        // Update title bar and current directory tracker
        self.set_title(Some(&path.to_string_lossy()));
        *self.imp().current_dir.borrow_mut() = path.to_path_buf();
    }

    pub fn set_search_text(&self, text: &str) {
        self.imp().search.get().unwrap().set_text(text);
    }

    pub fn set_sidebar_width(&self, width: i32) {
        self.imp().paned.get().unwrap().set_position(width);
    }

    pub fn set_back_enabled(&self, enabled: bool) {
        self.imp().btn_back.get().unwrap().set_sensitive(enabled);
    }

    pub fn set_forward_enabled(&self, enabled: bool) {
        self.imp().btn_forward.get().unwrap().set_sensitive(enabled);
    }

    // ─────────────────────────────────────────
    //  UI Builders
    // ─────────────────────────────────────────

    fn build_menu_model() -> gio::Menu {
        let menubar = gio::Menu::new();

        // Files menu
        let files = gio::Menu::new();
        files.append(Some("New Window"), Some("app.new-window"));
        files.append(Some("Open Terminal Here"), Some("app.open-terminal"));
        files.append(Some("Close Window"), Some("app.close-window"));
        menubar.append_submenu(Some("Files"), &files);

        // Edit menu
        let edit = gio::Menu::new();
        edit.append(Some("Copy"), Some("app.copy"));
        edit.append(Some("Cut"), Some("app.cut"));
        edit.append(Some("Paste"), Some("app.paste"));
        edit.append(Some("Select All"), Some("app.select-all"));
        menubar.append_submenu(Some("Edit"), &edit);

        // View menu
        let view = gio::Menu::new();

        // Section 1: refresh and hidden files toggle
        let view_s1 = gio::Menu::new();
        view_s1.append(Some("Refresh"), Some("app.refresh"));
        view_s1.append(Some("Show Hidden Files"), Some("app.show-hidden"));
        view.append_section(None, &view_s1);

        // Section 2: sort submenu
        let view_s2 = gio::Menu::new();
        let sort_menu = gio::Menu::new();

        let sort_menu_s1 = gio::Menu::new();

        //Section 1 : Sort by field
        sort_menu_s1.append(Some("Name"), Some("app.sort-name"));
        sort_menu_s1.append(Some("Size"), Some("app.sort-size"));
        sort_menu_s1.append(Some("Type"), Some("app.sort-type"));
        sort_menu_s1.append(Some("Date Modified"), Some("app.sort-date"));
        sort_menu.append_section(None, &sort_menu_s1);

        //Section 2: Sort order display
        let sort_menu_s2 = gio::Menu::new();
        sort_menu_s2.append(Some("Ascending"), Some("app.order-ascending"));
        sort_menu_s2.append(Some("Descending"), Some("app.order-descending"));

        sort_menu.append_section(None, &sort_menu_s2);

        view_s2.append_submenu(Some("Sort By"), &sort_menu);
        view.append_section(None, &view_s2);

        // Section 3: view mode options
        let view_s3 = gio::Menu::new();
        view_s3.append(Some("Grid View"), Some("app.grid-view"));
        view_s3.append(Some("List View"), Some("app.list-view"));
        view_s3.append(Some("Compact View"), Some("app.compact-view"));
        view.append_section(None, &view_s3);

        menubar.append_submenu(Some("View"), &view);

        // Go menu
        let go = gio::Menu::new();

        // Section 1: navigation actions
        let go_s1 = gio::Menu::new();
        go_s1.append(Some("Open Parent Directory"), Some("app.go-parent"));
        go_s1.append(Some("Back"), Some("app.go-back"));
        go_s1.append(Some("Forward"), Some("app.go-forward"));
        go.append_section(None, &go_s1);

        // Section 2: user directories
        let go_s2 = gio::Menu::new();
        go_s2.append(Some("Personal Directory"), Some("app.go-personal"));
        go_s2.append(Some("Desktop"), Some("app.go-desktop"));
        go_s2.append(Some("Templates"), Some("app.go-templates"));
        go.append_section(None, &go_s2);

        // Section 3: system locations
        let go_s3 = gio::Menu::new();
        go_s3.append(Some("File System"), Some("app.go-file-system"));
        go.append_section(None, &go_s3);

        menubar.append_submenu(Some("Go"), &go);

        // Bookmarks menu
        let bookmarks = gio::Menu::new();
        bookmarks.append(Some("Add Bookmark"), Some("app.add-bookmark"));
        bookmarks.append(Some("Edit Bookmarks"), Some("app.edit-bookmarks"));
        menubar.append_submenu(Some("Bookmarks"), &bookmarks);

        // Help menu
        let help = gio::Menu::new();
        help.append(Some("Index"), Some("app.help-index"));
        help.append(Some("About"), Some("app.help-about"));
        menubar.append_submenu(Some("Help"), &help);

        menubar
    }

    fn build_navbar() -> (
        gtk::Box,
        gtk::SearchEntry,
        gtk::Button,
        gtk::Button,
        gtk::Button,
        gtk::Button,
        gtk::Button,
        gtk::Button,
    ) {
        let bar = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        bar.set_margin_top(4);
        bar.set_margin_bottom(4);
        bar.set_margin_start(6);
        bar.set_margin_end(6);

        let btn_back = gtk::Button::from_icon_name("go-previous-symbolic");
        let btn_forward = gtk::Button::from_icon_name("go-next-symbolic");
        let btn_up = gtk::Button::from_icon_name("go-up-symbolic");
        let btn_home = gtk::Button::from_icon_name("go-home-symbolic");
        let btn_refresh = gtk::Button::from_icon_name("view-refresh-symbolic");
        let btn_search = gtk::Button::from_icon_name("system-search-symbolic");

        bar.append(&btn_back);
        bar.append(&btn_forward);
        bar.append(&btn_up);
        bar.append(&btn_home);
        bar.append(&btn_refresh);

        let search = gtk::SearchEntry::new();
        search.set_hexpand(true);
        bar.append(&search);
        bar.append(&btn_search);

        (
            bar,
            search,
            btn_back,
            btn_forward,
            btn_up,
            btn_home,
            btn_refresh,
            btn_search,
        )
    }

    fn sidebar_label(text: &str) -> gtk::Label {
        let label = gtk::Label::new(Some(text));
        label.set_xalign(0.0);
        label.set_margin_top(10);
        label.set_margin_bottom(2);
        label.set_margin_start(8);
        label.add_css_class("caption");
        label.add_css_class("dim-label");
        label
    }

    fn sidebar_button(icon: &str, label: &str) -> gtk::Button {
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        hbox.set_margin_start(4);
        let image = gtk::Image::from_icon_name(icon);
        let text = gtk::Label::new(Some(label));
        text.set_xalign(0.0);
        text.set_hexpand(true);
        hbox.append(&image);
        hbox.append(&text);

        let btn = gtk::Button::new();
        btn.set_child(Some(&hbox));
        btn.add_css_class("flat");
        btn.set_hexpand(true);
        btn
    }

    fn build_side_panel() -> gtk::Box {
        let side_box = gtk::Box::new(gtk::Orientation::Vertical, 2);
        side_box.set_margin_top(4);
        side_box.set_margin_start(4);
        side_box.set_margin_end(4);

        side_box.append(&Self::sidebar_label("Places"));
        side_box.append(&Self::sidebar_button("go-home-symbolic", "Home"));
        side_box.append(&Self::sidebar_button("folder-symbolic", "Documents"));
        side_box.append(&Self::sidebar_button(
            "folder-download-symbolic",
            "Downloads",
        ));
        side_box.append(&Self::sidebar_button(
            "folder-pictures-symbolic",
            "Pictures",
        ));
        side_box.append(&Self::sidebar_button("folder-music-symbolic", "Music"));
        side_box.append(&Self::sidebar_button("folder-videos-symbolic", "Videos"));

        side_box.append(&Self::sidebar_label("Devices"));
        side_box.append(&Self::sidebar_button(
            "drive-harddisk-symbolic",
            "File System",
        ));

        side_box
    }

    fn build_column_header() -> gtk::Box {
        let header = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        header.add_css_class("toolbar");

        let name_btn = gtk::Button::with_label("Name");
        name_btn.add_css_class("flat");
        name_btn.set_hexpand(true);
        header.append(&name_btn);

        for col in ["Size", "Type", "Date Modified"] {
            let btn = gtk::Button::with_label(col);
            btn.add_css_class("flat");
            btn.set_width_request(140);
            header.append(&btn);
        }
        header
    }

    fn build_file_row(
        icon: &str,
        name: &str,
        size: &str,
        kind: &str,
        date: &str,
    ) -> gtk::ListBoxRow {
        let row = gtk::ListBoxRow::new();
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        // Name column with icon (expands to fill available space)
        let name_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        name_box.set_hexpand(true);
        name_box.set_margin_start(6);
        name_box.append(&gtk::Image::from_icon_name(icon));
        let name_lbl = gtk::Label::new(Some(name));
        name_lbl.set_xalign(0.0);
        name_lbl.set_hexpand(true);
        name_box.append(&name_lbl);
        hbox.append(&name_box);

        // Fixed-width columns for size, type and date
        for text in [size, kind, date] {
            let lbl = gtk::Label::new(Some(text));
            lbl.set_xalign(0.0);
            lbl.set_width_request(140);
            lbl.add_css_class("dim-label");
            hbox.append(&lbl);
        }

        row.set_child(Some(&hbox));
        row
    }

    fn build_content_pane() -> (gtk::Box, gtk::ListBox) {
        let content_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        content_box.set_hexpand(true);
        content_box.set_vexpand(true);

        content_box.append(&Self::build_column_header());
        content_box.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

        let file_list = gtk::ListBox::new();
        file_list.set_selection_mode(gtk::SelectionMode::Multiple);
        file_list.add_css_class("rich-list");
        file_list.set_vexpand(true);
        file_list.set_hexpand(true);
        file_list.set_activate_on_single_click(false);

        let scroll = gtk::ScrolledWindow::new();
        scroll.set_child(Some(&file_list));
        scroll.set_vexpand(true);

        let gesture = gtk::GestureClick::new();
        gesture.connect_pressed({
            let _list = file_list.clone();
            move |gesture, _, _, y| {
                if let Some(widget) = gesture.widget()
                    && let Some(listbox) = widget.downcast_ref::<gtk::ListBox>()
                {
                    // ¿El click fue sobre una fila?
                    if listbox.row_at_y(y as i32).is_none() {
                        listbox.unselect_all();
                    }
                }
            }
        });
        file_list.add_controller(gesture);

        content_box.append(&scroll);

        (content_box, file_list)
    }

    fn build_ui() -> BuiltUi {
        let (navbar, search, btn_back, btn_forward, btn_up, btn_home, btn_refresh, btn_search) =
            Self::build_navbar();

        let toolbar = gtk::PopoverMenuBar::from_model(Some(&Self::build_menu_model()));
        let side_panel = Self::build_side_panel();
        let (content_pane, file_list) = Self::build_content_pane();

        let sidebar_scroll = gtk::ScrolledWindow::new();
        sidebar_scroll.set_child(Some(&side_panel));
        sidebar_scroll.set_vexpand(true);

        let paned = gtk::Paned::new(gtk::Orientation::Horizontal);
        paned.set_position(200);
        paned.set_vexpand(true);
        paned.set_start_child(Some(&sidebar_scroll));
        paned.set_end_child(Some(&content_pane));
        paned.set_resize_start_child(false);
        paned.set_resize_end_child(true);

        let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
        root.append(&toolbar);
        root.append(&navbar);
        root.append(&paned);

        BuiltUi {
            root,
            search,
            btn_back,
            btn_forward,
            btn_up,
            btn_home,
            btn_refresh,
            btn_search,
            side_panel,
            file_list,
            paned,
        }
    }
}
