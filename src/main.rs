mod html_gen;
mod md_event;
mod settings;

use handlebars::Handlebars;
use pulldown_cmark::{html, Options, Parser};
use serde_json::json;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf, MAIN_SEPARATOR},
};

use crate::settings::settings::Settings;

fn enum_files(path: &Path, recursive: bool, callback: &mut dyn FnMut(&Path)) -> Result<(), String> {
    if !path.is_dir() {
        return Err(format!("{} is invalid", path.to_string_lossy()));
    }

    let files = path.read_dir().unwrap();

    for dir_entry in files {
        let dir_entry = dir_entry.unwrap();
        let path = dir_entry.path();

        if recursive && path.is_dir() {
            enum_files(&path, recursive, callback)?;
        } else {
            // println!("{}", path.to_str().unwrap());
            callback(&path);
        }
    }

    Ok(())
}

fn change_extension(source: &Path, ext: &str) -> Result<PathBuf, String> {
    let name = match source.file_name() {
        Some(n) => {
            let ext = match source.extension() {
                Some(e) => e.to_string_lossy().to_string(),
                None => {
                    return Err(format!(
                        "cannot get file extension: {}",
                        source.to_string_lossy()
                    ));
                }
            };

            let n_str = n.to_string_lossy().to_string();
            let splitted = n_str.as_str().split(ext.as_str()).collect::<Vec<&str>>();
            splitted[0].to_string()
        }
        None => {
            return Err(format!("cannot get filename: {}", source.to_string_lossy()));
        }
    };

    Ok(PathBuf::from(format!("{}{}", name, ext)))
}

fn make_output_html_filename(source: &Path) -> Result<PathBuf, String> {
    change_extension(source, "html")
}

fn make_output_css_filename(source: &Path) -> Result<PathBuf, String> {
    change_extension(source, "css")
}

fn make_directory_path_with_exists_check(path: &String) -> Result<PathBuf, String> {
    let result = PathBuf::from(path);
    if !result.exists() {
        return Err(format!("{} is not exists.", path));
    }

    Ok(result)
}

fn make_md_directory_from(settings: &Settings) -> Result<PathBuf, String> {
    make_directory_path_with_exists_check(&settings.input.markdown_dir)
}

fn make_scss_directory_from(settings: &Settings) -> Result<PathBuf, String> {
    make_directory_path_with_exists_check(&settings.input.sass_dir)
}

fn make_css_directory_from(settings: &Settings) -> Result<PathBuf, String> {
    make_directory_path_with_exists_check(&settings.output.css_dir)
}

fn make_html_directory_from(settings: &Settings) -> Result<PathBuf, String> {
    make_directory_path_with_exists_check(&settings.output.html_dir)
}

fn main() {
    println!("Hello, world!");

    let settings = match Settings::load(&Path::new(".mtoh.toml")) {
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

    // let mut template_files: Vec<PathBuf> = vec![];
    // let template_dir_path = Path::new(&settings.input.template_dir);
    // if !template_dir_path.exists() {
    //     eprintln!("{} is not exists.", template_dir_path.to_string_lossy());
    //     std::process::exit(1);
    // }

    // match enum_files(&template_dir_path, false, &mut |p: &Path| {
    //     template_files.push(p.to_path_buf());
    // }) {
    //     Ok(()) => {}
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         std::process::exit(1);
    //     }
    // }

    // for template in template_files.iter() {
    //     let text = match fs::read_to_string(template) {
    //         Ok(s) => s,
    //         Err(e) => {
    //             eprintln!("{}", e);
    //             std::process::exit(1);
    //         }
    //     };

    // let ps = SyntaxSet::load_defaults_newlines();
    // for ele in ps.syntaxes() {
    //     println!("{} === {}", ele.name, ele.file_extensions.join(", "));
    // }

    // let ts = ThemeSet::load_defaults();
    // for ele in ts.themes {
    //     println!("{}", ele.0);
    // }

    let mut markdown_files: Vec<PathBuf> = vec![];
    let markdown_dir_path = match make_md_directory_from(&settings) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    match enum_files(&markdown_dir_path, false, &mut |p: &Path| {
        if p.is_file() {
            markdown_files.push(p.to_path_buf());
        }
    }) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    let mut sass_files: Vec<PathBuf> = vec![];
    let sass_dir_path = match make_scss_directory_from(&settings) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    match enum_files(&sass_dir_path, false, &mut |p: &Path| {
        sass_files.push(p.to_path_buf());
    }) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    let css_dir_path = match make_css_directory_from(&settings) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let html_dir_path = match make_html_directory_from(&settings) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

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

        let new_name = match make_output_css_filename(s) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        let result = css_dir_path.join(new_name);
        let mut file = match File::create(&result) {
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
            .strip_prefix(html_dir_path.as_path())
            .unwrap()
            .to_path_buf()
    }));

    let link_tags = css_files
        .iter()
        .map(|s| {
            format!(
                r#"<link href="{}" rel="stylesheet" type="text/css" />"#,
                // for windows
                s.to_string_lossy().replace(MAIN_SEPARATOR, "/")
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let html_template =
        match fs::read_to_string(Path::new(&settings.input.template_dir).join("code.hbs")) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    for md in markdown_files.iter() {
        let text = match fs::read_to_string(md) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };

        let mut metadata: HashMap<String, String> = HashMap::new();
        let parser = html_gen::setup_parser(
            Parser::new_ext(text.as_str(), options),
            &settings,
            &mut metadata,
        );

        let mut html = String::new();
        html::push_html(&mut html, parser);

        let reg = Handlebars::new();
        let output = reg
            .render_template(
                &html_template.as_str(),
                &json!({"title": "test", "content": html, "css_link": link_tags.as_str()}),
            )
            .unwrap();

        let new_name = match make_output_html_filename(md) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        let result = html_dir_path.join(new_name);

        let mut file = match File::create(result) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e.to_string());
                std::process::exit(1);
            }
        };

        let buf = output.as_bytes();
        match file.write_all(&buf) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
