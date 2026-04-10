use crate::enums::AppInfo;
use crate::utils::applications::{list_applications, load_icon};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use glib::Properties;
use glib::subclass::Signal;
use glib::types::StaticType;
use gtk::glib;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::*;
use gtk::{Box, Revealer};
use std::{cell::RefCell, rc::Rc};

use crate::utils::launch_app;
use gtk::prelude::*;
use once_cell::sync::Lazy;

static WINDOW_CLOSED_SIGNAL: Lazy<Signal> = Lazy::new(|| {
    Signal::builder("window-closed")
        .param_types([String::static_type()])
        .return_type::<()>()
        .build()
});
mod imp {

    use crate::enums::AppInfo;

    use super::*;

    #[derive(Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/com/elstron/rustfy/revealers/apps_revealer.ui")]
    #[properties(wrapper_type = super::AppsRevealer)]
    pub struct AppsRevealer {
        #[template_child]
        pub apps_box: TemplateChild<Box>,
        #[template_child]
        pub revealer: TemplateChild<Revealer>,

        pub apps_list: Rc<RefCell<Vec<AppInfo>>>,
        pub filtered_apps: Rc<RefCell<Vec<AppInfo>>>,
        //pub main_window: Weak<MainWindow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppsRevealer {
        const NAME: &'static str = "AppsRevealer";
        type Type = super::AppsRevealer;
        type ParentType = Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for AppsRevealer {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn signals() -> &'static [Signal] {
            std::slice::from_ref(&*WINDOW_CLOSED_SIGNAL)
        }
    }

    impl WidgetImpl for AppsRevealer {}
    impl BoxImpl for AppsRevealer {}
}

glib::wrapper! {
    pub struct AppsRevealer(ObjectSubclass<imp::AppsRevealer>)
        @extends Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl AppsRevealer {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn setup(&self) {
        let context = glib::MainContext::default();
        let (sender, receiver) = async_channel::unbounded::<Vec<AppInfo>>();

        std::thread::spawn(move || {
            let apps = list_applications();
            futures::executor::block_on(sender.send(apps)).unwrap();
        });

        let apps = Rc::clone(&self.imp().apps_list);
        context.spawn_local(glib::clone!(
            #[weak(rename_to = this)]
            self,
            async move {
                let message = receiver.recv().await.unwrap();
                *apps.borrow_mut() = message;
                this.set_apps();
            }
        ));
    }

    pub fn set_reveal_child(&self, reveal: bool) {
        self.imp().revealer.set_reveal_child(reveal);
    }

    pub fn search_apps(&self, query: &str) {
        self.hide_all_apps();
        let filtered_apps = self.filter_apps(query);

        match self.search_widget(0) {
            Some(widget) => widget.add_css_class("selected"),
            None => println!("No se encontró ningún widget para el índice 0"),
        }

        for app in filtered_apps.iter() {
            self.show_widget(&app.0.name);
        }
        self.set_reveal_child(true);
    }

    pub fn filter_apps(&self, query: &str) -> Vec<(AppInfo, i64)> {
        let imp = self.imp();
        let matcher = SkimMatcherV2::default();
        let results = imp
            .apps_list
            .borrow()
            .iter()
            .filter_map(|app| {
                matcher
                    .fuzzy_match(&app.name, query)
                    .map(|score| (app.clone(), score))
            })
            .collect::<Vec<_>>();

        imp.filtered_apps
            .replace(results.iter().map(|(app, _)| app.clone()).collect());
        results
    }

    fn set_apps(&self) {
        for app in self.imp().apps_list.borrow().iter() {
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
            self.imp().apps_box.append(&button);
        }
    }
    pub fn search_widget(&self, index: i32) -> Option<gtk::Widget> {
        let current_apps = self.imp().filtered_apps.borrow();

        let app_name = match current_apps.get(index as usize) {
            Some(app) => &app.name,
            None => return None,
        };

        let box_ = self.imp().apps_box.clone();
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

    pub fn hide_all_apps(&self) {
        let box_ = self.imp().apps_box.clone();
        let mut child = box_.first_child();
        while let Some(c) = child {
            child = c.next_sibling();
            c.remove_css_class("selected");
            c.hide();
        }
    }

    pub fn deselect_all_widgets(&self) {
        let box_ = self.imp().apps_box.clone();
        let mut child = box_.first_child();
        while let Some(c) = child {
            child = c.next_sibling();
            c.remove_css_class("selected");
        }
    }

    fn set_envent_click(&self, widget: &gtk::Button, exec_cmd: String) {
        widget.connect_clicked(glib::clone!(
            #[weak(rename_to = this)]
            self,
            move |_| {
                launch_app(&exec_cmd);
                this.emit_by_name::<()>("window-closed", &[&exec_cmd]);
            }
        ));
    }

    pub fn show_widget(&self, name: &str) {
        let box_ = self.imp().apps_box.clone();
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
}

impl Default for AppsRevealer {
    fn default() -> Self {
        Self::new()
    }
}
