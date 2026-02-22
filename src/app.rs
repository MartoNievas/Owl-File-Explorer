use crate::types::{SortBy, SortOrder};
use crate::win::FileExplorerWindow;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk4 as gtk;
use std::cell::RefCell;
use std::path::PathBuf;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct FileExplorerApplicationImp;

    #[glib::object_subclass]
    impl ObjectSubclass for FileExplorerApplicationImp {
        const NAME: &'static str = "FileExplorerApplication";
        type Type = super::FileExplorerApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for FileExplorerApplicationImp {}

    impl ApplicationImpl for FileExplorerApplicationImp {
        fn activate(&self) {
            self.parent_activate();
            let app = self.obj();
            let window = FileExplorerWindow::new(
                app.downcast_ref::<super::FileExplorerApplication>()
                    .unwrap(),
            );

            // Use navigate_to so the initial directory is recorded in history
            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
            window.navigate_to(&current_dir);

            // Register all actions before presenting the window
            super::FileExplorerApplication::register_actions(&app, &window);

            window.present();
        }
    }

    impl GtkApplicationImpl for FileExplorerApplicationImp {}
}

glib::wrapper! {
    pub struct FileExplorerApplication(ObjectSubclass<imp::FileExplorerApplicationImp>)
        @extends gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl FileExplorerApplication {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", "com.example.FileUI")
            .build()
    }

    fn action_handler<F>(&self, window: &FileExplorerWindow, name: &str, build: F)
    where
        F: Fn(
            FileExplorerApplication,
            FileExplorerWindow,
        ) -> Box<dyn Fn(&gio::SimpleAction, Option<&glib::Variant>) + 'static>,
    {
        let action = gio::SimpleAction::new(name, None);
        let handler = build(self.clone(), window.clone());
        action.connect_activate(handler);
        self.add_action(&action);
    }

    fn register_actions(&self, window: &FileExplorerWindow) {
        //New window handler
        self.action_handler(window, "new-window", |app, _win| {
            Box::new(move |_, _| {
                let new_win = FileExplorerWindow::new(&app);
                let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
                new_win.navigate_to(&PathBuf::from(home));
                new_win.present();
            })
        });

        self.action_handler(window, "open-terminal", |_app, win| {
            Box::new(move |_, _| {
                let path = win.current_dir();
                std::process::Command::new("st")
                    .current_dir(&path)
                    .spawn()
                    .unwrap_or_else(|_| {
                        std::process::Command::new("bash")
                            .spawn()
                            .expect("Could not open terminal")
                    });
            })
        });

        self.action_handler(window, "close-window", |_app, win| {
            Box::new(move |_, _| {
                win.close();
            })
        });

        self.action_handler(window, "refresh", |_app, win| {
            Box::new(move |_, _| {
                win.refresh();
            })
        });

        self.action_handler(window, "show-hidden", |_app, win| {
            Box::new(move |_, _| {
                {
                    let imp = win.imp();
                    let mut flag = imp.show_hidden_files.borrow_mut();
                    *flag = !*flag;
                } // <- borrow termina acá

                win.refresh();
            })
        });

        self.action_handler(window, "sort-name", |_app, win| {
            Box::new(move |_, _| {
                println!("Sort by names ");
                let imp = win.imp();
                *imp.sort_by.borrow_mut() = SortBy::Name;

                win.refresh();
            })
        });

        self.action_handler(window, "sort-size", |_, win| {
            Box::new(move |_, _| {
                let imp = win.imp();
                *imp.sort_by.borrow_mut() = SortBy::Size;

                win.refresh();
            })
        });

        self.action_handler(window, "sort-type", |_, win| {
            Box::new(move |_, _| {
                let imp = win.imp();
                *imp.sort_by.borrow_mut() = SortBy::Type;

                win.refresh();
            })
        });

        self.action_handler(window, "sort-date", |_, win| {
            Box::new(move |_, _| {
                let imp = win.imp();
                *imp.sort_by.borrow_mut() = SortBy::Date;

                win.refresh();
            })
        });

        self.action_handler(window, "order-ascending", |_, win| {
            Box::new(move |_, _| {
                let imp = win.imp();
                *imp.sort_order.borrow_mut() = SortOrder::Ascending;

                win.refresh();
            })
        });

        self.action_handler(window, "order-descending", |_, win| {
            Box::new(move |_, _| {
                let imp = win.imp();
                *imp.sort_order.borrow_mut() = SortOrder::Descending;

                win.refresh();
            })
        });
    }
}
