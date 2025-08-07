use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Copy, Clone)]
pub enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    fn width(&self) -> usize {
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
        let fragments = line_str
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
            .collect();

        Self { fragments }
    }


    fn replacement_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && for_str.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
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
}
