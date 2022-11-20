use std::collections::HashMap;

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
                start::event_code(&mut workarea, &settings, &ev, kind);
            }
            Tag::Image(link_type, src, title) => {
                start::event_image(&mut workarea, &settings, &ev, link_type, src, title);
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

    #[test]
    fn test_metadata_with_emoji() {
        let md = r#"# Hello
<!-- :🚿test🚿: 🚿data🚿 -->
<!--- :🚿author🚿: 🚿hogehoge piyopiyo🚿 --->
<!--- 
    :🚿memo🚿: 🚿テストデータ🚿
    ::🚿ignored🚿: 🚿test🚿
--->
<!--
    :🚿ignored2🚿: 🚿test🚿
-->
<!--
    :🚿ignored3🚿: 🚿test🚿 -->
<!---
    :🚿date🚿: 🚿2022/11/19🚿 --->
<!---
    :🚿pat1🚿: 🚿value🚿
--->
<!--- :🚿pat2🚿: 🚿value🚿 --->
<!---
    :🚿pat3🚿: 🚿value🚿 --->
<!--- :🚿pat4🚿: 🚿value🚿
--->
plain text
"#;

        let expected_output = r#"<h1>Hello</h1>
<!-- :🚿test🚿: 🚿data🚿 -->
<!--
    :🚿ignored2🚿: 🚿test🚿
-->
<!--
    :🚿ignored3🚿: 🚿test🚿 -->
<p>plain text</p>
"#;
        let expected_metadata: BTreeMap<String, String> = BTreeMap::from([
            (
                "🚿author🚿".to_string(),
                "🚿hogehoge piyopiyo🚿".to_string(),
            ),
            ("🚿date🚿".to_string(), "🚿2022/11/19🚿".to_string()),
            ("🚿memo🚿".to_string(), "🚿テストデータ🚿".to_string()),
            ("🚿pat1🚿".to_string(), "🚿value🚿".to_string()),
            ("🚿pat2🚿".to_string(), "🚿value🚿".to_string()),
            ("🚿pat3🚿".to_string(), "🚿value🚿".to_string()),
            ("🚿pat4🚿".to_string(), "🚿value🚿".to_string()),
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

    #[test]
    fn test_metadata_with_surrogate() {
        let md = r#"# Hello
<!-- :𩸽test𩸽: 𩸽data𩸽 -->
<!--- :𩸽author𩸽: 𩸽hogehoge piyopiyo𩸽 --->
<!--- 
    :𩸽memo𩸽: 𩸽テストデータ𩸽
    ::𩸽ignored𩸽: 𩸽test𩸽
--->
<!--
    :𩸽ignored2𩸽: 𩸽test𩸽
-->
<!--
    :𩸽ignored3𩸽: 𩸽test𩸽 -->
<!---
    :𩸽date𩸽: 𩸽2022/11/19𩸽 --->
<!---
    :𩸽pat1𩸽: 𩸽value𩸽
--->
<!--- :𩸽pat2𩸽: 𩸽value𩸽 --->
<!---
    :𩸽pat3𩸽: 𩸽value𩸽 --->
<!--- :𩸽pat4𩸽: 𩸽value𩸽
--->
plain text
"#;

        let expected_output = r#"<h1>Hello</h1>
<!-- :𩸽test𩸽: 𩸽data𩸽 -->
<!--
    :𩸽ignored2𩸽: 𩸽test𩸽
-->
<!--
    :𩸽ignored3𩸽: 𩸽test𩸽 -->
<p>plain text</p>
"#;
        let expected_metadata: BTreeMap<String, String> = BTreeMap::from([
            (
                "𩸽author𩸽".to_string(),
                "𩸽hogehoge piyopiyo𩸽".to_string(),
            ),
            ("𩸽date𩸽".to_string(), "𩸽2022/11/19𩸽".to_string()),
            ("𩸽memo𩸽".to_string(), "𩸽テストデータ𩸽".to_string()),
            ("𩸽pat1𩸽".to_string(), "𩸽value𩸽".to_string()),
            ("𩸽pat2𩸽".to_string(), "𩸽value𩸽".to_string()),
            ("𩸽pat3𩸽".to_string(), "𩸽value𩸽".to_string()),
            ("𩸽pat4𩸽".to_string(), "𩸽value𩸽".to_string()),
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
