use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};

use super::code::Code;
use super::input::Input;
use super::output::Output;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub version: i32,
    #[serde(default)]
    pub input: Input,
    #[serde(default)]
    pub output: Output,
    #[serde(default)]
    pub code: Code,
}

impl Settings {
    pub fn new(input: Option<Input>, output: Option<Output>, code: Option<Code>) -> Self {
        Self {
            version: 1,
            input: match input {
                Some(o) => o,
                None => Input::new(None, None, None, None),
            },
            output: match output {
                Some(o) => o,
                None => Output::new(None, None, None, None),
            },
            code: match code {
                Some(o) => o,
                None => Code::new(None),
            },
        }
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Err(format!("{} not found.", path.to_string_lossy()).to_string());
        }
        let Ok(file) = File::open(path) else         {
          return Err("an internal error has occurred.".to_string());
        };

        let Ok(toml) = std::io::read_to_string(&file) else {
          return Err("toml reading error.".to_string());
        };

        let Ok(obj ) = toml::from_str::<Self>(&toml) else
        {
          return Err("toml parsing error.".to_string())  ;
        };

        Ok(obj)
    }
}
