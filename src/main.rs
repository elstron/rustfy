use std::cell::RefCell;
use std::rc::Rc;

use gtk::{prelude::*, GestureClick, Revealer};
use gtk::{Application, Box as GtkBox, Entry, Orientation};
mod config;
mod enums;
mod rustfy;
mod utils;
use config::layer_shell_configure;
use utils::css;

use crate::rustfy::Rustfy;

pub enum State {
    Normal,
    Search,
}

#[derive(Debug, Clone)]
pub struct AppInfo {
    name: String,
    exec: String,
    icon: Option<String>,
    #[allow(dead_code)]
    vbox: Option<GtkBox>,
}
fn main() {
    let app = Application::builder()
        .application_id("com.example.LayerPanel")
        .build();

    app.connect_activate(move |app| {
        let rustfy = Rc::new(RefCell::new(Rustfy::new(app)));

        css::load_css();

        layer_shell_configure(&rustfy.borrow().window);
        let current_index = Rc::new(RefCell::new(0));

        let main_box = GtkBox::new(Orientation::Vertical, 0);
        main_box.add_css_class("main-box");
        main_box.set_width_request(600);
        main_box.set_halign(gtk::Align::Center);
        main_box.set_valign(gtk::Align::Center);
        main_box.set_baseline_position(gtk::BaselinePosition::Center);

        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some("Search..."));
        search_entry.set_hexpand(true);

        let vbox = rustfy.borrow().vbox.clone();
        let scrolled_window = gtk::ScrolledWindow::builder()
            .min_content_width(220)
            .min_content_height(100)
            .child(&vbox)
            .build();
        scrolled_window.add_css_class("scroll-box");
        let revealer = Revealer::builder()
            .child(&scrolled_window)
            .transition_type(gtk::RevealerTransitionType::SlideDown)
            .transition_duration(500)
            .reveal_child(false)
            .build();
        let rustfy_clone = Rc::clone(&rustfy);
        let current_index_clone = Rc::clone(&current_index);
        let current_search_type = Rc::new(RefCell::new(enums::SeatchType::App));
        search_entry.connect_changed({
            let revealer_clone = revealer.clone();
            let search_type_clone = Rc::clone(&current_search_type);
            move |entry| {
                revealer_clone.set_reveal_child(false);

                let search_type = entry
                    .text()
                    .as_str()
                    .parse::<enums::SeatchType>()
                    .unwrap_or(enums::SeatchType::App);

                *search_type_clone.borrow_mut() = search_type.clone();
                match search_type {
                    enums::SeatchType::Calculator => println!("Search type: Calculator"),
                    enums::SeatchType::Web => println!("Search type: Web"),
                    enums::SeatchType::WebSearch(w_type, _) => match w_type {
                        enums::WebSearchType::Google => {
                            println!("Search type: Web Search (Google)")
                        }
                        enums::WebSearchType::YouTube => {
                            println!("Search type: Web Search (YouTube)")
                        }
                        enums::WebSearchType::Other(s) => {
                            println!("Search type: Web Search (Other) - {}", s)
                        }
                    },
                    enums::SeatchType::File => println!("Search type: File"),
                    enums::SeatchType::App => {
                        let mut rustfy = rustfy_clone.borrow_mut();
                        let search_text = entry.text().to_string();
                        rustfy.filter_apps(&search_text);
                        *current_index_clone.borrow_mut() = 0;

                        revealer_clone.set_reveal_child(!search_text.is_empty());
                    }
                }
            }
        });

        let rustfy_clone = Rc::clone(&rustfy);
        let search_type_clone_2 = Rc::clone(&current_search_type);
        let key_controller = gtk::EventControllerKey::new();
        key_controller.set_propagation_phase(gtk::PropagationPhase::Capture);
        key_controller.connect_key_pressed(move |_, keyval, _, _| {
            use gtk::gdk::Key;
            let mut idx: usize = *current_index.borrow();
            let len = rustfy_clone.borrow().filtered_apps.borrow().len() as i32 - 1;
            rustfy_clone.borrow().deselect_all_widgets();

            match keyval {
                Key::Up => {
                    idx = idx.saturating_sub(1);
                    *current_index.borrow_mut() = idx;

                    match rustfy_clone.borrow().search_widget(idx as i32) {
                        Some(widget) => widget.add_css_class("selected"),
                        None => println!("No widget found for index: {}", idx as i32),
                    }

                    true.into()
                }
                Key::Down => {
                    let index = *current_index.borrow();

                    if idx + 1 < len as usize {
                        idx += 1;
                    } else if idx + 1 > len as usize {
                        idx = 0;
                    }
                    *current_index.borrow_mut() = idx;

                    match rustfy_clone.borrow().search_widget(index as i32 + 1) {
                        Some(widget) => widget.add_css_class("selected"),
                        None => println!("No widget found for index: {}", index + 1),
                    }

                    true.into()
                }
                Key::Return => {
                    let search_type = search_type_clone_2.borrow();
                    match search_type.clone() {
                        enums::SeatchType::Calculator => {}
                        enums::SeatchType::Web => {}
                        enums::SeatchType::WebSearch(w_type, value) => {
                            utils::web_browser::open_web_search(value.as_str(), w_type.clone());
                            rustfy_clone.borrow().window.hide();
                        }
                        enums::SeatchType::File => {}
                        enums::SeatchType::App => {
                            rustfy_clone.borrow().launch_app(*current_index.borrow());
                            rustfy_clone.borrow().window.hide();
                        }
                    }

                    true.into()
                }
                Key::Escape => {
                    rustfy_clone.borrow().window.hide();
                    true.into()
                }
                _ => false.into(),
            }
        });

        search_entry.add_controller(key_controller);
        main_box.append(&revealer);

        let entry_box = GtkBox::new(Orientation::Horizontal, 0);
        entry_box.set_valign(gtk::Align::End);
        entry_box.add_css_class("entry-box");
        entry_box.append(&search_entry);

        let icon_search = gtk::Image::from_icon_name("system-search-symbolic");
        icon_search.set_pixel_size(16);
        entry_box.append(&icon_search);

        main_box.append(&entry_box);

        let gesture = GestureClick::new();
        gesture.set_button(1);

        let rustfy_clone = Rc::clone(&rustfy);
        gesture.connect_pressed(move |_, _, _, _| {
            rustfy_clone.borrow().window.hide();
        });

        let background = GtkBox::new(Orientation::Vertical, 0);
        background.add_controller(gesture);

        let overlay = gtk::Overlay::new();
        overlay.set_child(Some(&background));
        overlay.add_overlay(&main_box);

        rustfy.borrow().window.set_child(Some(&overlay));
        rustfy.borrow().window.show();
        search_entry.grab_focus();
    });

    app.run();
}
