use clap::Parser;
use macro_fun::{amplify, amplify_attr, dev_only};
use serde::{Deserialize, Serialize};
use serde_json;

fn main() {
    dev_only! {
        println!("This line won't be in the release version!");
    };

    // demo amplifying the CLI help and greeting strings
    let args = Cli::parse();
    greet(args.name);

    // demo limitation of amplifying struct field names with serde
    show_json();

    // targeted amplification using function-like macro
    amplify!(println!("I love shouting!"));
}

/// A simple command line greeting for demo of Rust.
// [amplify_attr] must be top-most attribute to modify the underlying derive macro.
// It will AMPLIFY the doc strings.
// TODO: get it to AMPLIFY the struct field names in the CLI parse output
#[amplify_attr]
#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
    /// who to greet
    name: Option<String>,
}

#[amplify_attr] // will AMPLIFY all str literals in the function
fn greet(name: Option<String>) {
    match name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, World!"),
    };
}

#[amplify_attr] // TODO: figure out how to make it AMPLIFY the struct fields on serialize
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    social_security_number: String,
}

#[amplify_attr] // shows issue with json serialization names not getting AMPLIFIED
fn show_json() {
    let dude = Person {
        name: "Bob".to_owned(),
        age: 45,
        social_security_number: "123-45-6789".to_owned(),
    };

    let json = serde_json::to_string(&dude).unwrap();
    println!("Here's some json: {}", json);
}
