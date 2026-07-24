// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/maintenance_schedule
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "MaintenanceSchedule", table = "maintenance_schedule_data", data_service = "sqlite")]
pub struct MaintenanceSchedule {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    schedule_id: String,

// @source model.xml:2
    interval_days: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "vehicle_ref")]
    vehicle_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Vehicle", local_key = "vehicle_ref_id", foreign_key = "id"))]
    vehicle_ref: Option<crate::Vehicle>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl MaintenanceSchedule {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            schedule_id: String::new(),
            interval_days: String::new(),
            version: 0_i64,
            vehicle_ref_id: 0_u64,
            vehicle_ref: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("MaintenanceSchedule", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.vehicle_ref {
            entity.attach_root_recursive(root.clone());
        }
    }

    pub fn is_loaded(&self, field_or_relation: &str) -> bool {
        self.__load_state.is_loaded(field_or_relation)
    }

    pub fn set_load_state(&mut self, state: teaql_core::eval::LoadState) {
        self.__load_state = state;
    }

    pub fn id(&self) -> u64 {
        self.changed_id().and_then(|value| value.try_u64()).unwrap_or(self.id)
    }

    pub fn update_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.id = value.try_u64().unwrap_or(self.id.clone());
        self.root.set(self.entity_key(), "id", value);
        self
    }

    pub fn changed_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "id")
    }

    pub fn eval_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "id".to_string(), attempted_path: "id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.id())
                }}

    pub fn schedule_id(&self) -> String {
        self.changed_schedule_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.schedule_id.clone())
    }

    pub fn update_schedule_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.schedule_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.schedule_id.clone());
        self.root.set(self.entity_key(), "schedule_id", value);
        self
    }

    pub fn changed_schedule_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "schedule_id")
    }

    pub fn eval_schedule_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("schedule_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "schedule_id".to_string(), attempted_path: "schedule_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.schedule_id())
                }}

    pub fn interval_days(&self) -> String {
        self.changed_interval_days().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.interval_days.clone())
    }

    pub fn update_interval_days(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.interval_days = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.interval_days.clone());
        self.root.set(self.entity_key(), "interval_days", value);
        self
    }

    pub fn changed_interval_days(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "interval_days")
    }

    pub fn eval_interval_days(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("interval_days") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "interval_days".to_string(), attempted_path: "interval_days".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.interval_days())
                }}

    pub fn version(&self) -> i64 {
        self.changed_version().and_then(|value| value.try_i64()).unwrap_or(self.version)
    }

    pub fn update_version(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.version = value.try_i64().unwrap_or(self.version.clone());
        self.root.set(self.entity_key(), "version", value);
        self
    }

    pub fn changed_version(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "version")
    }

    pub fn eval_version(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("version") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "version".to_string(), attempted_path: "version".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.version())
                }}
    pub fn vehicle_ref_id(&self) -> u64 {
        self.changed_vehicle_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.vehicle_ref_id)
    }

    pub fn update_vehicle_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.vehicle_ref_id = value.try_u64().unwrap_or(self.vehicle_ref_id.clone());
        self.root.set(self.entity_key(), "vehicle_ref_id", value);
        self
    }

    pub fn changed_vehicle_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "vehicle_ref_id")
    }

    pub fn eval_vehicle_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("vehicle_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_ref_id".to_string(), attempted_path: "vehicle_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.vehicle_ref_id())
                }}
    pub fn vehicle_ref(&self) -> Option<&crate::Vehicle> {
        self.vehicle_ref.as_ref()
    }

    pub fn eval_vehicle_ref(&self) -> teaql_core::eval::EvalResult<&crate::Vehicle> {
        if !self.is_loaded("vehicle_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_ref".to_string(), attempted_path: "vehicle_ref".to_string() }
        } else {
            match &self.vehicle_ref {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn mark_as_delete(&mut self) -> &mut Self {
        self.root.mark_as_delete(self.entity_key());
        self
    }

    pub fn set_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.root.set_comment(comment);
        self
    }

    pub(crate) async fn save<'a, C>(
        &self,
        ctx: &'a C,
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let root = ctx.user_context().entity_root();
        let key = self.entity_key();
        let has_ledger_change = (self.id != 0)
            && (root.current_change_set().changes().contains_key(&key)
                || root.is_marked_as_delete(&key)
                || root.is_new(&key));
        let repository = ctx
            .maintenance_schedule_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("MaintenanceSchedule"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

