use std::fs;
use std::path::PathBuf;

use fluent::FluentResource;
use fluent::FluentBundle;

use unic_langid::LanguageIdentifier;

pub fn load_resource(langid: &LanguageIdentifier, file: &str) -> FluentResource {
    let mut path = PathBuf::from("i18n");
    path.push(langid.to_string());
    path.push(file);

    println!("Loading resource from: {:?}", path);
    let source = fs::read_to_string(path).expect("Failed to read file");
    FluentResource::try_new(source).expect("Failed to parse FluentResource")
}

pub fn print_message<R: std::borrow::Borrow<fluent::FluentResource>>(bundle: FluentBundle<R>, message: &str) {
    let msg = bundle.get_message(message).expect("Message doesn't exist.");
    let pattern = msg.value().expect("Message has no value.");
    let mut errors = vec![];
    let value = bundle.format_pattern(pattern, None, &mut errors);
    print!("{}", value);
}
