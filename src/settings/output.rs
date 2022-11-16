use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Output {
    #[serde(default = "default_output_html_dir")]
    pub html_dir: String,
    #[serde(default = "default_output_css_dir")]
    pub css_dir: String,
    #[serde(default = "default_output_js_dir")]
    pub js_dir: String,
    #[serde(default = "default_output_img_dir")]
    pub img_dir: String,
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

fn default_output_img_dir() -> String {
    Output::default().img_dir
}

impl Output {
    pub fn new(
        html_dir: Option<&str>,
        css_dir: Option<&str>,
        js_dir: Option<&str>,
        img_dir: Option<&str>,
    ) -> Self {
        let html = match html_dir {
            Some(s) => s,
            None => "html",
        };

        Self {
            html_dir: html.into(),
            css_dir: match css_dir {
                Some(s) => s.into(),
                None => Path::new(&html).join("css").to_string_lossy().into(),
            },
            js_dir: match js_dir {
                Some(s) => s.into(),
                None => Path::new(&html).join("css").to_string_lossy().into(),
            },
            img_dir: match img_dir {
                Some(s) => s.into(),
                None => Path::new(&html).join("img").to_string_lossy().into(),
            },
        }
    }
}

impl Default for Output {
    fn default() -> Self {
        Output::new(None, None, None, None)
    }
}
