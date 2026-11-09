#![deny(unsafe_code)]

mod app;
mod assets;
mod audio;
mod rendering;
pub mod runtime;
mod schedule;
mod settings;
mod speaki;
mod window;

#[cfg(not(target_arch = "wasm32"))]
pub mod validation;
