use ::clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
    /// who the greeting is for
    name: Option<String>,
}

fn greet(name: Option<String>) {
    match name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, World!"),
    };
}

fn main() {
    let args = Cli::parse();

    greet(args.name);
}
