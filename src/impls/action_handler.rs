use gtk::prelude::GtkWindowExt;
use std::process::Command;

use crate::{
    enums::SeatchType, traits::actions::HandleReturnAction, ui::window::MainWindow,
    utils::terminal::open_terminal,
};
use gtk::subclass::prelude::ObjectSubclassIsExt;

impl HandleReturnAction for MainWindow {
    fn handle_return_action(&self, str: &str) {
        println!("Handling search action for: {}", str);
        let search_type = str.parse::<SeatchType>().unwrap_or(SeatchType::App);

        match search_type {
            SeatchType::Calculator(_) => {}
            SeatchType::Web(site) => {
                let url = if site.starts_with("http://") || site.starts_with("https://") {
                    site
                } else {
                    format!("https://{}", site.trim())
                };
                let _ = Command::new("xdg-open").arg(&url).spawn();
                self.close();
            }
            SeatchType::WebSearch(_) => {}
            SeatchType::File => {}
            SeatchType::App => self.imp().apps_revealer.launch_selected(None),
            SeatchType::ShellCommand(cmd) => {
                open_terminal(&cmd, None);
                self.close();
            }
        }
    }
}
