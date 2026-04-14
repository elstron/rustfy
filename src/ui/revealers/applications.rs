use crate::enums::AppInfo;
use crate::utils::applications::{list_applications, load_icon};
use crate::wrappers::app_info::AppInfoObject;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use glib::Properties;
use glib::subclass::Signal;
use glib::types::StaticType;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::*;
use gtk::{Box, FilterListModel, ListView, Revealer};
use gtk::{SignalListItemFactory, glib};
use std::{cell::RefCell, rc::Rc};

use crate::utils::launch_app;
use gtk::prelude::*;
use once_cell::sync::Lazy;

static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
    vec![
        Signal::builder("window-closed")
            .param_types([String::static_type()])
            .build(),
        // Aquí registras tu segunda señal
        Signal::builder("restore-focus")
            .param_types([String::static_type()])
            .build(),
    ]
});

mod imp {

    use gtk::CustomFilter;

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
        pub filter: CustomFilter,
        pub selection_model: gtk::SingleSelection,
        pub focused: i32,
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
            SIGNALS.as_slice()
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

        let (filter_model, list_store) = self.model();
        let model_clone = list_store.clone();
        let apps = Rc::clone(&self.imp().apps_list);
        context.spawn_local(glib::clone!(
            #[weak(rename_to = this)]
            self,
            async move {
                let message = receiver.recv().await.unwrap();
                *apps.borrow_mut() = message;

                for item in this.imp().apps_list.borrow().iter() {
                    model_clone.append(&AppInfoObject::new(item.clone()));
                }
            }
        ));

        let factory = self.factory();
        self.imp().selection_model.set_model(Some(&filter_model));
        let list_view = ListView::new(Some(self.imp().selection_model.clone()), Some(factory));

        list_view.connect_activate(glib::clone!(
            #[weak(rename_to = this)]
            self,
            move |list, position| {
                let item = list
                    .model()
                    .and_then(|m| m.item(position))
                    .and_then(|i| i.downcast::<AppInfoObject>().ok());
                if let Some(app_info) = item {
                    launch_app(&app_info.data().exec);
                    this.emit_by_name::<()>("window-closed", &[&app_info.data().name]);
                }
            }
        ));

        let key_controller = gtk::EventControllerKey::new();
        key_controller.connect_key_pressed(glib::clone!(
            #[weak(rename_to = this)]
            self,
            #[upgrade_or]
            glib::signal::Propagation::Stop,
            move |_, keyval, _, _| {
                use gtk::gdk::Key;

                if let Some(c) = keyval.to_unicode()
                    && !matches!(keyval, Key::Up | Key::Down | Key::Return)
                {
                    this.emit_by_name::<()>("restore-focus", &[&c.to_string()]);

                    return gtk::glib::Propagation::Stop;
                }

                gtk::glib::Propagation::Proceed
            }
        ));
        list_view.add_controller(key_controller);
        self.imp().apps_box.append(&list_view);
    }

    pub fn set_reveal_child(&self, reveal: bool) {
        self.imp().revealer.set_reveal_child(reveal);
    }

    pub fn search_apps(&self, query: &str) {
        self.update_filter(query);
        self.imp().filter.changed(gtk::FilterChange::Different);
        self.set_reveal_child(true);
    }

    pub fn set_widget(&self, vbox: &gtk::Box, app: &AppInfo) {
        vbox.set_widget_name(&app.name);
        match app.icon.as_ref() {
            Some(icon) => {
                let image = load_icon(icon, 24);
                vbox.append(&image);
            }
            None => {
                let placeholder = gtk::Image::from_icon_name("application-x-executable");
                vbox.append(&placeholder);
            }
        }
        vbox.append(&gtk::Label::new(Some(&app.name)));
        vbox.add_css_class("app-button");
    }

    pub fn factory(&self) -> SignalListItemFactory {
        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let _box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
            list_item.set_child(Some(&_box));
        });

        factory.connect_bind(glib::clone!(
            #[weak(rename_to = this)]
            self,
            move |_, list_item| {
                let item_obj = list_item
                    .item()
                    .unwrap()
                    .downcast::<AppInfoObject>()
                    .unwrap();
                let widget = list_item
                    .child()
                    .unwrap()
                    .downcast::<gtk::Box>()
                    .expect("El widget del ListItem no es un Label");
                let app = item_obj.data();

                this.set_widget(&widget, &app);
            }
        ));
        factory
    }

    pub fn model(&self) -> (FilterListModel, gtk::gio::ListStore) {
        let model = gtk::gio::ListStore::new::<AppInfoObject>();
        (
            FilterListModel::new(Some(model.clone()), Some(self.imp().filter.clone())),
            model,
        )
    }

    pub fn update_filter(&self, query: &str) {
        let query_clone = query.to_string();
        self.imp().filter.set_filter_func(glib::clone!(
            #[weak(rename_to = _this)]
            self,
            #[upgrade_or]
            false,
            move |item| {
                let item_obj = item
                    .downcast_ref::<AppInfoObject>()
                    .expect("El item no es un AppInfoObject");
                let app = item_obj.data();
                let matcher = SkimMatcherV2::default();
                matcher
                    .fuzzy_match(
                        &app.name.to_lowercase(),
                        query_clone.to_lowercase().as_str(),
                    )
                    .is_some()
            }
        ));
    }
}

impl Default for AppsRevealer {
    fn default() -> Self {
        Self::new()
    }
}
