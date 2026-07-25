// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/work_shift
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "WorkShift", table = "work_shift_data", data_service = "sqlite")]
pub struct WorkShift {
#[teaql(id)]
    id: u64,

// @source module_2.xml:6
    start_time: chrono::DateTime<chrono::Utc>,

// @source module_2.xml:6
    end_time: chrono::DateTime<chrono::Utc>,

// @source module_2.xml:6
    shift_kind: String,

// @source module_2.xml:6
    location_code: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl WorkShift {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            start_time: chrono::Utc::now(),
            end_time: chrono::Utc::now(),
            shift_kind: String::new(),
            location_code: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("WorkShift", self.id)
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

    pub fn shift_kind(&self) -> String {
        self.changed_shift_kind().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.shift_kind.clone())
    }

    pub fn update_shift_kind(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.shift_kind = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.shift_kind.clone());
        self.root.set(self.entity_key(), "shift_kind", value);
        self
    }

    pub fn changed_shift_kind(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "shift_kind")
    }

    pub fn eval_shift_kind(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("shift_kind") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "shift_kind".to_string(), attempted_path: "shift_kind".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.shift_kind())
                }}

    pub fn location_code(&self) -> String {
        self.changed_location_code().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.location_code.clone())
    }

    pub fn update_location_code(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.location_code = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.location_code.clone());
        self.root.set(self.entity_key(), "location_code", value);
        self
    }

    pub fn changed_location_code(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "location_code")
    }

    pub fn eval_location_code(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("location_code") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "location_code".to_string(), attempted_path: "location_code".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.location_code())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::WorkShiftRepository<'a>>>
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
            .work_shift_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("WorkShift"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

