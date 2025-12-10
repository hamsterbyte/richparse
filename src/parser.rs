use crate::span::Span;
use crate::style::{Color, Style};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RichString {
    pub spans: Vec<Span>,
}

impl RichString {
    pub fn new(spans: Vec<Span>) -> Self {
        Self { spans }
    }

    pub fn parse(input: &str) -> Self {
        let mut spans = Vec::new();
        let mut style_stack = vec![Style::default()];
        let mut current_text = String::new();
        
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '<' {
                // Check if escaped (e.g. "<<" -> "<")
                if let Some(&'<') = chars.peek() {
                    chars.next(); // consume the second '<'
                    current_text.push('<');
                    continue;
                }

                // Potential tag start
                let mut tag_content = String::new();
                let mut is_closing = false;
                
                // Peek to see if it's a closing tag
                if let Some(&'/') = chars.peek() {
                    is_closing = true;
                    chars.next(); // consume '/'
                }

                // Read until '>'
                let mut valid_tag = false;
                while let Some(&next_c) = chars.peek() {
                    if next_c == '>' {
                        chars.next(); // consume '>'
                        valid_tag = true;
                        break;
                    }
                    tag_content.push(chars.next().unwrap());
                }

                if valid_tag {
                    // Flush current text if any
                    if !current_text.is_empty() {
                        // Current style is the top of the stack
                        let current_style = style_stack.last().cloned().unwrap_or_default();
                        spans.push(Span::new(&current_text, current_style));
                        current_text.clear();
                    }

                    if is_closing {
                        // Pop style if stack has more than base style
                        if style_stack.len() > 1 {
                            style_stack.pop();
                        }
                    } else {
                        // Push new style based on current + tag
                        let current_style = style_stack.last().cloned().unwrap_or_default();
                        let new_style = apply_tag(&tag_content, current_style);
                        style_stack.push(new_style);
                    }
                } else {
                    current_text.push('<');
                    if is_closing { current_text.push('/'); }
                    current_text.push_str(&tag_content);
                }
            } else {
                current_text.push(c);
            }
        }

        // Flush remaining text
        if !current_text.is_empty() {
            let current_style = style_stack.last().cloned().unwrap_or_default();
            spans.push(Span::new(&current_text, current_style));
        }

        Self { spans }
    }
}

fn apply_tag(tag: &str, mut style: Style) -> Style {
    let parts: Vec<&str> = tag.split('=').collect();
    let key = parts[0].trim().to_lowercase();
    let value = if parts.len() > 1 { Some(parts[1].trim().to_lowercase()) } else { None };

    match (key.as_str(), value) {
        // Explicit color=...
        ("color" | "fg", Some(val)) => {
            if let Some(c) = parse_color(&val) {
                style.fg = c;
            }
        }
        // Explicit bg=...
        ("background" | "bg", Some(val)) => {
            if let Some(c) = parse_color(&val) {
                style.bg = c;
            }
        }
        
        // Explicit link=...
        ("link", Some(val)) => {
            style.url = Some(val.to_string());
        }

        // Explicit curly underline with color
        ("cu" | "cunderline", Some(val)) => {
            style.curly_underline = true;
            if let Some(c) = parse_color(&val) {
                style.underline_color = Some(c);
            }
        }

        // Explicit underline with color
        ("u" | "underline", Some(val)) => {
            style.underline = true;
            if let Some(c) = parse_color(&val) {
                style.underline_color = Some(c);
            }
        }

        // Explicit double underline with color
        ("uu" | "dunderline", Some(val)) => {
            style.double_underline = true;
            if let Some(c) = parse_color(&val) {
                style.underline_color = Some(c);
            }
        }

        // Standard tags
        (k, None) => match k {
            // Colors
            "black" => style.fg = Color::Black,
            "red" => style.fg = Color::Red,
            "green" => style.fg = Color::Green,
            "yellow" => style.fg = Color::Yellow,
            "blue" => style.fg = Color::Blue,
            "magenta" => style.fg = Color::Magenta,
            "cyan" => style.fg = Color::Cyan,
            "white" => style.fg = Color::White,
            
            // Backgrounds (convention: bg_color)
            "bg_black" => style.bg = Color::Black,
            "bg_red" => style.bg = Color::Red,
            "bg_green" => style.bg = Color::Green,
            "bg_yellow" => style.bg = Color::Yellow,
            "bg_blue" => style.bg = Color::Blue,
            "bg_magenta" => style.bg = Color::Magenta,
            "bg_cyan" => style.bg = Color::Cyan,
            "bg_white" => style.bg = Color::White,

            // Modifiers
            "b" | "bold" => style.bold = true,
            "i" | "italic" => style.italic = true,
            "u" | "underline" => style.underline = true,
            "uu" | "dunderline" => style.double_underline = true,
            "cu" | "cunderline" => style.curly_underline = true,
            "o" | "overline" => style.overline = true,
            "s" | "strikethrough" => style.strikethrough = true,
            "dim" => style.dim = true,
            "blink" => style.blink =true,
            "hidden" => style.hidden = true,
            "inverse" => style.inverse = true,
            
            // Reset/Default
            "clear" => style = Style::default(),
            
            _ => {}
        },
        _ => {}
    }
    style
}

fn parse_color(name: &str) -> Option<Color> {
    match name {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "bright_black" | "bright black" => Some(Color::BrightBlack),
        "bright_red" | "bright red" => Some(Color::BrightRed),
        "bright_green" | "bright green" => Some(Color::BrightGreen),
        "bright_yellow" | "bright yellow" => Some(Color::BrightYellow),
        "bright_blue" | "bright blue" => Some(Color::BrightBlue),
        "bright_magenta" | "bright magenta" => Some(Color::BrightMagenta),
        "bright_cyan" | "bright cyan" => Some(Color::BrightCyan),
        "bright_white" | "bright white" => Some(Color::BrightWhite),
        "white" => Some(Color::White),
        _ => {
            // Check for hex color: #RRGGBB
            if name.starts_with('#') && name.len() == 7 {
                let r = u8::from_str_radix(&name[1..3], 16).ok();
                let g = u8::from_str_radix(&name[3..5], 16).ok();
                let b = u8::from_str_radix(&name[5..7], 16).ok();
                
                if let (Some(r), Some(g), Some(b)) = (r, g, b) {
                    return Some(Color::Rgb(r, g, b));
                }
            }
            
            // Try parsing as 256-color code
            name.parse::<u8>().ok().map(Color::Ansi256)
        }
    }
}

impl fmt::Display for RichString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for span in &self.spans {
            write!(f, "{}", span)?;
        }
        Ok(())
    }
}

#[cfg(feature = "intl")]
impl<'source> From<RichString> for fluent_bundle::FluentValue<'source> {
    fn from(rs: RichString) -> Self {
        fluent_bundle::FluentValue::String(std::borrow::Cow::Owned(rs.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_clear_tag() {
        let rs = RichString::parse("<red>Red <clear>Clear</clear> Red</red>");
        assert_eq!(rs.spans.len(), 3);
        
        assert_eq!(rs.spans[0].text, "Red ");
        assert_eq!(rs.spans[0].style.fg, Color::Red);
        
        assert_eq!(rs.spans[1].text, "Clear");
        assert_eq!(rs.spans[1].style, Style::default()); // Should be completely default
        
        assert_eq!(rs.spans[2].text, " Red");
        assert_eq!(rs.spans[2].style.fg, Color::Red);
    }

    #[test]
    fn test_parse_escaped_tag() {
        let rs = RichString::parse("Escaped <<red>Tag");
        assert_eq!(rs.spans.len(), 1);
        assert_eq!(rs.spans[0].text, "Escaped <red>Tag");
        assert_eq!(rs.spans[0].style, Style::default());
    }

    #[test]
    fn test_parse_curly_underline_color() {
        let rs = RichString::parse("<cu=red>Curly Red</cu>");
        assert_eq!(rs.spans.len(), 1);
        assert_eq!(rs.spans[0].text, "Curly Red");
        assert_eq!(rs.spans[0].style.curly_underline, true);
        assert_eq!(rs.spans[0].style.underline_color, Some(Color::Red));
    }

    #[test]
    fn test_parse_link() {
        let rs = RichString::parse("<link=https://example.com>Click Me</link>");
        assert_eq!(rs.spans.len(), 1);
        assert_eq!(rs.spans[0].text, "Click Me");
        assert_eq!(rs.spans[0].style.url, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_parse_bright_colors() {
        let rs = RichString::parse("<color=bright red>Bright Red FG</color> <bg=bright blue>Bright Blue BG</bg>");
        assert_eq!(rs.spans.len(), 3);
        
        assert_eq!(rs.spans[0].text, "Bright Red FG");
        assert_eq!(rs.spans[0].style.fg, Color::BrightRed);
        
        assert_eq!(rs.spans[2].text, "Bright Blue BG");
        assert_eq!(rs.spans[2].style.bg, Color::BrightBlue);
    }

    #[test]
    fn test_parse_rgb_hex() {
        let rs = RichString::parse("<color=#FF0000>RedHex</color> <bg=#00FF00>GreenHex</bg>");
        assert_eq!(rs.spans.len(), 3);
        
        assert_eq!(rs.spans[0].text, "RedHex");
        assert_eq!(rs.spans[0].style.fg, Color::Rgb(255, 0, 0));
        
        assert_eq!(rs.spans[2].text, "GreenHex");
        assert_eq!(rs.spans[2].style.bg, Color::Rgb(0, 255, 0));
    }

    #[test]
    fn test_parse_ansi256_colors() {
        let rs = RichString::parse("<color=123>Foreground 123</color> <bg=200>Background 200</bg>");
        assert_eq!(rs.spans.len(), 3);
        
        assert_eq!(rs.spans[0].text, "Foreground 123");
        assert_eq!(rs.spans[0].style.fg, Color::Ansi256(123));
        
        assert_eq!(rs.spans[1].text, " ");
        assert_eq!(rs.spans[1].style, Style::default());
        
        assert_eq!(rs.spans[2].text, "Background 200");
        assert_eq!(rs.spans[2].style.bg, Color::Ansi256(200));
    }

    #[test]
    fn test_parse_kv_tags() {
        let rs = RichString::parse("KV: <color=red>Red</color> <bg=blue>BgBlue</bg>");
        assert_eq!(rs.spans.len(), 4); // "KV: ", "Red", " ", "BgBlue"
        
        assert_eq!(rs.spans[1].text, "Red");
        assert_eq!(rs.spans[1].style.fg, Color::Red);
        
        assert_eq!(rs.spans[3].text, "BgBlue");
        assert_eq!(rs.spans[3].style.bg, Color::Blue);
    }

    #[test]
    fn test_parse_strikethrough() {
        let rs = RichString::parse("<s>Strike</s>");
        assert_eq!(rs.spans.len(), 1);
        assert_eq!(rs.spans[0].text, "Strike");
        assert_eq!(rs.spans[0].style.strikethrough, true);
    }

    #[test]
    fn test_parse_simple() {
        let rs = RichString::parse("Hello <red>World</red>");
        assert_eq!(rs.spans.len(), 2);
        assert_eq!(rs.spans[0].text, "Hello ");
        assert_eq!(rs.spans[0].style, Style::default());
        assert_eq!(rs.spans[1].text, "World");
        assert_eq!(rs.spans[1].style.fg, Color::Red);
    }

    #[test]
    fn test_parse_nested() {
        let rs = RichString::parse("<red>R <blue>B</blue> R</red>");
       
        assert_eq!(rs.spans.len(), 3);
        assert_eq!(rs.spans[0].text, "R "); 
        assert_eq!(rs.spans[0].style.fg, Color::Red);
        
        assert_eq!(rs.spans[1].text, "B");
        assert_eq!(rs.spans[1].style.fg, Color::Blue); // Overrides red? Yes, logic replaces FG.
        
        assert_eq!(rs.spans[2].text, " R");
        assert_eq!(rs.spans[2].style.fg, Color::Red);
    }
}
