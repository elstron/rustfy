use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{TemplateChild, glib};

use super::revealers::{AppsRevealer, CalculatorRevealer, FileRevealer, WebRevealer};

mod imp {

    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/elstron/rustfy/main_window.ui")]
    pub struct MainWindow {
        #[template_child]
        pub main_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub background_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub footer_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub separator: TemplateChild<gtk::Separator>,

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
        //pub search_query: Rc<RefCell<String>>,
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
        obj.keyboard_listener();
        obj
    }

    fn setup(&self) {
        let imp = self.imp();
        imp.apps_revealer.setup();

        imp.search_entry.connect_changed(glib::clone!(
            #[weak(rename_to = this)]
            self,
            move |entry| {
                let text = entry.text();
                this.set_active_panel(text.as_str());
            }
        ));

        self.imp().apps_revealer.connect_local(
            "window-closed",
            false,
            glib::clone!(
                #[weak(rename_to = this)]
                self,
                #[upgrade_or]
                None,
                move |_| {
                    this.close();
                    None
                }
            ),
        );

        let gesture = gtk::GestureClick::new();
        gesture.set_button(1);

        gesture.connect_pressed(glib::clone!(
            #[weak(rename_to = this)]
            self,
            move |_, _, _, _| {
                this.close();
            }
        ));

        self.imp().background_box.add_controller(gesture);
    }
}
