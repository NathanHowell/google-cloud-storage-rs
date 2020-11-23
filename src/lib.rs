#![forbid(unsafe_code)]

mod bucket;
mod bucket_access_control;
mod client;
mod default_object_access_control;
mod error;
mod google;
mod hmac_key;
mod iam;
mod notifications;
mod object;
mod object_access_control;
mod paginate;
mod query;
mod request;
mod serde;

#[cfg(test)]
mod tests;

pub use crate::error::*;
pub use client::{Client, ClientBuilder};
pub use google::*;

pub type Result<T> = std::result::Result<T, crate::Error>;
