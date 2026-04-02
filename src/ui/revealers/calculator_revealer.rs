use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Box, Revealer};

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/com/elstron/rustfy/revealers/calculator_revealer.ui")]
    #[properties(wrapper_type = super::CalculatorRevealer)]
    pub struct CalculatorRevealer {
        #[template_child]
        pub content_box: TemplateChild<Box>,
        #[template_child]
        pub revealer: TemplateChild<Revealer>,
        #[template_child]
        pub operation_result: TemplateChild<gtk::Label>,

        #[property(get, set)]
        reveal_child: std::cell::Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CalculatorRevealer {
        const NAME: &'static str = "CalculatorRevealer";
        type Type = super::CalculatorRevealer;
        type ParentType = Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for CalculatorRevealer {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.bind_property("reveal-child", &*self.revealer, "reveal-child")
                .bidirectional()
                .sync_create()
                .build();
        }
    }

    impl WidgetImpl for CalculatorRevealer {}
    impl BoxImpl for CalculatorRevealer {}
}

glib::wrapper! {
    pub struct CalculatorRevealer(ObjectSubclass<imp::CalculatorRevealer>)
        @extends Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl CalculatorRevealer {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn content_box(&self) -> Box {
        self.imp().content_box.get()
    }

    pub fn show_result(&self, op: Option<f64>) {
        self.imp().revealer.set_reveal_child(true);

        match op {
            Some(res) => self
                .imp()
                .operation_result
                .set_text(&format!("Result: {}", res)),
            None => self.imp().operation_result.set_text("Invalid expression"),
        }
        println!("calculator expression");
    }
}

impl Default for CalculatorRevealer {
    fn default() -> Self {
        Self::new()
    }
}
