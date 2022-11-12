use super::workarea::Workarea;
use crate::settings::Settings;
use pulldown_cmark::{CowStr, Event};
use regex::Captures;

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

fn replace_emoji_shortcode<'a>(workarea: &Workarea, content: &CowStr) -> Result<String, String> {
    let result = workarea
        .re
        .emoji_shortcode
        .replace_all(content, |caps: &Captures| {
            let a = caps.get(1).unwrap().as_str();
            println!("a={} => {}", a, emojis::get_by_shortcode(a).unwrap());
            emojis::get_by_shortcode(a).unwrap().as_str()
        });

    Ok(result.to_string())
}
