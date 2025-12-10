# richparse

A Rust library for parsing rich text markup into ANSI escape sequences for terminal output.

`richparse` allows you to define styled text using a simple HTML-like tag syntax within your strings, and it automatically converts these into the appropriate ANSI escape codes for display in modern terminal emulators. This enables easy creation of colorful and formatted command-line interfaces.

## Features

### Colors (Foreground & Background)

You can set the foreground color using `<color_name>` or `<color=...>` tags. 
For background colors, use `<bg_color_name>` or `<bg=...>` tags.

<background=...> and <fg=...> are also valid syntax


**Standard Colors:**
*   `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
*   `bright_black` (or `bright black`), `bright_red`, `bright_green`, ... `bright_white`

**Tag Examples:**
*   `<red>Text</red>` (Foreground Red)
*   `<bg_blue>Text</bg_blue>` (Background Blue)
*   `<color=red>Text</color>` (Explicit Foreground)
*   `<bg=green>Text</bg>` (Explicit Background)
*   `<fg=bright yellow>Text</fg>` (Foreground Bright Yellow)

**Extended Colors:**
*   **256 Colors:** `<color=123>`, `<bg=208>` (using ANSI 0-255 codes)
*   **RGB Hex:** `<color=#FF0000>`, `<bg=#0000FF>`

### Text Modifiers

| Modifier | Tag | Alias | 
| :--- | :--- | :--- |
| **Bold** | `<bold>` | `<b>` |
| **Italic** | `<italic>` | `<i>` |
| **Underline** | `<underline>` | `<u>` |
| **Double Underline** | `<dunderline>` | `<uu>` |
| **Curly Underline** | `<cunderline>` | `<cu>` |
| **Strikethrough** | `<strikethrough>` | `<s>` |
| **Dim** | `<dim>` | | 
| **Blink** | `<blink>` | | 
| **Inverse** | `<inverse>` | | 
| **Hidden** | `<hidden>` | | 
| **Clear** | `<clear>` | | 
| **Hyperlink** | `<link=url>` | | 
| **Overline** | `<overline>` | `<o>` |

*   `hidden`
*   `hyperlink` (`<link=url>`)
*   `overline`

### Escaping Tags

If you want to display a literal `<` character that starts a tag-like sequence, you can escape it by doubling the character: `<<`.

Example: `<<red>` will be rendered as the text `<red>` instead of changing the color.

### Underline Colors

You can set the color of the underline decoration (if supported by your terminal) using `underline_color`, or specifically for curly/double underlines.

*   `<u=red>Red Underline</u>`
*   `<uu=#00FF00>Green Double Underline</uu>`
*   `<cu=blue>Blue Curly Underline</cu>`

## Installation

Add `richparse` to your `Cargo.toml` dependencies:

```toml
[dependencies]
richparse = "0.1" # Or the latest version available on crates.io
```

To enable Fluent (i18n) integration:

```toml
[dependencies]
richparse = { version = "0.1", features = ["intl"] }
```

## Usage

Here's a quick example of how to use `richparse`:

```rust
use richparse::{rich, Color, Style};

fn main() {
    // Basic colored text
    println!("{}", rich!("<red>Hello</red> <blue><bold>World!</bold></blue>"));

    // Combining modifiers and colors
    println!("{}", rich!("<u=green>Green Underlined Text</u=green> and <o>Overlined</o>"));
    
    // 256-colors
    println!("{}", rich!("<fg=208>Orange 256-color</fg> and <bg=123><black>256-color BG</black></bg>"));

    // RGB Hex colors
    println!("{}", rich!("<color=#FF00FF>Magenta RGB</color> on <bg=#008888>Teal BG</bg>"));

    // Hyperlinks
    println!("{}", rich!("Visit our <link=https://example.com>Example Website</link>"));

    // Escaping tags
    println!("{}", rich!("This is a literal <<red> tag."));
    
    // Clear tag
    println!("{}", rich!("<red>This is red <clear>but this is default</clear> again red</red>"));

    // You can also build RichString programmatically
    let custom_style = Style::new().fg(Color::BrightMagenta).underline().double_underline();
    let programmatic_text = richparse::RichString::new(vec![
        richparse::Span::new("Programmatic Text", custom_style)
    ]);
    println!("{}", programmatic_text);
}
```

### With Fluent (i18n) Integration

If you enabled the `intl` feature, you can pass `RichString` directly to Fluent bundles:

```toml
# Cargo.toml
[dependencies]
richparse = { version = "0.1", features = ["intl"] }
```

```rust
// main.rs or a new example file
#[cfg(feature = "intl")]
fn main() {
    use fluent_bundle::{FluentBundle, FluentResource, FluentArgs};
    use richparse::{rich, RichString};
    use unic_langid::langid;

    let ftl_string = "welcome-message = Welcome, {$userName}! Enjoy the {\"<red><bold>\"}Rich{\"</bold></red>\"} experience.";
    let res = FluentResource::try_new(ftl_string.to_string()).expect("Failed to parse FTL.");

    let langid_en = langid!("en-US");
    let mut bundle = FluentBundle::new(vec![langid_en]);
    bundle.add_resource(res).expect("Failed to add resource.");

    let rich_name = rich!("<blue>Jane Doe</blue>"); // RichString passed as an argument

    let mut args = FluentArgs::new();
    args.set("userName", rich_name);

    let msg = bundle.get_message("welcome-message").expect("Message not found.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);

    println!("{}", value);
}

#[cfg(not(feature = "intl"))]
fn main() {
    println!("This example requires the 'intl' feature to be enabled.");
    println!("Run with: cargo run --features intl --example fluent_example");
}
```

For more examples, refer to the `examples/test.rs` file in the repository.

## Zero-Copy & Lifetimes

`richparse` is designed to be zero-copy where possible. The `RichString`, `Span`, and `Style` structs carry a lifetime parameter `'a` and borrow text from the input string. This ensures high performance but means you cannot return a `RichString` created from a temporary string (unless you use static strings or manage the lifetime yourself).

If you need to keep the `RichString` longer than the input string, you can use `.into_owned()` to convert it to `RichString<'static>`:

```rust
let rich = {
    let input = String::from("<red>Temporary</red>");
    richparse::RichString::parse(&input).into_owned()
};
// `rich` is now independent of `input`.
```

## License

This project is licensed under the [MIT License](LICENSE).