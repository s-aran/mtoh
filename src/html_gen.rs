use crate::md_event::workarea::{self, Workarea};
use crate::md_event::{end, html, start, text};
use crate::settings::Settings;
use pulldown_cmark::{Event, Tag};

pub fn setup_parser<'a, I>(iter: I, settings: &Settings) -> impl Iterator<Item = Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut workarea = Workarea::new();

    iter.for_each(|ev| match &ev {
        Event::Start(Tag::CodeBlock(ref kind)) => {
            workarea.break_frags();
            workarea.is_code = true;
            start::event_code(&mut workarea, &settings, &ev, &kind);
        }
        Event::End(Tag::CodeBlock(ref kind)) => {
            workarea.break_frags();
            workarea.is_code = true;
            end::event_code(&mut workarea, &settings, &ev, &kind);
            workarea.is_code = false;
        }
        Event::Text(ref content) => {
            // keep flags
            text::event(&mut workarea, &settings, &ev, &content);
        }
        Event::Html(ref content) => {
            workarea.break_frags();
            workarea.is_html = true;
            html::event(&mut workarea, &settings, &ev, &content);
        }
        _ => {
            workarea.events.push(ev);
        }
    });

    workarea.events.into_iter()
}
