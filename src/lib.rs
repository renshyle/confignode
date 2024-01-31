#![warn(missing_docs)]

//! A library to parse Kerbal Space Program 1 ConfigNode files
//!
//! Parses configuration files, savefiles and other types of files used in Kerbal Space Program that use this format.
//!
//! # Usage
//!
//! Parse a ConfigNode string using [`ConfigNodeParser::parse()`]. Returned is a root node [`ConfigNode`] whose keys and
//! values you can access through the struct's `children` field.
//! With a [`ConfigNodeValue`], you can use [`as_text()`](ConfigNodeValue::as_text()) or [`as_node()`](ConfigNodeValue::as_node()) to get the inner
//! value if you know what type to expect. You can also use normal enum pattern matching to handle different types.
//!
//! # Examples
//! ```no_run
//! let content = std::fs::read_to_string("persistent.sfs").unwrap();
//! let savefile = confignode::ConfigNodeParser::parse(&content).unwrap();
//! let game = savefile.children.get("GAME").unwrap().as_node().unwrap();
//!
//! println!(
//!     "Name: {}",
//!     game.children.get("Title").unwrap().as_text().unwrap()
//! );
//! ```

mod confignode;

/// ConfigNode-related errors
pub mod error;

pub use confignode::*;
