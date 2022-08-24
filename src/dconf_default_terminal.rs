pub fn get_default_terminal() -> Result<String, String> {
    dconf_rs::get_string("/org/gnome/desktop/applications/terminal/exec")
}
