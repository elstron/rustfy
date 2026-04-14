use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Box, Revealer};
use glib::Properties;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/com/elstron/rustfy/revealers/file_revealer.ui")]
    #[properties(wrapper_type = super::FileRevealer)]
    pub struct FileRevealer {
        #[template_child]
        pub content_box: TemplateChild<Box>,
        #[template_child]
        pub revealer: TemplateChild<Revealer>,
        
        #[property(get, set)]
        reveal_child: std::cell::Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FileRevealer {
        const NAME: &'static str = "FileRevealer";
        type Type = super::FileRevealer;
        type ParentType = Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for FileRevealer {
        fn constructed(&self) {
            self.parent_constructed();
            
            let obj = self.obj();
            obj.bind_property("reveal-child", &*self.revealer, "reveal-child")
                .bidirectional()
                .sync_create()
                .build();
        }
    }
    
    impl WidgetImpl for FileRevealer {}
    impl BoxImpl for FileRevealer {}
}

glib::wrapper! {
    pub struct FileRevealer(ObjectSubclass<imp::FileRevealer>)
        @extends Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl FileRevealer {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn content_box(&self) -> Box {
        self.imp().content_box.get()
    }
}

impl Default for FileRevealer {
    fn default() -> Self {
        Self::new()
    }
}
