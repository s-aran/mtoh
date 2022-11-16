use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeHighlight {
    #[serde(default = "default_code_highlight_theme")]
    pub theme: String,
}

fn default_code_highlight_theme() -> String {
    CodeHighlight::default().theme
}

impl CodeHighlight {
    pub fn new(theme: Option<&str>) -> Self {
        Self {
            theme: match theme {
                Some(s) => s,
                None => "Solarized (light)",
            }
            .into(),
        }
    }
}

impl Default for CodeHighlight {
    fn default() -> Self {
        CodeHighlight::new(None)
    }
}
