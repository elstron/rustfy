mod config;
mod enums;
mod keyboard;
mod resources;
mod selector;
mod ui;
mod utils;
use config::layer_shell_configure;
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::prelude::*;

use utils::css;
fn main() {
    resources::load_resources().unwrap_or_else(|e| {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    });

    let app = gtk::Application::builder()
        .application_id("com.elstron.rustfy")
        .build();

    app.connect_activate(|app| {
        css::load_css();
        let window = ui::window::MainWindow::new(app);
        layer_shell_configure(&window);
        window.imp().search_entry.grab_focus();
        window.present();
    });

    app.run();
}
