use serde::{Deserialize, Serialize};

use crate::settings::code_settings::highlight::CodeHighlight;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Code {
    #[serde(default)]
    pub highlight: CodeHighlight,
}

impl Code {
    pub fn new(highlight: Option<CodeHighlight>) -> Self {
        Self {
            highlight: match highlight {
                Some(o) => o,
                None => CodeHighlight::new(None),
            },
        }
    }
}

impl Default for Code {
    fn default() -> Self {
        Code::new(None)
    }
}
