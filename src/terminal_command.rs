use std::{
    io,
    process::{Child, Command},
};

pub trait TerminalCommand {
    fn spawn(&self, terminal: String, path: String) -> io::Result<Child>;
}

pub struct AlacrittyWayland;
impl TerminalCommand for AlacrittyWayland {
    fn spawn(&self, terminal: String, path: String) -> io::Result<Child> {
        Command::new("sh")
            .arg("-C")
            .arg(terminal)
            .arg("--working-directory=".to_string() + path.as_str())
            .spawn()
    }
}

pub struct Alacritty;
impl TerminalCommand for Alacritty {
    fn spawn(&self, terminal: String, path: String) -> io::Result<Child> {
        Command::new(terminal)
            .arg("--working-directory=".to_string() + path.as_str())
            .spawn()
    }
}

pub struct GnomeConsole;
impl TerminalCommand for GnomeConsole {
    fn spawn(&self, terminal: String, path: String) -> io::Result<Child> {
        Command::new(terminal)
            .arg("--working-directory=".to_string() + path.as_str())
            .spawn()
    }
}
