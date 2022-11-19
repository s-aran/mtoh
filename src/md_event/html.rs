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
    let tag_matched = matches.is_some();

    let special_begin_matched = workarea.re.special_comment_begin.is_match(content);
    let special_end_matched = workarea.re.special_comment_end.is_match(content);

    workarea.is_comment = (special_begin_matched & !special_end_matched)
        | (!special_end_matched & workarea.is_comment);

    if tag_matched && (special_end_matched || workarea.is_comment) {
        let captures = matches.as_ref().unwrap();

        let key = captures.name("key");
        let value = captures.name("value");

        if key.is_some() && value.is_some() && !key.unwrap().as_str().starts_with(":") {
            workarea.meta.insert(
                key.unwrap().as_str().to_owned(),
                value.unwrap().as_str().to_owned(),
            );
        }
    }

    if special_begin_matched || special_end_matched || workarea.is_comment {
        return;
    }

    workarea.push_event(ev);
}
