extern crate proc_macro;

use litrs::StringLit;
use proc_macro::{Group, Literal, TokenStream, TokenTree};

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

#[proc_macro_attribute]
pub fn amplify(_attr: TokenStream, item: TokenStream) -> TokenStream {
    make_uppercase(item)
}
