use crate::enums::AppInfo;
use glib::Object;
use gtk::subclass::prelude::ObjectSubclassIsExt;

glib::wrapper! {
    pub struct AppInfoObject(ObjectSubclass<imp::AppInfoObject>);
}

impl AppInfoObject {
    pub fn new(data: AppInfo) -> Self {
        let obj: Self = Object::builder().build();
        obj.imp().data.replace(data);
        obj
    }

    pub fn data(&self) -> AppInfo {
        self.imp().data.borrow().clone()
    }
}

mod imp {
    use super::*;
    use glib::subclass::prelude::*;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct AppInfoObject {
        pub data: RefCell<AppInfo>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppInfoObject {
        const NAME: &'static str = "AlgoObject";
        type Type = super::AppInfoObject;
    }

    impl ObjectImpl for AppInfoObject {}
}
