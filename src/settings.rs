use std::{
    fs::File,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Input {
    #[serde(default = "default_input_markdown_dir")]
    pub markdown_dir: String,
    #[serde(default = "default_input_sass_dir")]
    pub sass_dir: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Output {
    #[serde(default = "default_output_html_dir")]
    pub html_dir: String,
    #[serde(default = "default_output_css_dir")]
    pub css_dir: String,
    #[serde(default = "default_output_js_dir")]
    pub js_dir: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Code {
    #[serde(default)]
    pub highlight: CodeHighlight,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeHighlight {
    #[serde(default = "default_code_highlight_theme")]
    pub theme: String,
}

fn default_input_markdown_dir() -> String {
    Input::default().markdown_dir
}

fn default_input_sass_dir() -> String {
    Input::default().sass_dir
}

fn default_output_html_dir() -> String {
    Output::default().html_dir
}

fn default_output_css_dir() -> String {
    Output::default().css_dir
}

fn default_output_js_dir() -> String {
    Output::default().js_dir
}

fn default_code_highlight_theme() -> String {
    CodeHighlight::default().theme
}

impl Settings {
    pub fn new(input: Option<Input>, output: Option<Output>, code: Option<Code>) -> Self {
        Self {
            version: 1,
            input: match input {
                Some(o) => o,
                None => Input::new(None, None),
            },
            output: match output {
                Some(o) => o,
                None => Output::new(None, None, None),
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

impl Input {
    pub fn new(markdown_dir: Option<&str>, sass_dir: Option<&str>) -> Self {
        Self {
            markdown_dir: match markdown_dir {
                Some(s) => s.to_owned(),
                None => "md".to_string(),
            },
            sass_dir: match sass_dir {
                Some(s) => s.to_owned(),
                None => "sass".to_string(),
            },
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Input::new(None, None)
    }
}

impl Output {
    pub fn new(html_dir: Option<&str>, css_dir: Option<&str>, js_dir: Option<&str>) -> Self {
        let html = match html_dir {
            Some(s) => s,
            None => "html",
        };

        Self {
            html_dir: html.into(),
            css_dir: match css_dir {
                Some(s) => s.into(),
                None => Path::new(&html).join("css").to_string_lossy().to_string(),
            },
            js_dir: match js_dir {
                Some(s) => s.into(),
                None => Path::new(&html).join("css").to_string_lossy().to_string(),
            },
        }
    }
}

impl Default for Output {
    fn default() -> Self {
        Output::new(None, None, None)
    }
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

impl CodeHighlight {
    pub fn new(theme: Option<&str>) -> Self {
        Self {
            theme: match theme {
                Some(s) => s.into(),
                None => "Solarized (light)".to_string(),
            },
        }
    }
}

impl Default for CodeHighlight {
    fn default() -> Self {
        CodeHighlight::new(None)
    }
}
