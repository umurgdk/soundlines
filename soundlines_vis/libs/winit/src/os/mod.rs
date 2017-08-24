//! Contains traits with platform-specific methods in them.
//!
//! Contains the follow modules:
//!
//!  - `android`
//!  - `macos`
//!  - `unix`
//!  - `windows`
//!
//! However only the module corresponding to the platform you're compiling to will be available.
//!
pub mod android;
pub mod macos;
pub mod unix;
pub mod windows;
