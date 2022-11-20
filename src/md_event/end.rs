use super::workarea::Workarea;
use crate::settings::settings::Settings;
use pulldown_cmark::{CodeBlockKind, Event};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn event_code<'a>(
    workarea: &mut Workarea<'a>,
    settings: &Settings,
    ev: &Event<'a>,
    kind: &CodeBlockKind,
) {
    let lang = match &kind {
        CodeBlockKind::Fenced(f) => f.to_string(),
        _ => return,
    };

    // println!("{}", l);
    let code = workarea.contents.join("");

    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let language = match lang.as_str() {
        "rust" => "Rust",
        "python" => "Python",
        "cpp" => "C++",
        _ => "Plain Text",
    };

    let syntax = match ps.find_syntax_by_name(&language) {
        Some(s) => s,
        None => {
            eprintln!("{} not found.", lang);
            ps.find_syntax_by_name("Plain Text").unwrap()
        }
    };

    let hh = highlighted_html_for_string(
        &code,
        &ps,
        &syntax,
        &ts.themes[&settings.code.highlight.theme],
    );

    let t = hh.unwrap();
    // t.push_str(r#"</code></pre>"#);

    // cleanup
    workarea.clear_content();

    // println!("{}", t);

    // return Event::Html(t.into());
    workarea.push_event(&Event::Html(t.into()));
    workarea.push_event(ev);
}
