/// Implements the error types
#[macro_use] pub mod error;
/// Implements a thin wrapper around the raw select call
pub mod select_impl;
/// Implements types and traits to access a type's underlying raw file descriptor
pub mod io_handle;
/// Implements a few high-level wrappers around `select_impl`
mod easy;

/// Re-export the public API
pub use crate::easy::{ select_read, select_write, select_readwrite };