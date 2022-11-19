use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use pulldown_cmark::{Event, Tag};

use crate::md_event::workarea::Workarea;
use crate::md_event::{end, html, start, text};
use crate::settings::settings::Settings;

pub fn setup_parser<'a, I>(
    iter: I,
    settings: &Settings,
    metadata: &mut HashMap<String, String>,
) -> impl Iterator<Item = Event<'a>>
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
            Tag::Image(link_type, src, title) => {
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

    metadata.extend(workarea.meta.into_iter());
    workarea.events.into_iter()
}

#[cfg(test)]
mod tests {
    use std::{
        borrow::Cow,
        collections::{BTreeMap, HashMap},
    };

    use crate::settings::settings::Settings;
    use pulldown_cmark::{html, Options, Parser};

    use super::setup_parser;

    fn generate(markdown: &str) -> (Cow<'_, str>, HashMap<String, String>) {
        let settings = Settings::new(None, None, None);
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

        let mut metadata: HashMap<String, String> = HashMap::new();

        let parser = setup_parser(Parser::new_ext(markdown, options), &settings, &mut metadata);
        let mut html = String::new();
        html::push_html(&mut html, parser);

        (html.into(), metadata)
    }

    #[test]
    fn test_metadata() {
        let md = r#"# Hello
<!-- :test: data -->
<!--- :author: hogehoge piyopiyo --->
<!--- 
    :memo: テストデータ
    ::ignored: test
--->
<!--
    :ignored2: test
-->
<!--
    :ignored3: test -->
<!---
    :date: 2022/11/19 --->
<!---
    :pat1: value
--->
<!--- :pat2: value --->
<!---
    :pat3: value --->
<!--- :pat4: value
--->
plain text
"#;

        let expected_output = r#"<h1>Hello</h1>
<!-- :test: data -->
<!--
    :ignored2: test
-->
<!--
    :ignored3: test -->
<p>plain text</p>
"#;
        let expected_metadata: BTreeMap<String, String> = BTreeMap::from([
            ("author".to_string(), "hogehoge piyopiyo".to_string()),
            ("date".to_string(), "2022/11/19".to_string()),
            ("memo".to_string(), "テストデータ".to_string()),
            ("pat1".to_string(), "value".to_string()),
            ("pat2".to_string(), "value".to_string()),
            ("pat3".to_string(), "value".to_string()),
            ("pat4".to_string(), "value".to_string()),
        ]);
        let (actual_output, actual_metadata) = generate(md);

        assert_eq!(
            expected_metadata,
            actual_metadata
                .into_iter()
                .collect::<BTreeMap<String, String>>(),
        );
        assert_eq!(expected_output, actual_output);
    }
}
