use crate::settings::settings::Settings;

use super::workarea::Workarea;
use pulldown_cmark::{CowStr, Event};

pub fn event<'a>(
    workarea: &mut Workarea<'a>,
    settings: &Settings,
    ev: &Event<'a>,
    content: &CowStr,
) {
    let matches = workarea.re.comment_tag.captures(content);

    if matches.is_some() {
        let captures = matches.as_ref().unwrap();

        let key = captures.name("key");
        let value = captures.name("value");

        if key.is_some() && value.is_some() {
            workarea.meta.insert(
                key.unwrap().as_str().to_owned(),
                value.unwrap().as_str().to_owned(),
            );
        }
    }

    workarea.is_comment = (workarea.is_comment | workarea.re.comment_begin.is_match(content))
        & !workarea.re.comment_end.is_match(content);

    workarea.push_event(ev);
}
