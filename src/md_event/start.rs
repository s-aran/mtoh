use super::workarea::Workarea;
use crate::settings::settings::Settings;
use pulldown_cmark::{CodeBlockKind, Event};

pub fn event_code<'a>(
    workarea: &mut Workarea<'a>,
    settings: &Settings,
    ev: &Event<'a>,
    kind: &CodeBlockKind,
) {
    workarea.push_event(ev);
}
