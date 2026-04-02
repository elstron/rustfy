mod config;
mod resources;
mod ui;

mod enums;
mod utils;
use config::layer_shell_configure;
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
        let window = ui::main_window::MainWindow::new(app);
        layer_shell_configure(&window);
        window.present();
    });

    app.run();
}
