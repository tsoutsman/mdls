#![deny(
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    meta_variable_misuse,
    missing_debug_implementations,
    non_ascii_idents,
    noop_method_call,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    variant_size_differences,
    clippy::all,
    clippy::pedantic,
    clippy::clone_on_ref_ptr,
    clippy::decimal_literal_representation,
    clippy::empty_structs_with_brackets,
    clippy::rc_buffer,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::str_to_string,
    clippy::undocumented_unsafe_blocks,
    clippy::unreachable,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls
)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

mod ctx;
mod error;
mod fmt;
mod index;
mod proto;

pub mod handle;

pub use ctx::Context;
pub use error::{Error, Result};

pub(crate) use fmt::fmt;
