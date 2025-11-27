//! Database module for Resona
//!
//! This module handles all database operations using redb as the storage backend.

mod database;
mod error;
mod tables;

pub use database::Database;
pub use error::{DbError, DbResult};
pub use tables::*;
