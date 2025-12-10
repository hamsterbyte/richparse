use richparse::{RichString, Color, Style, rich};

fn main() {  
    single_tests();
    multiple_test();
    custom_span_test();
    
}

fn single_tests() {

    let mut t = Vec::new();

    // Standard Foreground Colors
    t.push("Standard Colors:");
    t.push("<black>Black</black>");
    t.push("<red>Red</red>");
    t.push("<green>Green</green>");
    t.push("<yellow>Yellow</yellow>");
    t.push("<blue>Blue</blue>");
    t.push("<magenta>Magenta</magenta>");
    t.push("<cyan>Cyan</cyan>");
    t.push("<white>White</white>");

    // Standard Background Colors
    t.push("\nBackground Colors:");
    t.push("<bg_black>BG Black</bg_black>");
    t.push("<bg_red>BG Red</bg_red>");
    t.push("<bg_green>BG Green</bg_green>");
    t.push("<bg_yellow>BG Yellow</bg_yellow>");
    t.push("<bg_blue>BG Blue</bg_blue>");
    t.push("<bg_magenta>BG Magenta</bg_magenta>");
    t.push("<bg_cyan>BG Cyan</bg_cyan>");
    t.push("<bg_white>BG White</bg_white>");

    // Bright Colors (space-separated)
    t.push("\nBright Colors (space-separated):");
    t.push("<color=bright black>Bright Black</color>");
    t.push("<color=bright red>Bright Red</color>");
    t.push("<color=bright green>Bright Green</color>");
    t.push("<color=bright yellow>Bright Yellow</color>");
    t.push("<color=bright blue>Bright Blue</color>");
    t.push("<color=bright magenta>Bright Magenta</color>");
    t.push("<color=bright cyan>Bright Cyan</color>");
    t.push("<color=bright white>Bright White</color>");
    t.push("<bg=bright red>BG Bright Red</bg>");

    // Modifiers
    t.push("\nModifiers:");
    t.push("<bold>Bold</bold>");
    t.push("<b>Bold (alias)</b>");
    t.push("<italic>Italic</italic>");
    t.push("<i>Italic (alias)</i>");
    t.push("<underline>Underline</underline>");
    t.push("<u>Underline (alias)</u>");
    t.push("<u=blue>Underline Blue</u>");
    t.push("<dunderline>Double Underline</dunderline>");
    t.push("<uu>Double Underline (alias)</uu>");
    t.push("<uu=red>Double Underline Red</uu>");
    t.push("<cunderline>Curly Underline</cunderline>");
    t.push("<cu>Curly Underline (alias)</cu>");
    t.push("<cu=red>Curly Underline Red</cu>");
    t.push("<cu=bright green>Curly Underline Bright Green</cu>");
    t.push("<overline>Overline</overline>");
    t.push("<o>Overline (alias)</o>");
    t.push("<strikethrough>Strikethrough</strikethrough>");
    t.push("<s>Strikethrough (alias)</s>");
    t.push("<dim>Dim</dim>");
    t.push("<blink>Blink</blink>");
    t.push("<inverse>Inverse</inverse>");
    t.push("<hidden>Hidden (You shouldn't see this)</hidden>");
    t.push("<link=https://example.com>Hyperlink to example.com</link>");
    t.push("<red>Red <clear>Clear (Default Style)</clear> Red</red>");

    // Key-Value Syntax
    t.push("\nKey-Value Syntax:");
    t.push("<color=red>Color=Red</color>");
    t.push("<fg=blue>Fg=Blue</fg>");
    t.push("<bg=yellow><black>Bg=Yellow</black></bg>");
    t.push("<background=green>Background=Green</background>");

    t.push("\nFG Key-Value for other color types:");
    t.push("<fg=208>FG Orange (208)</fg>");
    t.push("<fg=#FF00FF>FG Magenta (#FF00FF)</fg>");
    t.push("<fg=bright green>FG Bright Green</fg>");

    // 256 Colors
    t.push("\n256 Colors:");
    t.push("<color=208>Orange (208)</color>");
    t.push("<color=123>Light Cyan (123)</color>");
    t.push("<bg=55>BG Purple (55)</bg>");

    // RGB Hex Colors
    t.push("\nRGB Hex Colors:");
    t.push("<color=#FF0000>Red (#FF0000)</color>");
    t.push("<color=#00FF00>Green (#00FF00)</color>");
    t.push("<color=#0000FF>Blue (#0000FF)</color>");
    t.push("<color=#FF00FF>Magenta (#FF00FF)</color>");
    t.push("<color=#00FFFF>Cyan (#00FFFF)</color>");

    // Escaping
    t.push("\nEscaping:");
    t.push("This tag is escaped: <<red>Red<<red>");
    t.push("This tag is not: <red>Red</red>");

    for line in t.iter(){
        println!("{}", rich!(line));
    }

}

fn multiple_test() {
    let example_text = 
        "<red>Hello</red> <blue><bold>World!</bold></blue> This is <green>green and <italic>italic</italic></green> text.
        And this is <bg_yellow><black>black text on yellow background</black></bg_yellow>.
        Finally, <underline>underlined text</underline>.";

        let rich_string = rich!(example_text);

        println!("{}", rich_string);

}

fn custom_span_test() {
    // You can also construct RichString manually and print it
    let mut custom_spans = Vec::new();
    custom_spans.push(richparse::Span::new(
        "Custom ",
        Style::new().fg(Color::Cyan),
    ));
    custom_spans.push(richparse::Span::new(
        "Styled ",
        Style::new().fg(Color::Magenta).italic(),
    ));
    custom_spans.push(richparse::Span::new(
        "Text",
        Style::new().fg(Color::Yellow).underline(),
    ));

    custom_spans.push(richparse::Span::new(
        " With Background",
        Style::new().bg(Color::Yellow).fg(Color::Black)
    ));

    let custom_rich_string = RichString::new(custom_spans);
    println!("{}", custom_rich_string);

}
