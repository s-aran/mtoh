use super::workarea::Workarea;
use crate::settings::Settings;
use pulldown_cmark::{CowStr, Event};

pub fn event<'a>(
    workarea: &mut Workarea<'a>,
    settings: &Settings,
    ev: &Event<'a>,
    content: &CowStr,
) {
    if workarea.is_code {
        // println!("Text: {}", &t.to_string());
        workarea.push_content(&content.to_string());
        return;
    }

    // let t = l.unwrap();
    // Event::End(Tag::CodeBlock(CowStr::from(hh.unwrap())))
    // Event::Text(t)
    workarea.push_event(ev);
}
