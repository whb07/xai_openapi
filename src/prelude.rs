//! Internal prelude for `std`/`no_std` compatibility.

#[cfg(feature = "std")]
pub use std::string::{String, ToString};

#[cfg(feature = "std")]
pub use std::vec::Vec;

#[cfg(feature = "std")]
pub use std::collections::HashMap;

#[cfg(not(feature = "std"))]
pub use alloc::string::{String, ToString};

#[cfg(not(feature = "std"))]
pub use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
pub use hashbrown::HashMap;
