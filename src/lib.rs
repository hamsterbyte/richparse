//! # richparse
//!
//! `richparse` is a library for parsing rich text markup (e.g., `<red>Hello</red>`) into
//! ANSI escape sequences for terminal output.
//!
//! It is designed to be zero-copy, meaning `RichString`, `Span`, and `Style` borrow
//! the input text whenever possible to avoid allocations.
//!
//! ## Example
//!
//! ```rust
//! use richparse::rich;
//!
//! println!("{}", rich!("<green><bold>Success!</bold></green>"));
//! ```

pub mod style;
pub mod span;
pub mod parser;

pub use style::{Color, Style};
pub use span::Span;
pub use parser::RichString;

#[macro_export]
macro_rules! rich {
    ($input:expr) => {
        $crate::RichString::parse($input)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manual_construction() {
        let style = Style::new().fg(Color::Red).bold();
        let span = Span::new("Hello", style);
        let rich_string = RichString::new(vec![span]);
        

        let expected = "\x1b[31m\x1b[49m\x1b[1mHello\x1b[0m";
        assert_eq!(rich_string.to_string(), expected);
    }
}