use pulldown_cmark::{CowStr, Event};

pub struct Workarea<'a> {
    pub events: Vec<Event<'a>>,
    pub contents: Vec<String>,

    pub author: String,

    pub is_comment: bool,
    pub is_code: bool,
    pub is_html: bool,
}

impl<'a> Workarea<'a> {
    pub fn new() -> Self {
        Self {
            events: vec![],
            contents: vec![],

            author: "".to_string(),

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
