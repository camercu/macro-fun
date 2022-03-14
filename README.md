# Macro Fun

This is a practice workspace to play around with/learn Rust's procedural macros.

The cool things:
- `dev_only!` macro to remove code at compile time unless you're building in debug mode.
- `amplify!` macro to demo modifying all string literals at compile time... makes them LOUDER ;-p

The macro code is in the `macro-fun` crate. Example usage is in `greet` (a hello world program).

## Limitations

Sadly I haven't figured out a way to modify the string literals derived from struct field names when
`#[derive()]` macro attributes are used in libraries like `clap` and `serde`. Therefore, the
unmodified string literals still show up in the compiled binary.