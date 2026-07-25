// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/time_slot
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "TimeSlot", table = "time_slot_data", data_service = "sqlite")]
pub struct TimeSlot {
#[teaql(id)]
    id: u64,

// @source module_0.xml:13
    slot_code: String,

// @source module_0.xml:13
    start_time: chrono::DateTime<chrono::Utc>,

// @source module_0.xml:13
    end_time: chrono::DateTime<chrono::Utc>,

// @source module_0.xml:13
    capacity: i64,

// @source module_0.xml:13
    is_available: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl TimeSlot {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            slot_code: String::new(),
            start_time: chrono::Utc::now(),
            end_time: chrono::Utc::now(),
            capacity: 0_i64,
            is_available: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("TimeSlot", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
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

    pub fn slot_code(&self) -> String {
        self.changed_slot_code().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.slot_code.clone())
    }

    pub fn update_slot_code(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.slot_code = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.slot_code.clone());
        self.root.set(self.entity_key(), "slot_code", value);
        self
    }

    pub fn changed_slot_code(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "slot_code")
    }

    pub fn eval_slot_code(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("slot_code") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "slot_code".to_string(), attempted_path: "slot_code".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.slot_code())
                }}

    pub fn start_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_start_time().and_then(|value| value.try_timestamp()).unwrap_or(self.start_time)
    }

    pub fn update_start_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.start_time = value.try_timestamp().unwrap_or(self.start_time.clone());
        self.root.set(self.entity_key(), "start_time", value);
        self
    }

    pub fn changed_start_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "start_time")
    }

    pub fn eval_start_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("start_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "start_time".to_string(), attempted_path: "start_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.start_time())
                }}

    pub fn end_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_end_time().and_then(|value| value.try_timestamp()).unwrap_or(self.end_time)
    }

    pub fn update_end_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.end_time = value.try_timestamp().unwrap_or(self.end_time.clone());
        self.root.set(self.entity_key(), "end_time", value);
        self
    }

    pub fn changed_end_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "end_time")
    }

    pub fn eval_end_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("end_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "end_time".to_string(), attempted_path: "end_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.end_time())
                }}

    pub fn capacity(&self) -> i64 {
        self.changed_capacity().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.capacity)
    }

    pub fn update_capacity(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.capacity = value.try_i64().map(|value| value as i64).unwrap_or(self.capacity.clone());
        self.root.set(self.entity_key(), "capacity", value);
        self
    }

    pub fn changed_capacity(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "capacity")
    }

    pub fn eval_capacity(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("capacity") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "capacity".to_string(), attempted_path: "capacity".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.capacity())
                }}

    pub fn is_available(&self) -> String {
        self.changed_is_available().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.is_available.clone())
    }

    pub fn update_is_available(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.is_available = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.is_available.clone());
        self.root.set(self.entity_key(), "is_available", value);
        self
    }

    pub fn changed_is_available(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "is_available")
    }

    pub fn eval_is_available(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("is_available") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "is_available".to_string(), attempted_path: "is_available".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.is_available())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::TimeSlotRepository<'a>>>
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
            .time_slot_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("TimeSlot"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

