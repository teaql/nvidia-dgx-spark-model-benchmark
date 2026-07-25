//! #  Service Workspace
//!
//! **Before writing queries**, read the generated `AGENTS.md` at the workspace root.
//! It contains the entity list and the exact `cargo teaql` commands to fetch API prompts.

pub use module_7_service_core::{teaql_core, E, Q};

pub fn generated_domain_crate() -> &'static str {
    "module-7-service-core"
}