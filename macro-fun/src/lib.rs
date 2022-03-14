extern crate proc_macro;

use litrs::StringLit;
use proc_macro::{Group, Literal, TokenStream, TokenTree};

/// Turns every string literal in the TokenStream to UPPERCASE
fn make_uppercase(item: TokenStream) -> TokenStream {
    item.into_iter()
        .map(|itm| {
            match itm {
                // pass Group items back through recursively
                TokenTree::Group(grp) => {
                    let upper = make_uppercase(grp.stream());
                    let grp = Group::new(grp.delimiter(), upper);
                    TokenTree::Group(grp)
                }
                // modify string literals
                TokenTree::Literal(lit) => {
                    if let Ok(strlit) = StringLit::try_from(&lit) {
                        // Convert string literals to uppercase versions
                        let upper = strlit.value().to_ascii_uppercase();
                        TokenTree::Literal(Literal::string(&upper))
                    } else {
                        // pass through non-string literals
                        TokenTree::Literal(lit)
                    }
                }
                // let all other tokens pass through
                tt => tt,
            }
        })
        .collect()
}

/// Attribute to turn all contained str-literals to UPPERCASE
#[proc_macro_attribute]
pub fn amplify_attr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = make_uppercase(item);
    println!("{:#?}", item);
    item
}

/// Function-like macro to turn all contained str-literals to UPPERCASE
#[proc_macro]
pub fn amplify(item: TokenStream) -> TokenStream {
    make_uppercase(item)
}

/// Takes any token stream and returns an empty stream. That is, any code
/// sent to blackhole will be removed from the compiled binary.
fn blackhole(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// When in debug mode, returns the code as-is. In release, blackhole's the code.
fn release_blackhole(item: TokenStream) -> TokenStream {
    #[cfg(debug_assertions)]
    {
        item
    }
    #[cfg(not(debug_assertions))]
    {
        blackhole(item)
    }
}

/// Attribute that blackholes contained code if not in debug build
#[proc_macro_attribute]
pub fn dev_only_attr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    release_blackhole(item)
}

/// Function-like macro that blackholes contained code if not in debug build
#[proc_macro]
pub fn dev_only(item: TokenStream) -> TokenStream {
    release_blackhole(item)
}
