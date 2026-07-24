//! Generated TeaQL domain crate for `moving-company-service-core`.
//!
//! **Before writing queries**, read the `AGENTS.md` at the workspace root.
//! It contains the entity list and the exact `cargo teaql` commands to fetch API prompts.
//!
//! AI coding agents must read this crate's `AGENTS.md` before using generated
//! APIs. If this crate was downloaded from a Cargo registry, locate the
//! unpacked crate source or vendor the dependency, then read `AGENTS.md` from
//! the crate root before writing code against it.

pub mod e;
pub mod q;
pub mod request_support;
pub mod runtime;
pub mod sample_data;
pub mod platform;
pub mod merchant;
pub mod move_order;
pub mod employee;
pub mod customer;
pub mod product;
pub mod campaign;
pub mod payment;
pub mod vehicle;
pub mod contract;
pub mod user_account;
pub mod activity_log;
pub mod notification;
pub mod api_client;

pub use teaql_core;
pub use e::*;
pub use q::*;
pub use request_support::*;
pub use runtime::*;
pub use sample_data::*;
pub use platform::*;
pub use merchant::*;
pub use move_order::*;
pub use employee::*;
pub use customer::*;
pub use product::*;
pub use campaign::*;
pub use payment::*;
pub use vehicle::*;
pub use contract::*;
pub use user_account::*;
pub use activity_log::*;
pub use notification::*;
pub use api_client::*;