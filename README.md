# File Explorer

A desktop file manager for Linux built with Rust and GTK4. It lets you browse your filesystem, navigate between directories, and view file metadata in a familiar two-panel layout similar to classic file managers like Thunar or Nautilus.

## What it does

When you open the app it loads your home directory and shows its contents in a list with four columns: name, size, type and date modified. You can double-click any folder to enter it, use the back and forward buttons to retrace your steps, go up to the parent directory, or jump home with a single click. The left sidebar shows shortcuts to common places like Documents, Downloads and Pictures, and a Devices section for storage locations like the filesystem root. At the top there is a menu bar with the usual Files, Edit, View, Go, Bookmarks and Help menus, and a navigation bar with a search entry that lets you jump directly to any path you type.

## Technologies

### Rust
The entire application is written in Rust. Rust was chosen for its memory safety guarantees, zero-cost abstractions and strong type system, which pair well with the GTK object system and avoid the common pitfalls of C/C++ GTK apps.

### GTK4
GTK4 (GIMP Toolkit 4) is the UI framework. It is the same toolkit used by GNOME applications on Linux. GTK4 provides all the widgets used in the app: windows, buttons, list boxes, paned containers, search entries, popovers and more. It also handles rendering, theming via the Adwaita theme, and the main event loop.

### gtk4-rs
`gtk4-rs` is the official Rust bindings for GTK4. It exposes the GTK4 C library to Rust in a safe and idiomatic way. The crate is published as `gtk4` on crates.io.

### GLib / GIO
GLib is the low-level utility library that GTK is built on. It provides the GObject type system (used for subclassing), signals, main loop and reference counting. GIO is the I/O layer of GLib and provides `gio::Menu`, `gio::SimpleAction` and `gio::Application`, which back the menu bar and application actions.

### GObject subclassing
Rather than using GTK widgets directly, the app defines two custom GObject subclasses:
- `FileExplorerApplication` — extends `gtk::Application`
- `FileExplorerWindow` — extends `gtk::ApplicationWindow`

This is done using the `#[glib::object_subclass]` macro and the `glib::wrapper!` macro from the `gtk4-rs` ecosystem. This pattern is the idiomatic way to build GTK4 apps in Rust and matches how GNOME apps written in C or Vala are structured.

### std::fs
The standard library's filesystem module is used to read directory contents, file metadata (size, type, modification time) and resolve paths. No external crate is needed for filesystem access.

## Architecture overview

```
FileExplorerApplication  (app.rs)
└── activates
    └── FileExplorerWindow  (win.rs)
        ├── PopoverMenuBar    ← menu bar (Files, Edit, View, Go...)
        ├── NavBar            ← back, forward, up, home, refresh, search
        └── Paned
            ├── SidePanel     ← Places and Devices shortcuts
            └── ContentPane
                ├── ColumnHeader  ← Name, Size, Type, Date Modified
                └── FileList      ← one ListBoxRow per file/folder

FileEntry  (entry.rs)
└── reads the filesystem and provides display helpers for each file
```

## Project structure

```
src/
├── main.rs       # Entry point, registers modules and runs the app
├── app.rs        # FileExplorerApplication — subclasses gtk::Application
├── win.rs        # FileExplorerWindow — subclasses gtk::ApplicationWindow
└── entry.rs      # FileEntry struct, list_directory, display helpers
```

## Dependencies

```toml
[dependencies]
gtk4 = { version = "0.9", features = ["v4_10"] }
```

No other external crates are required. All UI, filesystem access and application logic uses GTK4, GLib/GIO and the Rust standard library.

## Build & Run

```bash
# Install GTK4 development libraries (Debian/Ubuntu)
sudo apt install libgtk-4-dev

# Install GTK4 development libraries (Fedora)
sudo dnf install gtk4-devel

# Install GTK4 development libraries (Arch)
sudo pacman -S gtk4

# Build
cargo build

# Run
cargo run
```

## Requirements

- Linux (GTK4 is supported on macOS and Windows but the app targets Linux)
- Rust 1.70 or newer
- GTK 4.10 or newer
