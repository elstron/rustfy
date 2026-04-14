use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Clone, Default)]
pub struct AppInfo {
    pub name: String,
    pub generic_name: Option<String>,
    pub exec: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SeatchType {
    App,
    File,
    Calculator(Option<f64>),
    Web,
    WebSearch(WebSearchType),
    #[allow(dead_code)]
    ShellCommand,
}

#[derive(Debug, Clone)]
pub enum WebSearchType {
    Google,
    YouTube,
}

impl SeatchType {
    fn is_calculator(_str: &str) -> (bool, Option<f64>) {
        match meval::eval_str(_str) {
            Ok(res) => (true, Some(res)),
            Err(_) => (false, None),
        }
    }

    fn is_web(str: &str) -> bool {
        str.starts_with("http://") || str.starts_with("https://")
    }

    fn is_web_search(str: &str) -> (bool, WebSearchType) {
        match str {
            s if s.starts_with("!g") => (true, WebSearchType::Google),
            s if s.starts_with("!y") => (true, WebSearchType::YouTube),
            _ => (false, WebSearchType::Google),
        }
    }

    fn is_file(str: &str) -> bool {
        str.starts_with("!f")
    }

    fn is_app(str: &str) -> bool {
        let re = Regex::new(r"[^\p{L}\p{N}]").unwrap();

        if re.is_match(str) {
            return false;
        }

        true
    }
}

impl FromStr for SeatchType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (
            Self::is_calculator(s),
            Self::is_web(s),
            Self::is_web_search(s),
            Self::is_file(s),
            Self::is_app(s),
        ) {
            ((true, res), _, _, _, _) => Ok(SeatchType::Calculator(res)),
            (_, true, _, _, _) => Ok(SeatchType::Web),
            (_, _, (true, t), _, _) => Ok(SeatchType::WebSearch(t)),
            (_, _, _, true, _) => Ok(SeatchType::File),
            (_, _, _, _, true) => Ok(SeatchType::App),
            _ => Ok(SeatchType::WebSearch(WebSearchType::Google)),
        }
    }
}
