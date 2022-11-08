use super::workarea::Workarea;
use crate::settings::Settings;
use pulldown_cmark::{CowStr, Event};
use regex::Regex;

const COMMENT_BEGIN: &str = "<!--";
const COMMENT_END: &str = "-->";

pub fn event<'a>(
    workarea: &mut Workarea<'a>,
    settings: &Settings,
    ev: &Event<'a>,
    content: &CowStr,
) {
    workarea.push_content(&content.to_string());
    workarea.push_event(ev);
}
