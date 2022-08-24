use glib_sys::gpointer;
use glib_sys::GType;
use gobject_sys::GObject;
use gobject_sys::GTypeModule;

use gtk_sys::GtkWidget;
use nautilus_extension::nautilus_menu_background_activate_cb;
use nautilus_extension::{ nautilus_module, FileInfo, MenuItem, MenuProvider,
    NautilusModule,
};
use std::os::raw::c_int;
use std::thread;

use std::thread::JoinHandle;
nautilus_module!(init_module);
static mut THREADS: Vec<JoinHandle<()>> = vec![];

fn init_module(module: *mut GTypeModule) -> GType {
    let terminal = dconf_rs::get_string("/org/gnome/desktop/applications/terminal/exec");
    println!("Open Terminal extension loaded. Dconf terminal : {:?}", terminal);
    NautilusModule::new(module, "OpenInTerminalExtension")
        .add_menu_provider(OpenTerminalMenuProvider::new())
        .register()
}

struct OpenTerminalMenuProvider {
    menu_item: MenuItem,
}

impl OpenTerminalMenuProvider {
    fn new() -> OpenTerminalMenuProvider {
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

fn on_open_terminal(file: FileInfo) {
    let default_terminal_dconf =
        dconf_rs::get_string("/org/gnome/desktop/applications/terminal/exec");

    match default_terminal_dconf {
        Ok(terminal_command) => {
            let path = file.get_uri().replace("file://", "");

            println!(
                "executing terminal: {}{}",
                terminal_command,
                path.clone()
            );
            let handle = thread::spawn(move || {
                let child = std::process::Command::new(terminal_command)
                    .args(["--working-directory=".to_string() + path.as_str()])
                    .spawn()
                    .unwrap()
                    .wait();
            });
            unsafe {// TODO use an pool thread or other manager.
                THREADS.push(handle);
            }
        }
        Err(erro) => {
            println!("Error on get dconf {}", erro);
        }
    }
}

nautilus_menu_background_activate_cb!(activate_cb, on_open_terminal);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
