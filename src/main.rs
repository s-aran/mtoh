mod settings;

use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf, MAIN_SEPARATOR},
};
use syntect::highlighting::{Style, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub struct PParser {
    pub theme: String,
}

impl PParser {
    pub fn setup_parser<'a, I>(iter: I, p: &PParser) -> impl Iterator<Item = Event<'a>>
    where
        I: Iterator<Item = Event<'a>>,
    {
        let mut code_language: Option<String> = None;
        let mut events: Vec<Event> = vec![];
        let mut codes: Vec<String> = vec![];

        iter.for_each(|ev| match &ev {
            Event::Start(Tag::CodeBlock(ref c)) => {
                code_language = match &c {
                    CodeBlockKind::Fenced(f) => Some(f.to_string()),
                    _ => None,
                };

                // return Event::Start(Tag::CodeBlock(c.clone()));
                events.push(ev.clone());
            }
            Event::End(Tag::CodeBlock(ref c)) => {
                match &code_language {
                    Some(l) => {
                        // println!("{}", l);
                        let code = codes.join("");

                        // Load these once at the start of your program
                        let ps = SyntaxSet::load_defaults_newlines();
                        let ts = ThemeSet::load_defaults();

                        let language = match l.as_str() {
                            "rust" => "Rust",
                            "python" => "Python",
                            "cpp" => "C++",
                            _ => "Plain Text",
                        };

                        let syntax = match ps.find_syntax_by_name(&language) {
                            Some(s) => s,
                            None => {
                                eprintln!("{} not found.", l);
                                ps.find_syntax_by_name("Plain Text").unwrap()
                            }
                        };
                        let hh =
                            highlighted_html_for_string(&code, &ps, &syntax, &ts.themes[&p.theme]);

                        let t = hh.unwrap();
                        // t.push_str(r#"</code></pre>"#);

                        // cleanup
                        code_language = None;
                        codes = vec![];

                        // println!("{}", t);

                        // return Event::Html(t.into());
                        events.push(Event::Html(t.into()));
                    }
                    _ => {}
                };

                // Event::End(Tag::CodeBlock(c.clone()))
                events.push(ev.clone());
            }
            Event::Text(ref t) => {
                match code_language {
                    Some(_) => {
                        codes.push(t.clone().to_string());
                        return;
                    }
                    _ => {}
                }

                // let t = l.unwrap();
                // Event::End(Tag::CodeBlock(CowStr::from(hh.unwrap())))
                // Event::Text(t)
                events.push(ev);
            }
            _ => {
                events.push(ev);
            }
        });

        events.into_iter()
    }
}

fn main() {
    println!("Hello, world!");

    let settings = match settings::Settings::load(&Path::new(".mtoh.toml")) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    println!(
        "version={}, theme={}",
        settings.version, settings.code.highlight.theme,
    );

    // let ps = SyntaxSet::load_defaults_newlines();
    // for ele in ps.syntaxes() {
    //     println!("{} === {}", ele.name, ele.file_extensions.join(", "));
    // }

    // let ts = ThemeSet::load_defaults();
    // for ele in ts.themes {
    //     println!("{}", ele.0);
    // }

    let binding = Path::new("md").join("test.md");
    let path = match binding.to_str() {
        Some(p) => p,
        None => {
            eprintln!("{}", "unsupport path");
            std::process::exit(1);
        }
    };

    let text = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} {}", "cannot read", path);
            std::process::exit(1);
        }
    };

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    // let mut code_language: Option<CowStr> = None;

    let mut sass_files: Vec<PathBuf> = vec![];
    sass_files.push(Path::new("sass").join("main.scss"));

    let mut css_files: Vec<PathBuf> = vec![Path::new("..")
        .join("modern-css-reset")
        .join("dist")
        .join("reset.min.css")];
    css_files.extend(sass_files.iter().map(|s| {
        if !s.exists() {
            eprintln!("{} not found.", s.to_string_lossy());
            return Path::new("").to_path_buf();
        }

        let options = grass::Options::default();
        let path_str = match s.to_str() {
            Some(ps) => ps,
            None => {
                eprintln!("cannot convert to string from {}.", s.to_string_lossy());
                return Path::new("").to_path_buf();
            }
        };
        let sass = match grass::from_path(path_str, &options) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                return Path::new("").to_path_buf();
            }
        };

        let name = match s.file_name() {
            Some(n) => {
                let ext = match s.extension() {
                    Some(e) => e.to_string_lossy().to_string(),
                    None => {
                        eprintln!("cannot get file extension: {}", s.to_string_lossy());
                        return Path::new("").to_path_buf();
                    }
                };

                let n_str = n.to_string_lossy().to_string();
                let splitted = n_str.as_str().split(ext.as_str()).collect::<Vec<&str>>();
                splitted[0].to_string()
            }
            None => {
                eprintln!("cannot get filename: {}", s.to_string_lossy());
                return Path::new("").to_path_buf();
            }
        };
        let new_name = format!("{}css", name);
        let result = Path::new("css").join(new_name);
        let out = Path::new("html").join(result.as_path());
        let mut file = match File::create(&out) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e.to_string());
                std::process::exit(1);
            }
        };

        let buf = sass.as_bytes();
        match file.write_all(&buf) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }

        result
    }));

    let link_tags = css_files
        .iter()
        .map(|s| {
            format!(
                r#"<link href="{}" rel="stylesheet" type="text/css" />"#,
                // r#"<link href="{}" rel="stylesheet" />"#,
                // for windows
                s.to_string_lossy().replace(MAIN_SEPARATOR, "/")
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let ppp = PParser {
        theme: settings.code.highlight.theme.to_string(),
    };
    let parser = PParser::setup_parser(Parser::new_ext(text.as_str(), options), &ppp);

    let mut html = String::new();
    html.push_str(&format!(r#"<!DOCTYPE html><html lang="ja-JP"><head><meta charset="utf-8" /><title>{}</title>{}</head><body>"#,"test",link_tags)
    );

    html::push_html(&mut html, parser);
    html.push_str(r#"</body></html>"#);

    let mut file = match File::create(Path::new("html").join("test.html")) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
    };

    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let s = "pub struct Wow { hi: u64 }\nfn blah() -> u64 {}";

    let hh =
        highlighted_html_for_string(&s, &ps, &syntax, &ts.themes[&settings.code.highlight.theme]);
    println!("{}", hh.as_ref().unwrap());
    // html.push_str(hh.as_ref().unwrap());

    let buf = html.as_bytes();
    match file.write_all(&buf) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
