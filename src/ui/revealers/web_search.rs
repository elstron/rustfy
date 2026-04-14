use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Box, Revealer};

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/com/elstron/rustfy/revealers/web_search_revealer.ui")]
    #[properties(wrapper_type = super::WebRevealer)]
    pub struct WebRevealer {
        #[template_child]
        pub content_box: TemplateChild<Box>,
        #[template_child]
        pub revealer: TemplateChild<Revealer>,

        #[property(get, set)]
        reveal_child: std::cell::Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WebRevealer {
        const NAME: &'static str = "WebRevealer";
        type Type = super::WebRevealer;
        type ParentType = Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for WebRevealer {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.bind_property("reveal-child", &*self.revealer, "reveal-child")
                .bidirectional()
                .sync_create()
                .build();
        }
    }

    impl WidgetImpl for WebRevealer {}
    impl BoxImpl for WebRevealer {}
}

glib::wrapper! {
    pub struct WebRevealer(ObjectSubclass<imp::WebRevealer>)
        @extends Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl WebRevealer {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn content_box(&self) -> Box {
        self.imp().content_box.get()
    }
}

impl Default for WebRevealer {
    fn default() -> Self {
        Self::new()
    }
}
