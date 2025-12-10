#[cfg(feature = "intl")]
fn main() {
    use fluent_bundle::{FluentBundle, FluentResource, FluentArgs};
    use richparse::rich;
    use unic_langid::langid;

    // 1. Define a simple Fluent resource (usually loaded from a file)
    let ftl_string = "hello-user = Hello, {$userName}! Welcome to the {\"<bold>\"}Rich{\"</bold>\"} world.";
    let res = FluentResource::try_new(ftl_string.to_string())
        .expect("Failed to parse FTL.");

    // 2. Setup the bundle
    let langid_en = langid!("en-US");
    let mut bundle = FluentBundle::new(vec![langid_en]);
    bundle.add_resource(res).expect("Failed to add resource.");

    // 3. Create a RichString argument
    let rich_name = rich!("<red><bold>Alice</bold></red>");

    // 4. Pass it as an argument
    let mut args = FluentArgs::new();
    args.set("userName", rich_name);

    // 5. Format the message
    let msg = bundle.get_message("hello-user").expect("Message not found.");
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
