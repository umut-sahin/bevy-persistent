#![cfg_attr(doctest, doc = "````no_test")]
#![doc = include_str!("../README.md")]

#[rustfmt::skip]
#[cfg(not(any(
    feature = "library",
    feature = "bincode",
    feature = "ini",
    feature = "json",
    feature = "ron",
    feature = "toml",
    feature = "yaml",
)))]
compile_error!(concat!(r#"no storage formats are selected!

    If you're not sure which formats you'll need,
    you can start with selecting all of them:

    bevy-persistent = { version = ""#, env!("CARGO_PKG_VERSION"), r#"", features = ["all"] }

"#));

pub mod builder;
pub mod error;
pub mod format;
pub mod persistent;
pub mod prelude;
pub mod storage;

pub use crate::{
    builder::PersistentBuilder, error::PersistenceError, format::StorageFormat,
    persistent::Persistent, storage::Storage,
};
