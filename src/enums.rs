use std::str::FromStr;

pub enum SeatchType {
    App,
    File,
    Calculator,
    Web,
    WebSearch,
}

impl SeatchType {
    fn is_calculator(_str: &str) -> bool {
        //TODO: Implement a more robust calculator detection, maybe using regex to detect mathematical expressions

        false
    }

    fn is_web(str: &str) -> bool {
        str.starts_with("http://") || str.starts_with("https://")
    }

    fn is_web_search(str: &str) -> bool {
        str.starts_with("s:")
    }

    fn is_file(str: &str) -> bool {
        str.starts_with("f:")
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
        ) {
            (true, _, _, _) => Ok(SeatchType::Calculator),
            (_, true, _, _) => Ok(SeatchType::Web),
            (_, _, true, _) => Ok(SeatchType::WebSearch),
            (_, _, _, true) => Ok(SeatchType::File),
            _ => Ok(SeatchType::App),
        }
    }
}
