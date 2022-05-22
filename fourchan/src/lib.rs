#![feature(let_chains)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

pub mod board;
pub mod client;
pub mod post;
pub mod thread;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate sqlx;

pub use board::{Board, BoardsResponse};
pub use client::Client;
pub use post::{Post, PostAttachment};
pub use thread::{ThreadPageListResponse, ThreadResponse};
