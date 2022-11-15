use crate::md_event::workarea::{self, Workarea};
use crate::md_event::{end, html, start, text};
use crate::settings::Settings;
use linked_hash_set::LinkedHashSet;
use pulldown_cmark::{CowStr, Event, Tag};
use std::ops::Range;

pub fn setup_parser<'a, I>(iter: I, settings: &Settings) -> impl Iterator<Item = Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut workarea = Workarea::new();

    iter.for_each(|ev| match &ev {
        Event::Start(start_tag) => match &start_tag {
            Tag::CodeBlock(kind) => {
                // println!("event: Start (CodeBlock)");
                workarea.break_frags();
                workarea.is_code = true;
                start::event_code(&mut workarea, &settings, &ev, &kind);
            }
            _ => {
                // println!("event: Start (Other)");
                workarea.push_event(&ev);
            }
        },

        Event::End(end_tag) => match &end_tag {
            Tag::CodeBlock(kind) => {
                // println!("event: End (CodeBlock)");
                workarea.break_frags();
                workarea.is_code = true;
                end::event_code(&mut workarea, &settings, &ev, &kind);
                workarea.is_code = false;
            }
            _ => {
                // println!("event: End (Other)");
                workarea.push_event(&ev);
            }
        },
        Event::Text(content) => {
            // println!("event: Text => {}", content);
            // keep flags
            text::event(&mut workarea, &settings, &ev, &content);
        }
        Event::Html(content) => {
            // println!("event: Html => {}", content);
            let is_comment = workarea.is_comment; // backup
            workarea.break_frags();
            workarea.is_html = true;
            workarea.is_comment = is_comment; // restore
            html::event(&mut workarea, &settings, &ev, &content);
        }
        _ => {
            // println!("event: ???");
            workarea.push_event(&ev);
        }
    });

    for (k, v) in workarea.meta.iter() {
        println!("{}: {}", k, v);
    }

    workarea.events.into_iter()
}

pub fn replace_emoji_shortcode<'a>(
    workarea: &Workarea,
    content: &CowStr,
) -> Result<String, String> {
    let r = &workarea.re.emoji_shortcode;
    let mut result = content.to_owned().to_string();
    let mut range_set: LinkedHashSet<Range<usize>> = LinkedHashSet::new();

    let mut pos = 0;
    // find emoji shortcodes (including "maybe")
    while r.is_match_at(&content, pos) {
        let c = r.find_at(&content, pos).unwrap();
        range_set.insert(c.range());
        pos = c.end() - 2;
    }

    // replacing the LinkedHashSet with an actual emoji from the emoji shortcodes
    for v in range_set.iter().rev() {
        // get emoji shortcode. e.g. :shower:
        let k = content.to_string().as_str()[v.start..v.end].to_string();
        // :shower: -> shower
        let shortcode = &k[1..k.len() - 1];
        match emojis::get_by_shortcode(shortcode) {
            Some(e) => {
                // replace is in reverse order of the emoji shortcodes,
                // so there is no need to recalculate the index (v.start and v.end)
                let before = &result[..v.start];
                let after = &result[v.end..];
                result = [before, e.as_str(), after].join("");
            }
            None => {}
        }
    }

    Ok(result.to_string())
}
