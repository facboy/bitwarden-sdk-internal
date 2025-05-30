#![doc = include_str!("../README.md")]

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
#[cfg(feature = "uniffi")]
mod uniffi_support;

#[cfg(feature = "internal")]
pub mod admin_console;
pub mod auth;
pub mod client;
mod error;
pub mod key_management;
pub use error::{
    ApiError, MissingFieldError, MissingPrivateKeyError, NotAuthenticatedError, VaultLockedError,
    WrongPasswordError,
};
#[cfg(feature = "internal")]
pub mod mobile;
#[cfg(feature = "internal")]
pub mod platform;
#[cfg(feature = "secrets")]
pub mod secrets_manager;
mod util;

pub use bitwarden_crypto::ZeroizingAllocator;
pub use client::{Client, ClientSettings, DeviceType};

mod ids;
pub use ids::*;
