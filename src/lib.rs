#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
// False positive with None
#![allow(non_snake_case)]
#![deny(dead_code)]

pub mod aof;
pub mod commands;
pub mod config;
pub mod database;
pub mod errors;
pub mod keys;
pub mod parser;
pub mod process;
pub mod resp;
pub mod response;
pub mod server;
pub mod types;
