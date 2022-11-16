use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Input {
    #[serde(default = "default_input_markdown_dir")]
    pub markdown_dir: String,
    #[serde(default = "default_input_sass_dir")]
    pub sass_dir: String,
    #[serde(default = "default_input_template_dir")]
    pub template_dir: String,
    #[serde(default = "default_input_img_dir")]
    pub img_dir: String,
}

fn default_input_markdown_dir() -> String {
    Input::default().markdown_dir
}

fn default_input_sass_dir() -> String {
    Input::default().sass_dir
}

fn default_input_template_dir() -> String {
    Input::default().template_dir
}

fn default_input_img_dir() -> String {
    Input::default().img_dir
}

impl Input {
    pub fn new(
        markdown_dir: Option<&str>,
        sass_dir: Option<&str>,
        template_dir: Option<&str>,
        img_dir: Option<&str>,
    ) -> Self {
        let md = match markdown_dir {
            Some(s) => s,
            None => "md",
        };
        Self {
            markdown_dir: md.to_owned(),
            sass_dir: match sass_dir {
                Some(s) => s,
                None => "sass",
            }
            .to_owned(),
            template_dir: match template_dir {
                Some(s) => s,
                None => "template",
            }
            .to_owned(),
            img_dir: match img_dir {
                Some(s) => s.into(),
                None => Path::new(&md).join("img").to_string_lossy().into(),
            },
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Input::new(None, None, None, None)
    }
}
