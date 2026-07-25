//! Generated TeaQL domain crate for `operations-service-core`.
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
pub mod route;
pub mod invoice;
pub mod payment;
pub mod schedule;
pub mod warehouse;
pub mod inventory;
pub mod cargo;
pub mod loading;
pub mod unloading;
pub mod driver;
pub mod move_order;
pub mod quote;
pub mod estimate;
pub mod contract;
pub mod feedback;
pub mod rating;
pub mod address;
pub mod contact;
pub mod phone;
pub mod email;
pub mod document;
pub mod license;
pub mod insurance;
pub mod maintenance;
pub mod fuel_log;
pub mod timesheet;
pub mod expense;

pub use teaql_core;
pub use e::*;
pub use q::*;
pub use request_support::*;
pub use runtime::*;
pub use sample_data::*;
pub use customer::*;
pub use employee::*;
pub use truck::*;
pub use route::*;
pub use invoice::*;
pub use payment::*;
pub use schedule::*;
pub use warehouse::*;
pub use inventory::*;
pub use cargo::*;
pub use loading::*;
pub use unloading::*;
pub use driver::*;
pub use move_order::*;
pub use quote::*;
pub use estimate::*;
pub use contract::*;
pub use feedback::*;
pub use rating::*;
pub use address::*;
pub use contact::*;
pub use phone::*;
pub use email::*;
pub use document::*;
pub use license::*;
pub use insurance::*;
pub use maintenance::*;
pub use fuel_log::*;
pub use timesheet::*;
pub use expense::*;