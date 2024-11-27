#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod http;
mod macros;
pub mod model;
mod vndb;

pub use http::Endpoint;
pub use model::prelude::*;
pub use vndb::Vndb;
