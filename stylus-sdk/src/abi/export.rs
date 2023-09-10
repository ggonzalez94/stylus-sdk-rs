// Copyright 2023, Offchain Labs, Inc.
// For licensing, see https://github.com/OffchainLabs/stylus-sdk-rs/blob/stylus/licenses/COPYRIGHT.md

//! Traits for exporting Solidity interfaces.
//!
//! The contents of this module are imported when the `export-abi` feature flag is enabled,
//! which happens automatically during [`cargo stylus export-abi`][cargo].
//!
//! [cargo]: https://github.com/OffchainLabs/cargo-stylus#exporting-solidity-abis

use core::{fmt, marker::PhantomData};
use lazy_static::lazy_static;
use regex::Regex;

/// Trait for storage types so that users can print a Solidity interface to the console.
/// This is auto-derived via the [`external`] marco when the `export-abi` feature is enabled.
///
/// [`external`]: stylus-proc::external
pub trait GenerateAbi {
    /// The interface's name.
    const NAME: &'static str;

    /// How to format the ABI. Analogous to [`Display`](std::fmt::Display).
    fn fmt_abi(f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

/// Type that makes an ABI printable.
struct AbiPrinter<T: GenerateAbi>(PhantomData<T>);

impl<T: GenerateAbi> fmt::Display for AbiPrinter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        T::fmt_abi(f)
    }
}

/// Prints the full contract ABI to standard out
pub fn print_abi<T: GenerateAbi>() {
    println!("/**");
    println!(" * This file was automatically generated by Stylus and represents a Rust program.");
    println!(" * For more information, please see [The Stylus SDK](https://github.com/OffchainLabs/stylus-sdk-rs).");
    println!(" */");
    println!();
    print!("{}", AbiPrinter::<T>(PhantomData));
}

lazy_static! {
    static ref UINT_REGEX: Regex = Regex::new(r"^uint(\d+)$").unwrap();
    static ref INT_REGEX: Regex = Regex::new(r"^int(\d+)$").unwrap();
    static ref BYTES_REGEX: Regex = Regex::new(r"^bytes(\d+)$").unwrap();
}

/// Prepends the string with an underscore if it is a Solidity keyword.
/// Otherwise, the string is unchanged.
/// Note: also prepends a space when the input is nonempty.
pub fn underscore_if_sol(name: &str) -> String {
    let underscore = || format!(" _{name}");

    if let Some(caps) = UINT_REGEX.captures(name) {
        let bits: usize = caps[1].parse().unwrap();
        if bits % 8 == 0 {
            return underscore();
        }
    }

    if let Some(caps) = INT_REGEX.captures(name) {
        let bits: usize = caps[1].parse().unwrap();
        if bits % 8 == 0 {
            return underscore();
        }
    }

    if let Some(caps) = BYTES_REGEX.captures(name) {
        let bits: usize = caps[1].parse().unwrap();
        if bits <= 32 {
            return underscore();
        }
    }

    match name {
        "" => "".to_string(),

        // other types
        "address" | "bool" | "int" | "uint" => underscore(),

        // other words
        "is" | "contract" | "interface" => underscore(),

        // reserved keywords
        "after" | "alias" | "apply" | "auto" | "byte" | "case" | "copyof" | "default"
        | "define" | "final" | "implements" | "in" | "inline" | "let" | "macro" | "match"
        | "mutable" | "null" | "of" | "partial" | "promise" | "reference" | "relocatable"
        | "sealed" | "sizeof" | "static" | "supports" | "switch" | "typedef" | "typeof" | "var" => {
            underscore()
        }
        _ => format!(" {name}"),
    }
}
