use std::collections::HashMap;

use pulldown_cmark::{CowStr, Event};
use regex::Regex;

const COMMENT_BEGIN: &str = r"<!--";
const COMMENT_END: &str = r"-->";
const COMMENT_TAG: &str = r#":(?P<key>.+): *["']?(?P<value>.+?)["']? *"#;

pub struct ReCollection {
    pub comment_begin: Regex,
    pub comment_tag: Regex,
    pub comment_end: Regex,
}

impl ReCollection {
    pub fn new() -> Self {
        let Ok(re_comment_begin) = Regex::new(format!(r"^{} ?", COMMENT_BEGIN).as_str()) else {
            panic!("regex compile failed: {}", COMMENT_BEGIN);
        };

        let Ok(re_comment_tag) = Regex::new(format!(r"{}($|{})", COMMENT_TAG, COMMENT_END).as_str()) else {
            panic!("{} failed", COMMENT_TAG);
        };

        let Ok(re_comment_end) = Regex::new(format!(r" ?{}$", COMMENT_END).as_str()) else {
            panic!("{} failed", COMMENT_END);
        };

        Self {
            comment_begin: re_comment_begin,
            comment_tag: re_comment_tag,
            comment_end: re_comment_end,
        }
    }
}

pub struct Workarea<'a> {
    pub re: ReCollection,

    pub events: Vec<Event<'a>>,
    pub contents: Vec<String>,

    pub meta: HashMap<String, String>,

    pub is_comment: bool,
    pub is_code: bool,
    pub is_html: bool,
}

impl<'a> Workarea<'a> {
    pub fn new() -> Self {
        Self {
            re: ReCollection::new(),

            events: vec![],
            contents: vec![],

            meta: HashMap::new(),

            is_comment: false,
            is_code: false,
            is_html: false,
        }
    }

    pub fn break_frags(&mut self) {
        self.is_comment = false;
        self.is_code = false;
        self.is_html = false;
    }

    pub fn clear_content(&mut self) {
        self.contents = vec![];
    }

    pub fn push_event(&mut self, ev: &Event<'a>) {
        self.events.push(ev.clone());
    }

    pub fn push_content(&mut self, content: &String) {
        self.contents.push(content.to_owned());
    }
    pub fn print_contents(&self) {
        for c in self.contents.iter() {
            println!("{}", c);
        }
    }
}
