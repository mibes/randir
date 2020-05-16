//! # Example use
//! ```rust
//! use randir::utils::generate_entries;
//!
//! //! generate 100 random names and telephone numbers
//! let directory = generate_entries(100);
//! for entry in directory {
//!     println!("{}", entry)
//! }
//! ```
//!
//! # Detail
//! The result type of the `generate_entries()` function is a `Vec<Entry>`. `Entry` is defined as:
//!
//! ```rust
//! pub struct Entry {
//!     pub uid: usize,
//!     pub first_name: String,
//!     pub last_name: String,
//!     pub phone_nr: String,
//! }
//! ```
pub mod entry;
pub mod utils;
