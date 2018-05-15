//! Contains possible destination for writes
//!
//! All writes are unprotected.
//! If you'd like to have option to configure your writes
//! with interrupt protection, see [printer](../printer/index.html)

pub mod dummy;
pub use self::dummy::Dummy;

pub mod itm;
pub use self::itm::Itm;

#[cfg(feature = "semihosting")]
pub mod semihosting;
#[cfg(feature = "semihosting")]
pub use self::semihosting::{SHerr, SHout};
