use glib_sys::GType;
use gobject_sys::GTypeModule;

use nautilus_extension::{nautilus_module, NautilusModule};
use std::os::raw::c_int;

mod open_terminal_menu_provider;
mod terminal_command;
mod dconf_default_terminal;
use open_terminal_menu_provider::OpenTerminalMenuProvider;


nautilus_module!(init_module);

fn init_module(module: *mut GTypeModule) -> GType {
    let terminal = dconf_default_terminal::get_default_terminal();
    println!(
        "Open Terminal extension loaded. Dconf terminal : {:?}",
        terminal
    );
    NautilusModule::new(module, "OpenInTerminalExtension")
        .add_menu_provider(OpenTerminalMenuProvider::new())
        .register()
}
