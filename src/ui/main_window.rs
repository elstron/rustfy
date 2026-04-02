use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, TemplateChild};

use crate::enums::{SeatchType, WebSearchType};

use super::revealers::{AppsRevealer, CalculatorRevealer, FileRevealer, WebRevealer};

const CALCULATOR_ICON: &str = "accessories-calculator-symbolic";
const WEB_ICON: &str = "insert-link-symbolic";
const GOOGLE_ICON: &str = "help-browser-symbolic";
const YOUTUBE_ICON: &str = "youtube-symbolic";
const _FILE_ICON: &str = "folder-symbolic";
const SEARCH_ICON: &str = "system-search-symbolic";

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/elstron/rustfy/main_window.ui")]
    pub struct MainWindow {
        #[template_child]
        pub search_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub entry_icon: TemplateChild<gtk::Image>,

        #[template_child]
        pub apps_revealer: TemplateChild<AppsRevealer>,
        #[template_child]
        pub calculator_revealer: TemplateChild<CalculatorRevealer>,
        #[template_child]
        pub web_search_revealer: TemplateChild<WebRevealer>,
        #[template_child]
        pub file_revealer: TemplateChild<FileRevealer>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {
        const NAME: &'static str = "RustfyMainWindow";
        type Type = super::MainWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            super::super::revealers::AppsRevealer::static_type();
            super::super::revealers::CalculatorRevealer::static_type();
            super::super::revealers::FileRevealer::static_type();
            super::super::revealers::WebRevealer::static_type();

            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MainWindow {}
    impl WidgetImpl for MainWindow {}
    impl WindowImpl for MainWindow {}
    impl ApplicationWindowImpl for MainWindow {}
}

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow;
}

impl MainWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let obj: Self = glib::Object::builder().property("application", app).build();

        obj.setup();
        obj
    }

    fn setup(&self) {
        let imp = self.imp();

        imp.search_entry.connect_changed(glib::clone!(
            #[weak(rename_to = this)]
            self,
            move |entry| {
                let text = entry.text();
                this.set_active_panel(text.as_str());
            }
        ));

        let gesture = gtk::GestureClick::new();
        gesture.set_button(1);

        gesture.connect_pressed(glib::clone!(
            #[weak(rename_to = this)]
            self,
            move |_, _, _, _| {
                this.on_connect_pressed();
            }
        ));

        self.add_controller(gesture);
    }

    fn on_connect_pressed(&self) {
        self.close();
    }

    fn set_active_panel(&self, query: &str) {
        let imp = self.imp();
        self.hide_panels();

        let search_type = query.parse::<SeatchType>().unwrap_or(SeatchType::App);

        if query.is_empty() {
            return;
        }

        match search_type {
            SeatchType::Calculator(res) => {
                imp.calculator_revealer.show_result(res);
                imp.entry_icon.set_icon_name(Some(CALCULATOR_ICON));
            }
            SeatchType::Web => {
                imp.web_search_revealer.set_reveal_child(true);
                imp.entry_icon.set_icon_name(Some(WEB_ICON));
            }
            SeatchType::WebSearch(s) => {
                imp.web_search_revealer.set_reveal_child(true);
                match s {
                    WebSearchType::Google => {
                        imp.entry_icon.set_icon_name(Some(GOOGLE_ICON));
                    }
                    WebSearchType::YouTube => {
                        imp.entry_icon.set_icon_name(Some(YOUTUBE_ICON));
                    }
                }
            }
            SeatchType::File => imp.file_revealer.set_reveal_child(true),
            SeatchType::App => {
                imp.apps_revealer.set_reveal_child(true);
                imp.entry_icon.set_icon_name(Some(SEARCH_ICON));
            }
        }
    }

    fn hide_panels(&self) {
        let imp = self.imp();

        imp.apps_revealer.set_reveal_child(false);
        imp.calculator_revealer.set_reveal_child(false);
        imp.web_search_revealer.set_reveal_child(false);
        imp.file_revealer.set_reveal_child(false);
    }
}
