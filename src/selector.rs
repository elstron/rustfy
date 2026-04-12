use glib::subclass::types::ObjectSubclassIsExt;

use crate::{
    enums::{SeatchType, WebSearchType},
    ui::window::MainWindow,
};
use gtk::prelude::*;

const CALCULATOR_ICON: &str = "accessories-calculator-symbolic";
const WEB_ICON: &str = "insert-link-symbolic";
const GOOGLE_ICON: &str = "help-browser-symbolic";
const YOUTUBE_ICON: &str = "youtube-symbolic";
const _FILE_ICON: &str = "folder-symbolic";
const SEARCH_ICON: &str = "system-search-symbolic";

impl MainWindow {
    pub fn set_active_panel(&self, query: &str) {
        self.hide_panels();

        let imp = self.imp();

        if query.trim().is_empty() {
            imp.footer_box.set_visible(false);
            imp.separator.set_visible(false);
            imp.main_box.set_spacing(0);
            return;
        };

        if !imp.footer_box.is_visible() {
            imp.main_box.set_spacing(10);
            //imp.separator.set_visible(true);
            imp.footer_box.set_visible(true);
        }

        let search_type = query.parse::<SeatchType>().unwrap_or(SeatchType::App);

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
                //*imp.search_query.borrow_mut() = query.to_string();
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
                imp.apps_revealer.search_apps(query);
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
