use crate::ui::main_window::MainWindow;
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::prelude::*;
impl MainWindow {
    pub fn keyboard_listener(&self) {
        let key_controller = gtk::EventControllerKey::new();

        key_controller.set_propagation_phase(gtk::PropagationPhase::Capture);
        key_controller.connect_key_pressed(glib::clone!(
            #[weak(rename_to = this)]
            self,
            #[upgrade_or]
            glib::signal::Propagation::Proceed,
            move |_, keyval, _, _| {
                use gtk::gdk::Key;
                match keyval {
                    Key::Up => {
                        println!("Up key pressed");
                        glib::signal::Propagation::Stop
                    }
                    Key::Escape => {
                        this.close();
                        glib::signal::Propagation::Stop
                    }
                    Key::Return => glib::signal::Propagation::Stop,
                    _ => glib::signal::Propagation::Proceed,
                }
            }
        ));

        self.imp().search_entry.add_controller(key_controller);
    }
}
