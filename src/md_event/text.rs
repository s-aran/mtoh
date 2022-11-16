use std::ops::Range;

use linked_hash_set::LinkedHashSet;
use pulldown_cmark::{CowStr, Event};
use regex::Regex;

use super::workarea::Workarea;
use crate::settings::settings::Settings;

pub fn event<'a>(
    workarea: &mut Workarea<'a>,
    settings: &Settings,
    ev: &Event<'a>,
    content: &CowStr,
) {
    if workarea.is_code {
        workarea.push_content(&content.to_string());
        return;
    }

    let result = replace_emoji_shortcode(&workarea.re.emoji_shortcode, content);
    workarea.push_event(&Event::Text(result.into()));
}

fn replace_emoji_shortcode<'a>(re: &Regex, content: &CowStr) -> String {
    let mut result = content.to_owned().to_string();
    let mut range_set: LinkedHashSet<Range<usize>> = LinkedHashSet::new();

    let mut pos = 0;
    // find emoji shortcodes (including "maybe")
    while re.is_match_at(&content, pos) {
        let c = re.find_at(&content, pos).unwrap();
        range_set.insert(c.range());
        pos = c.end() - 2;
    }

    // replacing the LinkedHashSet with an actual emoji from the emoji shortcodes
    for v in range_set.iter().rev() {
        // get emoji shortcode. e.g. :shower:
        let k = content.to_string().as_str()[v.start..v.end].to_string();
        // :shower: -> shower
        let shortcode = &k[1..k.len() - 1];
        match emojis::get_by_shortcode(shortcode) {
            Some(e) => {
                // replace is in reverse order of the emoji shortcodes,
                // so there is no need to recalculate the index (v.start and v.end)
                let before = &result[..v.start];
                let after = &result[v.end..];
                result = [before, e.as_str(), after].join("");
            }
            None => {}
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use pulldown_cmark::CowStr;

    use crate::md_event::workarea::Workarea;

    use super::replace_emoji_shortcode;

    fn replace_emoji(content: &str) -> String {
        let workarea = Workarea::new();
        replace_emoji_shortcode(&workarea.re.emoji_shortcode, &CowStr::from(content))
    }

    #[test]
    fn test_emoji_only_1() {
        let content = ":rocket:";
        assert_eq!(replace_emoji(content), "🚀");
    }

    #[test]
    fn test_emoji_only_many_colon() {
        let content = ": :: :::rocket::: :: :";
        assert_eq!(replace_emoji(content), ": :: ::🚀:: :: :");
    }

    #[test]
    fn test_emoji_multiple() {
        let content = ":rocket::shower::alien:";
        assert_eq!(replace_emoji(content), "🚀🚿👽");
    }

    #[test]
    fn test_emoji_multiple_with_space() {
        let content = ":rocket: :shower: :alien:";
        assert_eq!(replace_emoji(content), "🚀 🚿 👽");
    }

    #[test]
    fn test_emoji_with_string() {
        let content = "rocket: :rocket: shower::shower: alien:alien:";
        assert_eq!(replace_emoji(content), "rocket: 🚀 shower:🚿 alien👽");
    }

    #[test]
    fn test_emoji_with_japanese() {
        let content = "ロケット: :rocket:　シャワー::shower:\tエイリアン:alien:";
        assert_eq!(
            replace_emoji(content),
            "ロケット: 🚀　シャワー:🚿\tエイリアン👽"
        );

        // with surrogate
        let content = "𩸽ロケット: 𩸽:rocket:𩸽　シャワー:𩸽:shower:𩸽\tエイリアン𩸽:alien:𩸽";
        assert_eq!(
            replace_emoji(content),
            "𩸽ロケット: 𩸽🚀𩸽　シャワー:𩸽🚿𩸽\tエイリアン𩸽👽𩸽"
        );
    }

    #[test]
    fn test_emoji_with_unknown_shortcode() {
        let content = "ロケット: :rocket1:　シャワー::🖤:\tエイリアン:𩸽:";
        assert_eq!(
            replace_emoji(content),
            "ロケット: :rocket1:　シャワー::🖤:\tエイリアン:𩸽:"
        );
    }

    #[test]
    fn test_emoji_without_shortcode() {
        let content =
            "::Alice was beginning to get very tired of sitting by her sister on the bank::";
        assert_eq!(replace_emoji(content), content);
    }
}
