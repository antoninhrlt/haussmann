// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Controllers are wrappers for [`Widget`s](crate::Widget) having a function 
//! to call when a certain event happen.
//! 
//! For example, to detect a widget has been tapped, there would be a wrapper 
//! like:
//! ```
//! TapDetector {
//!     widget // Widget
//!     on_tap // function
//! }
//! ```
//! This controller actually exists and it is [`tap::Detector`].

pub mod tap;
