//! # rust-primitive_enum
//! Little utility for dealing with C-style enums
//!
//! This crate exports just the single macro `primitive_enum!`
//! that defines an enum backed by a user specified primitive
//! integer type.
//!
//! The intent is to emulate traditional C-style enums while
//! adding some handy associated functions useful in such
//! contexts (e.g. enumerating over each enum and converting
//! between the underlying types).
//!
//! # Example
//!
//! ```rust
//! #[macro_use] extern crate primitive_enum;
//!
//! primitive_enum! { MyEnum u16 ;
//!     A,
//!     B,
//!     C,
//!     D = 500,
//!     E,       // as you would expect, E maps to 501
//! }
//!
//! fn main() {
//!     use MyEnum::*;
//!
//!     // Get a slice of all enum elements:
//!     assert_eq!(
//!         MyEnum::list(),
//!         &[A, B, C, D, E],
//!     );
//!
//!     // Get the enum value given its integer value:
//!     assert_eq!(MyEnum::from(0), Some(A));
//!     assert_eq!(MyEnum::from(1000), None);
//!
//!     // User specified enum values behave as you would expect
//!     assert_eq!(D as u16, 500);
//!     assert_eq!(MyEnum::from(501), Some(E));
//!
//!     // You can also get an enum by its name
//!     assert_eq!(MyEnum::from_name("E"), Some(E));
//! }
//! ```
//!
//! # Expansion
//!
//! As of the current version, the macro
//!
//! ```rust
//! #[macro_use] extern crate primitive_enum;
//!
//! primitive_enum! { MyEnum u16 ;
//!     A,
//!     B,
//!     C,
//!     D = 500,
//!     E,
//! }
//!
//! ```
//!
//! is effectively equivalent to
//!
//! ```rust
//! #[repr(u16)]
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//! pub enum MyEnum {
//!     A = 0,
//!     B = 1,
//!     C = 2,
//!     D = 500,
//!     E = 501,
//! }
//!
//! impl MyEnum {
//!     pub fn from(x: u16) -> Option<MyEnum> {
//!         // ...
//!         None
//!     }
//!
//!     pub fn from_name(name: &str) -> Option<MyEnum> {
//!         // ...
//!         None
//!     }
//!
//!     pub fn list() -> &'static [MyEnum] {
//!         &[
//!             MyEnum::A,
//!             MyEnum::B,
//!             MyEnum::C,
//!             MyEnum::D,
//!             MyEnum::E,
//!         ]
//!     }
//! }
//! ```
//!
//! # Doc comments
//!
//! Starting from version 1.1.0, doc comments are supported.
//!
//! ```rust
//! #[macro_use] extern crate primitive_enum;
//!
//! primitive_enum! {
//! /// Some comments about 'MyEnum'
//! MyEnum u16 ;
//!     A,
//!     B,
//!
//!     /// Some special comments about variant C
//!     C,
//!     D = 500,
//!     E,
//! }
//! ```
//!
//! Starting from version 1.1.0 this crate is implemented as a procedural macro
//! to improve space efficiency of the generated code.
//! Prior to version 1.1.0, this crate was implemented as a simple declarative macro.
//!
//! This crate is a clean macro implementation that
//! expands to code shown above and doesn't rely on any
//! outside dependencies or magic.

extern crate proc_macro;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};
use std::iter::FromIterator;

macro_rules! error {
    ($message:expr $(,)?) => {
        return format!("compile_error!({:?})", $message).parse().unwrap()
    };
}

fn at_punc(peek: &Option<TokenTree>, punc_char: char) -> bool {
    match peek {
        Some(TokenTree::Punct(p)) => p == &punc_char,
        _ => false,
    }
}

fn ident_token(name: &str) -> TokenTree {
    TokenTree::Ident(Ident::new(name, Span::call_site()))
}

fn punct_token(ch: char) -> TokenTree {
    TokenTree::Punct(Punct::new(ch, proc_macro::Spacing::Alone))
}

fn punct_cont_token(ch: char) -> TokenTree {
    TokenTree::Punct(Punct::new(ch, proc_macro::Spacing::Joint))
}

fn punc2_tokens(ch1: char, ch2: char) -> Vec<TokenTree> {
    vec![
        TokenTree::Punct(Punct::new(ch1, proc_macro::Spacing::Joint)),
        TokenTree::Punct(Punct::new(ch2, proc_macro::Spacing::Alone)),
    ]
}

fn int_token(value: i32) -> TokenTree {
    TokenTree::Literal(Literal::i32_unsuffixed(value))
}

fn group_token(delimiter: Delimiter, tokens: Vec<TokenTree>) -> TokenTree {
    TokenTree::Group(Group::new(delimiter, TokenStream::from_iter(tokens)))
}

fn paren_token(tokens: Vec<TokenTree>) -> TokenTree {
    group_token(Delimiter::Parenthesis, tokens)
}

fn bracket_token(tokens: Vec<TokenTree>) -> TokenTree {
    group_token(Delimiter::Bracket, tokens)
}

fn brace_token(tokens: Vec<TokenTree>) -> TokenTree {
    group_token(Delimiter::Brace, tokens)
}

fn concat<T>(mut v1: Vec<T>, mut v2: Vec<T>) -> Vec<T> {
    v1.append(&mut v2);
    v1
}

fn check_for_default(triples: &mut Vec<(TokenStream, Ident, TokenTree)>) {
    let mut default_position: Option<usize> = None;
    for (attributes, _variant_name, variant_value) in triples.into_iter() {
        if attributes.to_string().contains("default") {
            if default_position.is_some() {
                // error!("Multiple variants marked as default");
            }
            default_position = Some(variant_value.to_string().parse::<usize>().unwrap());
        }
    }
    if !default_position.is_some() {
        // No default specified, so we'll just use the first variant
        triples[0].0.extend(vec![
            punct_token('#'),
            bracket_token(vec![ident_token("default")]).into(),
        ]);
    }
}

#[proc_macro]
pub fn primitive_enum(tokens: TokenStream) -> TokenStream {
    let mut iter = tokens.into_iter();
    let mut peek = iter.next();

    ////////////////////////////////////////////////////////////////////
    // Part 1: Parse Contents
    ////////////////////////////////////////////////////////////////////

    let enum_attributes = {
        let mut tokens = Vec::<TokenTree>::new();
        while at_punc(&peek, '#') {
            tokens.push(peek.unwrap());
            peek = iter.next();
            if peek.is_none() {
                error!("Dangling '#'");
            }
            tokens.push(peek.unwrap());
            peek = iter.next();
        }
        tokens
    };

    let enum_identifier = match peek {
        Some(TokenTree::Ident(ident)) => {
            peek = iter.next();
            ident
        }
        Some(token) => error!(format!("Expected enum name but got {:?}", token)),
        None => error!("Expected enum name but got end of macro"),
    };

    let repr_type = {
        let mut tokens = Vec::<TokenTree>::new();
        while peek.is_some() && !at_punc(&peek, ';') {
            tokens.push(peek.unwrap());
            peek = iter.next();
        }
        tokens
    };

    match peek {
        Some(TokenTree::Punct(p)) if p == ';' => {
            peek = iter.next();
        }
        Some(token) => error!(format!("Expected ';' but got {:?}", token)),
        None => error!("Expected ';' but got end of macro"),
    }

    let triples = {
        // Each triple contains information about a variant of the enum.
        // (Attributes, Identifier, Value-Expression)
        let mut triples = Vec::<(TokenStream, Ident, TokenTree)>::new();
        let mut base_value: Option<Vec<TokenTree>> = None;
        let mut offset = 0;
        while peek.is_some() {
            let variant_attributes = {
                let mut tokens = Vec::<TokenTree>::new();
                while at_punc(&peek, '#') {
                    tokens.push(peek.unwrap());
                    peek = iter.next();
                    if peek.is_none() {
                        error!("Dangling '#'");
                    }
                    tokens.push(peek.unwrap());
                    peek = iter.next();
                }
                TokenStream::from_iter(tokens)
            };
            let variant_name = match peek {
                Some(TokenTree::Ident(ident)) => {
                    peek = iter.next();
                    ident
                }
                Some(token) => error!(format!("Expected variant identifier but got {:?}", token)),
                None => error!("Expected variant identifier but got end of macro"),
            };
            if at_punc(&peek, '=') {
                // Explicit assignment
                peek = iter.next(); // consume '='
                let mut expr_tokens = Vec::<TokenTree>::new();
                while peek.is_some() && !at_punc(&peek, ',') {
                    expr_tokens.push(peek.unwrap());
                    peek = iter.next();
                }
                base_value = Some(expr_tokens);
                offset = 0;
            }
            let value = match &base_value {
                Some(base_value_tokens) => {
                    let base_value_rep = if base_value_tokens.len() == 1 {
                        base_value_tokens[0].clone()
                    } else {
                        TokenTree::Group(Group::new(
                            proc_macro::Delimiter::Parenthesis,
                            TokenStream::from_iter(base_value_tokens.clone()),
                        ))
                    };

                    if offset == 0 {
                        base_value_rep
                    } else {
                        paren_token(vec![base_value_rep, punct_token('+'), int_token(offset)])
                    }
                }
                None => int_token(offset),
            };
            if at_punc(&peek, ',') {
                peek = iter.next();
            } else if let Some(token) = peek {
                error!(format!("Expected ',' but got {:?}", token));
            }
            offset += 1;
            triples.push((variant_attributes, variant_name, value));
        }
        check_for_default(&mut triples); // make sure there's a default, if the user didn't specify one
        triples
    };

    ////////////////////////////////////////////////////////////////////
    // Part 2: Code Generation
    ////////////////////////////////////////////////////////////////////

    // Make sure doc comments get passed to the enum itself
    let mut out = enum_attributes;

    // Basically:
    //   #[repr(`repr_type`)]
    // This would be a lot more elegant with `quote`, but it seems to still
    // be considered an unstable API as of April 2023
    // https://github.com/rust-lang/rust/issues/54722
    out.push(punct_token('#'));
    out.push(bracket_token(vec![
        ident_token("repr"),
        paren_token(repr_type.clone()),
    ]));
    // #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    out.push(punct_token('#'));
    out.push(bracket_token(vec![
        ident_token("derive"),
        paren_token(vec![
            ident_token("Debug"),
            punct_token(','),
            ident_token("Clone"),
            punct_token(','),
            ident_token("Copy"),
            punct_token(','),
            ident_token("PartialEq"),
            punct_token(','),
            ident_token("Eq"),
            punct_token(','),
            ident_token("Hash"),
            punct_token(','),
            ident_token("Default"),
        ]),
    ]));

    out.push(ident_token("pub"));
    out.push(ident_token("enum"));
    out.push(TokenTree::Ident(enum_identifier.clone()));
    out.push(brace_token({
        let mut tokens = Vec::<TokenTree>::new();
        for triple in &triples {
            tokens.extend(triple.0.clone());
            tokens.push(TokenTree::Ident(triple.1.clone()));
            tokens.push(punct_token('='));
            tokens.push(triple.2.clone());
            tokens.push(punct_token(','));
        }
        tokens.extend("\n".parse::<TokenStream>().unwrap());
        tokens
    }));

    out.push(ident_token("impl"));
    out.push(TokenTree::Ident(enum_identifier.clone()));
    out.push(brace_token({
        let mut tokens = Vec::new();

        // pub fn from(x: u16) -> Option<MyEnum>
        tokens.extend(vec![
            ident_token("pub"),
            ident_token("fn"),
            ident_token("from"),
        ]);
        tokens.push(paren_token(concat(
            vec![ident_token("x"), punct_token(':')],
            repr_type.clone(),
        )));
        tokens.extend(punc2_tokens('-', '>'));
        tokens.push(ident_token("Option"));
        tokens.push(punct_token('<'));
        tokens.push(TokenTree::Ident(enum_identifier.clone()));
        tokens.push(punct_token('>'));
        tokens.push(brace_token({
            // NOTE: You might be wondering why we use a chain of if statements instead
            // of a match statement.
            // The problem is that if a user provides an expression for one of the
            // values, it may not always be possible to infer the exact literal value
            // during macro expansion (e.g. what if a const variable is used?).
            // And when we have to use user provided expressions for some of the values,
            // it's tricky to find a match pattern that will allow us to match against it.
            // And besides, the Rust compiler is probably smart enough to optimize
            // a chain of if statements that tests a variable against a bunch of constants
            // as much as a simple match.
            let mut tokens = Vec::new();
            for (_, variant_name, variant_value) in &triples {
                tokens.push(ident_token("if"));
                tokens.push(ident_token("x"));
                tokens.extend(punc2_tokens('=', '='));
                tokens.push(variant_value.clone());
                tokens.push(brace_token(vec![
                    ident_token("return"),
                    ident_token("Some"),
                    paren_token(vec![
                        TokenTree::Ident(enum_identifier.clone()),
                        punct_cont_token(':'),
                        punct_token(':'),
                        TokenTree::Ident(variant_name.clone()),
                    ]),
                ]));
            }
            tokens.push(ident_token("None"));
            tokens
        }));

        // pub fn from_name(name: &str) -> Option<MyEnum>
        tokens.extend(vec![
            ident_token("pub"),
            ident_token("fn"),
            ident_token("from_name"),
        ]);
        tokens.push(paren_token(vec![
            ident_token("name"),
            punct_token(':'),
            punct_token('&'),
            ident_token("str"),
        ]));
        tokens.extend(punc2_tokens('-', '>'));
        tokens.push(ident_token("Option"));
        tokens.push(punct_token('<'));
        tokens.push(TokenTree::Ident(enum_identifier.clone()));
        tokens.push(punct_token('>'));
        tokens.push(brace_token({
            let mut tokens = Vec::new();
            for (_, variant_name, _) in &triples {
                tokens.push(ident_token("if"));
                tokens.push(ident_token("name"));
                tokens.extend(punc2_tokens('=', '='));
                tokens.push(TokenTree::Literal(Literal::string(
                    &variant_name.to_string(),
                )));
                tokens.push(brace_token(vec![
                    ident_token("return"),
                    ident_token("Some"),
                    paren_token(vec![
                        TokenTree::Ident(enum_identifier.clone()),
                        punct_cont_token(':'),
                        punct_token(':'),
                        TokenTree::Ident(variant_name.clone()),
                    ]),
                ]));
            }
            tokens.push(ident_token("None"));
            tokens
        }));

        // pub fn list() -> &'static [MyEnum]
        tokens.extend(vec![
            ident_token("pub"),
            ident_token("fn"),
            ident_token("list"),
        ]);
        tokens.push(paren_token(vec![]));
        tokens.extend(punc2_tokens('-', '>'));
        tokens.push(punct_token('&'));
        tokens.push(punct_cont_token('\''));
        tokens.push(ident_token("static"));
        tokens.push(bracket_token(vec![TokenTree::Ident(
            enum_identifier.clone(),
        )]));
        tokens.push(brace_token(vec![
            punct_token('&'),
            bracket_token({
                let mut tokens = Vec::new();
                for (_, variant_name, _) in &triples {
                    tokens.push(TokenTree::Ident(enum_identifier.clone()));
                    tokens.push(punct_cont_token(':'));
                    tokens.push(punct_token(':'));
                    tokens.push(TokenTree::Ident(variant_name.clone()));
                    tokens.push(punct_token(','));
                }
                tokens
            }),
        ]));

        tokens
    }));

    return TokenStream::from_iter(out.into_iter());
}
