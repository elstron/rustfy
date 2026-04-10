use gtk::prelude::*;
use std::{cell::RefCell, rc::Rc};

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::enums::AppInfo;
use crate::utils::applications::load_icon;
use crate::utils::launch_app;

pub struct AppsList {
    window: gtk::ApplicationWindow,
    pub vbox: gtk::Box,
    pub filtered_apps: Rc<RefCell<Vec<AppInfo>>>,
}

impl AppsList {
    pub fn new(
        window: &gtk::ApplicationWindow,
        vbox: &gtk::Box,
        apps: Rc<RefCell<Vec<AppInfo>>>,
    ) -> Self {
        let mut imp = Self {
            window: window.clone(),
            vbox: vbox.clone(),
            filtered_apps: Rc::new(RefCell::new(apps.borrow().clone())),
        };
        imp.set_apps();
        imp
    }

    fn set_apps(&mut self) {
        for app in self.filtered_apps.borrow().iter() {
            let button = gtk::Button::new();
            let app_container = gtk::Box::new(gtk::Orientation::Horizontal, 3);
            button.set_widget_name(&app.name);
            match app.icon.as_ref() {
                Some(icon) => {
                    let image = load_icon(icon, 24);
                    app_container.append(&image);
                }
                None => {
                    let placeholder = gtk::Image::from_icon_name("application-x-executable");
                    app_container.append(&placeholder);
                }
            }
            app_container.append(&gtk::Label::new(Some(&app.name)));
            button.set_child(Some(&app_container));
            button.add_css_class("app-button");
            self.set_envent_click(&button, app.exec.clone());
            self.vbox.append(&button);
        }
    }

    pub fn filter_apps(&mut self, query: &str) {
        self.hide_all_widgets();
        let filtered_apps = self.search(query);

        match self.search_widget(0) {
            Some(widget) => widget.add_css_class("selected"),
            None => println!("No se encontró ningún widget para el índice 0"),
        }

        for app in filtered_apps.iter() {
            self.show_widget(&app.0.name);
        }
    }

    pub fn search(&mut self, query: &str) -> Vec<(AppInfo, i64)> {
        let matcher = SkimMatcherV2::default();
        let results = self
            .filtered_apps
            .borrow()
            .iter()
            .filter_map(|app| {
                matcher
                    .fuzzy_match(&app.name, query)
                    .map(|score| (app.clone(), score))
            })
            .collect::<Vec<_>>();

        self.filtered_apps
            .replace(results.iter().map(|(app, _)| app.clone()).collect());

        results
    }

    pub fn search_widget(&self, index: i32) -> Option<gtk::Widget> {
        let current_apps = self.filtered_apps.borrow();

        let app_name = match current_apps.get(index as usize) {
            Some(app) => &app.name,
            None => return None,
        };

        let box_ = self.vbox.clone();
        let mut child = box_.first_child();
        while let Some(c) = child {
            child = c.next_sibling();
            if c.widget_name() == app_name.as_str() {
                return Some(c);
            }
            continue;
        }
        None
    }

    pub fn show_widget(&self, name: &str) {
        let box_ = self.vbox.clone();
        let mut child = box_.first_child();
        while let Some(c) = child {
            child = c.next_sibling();
            if c.widget_name() == name {
                c.show();
                break;
            }
            continue;
        }
    }

    pub fn hide_all_widgets(&self) {
        let box_ = self.vbox.clone();
        let mut child = box_.first_child();
        while let Some(c) = child {
            child = c.next_sibling();
            c.remove_css_class("selected");
            c.hide();
        }
    }
    pub fn deselect_all_widgets(&self) {
        let box_ = self.vbox.clone();
        let mut child = box_.first_child();
        while let Some(c) = child {
            child = c.next_sibling();
            c.remove_css_class("selected");
        }
    }
    pub fn launch_app(&self, index: usize) {
        if let Some(app) = self.filtered_apps.borrow().get(index) {
            launch_app(&app.exec);
        }
    }
    fn set_envent_click(&self, widget: &gtk::Button, exec_cmd: String) {
        let window_clone = self.window.clone();
        widget.connect_clicked(move |_| {
            launch_app(&exec_cmd);
            window_clone.hide();
        });
    }
}
