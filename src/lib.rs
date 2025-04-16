#![no_std]

#[macro_use]
extern crate alloc;

mod did_resolver;
mod message_builder;

#[cfg(test)]
mod tests;

pub use did_resolver::UniversalDidResolver;
