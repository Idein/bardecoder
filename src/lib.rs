//! Barcode Decoder

#![allow(unknown_lints)]
#![allow(clippy::new_without_default)]
#![warn(missing_docs)]

use failure;

#[macro_use]
extern crate log;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate newtype_derive;

mod decoder;

pub mod decode;
pub mod detect;
pub mod extract;
pub mod prepare;
pub mod util;

pub use crate::decoder::{default_builder, default_decoder};
pub use crate::decoder::{Decoder, DecoderBuilder};
