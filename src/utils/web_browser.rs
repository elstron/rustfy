use std::process::Command;

pub fn open_web_search(query: &str, search_type: crate::enums::WebSearchType) {
    let clean = query.replace("!g", "").replace("!y", "").trim().to_string();
    let url = match search_type {
        crate::enums::WebSearchType::Google => format!("https://www.google.com/search?q={}", clean),
        crate::enums::WebSearchType::YouTube => {
            format!("https://www.youtube.com/results?search_query={}", clean)
        }
        crate::enums::WebSearchType::Other(base_url) => format!("{}{}", base_url, clean),
    };

    let _ = Command::new("xdg-open").arg(&url).spawn();
}
