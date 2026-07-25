//! Generated TeaQL domain crate for `main-service-core`.
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
pub mod customer;
pub mod employee;
pub mod truck;
pub mod inventory_item;
pub mod move_order;
pub mod route;
pub mod payment;
pub mod invoice;
pub mod feedback;
pub mod schedule;

pub use teaql_core;
pub use e::*;
pub use q::*;
pub use request_support::*;
pub use runtime::*;
pub use sample_data::*;
pub use customer::*;
pub use employee::*;
pub use truck::*;
pub use inventory_item::*;
pub use move_order::*;
pub use route::*;
pub use payment::*;
pub use invoice::*;
pub use feedback::*;
pub use schedule::*;