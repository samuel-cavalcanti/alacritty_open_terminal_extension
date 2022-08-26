use glib_sys::GType;
use gobject_sys::GTypeModule;

use nautilus_extension::{nautilus_module, NautilusModule};
use std::os::raw::c_int;

mod dconf_default_terminal;
mod open_terminal_menu_provider;
mod terminal_command;
use open_terminal_menu_provider::OpenTerminalMenuProvider;

nautilus_module!(init_module);

fn init_module(module: *mut GTypeModule) -> GType {
    NautilusModule::new(module, "OpenInTerminalExtension")
        .add_menu_provider(OpenTerminalMenuProvider::new())
        .register()
}
