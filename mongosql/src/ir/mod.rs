include!(concat!(env!("OUT_DIR"), "/ir/visitor.rs"));
include!(concat!(env!("OUT_DIR"), "/ir/walk.rs"));
pub mod binding_tuple;
pub mod definitions;
pub use definitions::*;
pub mod constant_folding;
pub mod flatten;
pub mod namespace;
pub mod schema;
#[cfg(test)]
mod test;
