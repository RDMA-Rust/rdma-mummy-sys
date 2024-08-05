#![deny(warnings)]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(clippy::missing_safety_doc, clippy::too_many_arguments)]

use libc::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod opcode;
mod types;
mod verbs;

pub use self::opcode::*;
pub use self::types::*;
pub use self::verbs::*;
