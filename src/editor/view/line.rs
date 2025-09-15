use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Copy, Clone)]
pub enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    fn width(self) -> usize {
        match self {
            GraphemeWidth::Half => 1,
            GraphemeWidth::Full => 2,
        }
    }
}

pub struct TextFragment {
    pub grapheme: String,
    pub rendered_width: GraphemeWidth,
    pub replacement: Option<char>,
}

pub struct Line {
    fragments: Vec<TextFragment>,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        let fragments = Self::str_to_fragments(line_str);
        Self { fragments }
    }

    pub fn insert(&mut self, at: usize, ch: char) {
        let mut result = String::new();

        for (index, fragment) in self.fragments.iter().enumerate() {
            if index == at {
                result.push(ch);
            }
            result.push_str(&fragment.grapheme);
        }

        // if inserting at the end
        if at >= self.fragments.len() {
            result.push(ch);
        }

        self.fragments = Self::str_to_fragments(&result);
    }

    pub fn delete(&mut self, at: usize) -> bool {
        if at >= self.fragments.len() {
            // nothing to remove
            return false;
        }
        self.fragments.remove(at);
        true
    }

    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        line_str
            .graphemes(true)
            .map(|grapheme| {
                let (replacement, rendered_width) = Self::replacement_character(grapheme)
                    .map_or_else(
                        || {
                            let unicode_width = grapheme.width();
                            let rendered_width = match unicode_width {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                    );

                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                }
            })
            .collect()
    }

    fn replacement_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && for_str.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next()
                    && ch.is_control()
                    && chars.next().is_none()
                {
                    return Some('▯');
                }
                Some('·')
            }
            _ => None,
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {
        use std::ops::ControlFlow::{Break, Continue};

        let result = self
            .fragments
            .iter()
            .scan(0, |pos, fragment| {
                let start = *pos;
                let end = start + fragment.rendered_width.width();
                *pos = end;
                Some((start, end, fragment))
            })
            .try_fold(String::new(), |mut acc, (start, end, fragment)| {
                if end <= range.start {
                    Continue(acc)
                } else if start >= range.end {
                    Break(acc)
                } else if start < range.start || end > range.end {
                    acc.push('⋯');
                    Break(acc)
                } else {
                    match fragment.replacement {
                        Some(c) => acc.push(c),
                        None => acc.push_str(fragment.grapheme.as_str()),
                    }
                    Continue(acc)
                }
            });

        match result {
            Break(acc) | Continue(acc) => acc,
        }
    }

    pub fn len(&self) -> usize {
        self.fragments.len()
    }

    pub fn position_of(&self, grapheme: usize) -> usize {
        let mut width = 0;
        for fragment in self.fragments.iter().take(grapheme) {
            width += fragment.rendered_width.width();
        }
        width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_line() {
        let line = Line::from("");
        assert_eq!(line.get(0..0), "");
        assert_eq!(line.get(0..1), "");
        assert_eq!(line.get(1..1), "");
        assert_eq!(line.get(1..2), "");
    }

    #[test]
    fn test_hello_world() {
        let line = Line::from("Hello, world!");
        assert_eq!(line.get(0..0), "");
        assert_eq!(line.get(0..1), "H");
        assert_eq!(line.get(0..12), "Hello, world");
        assert_eq!(line.get(0..13), "Hello, world!");
        assert_eq!(line.get(0..14), "Hello, world!");
        assert_eq!(line.get(1..14), "ello, world!");
        assert_eq!(line.get(14..16), "");
    }

    #[test]
    fn test_len() {
        let line = Line::from("Hello, world!");
        assert_eq!(line.len(), 13);
    }

    #[test]
    fn zero_width_replaced_with_mid_dot() {
        let line = Line::from("\u{200B}");
        assert_eq!(line.get(0..1), "·");
    }

    #[test]
    fn wide_char_truncated_shows_ellipsis() {
        let line = Line::from("👋");
        assert_eq!(line.get(0..1), "⋯");
        assert_eq!(line.get(0..2), "👋");
    }

    #[test]
    fn insert_at_start() {
        let mut line = Line::from("ello");
        line.insert(0, 'H');
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hello");
    }

    #[test]
    fn insert_in_middle() {
        let mut line = Line::from("Helo");
        line.insert(2, 'l');
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hello");
    }

    #[test]
    fn insert_at_end() {
        let mut line = Line::from("Hello");
        let end = line.len();
        line.insert(end, '!');
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hello!");
    }

    #[test]
    fn insert_beyond_end_appends() {
        let mut line = Line::from("Hello");
        line.insert(100, 'X');
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "HelloX");
    }

    #[test]
    fn insert_wide_grapheme() {
        let mut line = Line::from("ab");
        line.insert(1, '👋');
        let full_width = line.position_of(line.len());
        assert_eq!(full_width, 4);
        assert_eq!(line.get(0..full_width), "a👋b");
    }

    #[test]
    fn delete_at_start() {
        let mut line = Line::from("Hello");
        assert!(line.delete(0));
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "ello");
        assert_eq!(line.len(), 4);
    }

    #[test]
    fn delete_in_middle() {
        let mut line = Line::from("Hxllo");
        assert!(line.delete(2));
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hxlo");
        assert_eq!(line.len(), 4);
    }

    #[test]
    fn delete_at_end() {
        let mut line = Line::from("Hello!");
        let last = line.len() - 1;
        assert!(line.delete(last));
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hello");
        assert_eq!(line.len(), 5);
    }

    #[test]
    fn delete_beyond_end_noop() {
        let mut line = Line::from("Hello");
        assert!(!line.delete(100));
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hello");
        assert_eq!(line.len(), 5);
    }

    #[test]
    fn delete_wide_grapheme() {
        let mut line = Line::from("a👋b");
        // Positions are grapheme indices: [a, 👋, b]
        assert_eq!(line.len(), 3);
        // Before delete, total rendered width is 4 (a=1, 👋=2, b=1)
        let full_width_before = line.position_of(line.len());
        assert_eq!(full_width_before, 4);

        assert!(line.delete(1)); // remove the 👋

        // After delete, width should drop to 2 and content be "ab"
        let full_width_after = line.position_of(line.len());
        assert_eq!(full_width_after, 2);
        assert_eq!(line.get(0..full_width_after), "ab");
        assert_eq!(line.len(), 2);
    }

    #[test]
    fn delete_on_empty_line_noop() {
        let mut line = Line::from("");
        assert!(!line.delete(0));
        assert_eq!(line.len(), 0);
        assert_eq!(line.get(0..0), "");
    }
}
