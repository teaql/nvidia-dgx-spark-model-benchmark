//! Generated TeaQL domain crate for `finance-service-core`.
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
pub mod address;
pub mod truck;
pub mod driver;
pub mod move_order;
pub mod inventory_item;
pub mod packing_material;
pub mod route;
pub mod schedule;
pub mod loading_unloading;
pub mod equipment;
pub mod tool;
pub mod storage_facility;
pub mod warehouse;
pub mod container;
pub mod pallet;
pub mod label;
pub mod barcode;
pub mod tracking_number;
pub mod notification;
pub mod payment;
pub mod invoice;
pub mod claim;
pub mod feedback;
pub mod employee;
pub mod branch;
pub mod vehicle_maintenance;
pub mod fuel_log;
pub mod insurance_policy;
pub mod license;
pub mod permit;
pub mod customs_document;
pub mod communication_log;
pub mod audit_trail;
pub mod report;
pub mod dashboard;
pub mod settings;
pub mod user_role;
pub mod permission;
pub mod api_key;

pub use teaql_core;
pub use e::*;
pub use q::*;
pub use request_support::*;
pub use runtime::*;
pub use sample_data::*;
pub use customer::*;
pub use address::*;
pub use truck::*;
pub use driver::*;
pub use move_order::*;
pub use inventory_item::*;
pub use packing_material::*;
pub use route::*;
pub use schedule::*;
pub use loading_unloading::*;
pub use equipment::*;
pub use tool::*;
pub use storage_facility::*;
pub use warehouse::*;
pub use container::*;
pub use pallet::*;
pub use label::*;
pub use barcode::*;
pub use tracking_number::*;
pub use notification::*;
pub use payment::*;
pub use invoice::*;
pub use claim::*;
pub use feedback::*;
pub use employee::*;
pub use branch::*;
pub use vehicle_maintenance::*;
pub use fuel_log::*;
pub use insurance_policy::*;
pub use license::*;
pub use permit::*;
pub use customs_document::*;
pub use communication_log::*;
pub use audit_trail::*;
pub use report::*;
pub use dashboard::*;
pub use settings::*;
pub use user_role::*;
pub use permission::*;
pub use api_key::*;