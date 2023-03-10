// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

#![allow(unused)]

//! This file is designed to run the tests easier from the code studio rather 
//! than opening the test file and click on the "run test" just above the test
//! function.

// Run this test to create widgets from macros.
mod macros;
// Run this test to use the project on "SDL2".
mod sdl2;
// Run this test to use the project with "ratatui".
mod tui;
// Run this test to check unusual usages of the project.
mod unusual;

// Run the tests associated to Rust-things more than the project.
mod rust {
    // Test closures in Rust.
    mod closures;
    // Test downcast in Rust.
    mod downcast;
    // Test dynamics in Rust.
    mod r#dyn;
}
