use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Default,
    Ansi256(u8),
    Rgb(u8, u8, u8),
}

impl Color {
    pub fn as_ansi_fg(&self) -> String {
        match self {
            Color::Black => "\x1b[30m".to_string(),
            Color::Red => "\x1b[31m".to_string(),
            Color::Green => "\x1b[32m".to_string(),
            Color::Yellow => "\x1b[33m".to_string(),
            Color::Blue => "\x1b[34m".to_string(),
            Color::Magenta => "\x1b[35m".to_string(),
            Color::Cyan => "\x1b[36m".to_string(),
            Color::White => "\x1b[37m".to_string(),
            Color::BrightBlack => "\x1b[90m".to_string(),
            Color::BrightRed => "\x1b[91m".to_string(),
            Color::BrightGreen => "\x1b[92m".to_string(),
            Color::BrightYellow => "\x1b[93m".to_string(),
            Color::BrightBlue => "\x1b[94m".to_string(),
            Color::BrightMagenta => "\x1b[95m".to_string(),
            Color::BrightCyan => "\x1b[96m".to_string(),
            Color::BrightWhite => "\x1b[97m".to_string(),
            Color::Default => "\x1b[39m".to_string(),
            Color::Ansi256(n) => format!("\x1b[38;5;{}m", n),
            Color::Rgb(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
        }
    }

    pub fn as_ansi_bg(&self) -> String {
        match self {
            // Standard colors mapped to 256-color palette
            Color::Black => "\x1b[40m".to_string(),
            Color::Red => "\x1b[41m".to_string(),
            Color::Green => "\x1b[42m".to_string(),
            Color::Yellow => "\x1b[43m".to_string(),
            Color::Blue => "\x1b[44m".to_string(),
            Color::Magenta => "\x1b[45m".to_string(),
            Color::Cyan => "\x1b[46m".to_string(),
            Color::White => "\x1b[47m".to_string(),
            Color::BrightBlack => "\x1b[100m".to_string(),
            Color::BrightRed => "\x1b[101m".to_string(),
            Color::BrightGreen => "\x1b[102m".to_string(),
            Color::BrightYellow => "\x1b[103m".to_string(),
            Color::BrightBlue => "\x1b[104m".to_string(),
            Color::BrightMagenta => "\x1b[105m".to_string(),
            Color::BrightCyan => "\x1b[106m".to_string(),
            Color::BrightWhite => "\x1b[107m".to_string(),
            Color::Default => "\x1b[49m".to_string(), // Reset
            
            //Additonal color mapping
            Color::Ansi256(n) => format!("\x1b[48;5;{}m", n),
            Color::Rgb(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
        }
    }

    pub fn as_ansi_underline_color(&self) -> String {
        match self {
            // Standard colors mapped to 256-color palette
            Color::Black => "\x1b[58;5;0m".to_string(),
            Color::Red => "\x1b[58;5;1m".to_string(),
            Color::Green => "\x1b[58;5;2m".to_string(),
            Color::Yellow => "\x1b[58;5;3m".to_string(),
            Color::Blue => "\x1b[58;5;4m".to_string(),
            Color::Magenta => "\x1b[58;5;5m".to_string(),
            Color::Cyan => "\x1b[58;5;6m".to_string(),
            Color::White => "\x1b[58;5;7m".to_string(),
            Color::BrightBlack => "\x1b[58;5;8m".to_string(),
            Color::BrightRed => "\x1b[58;5;9m".to_string(),
            Color::BrightGreen => "\x1b[58;5;10m".to_string(),
            Color::BrightYellow => "\x1b[58;5;11m".to_string(),
            Color::BrightBlue => "\x1b[58;5;12m".to_string(),
            Color::BrightMagenta => "\x1b[58;5;13m".to_string(),
            Color::BrightCyan => "\x1b[58;5;14m".to_string(),
            Color::BrightWhite => "\x1b[58;5;15m".to_string(),
            Color::Default => "\x1b[59m".to_string(), // Reset
            
            // Additional color mapping
            Color::Ansi256(n) => format!("\x1b[58;5;{}m", n),
            Color::Rgb(r, g, b) => format!("\x1b[58;2;{};{};{}m", r, g, b),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Style<'a> {
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub double_underline: bool,
    pub curly_underline: bool,
    pub overline: bool,
    pub strikethrough: bool,
    pub dim: bool,
    pub blink: bool,
    pub inverse: bool,
    pub hidden: bool,
    pub url: Option<Cow<'a, str>>,
    pub underline_color: Option<Color>,
}

impl<'a> Default for Style<'a> {
    fn default() -> Self {
        Self {
            fg: Color::Default,
            bg: Color::Default,
            bold: false,
            italic: false,
            underline: false,
            double_underline: false,
            curly_underline: false,
            overline: false,
            strikethrough: false,
            dim: false,
            blink: false,
            inverse: false,
            hidden: false,
            url: None,
            underline_color: None,
        }
    }
}

impl<'a> Style<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }
    
    pub fn underline_color(mut self, color: Color) -> Self {
        self.underline_color = Some(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn double_underline(mut self) -> Self {
        self.double_underline = true;
        self
    }

    pub fn curly_underline(mut self) -> Self {
        self.curly_underline = true;
        self
    }

    pub fn overline(mut self) -> Self {
        self.overline = true;
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

   pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

   pub fn blink(mut self) -> Self {
        self.blink = true;
        self
    }

   pub fn inverse(mut self) -> Self {
        self.inverse = true;
        self
    }

    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    pub fn url<S: Into<Cow<'a, str>>>(mut self, url: S) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn into_owned(self) -> Style<'static> {
        Style {
            fg: self.fg,
            bg: self.bg,
            bold: self.bold,
            italic: self.italic,
            underline: self.underline,
            double_underline: self.double_underline,
            curly_underline: self.curly_underline,
            overline: self.overline,
            strikethrough: self.strikethrough,
            dim: self.dim,
            blink: self.blink,
            inverse: self.inverse,
            hidden: self.hidden,
            url: self.url.map(|u| Cow::Owned(u.into_owned())),
            underline_color: self.underline_color,
        }
    }
}