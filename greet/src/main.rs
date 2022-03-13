use ::clap::Parser;
use amplify::amplify;

/// A simple command line greeting for demo of Rust.
#[amplify] // this must be top-most attribute to modify the underlying derive macro
#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
    /// who to greet
    name: Option<String>,
}

fn main() {
    let args = Cli::parse();

    greet(args.name);
}

#[amplify]
fn greet(name: Option<String>) {
    match name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, World!"),
    };
}
