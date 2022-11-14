use super::workarea::Workarea;
use crate::{html_gen::replace_emoji_shortcode, settings::Settings};
use pulldown_cmark::{CowStr, Event};

pub fn event<'a>(
    workarea: &mut Workarea<'a>,
    settings: &Settings,
    ev: &Event<'a>,
    content: &CowStr,
) {
    if workarea.is_code {
        workarea.push_content(&content.to_string());
        return;
    }

    match replace_emoji_shortcode(workarea, content) {
        Ok(s) => {
            workarea.push_event(&Event::Text(s.into()));
            return;
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}
