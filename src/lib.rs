//! time-humanize - A crate to display `std::time::Duration` in a human readable way
//!
//! ## Example
//! ```rust
//! use std::time::Duration;
//! use time_humanize::HumanTime;
//!
//! let duration = Duration::from_secs(60);
//!
//! println!("{}", HumanTime::from(duration));
//! println!("{}", HumanTime::from_seconds(-60));
//!
//! ```
//!
mod humantime;

pub use crate::humantime::{Accuracy, HumanTime, Humanize, Tense};
