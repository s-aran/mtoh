use std::{fs::File, io::Read, path::Path};

use pulldown_cmark::{CodeBlockKind, CowStr, Event, LinkType, Tag};

use super::workarea::Workarea;
use crate::settings::settings::Settings;

pub fn event_code<'a>(
    workarea: &mut Workarea<'a>,
    settings: &Settings,
    ev: &Event<'a>,
    kind: &CodeBlockKind,
) {
    workarea.push_event(ev);
}

pub fn event_image<'a>(
    workarea: &mut Workarea<'a>,
    settings: &Settings,
    ev: &Event<'a>,
    link_type: &LinkType,
    src: &CowStr<'a>,
    title: &CowStr<'a>,
) {
    let src_str = src.to_string();
    let output_img_dir = settings.output.img_dir.to_string();
    let input_img_dir = settings.input.img_dir.to_string();
    let filename = Path::new(&src_str).file_name().unwrap();

    let from = Path::new(&input_img_dir).join(filename);
    let to = Path::new(&output_img_dir).join(filename);

    if settings.output.image.use_base64 {
        let mut file = match File::open(from.as_path()) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };

        let mut file_content: Vec<u8> = Vec::new();
        match file.read_to_end(&mut file_content) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };

        let encoded_image = base64::encode(&file_content);
        let img_src = format!("data:image/png;base64,{}", encoded_image);
        let tag = Tag::Image(
            *link_type,
            pulldown_cmark::CowStr::from(img_src),
            title.clone(),
        );
        workarea.push_event(&Event::Start(tag));
    } else {
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
    }
    workarea.push_event(&ev);
}
