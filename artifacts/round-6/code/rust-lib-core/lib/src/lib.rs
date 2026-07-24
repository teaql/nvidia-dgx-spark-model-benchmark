//! Generated TeaQL domain crate for `operations-microservice-core`.
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
pub mod route_status_type;
pub mod inventory_condition_type;
pub mod exception_severity;
pub mod order_status;
pub mod crew_role;
pub mod platform;
pub mod merchant;
pub mod move_quote;
pub mod move_order;
pub mod route_stop;
pub mod crew;
pub mod crew_member_assignment;
pub mod vehicle;
pub mod vehicle_assignment;
pub mod dispatch_assignment;
pub mod damage_report;
pub mod proof_of_delivery;
pub mod operational_exception;
pub mod pickup_instruction;
pub mod delivery_instruction;
pub mod move_inventory;
pub mod packaging_item;
pub mod logistics_provider;
pub mod third_party_dispatch;
pub mod fuel_log;
pub mod maintenance_record;
pub mod toll_receipt;
pub mod shift_log;
pub mod customer_feedback;
pub mod incident_report;

pub use teaql_core;
pub use e::*;
pub use q::*;
pub use request_support::*;
pub use runtime::*;
pub use sample_data::*;
pub use route_status_type::*;
pub use inventory_condition_type::*;
pub use exception_severity::*;
pub use order_status::*;
pub use crew_role::*;
pub use platform::*;
pub use merchant::*;
pub use move_quote::*;
pub use move_order::*;
pub use route_stop::*;
pub use crew::*;
pub use crew_member_assignment::*;
pub use vehicle::*;
pub use vehicle_assignment::*;
pub use dispatch_assignment::*;
pub use damage_report::*;
pub use proof_of_delivery::*;
pub use operational_exception::*;
pub use pickup_instruction::*;
pub use delivery_instruction::*;
pub use move_inventory::*;
pub use packaging_item::*;
pub use logistics_provider::*;
pub use third_party_dispatch::*;
pub use fuel_log::*;
pub use maintenance_record::*;
pub use toll_receipt::*;
pub use shift_log::*;
pub use customer_feedback::*;
pub use incident_report::*;