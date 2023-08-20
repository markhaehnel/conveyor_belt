#![cfg_attr(debug_assertions, allow(unused, dead_code))]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_const_for_fn)]

pub mod builder;
pub mod queue;
pub mod retry_strategy;
