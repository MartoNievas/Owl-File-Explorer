use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;
use std::path::PathBuf;

pub struct Bookmark {
    pub path: PathBuf,
    pub name: String,
}

impl Bookmark {
    pub fn new(path: PathBuf, name: &str) -> Self {
        Self {
            path,
            name: name.to_string(),
        }
    }

    pub fn from_path(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .unwrap_or_else(|| path.as_os_str())
            .to_string_lossy()
            .to_string();
        Self { path, name }
    }
}

mod imp {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(file = "../../data/side_panel.ui")]
    pub struct OwlSidePanel {
        #[template_child]
        pub places_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub devices_box: TemplateChild<gtk::Box>,
        pub bookmarks: RefCell<Vec<Bookmark>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for OwlSidePanel {
        const NAME: &'static str = "OwlSidePanel";
        type Type = super::OwlSidePanel;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for OwlSidePanel {
        fn constructed(&self) {
            self.parent_constructed();
            let panel = self.obj();
            panel.populate_places();
            panel.populate_devices();
            panel.reload_bookmarks();
        }
    }

    impl WidgetImpl for OwlSidePanel {}
    impl BoxImpl for OwlSidePanel {}
}

glib::wrapper! {
    pub struct OwlSidePanel(ObjectSubclass<imp::OwlSidePanel>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
                    gtk::Orientable;
}

impl OwlSidePanel {
    pub fn new() -> Self {
        // Solo esto — el resto va en constructed()
        glib::Object::new()
    }

    fn populate_places(&self) {
        let home = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/".to_string()));

        let places: &[(&str, &str, PathBuf)] = &[
            ("go-home-symbolic", "Home", home.clone()),
            ("folder-symbolic", "Documents", home.join("Documents")),
            (
                "folder-download-symbolic",
                "Downloads",
                home.join("Downloads"),
            ),
            (
                "folder-pictures-symbolic",
                "Pictures",
                home.join("Pictures"),
            ),
            ("folder-music-symbolic", "Music", home.join("Music")),
            ("folder-videos-symbolic", "Videos", home.join("Videos")),
        ];

        for (icon, name, path) in places {
            self.imp()
                .places_box
                .append(&Self::make_button(icon, name, path));
        }
    }

    fn populate_devices(&self) {
        self.imp().devices_box.append(&Self::make_button(
            "drive-harddisk-symbolic",
            "File System",
            &PathBuf::from("/"),
        ));
    }

    pub fn reload_bookmarks(&self) {
        // Carga Home por defecto siempre
        let home = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/".to_string()));

        let mut bookmarks = vec![Bookmark::from_path(home)];

        // Agrega los del archivo
        bookmarks.extend(Self::load_from_disk());

        *self.imp().bookmarks.borrow_mut() = bookmarks;
    }

    pub fn add_bookmark(&self, path: PathBuf) {
        let bookmark = Bookmark::from_path(path);
        let mut bookmarks = self.imp().bookmarks.borrow_mut();

        // No duplicar
        if bookmarks.iter().any(|b| b.path == bookmark.path) {
            return;
        }

        bookmarks.push(bookmark);
        Self::save_to_disk(&bookmarks);
        drop(bookmarks);
        self.reload_bookmarks();
    }

    pub fn remove_bookmark(&self, path: &PathBuf) {
        let mut bookmarks = self.imp().bookmarks.borrow_mut();
        bookmarks.retain(|b| &b.path != path);
        Self::save_to_disk(&bookmarks);
        drop(bookmarks);
        self.reload_bookmarks();
    }

    fn load_from_disk() -> Vec<Bookmark> {
        let config = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
        let path = PathBuf::from(config).join(".config/gtk-3.0/bookmarks");

        let Ok(content) = std::fs::read_to_string(&path) else {
            return vec![];
        };

        content
            .lines()
            .filter(|l| !l.is_empty())
            .filter_map(|line| {
                let mut parts = line.splitn(2, ' ');
                let url = parts.next()?;
                let path = url.strip_prefix("file://").map(PathBuf::from)?;
                let name = parts.next().map(|s| s.to_string()).unwrap_or_else(|| {
                    path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                });
                Some(Bookmark { path, name })
            })
            .collect()
    }

    fn save_to_disk(bookmarks: &[Bookmark]) {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
        let path = PathBuf::from(home).join(".config/gtk-3.0/bookmarks");

        let content = bookmarks
            .iter()
            .map(|b| format!("file://{} {}", b.path.display(), b.name))
            .collect::<Vec<_>>()
            .join("\n");

        let _ = std::fs::write(path, content);
    }

    fn make_button(icon: &str, name: &str, path: &PathBuf) -> gtk::Button {
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        hbox.set_margin_start(4);

        let image = gtk::Image::from_icon_name(icon);
        let label = gtk::Label::new(Some(name));
        label.set_xalign(0.0);
        label.set_hexpand(true);

        hbox.append(&image);
        hbox.append(&label);

        let btn = gtk::Button::new();
        btn.set_child(Some(&hbox));
        btn.add_css_class("flat");
        btn.set_hexpand(true);

        let path_clone = path.clone();
        btn.connect_clicked(move |_| {
            println!("Navigate to: {}", path_clone.display());
        });

        btn
    }
}
