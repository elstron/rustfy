use regex::Regex;
use std::str::FromStr;
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
    Web(String),
    WebSearch(WebSearchType),
    #[allow(dead_code)]
    ShellCommand(String),
}

#[derive(Debug, Clone)]
pub enum WebSearchType {
    Google(String),
    YouTube(String),
}

impl SeatchType {
    fn has_search_prefix(str: &str) -> (bool, SeatchType) {
        let t = match str {
            s if s.starts_with("!g") => {
                SeatchType::WebSearch(WebSearchType::Google(s[2..].to_string()))
            }
            s if s.starts_with("!y") => {
                SeatchType::WebSearch(WebSearchType::YouTube(s[2..].to_string()))
            }
            s if s.starts_with("http") => SeatchType::Web(s[2..].to_string()),
            s if s.starts_with("!f") => SeatchType::File,
            s if s.starts_with("!sh") => SeatchType::ShellCommand(s[3..].to_string()),
            _ => return (false, SeatchType::App),
        };
        (true, t)
    }

    fn is_application(str: &str) -> bool {
        str.split_whitespace().count() <= 3 && Regex::new(r"^[a-zA-Z0-9 ]+$").unwrap().is_match(str)
    }
}

impl FromStr for SeatchType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (meval::eval_str(s), Self::has_search_prefix(s)) {
            (Ok(r), _) => Ok(SeatchType::Calculator(Some(r))),
            (_, (true, st)) => Ok(st),
            (_, (false, st)) if Self::is_application(s) => Ok(st),
            _ => Ok(SeatchType::WebSearch(WebSearchType::Google(s.to_string()))),
        }
    }
}
