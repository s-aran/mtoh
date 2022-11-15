use std::path::Path;

use pulldown_cmark::{Event, Tag};

use crate::md_event::workarea::Workarea;
use crate::md_event::{end, html, start, text};
use crate::settings::Settings;

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
            Tag::Image(_, src, _) => {
                let src_str = src.to_string();
                let output_img_dir = settings.output.img_dir.to_string();
                let input_img_dir = settings.input.img_dir.to_string();
                let filename = Path::new(&src_str).file_name().unwrap();

                let from = Path::new(&input_img_dir).join(filename);
                let to = Path::new(&output_img_dir).join(filename);
                match std::fs::copy(&from, &to) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!(
                            "{} {} -> {}",
                            e,
                            from.to_string_lossy(),
                            to.to_string_lossy()
                        );
                    }
                }
                workarea.push_event(&ev);
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
