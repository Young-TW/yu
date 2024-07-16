use fluent::FluentResource;
use fluent::FluentBundle;

use unic_langid::LanguageIdentifier;

pub fn load_resource(langid: &LanguageIdentifier, file: &str) -> FluentResource {
    println!("Loading resource for language: {:?}", langid);
    FluentResource::try_new(file.to_string()).expect("Failed to parse FluentResource")
}

pub fn print_message<R: std::borrow::Borrow<fluent::FluentResource>>(bundle: FluentBundle<R>, message: &str) {
    let msg = bundle.get_message(message).expect("Message doesn't exist.");
    let pattern = msg.value().expect("Message has no value.");
    let mut errors = vec![];
    let value = bundle.format_pattern(pattern, None, &mut errors);
    print!("{}", value);
}
