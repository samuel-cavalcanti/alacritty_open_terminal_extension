use glib_sys::gpointer;
use gobject_sys::GObject;
use gtk_sys::GtkWidget;
use nautilus_extension::nautilus_menu_background_activate_cb;
use nautilus_extension::{FileInfo, MenuItem, MenuProvider};
use std::thread;

use crate::dconf_default_terminal;
use crate::terminal_command::{AlacrittyWayland, GnomeConsole, TerminalCommand, Alacritty};

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

impl MenuProvider for OpenTerminalMenuProvider {
    fn get_file_items(&self, _window: *mut GtkWidget, _files: &[FileInfo]) -> Vec<MenuItem> {
        vec![]
    }

    fn get_background_items(
        &self,
        _window: *mut GtkWidget,
        current_folder: &FileInfo,
    ) -> Vec<MenuItem> {
        println!("folder  {:?}", current_folder.get_uri());
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

    if terminal_path.ends_with("alacritty"){
        return  Box::new(Alacritty);
    }

    Box::new(GnomeConsole {})
}

fn on_open_terminal(file: FileInfo) {
    let default_terminal_dconf = dconf_default_terminal::get_default_terminal();

    match default_terminal_dconf {
        Ok(terminal_command) => {
            let path = file.get_uri().replace("file://", "");

            let _handle = thread::spawn(move || {
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
        Err(erro) => {
            println!("Error on get dconf {}", erro);
        }
    }
}

nautilus_menu_background_activate_cb!(activate_cb, on_open_terminal);
