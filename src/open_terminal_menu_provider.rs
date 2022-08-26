use crate::dconf_default_terminal;
use crate::terminal_command::{Alacritty, AlacrittyWayland, GnomeConsole, TerminalCommand};
use glib_sys::gpointer;
use gobject_sys::GObject;
use gtk_sys::GtkWidget;

use nautilus_extension::nautilus_menu_background_activate_cb;
use nautilus_extension::{FileInfo, MenuItem, MenuProvider};
use std::string::FromUtf8Error;
use std::thread;

pub struct OpenTerminalMenuProvider {
    menu_item: MenuItem,
}

impl OpenTerminalMenuProvider {
    pub fn new() -> OpenTerminalMenuProvider {
        let mut item = MenuItem::new(
            "OpenInTerminalExtension::Open",
            "Open in terminal",
            "open in terminal",
            None,
        );

        item.set_activate_cb(activate_cb);

        OpenTerminalMenuProvider { menu_item: item }
    }
}
nautilus_menu_background_activate_cb!(activate_cb, on_open_terminal);

impl MenuProvider for OpenTerminalMenuProvider {
    fn get_file_items(&self, _window: *mut GtkWidget, _files: &[FileInfo]) -> Vec<MenuItem> {
        vec![]
    }

    fn get_background_items(
        &self,
        _window: *mut GtkWidget,
        _current_folder: &FileInfo,
    ) -> Vec<MenuItem> {
        vec![self.menu_item.clone()]
    }
}

fn find_terminal(terminal_path: &String) -> Box<dyn TerminalCommand> {
    if terminal_path.ends_with("kgx") {
        return Box::new(GnomeConsole {});
    }
    if terminal_path.ends_with("alacritty_wayland") {
        return Box::new(AlacrittyWayland);
    }

    if terminal_path.ends_with("alacritty") {
        return Box::new(Alacritty);
    }

    Box::new(GnomeConsole {})
}

fn uri_to_path(uri: String) -> Result<String, FromUtf8Error> {
    let uri_decoded = urlencoding::decode(uri.as_str())?;

    Ok(uri_decoded.replace("file:///", "/"))
}

fn spawn_new_process(terminal_command: String, path: String) {
    thread::spawn(move || {
        println!(
            "executing terminal:{} path:{}",
            terminal_command,
            path.clone()
        );

        let command = find_terminal(&terminal_command);

        match command.spawn(terminal_command, path) {
            Ok(mut child) => {
                let _ = child.wait();
            }
            Err(error) => {
                println!("Unable to execute command {:?}", error);
            }
        }
    });
}

fn on_open_terminal(file: FileInfo) {
    let default_terminal_dconf = dconf_default_terminal::get_default_terminal();

    match default_terminal_dconf {
        Ok(terminal_command) => {
            let path = match uri_to_path(file.get_uri()) {
                Ok(path) => path,
                Err(utf8_error) => {
                    println!(
                        "Unable to parse to uf8 {} cstring: {:?}",
                        file.get_uri(),
                        utf8_error
                    );
                    return;
                }
            };

            spawn_new_process(terminal_command, path);
        }
        Err(dconf_error) => {
            println!("Error on get dconf {}", dconf_error);
        }
    }
}

#[test]
fn test_uri_to_path() {
    let uris = vec![
        "file:///Repositories/random/Andador-Robotico-Inteligente/Pe%D0%97as%20Individuais",
        "file:///Repositories/random/Andador-Robotico-Inteligente/",
    ];

    let expected_paths = vec![
        "/Repositories/random/Andador-Robotico-Inteligente/PeЗas Individuais",
        "/Repositories/random/Andador-Robotico-Inteligente/",
    ];

    for (uri, expected_path) in uris.iter().zip(expected_paths) {
        let path = uri_to_path(uri.to_string()).unwrap();
        assert_eq!(path, expected_path)
    }
}
