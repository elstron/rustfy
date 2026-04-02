use glib::Properties;
use gtk::glib;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::*;
use gtk::{Box, Revealer};

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/com/elstron/rustfy/revealers/apps_revealer.ui")]
    #[properties(wrapper_type = super::AppsRevealer)]
    pub struct AppsRevealer {
        #[template_child]
        pub apps_box: TemplateChild<Box>,
        #[template_child]
        pub revealer: TemplateChild<Revealer>,
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

    pub fn apps_box(&self) -> Box {
        self.imp().apps_box.get()
    }

    pub fn set_reveal_child(&self, reveal: bool) {
        self.imp().revealer.set_reveal_child(reveal);
    }
}

impl Default for AppsRevealer {
    fn default() -> Self {
        Self::new()
    }
}
