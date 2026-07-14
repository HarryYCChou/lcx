//! Render LeetCode's HTML problem statements into styled `ratatui` text.
//!
//! LeetCode descriptions use inline markup such as `<code>` (rendered by the
//! website with a grey background), `<strong>`/`<b>` (bold), `<em>` (italic),
//! superscripts, lists, and `<pre>` code blocks. We convert the HTML to rich,
//! annotated text with `html2text` and map each annotation to a terminal style
//! so the panes read much closer to the website.

use html2text::render::text_renderer::RichAnnotation;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};

/// Convert an HTML problem statement into styled terminal text, wrapping at
/// `width` columns.
pub fn render_html(html: &str, width: usize) -> Text<'static> {
    let width = width.max(20);
    let rich = html2text::from_read_rich(html.as_bytes(), width);

    let mut lines: Vec<Line<'static>> = Vec::new();
    for tagged_line in rich {
        let mut spans: Vec<Span<'static>> = Vec::new();
        for ts in tagged_line.tagged_strings() {
            let (text, style) = styled_piece(&ts.s, &ts.tag);
            if !text.is_empty() {
                spans.push(Span::styled(text, style));
            }
        }
        lines.push(Line::from(spans));
    }
    Text::from(lines)
}

/// Build the display text and style for one annotated piece.
fn styled_piece(raw: &str, tags: &[RichAnnotation]) -> (String, Style) {
    let mut style = Style::default();
    let mut text = raw.to_string();

    let has = |pred: fn(&RichAnnotation) -> bool| tags.iter().any(pred);
    let is_strong = has(|a| matches!(a, RichAnnotation::Strong));
    let is_code = has(|a| matches!(a, RichAnnotation::Code | RichAnnotation::Preformat(_)));

    // html2text wraps strong/code content with `*`/`` ` `` markers; strip them
    // since we apply real styling instead.
    if is_strong {
        text = strip_edges(&text, '*');
    }
    if is_code {
        text = strip_edges(&text, '`');
    }

    for tag in tags {
        match tag {
            RichAnnotation::Strong => style = style.add_modifier(Modifier::BOLD),
            RichAnnotation::Emphasis => style = style.add_modifier(Modifier::ITALIC),
            RichAnnotation::Strikeout => style = style.add_modifier(Modifier::CROSSED_OUT),
            RichAnnotation::Code | RichAnnotation::Preformat(_) => {
                // Inline code / code blocks: grey background like the website.
                style = style
                    .bg(Color::Rgb(60, 60, 60))
                    .fg(Color::Rgb(235, 235, 235));
            }
            RichAnnotation::Link(_) => {
                style = style
                    .fg(Color::Rgb(88, 166, 255))
                    .add_modifier(Modifier::UNDERLINED);
            }
            RichAnnotation::Colour(c) => style = style.fg(Color::Rgb(c.r, c.g, c.b)),
            RichAnnotation::BgColour(c) => style = style.bg(Color::Rgb(c.r, c.g, c.b)),
            _ => {}
        }
    }

    (text, style)
}

/// Remove at most one occurrence of `marker` from each end of `s`.
fn strip_edges(s: &str, marker: char) -> String {
    let trimmed = s.strip_prefix(marker).unwrap_or(s);
    trimmed.strip_suffix(marker).unwrap_or(trimmed).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Inline `<code>` gets a background; `<strong>` gets bold, and neither
    /// leaks its `*`/`` ` `` markers into the visible text.
    #[test]
    fn styles_code_and_strong() {
        let text = render_html("<p>Given <code>nums</code> and <strong>k</strong>.</p>", 80);
        let spans: Vec<_> = text.lines.iter().flat_map(|l| l.spans.iter()).collect();

        let code = spans
            .iter()
            .find(|s| s.content.contains("nums"))
            .expect("code span");
        assert_eq!(code.content.as_ref(), "nums");
        assert!(code.style.bg.is_some(), "code should have a background");

        let strong = spans
            .iter()
            .find(|s| s.content.contains('k') && s.content.len() == 1)
            .expect("strong span");
        assert_eq!(strong.content.as_ref(), "k");
        assert!(strong.style.add_modifier.contains(Modifier::BOLD));
    }
}
