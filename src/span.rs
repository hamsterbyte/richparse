use std::fmt;
use std::borrow::Cow;
use crate::style::Style;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span<'a> {
    pub text: Cow<'a, str>,
    pub style: Style<'a>,
}

impl<'a> Span<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(text: S, style: Style<'a>) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }

    pub fn into_owned(self) -> Span<'static> {
        Span {
            text: Cow::Owned(self.text.into_owned()),
            style: self.style.into_owned(),
        }
    }
}

impl<'a> fmt::Display for Span<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let style = &self.style;
        
        // Start style
        write!(f, "{}", style.fg.as_ansi_fg())?;
        write!(f, "{}", style.bg.as_ansi_bg())?;
        
        if let Some(ul_color) = &style.underline_color {
            write!(f, "{}", ul_color.as_ansi_underline_color())?;
        }
        
        if style.bold {
            write!(f, "\x1b[1m")?;
        }
        if style.italic {
            write!(f, "\x1b[3m")?;
        }
        if style.underline {
            write!(f, "\x1b[4m")?;
        }
        if style.double_underline {
            write!(f, "\x1b[4:2m")?;
        }
        if style.curly_underline {
            write!(f, "\x1b[4:3m")?;
        }
        if style.overline {
            write!(f, "\x1b[53m")?;
        }
        if style.strikethrough {
            write!(f, "\x1b[9m")?;
        }
        if style.dim {
            write!(f, "\x1b[2m")?;
        }
        if style.blink {
            write!(f, "\x1b[5m")?;
        }
        if style.inverse {
            write!(f, "\x1b[7m")?;
        }
        if style.hidden {
            write!(f, "\x1b[8m")?;
        }

        // Hyperlink start
        if let Some(url) = &style.url {
            write!(f, "\x1b]8;;{}\x1b\\", url)?;
        }

        // Text
        write!(f, "{}", self.text)?;

        // Hyperlink end
        if style.url.is_some() {
            write!(f, "\x1b]8;;\x1b\\")?;
        }

        // Reset all (not optimized)
        write!(f, "\x1b[0m")
    }
}