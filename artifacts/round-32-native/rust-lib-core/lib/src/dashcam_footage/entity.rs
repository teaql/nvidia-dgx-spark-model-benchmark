// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/dashcam_footage
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "DashcamFootage", table = "dashcam_footage_data", data_service = "sqlite")]
pub struct DashcamFootage {
#[teaql(id)]
    id: u64,

// @source module_8.xml:11
    recorded_at: chrono::DateTime<chrono::Utc>,

// @source module_8.xml:11
    duration_seconds: i64,

// @source module_8.xml:11
    file_path: String,

// @source module_8.xml:11
    resolution: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl DashcamFootage {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            recorded_at: chrono::Utc::now(),
            duration_seconds: 0_i64,
            file_path: String::new(),
            resolution: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("DashcamFootage", self.id)
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

    pub fn recorded_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_recorded_at().and_then(|value| value.try_timestamp()).unwrap_or(self.recorded_at)
    }

    pub fn update_recorded_at(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.recorded_at = value.try_timestamp().unwrap_or(self.recorded_at.clone());
        self.root.set(self.entity_key(), "recorded_at", value);
        self
    }

    pub fn changed_recorded_at(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "recorded_at")
    }

    pub fn eval_recorded_at(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("recorded_at") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "recorded_at".to_string(), attempted_path: "recorded_at".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.recorded_at())
                }}

    pub fn duration_seconds(&self) -> i64 {
        self.changed_duration_seconds().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.duration_seconds)
    }

    pub fn update_duration_seconds(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.duration_seconds = value.try_i64().map(|value| value as i64).unwrap_or(self.duration_seconds.clone());
        self.root.set(self.entity_key(), "duration_seconds", value);
        self
    }

    pub fn changed_duration_seconds(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "duration_seconds")
    }

    pub fn eval_duration_seconds(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("duration_seconds") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "duration_seconds".to_string(), attempted_path: "duration_seconds".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.duration_seconds())
                }}

    pub fn file_path(&self) -> String {
        self.changed_file_path().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.file_path.clone())
    }

    pub fn update_file_path(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.file_path = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.file_path.clone());
        self.root.set(self.entity_key(), "file_path", value);
        self
    }

    pub fn changed_file_path(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "file_path")
    }

    pub fn eval_file_path(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("file_path") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "file_path".to_string(), attempted_path: "file_path".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.file_path())
                }}

    pub fn resolution(&self) -> String {
        self.changed_resolution().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.resolution.clone())
    }

    pub fn update_resolution(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.resolution = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.resolution.clone());
        self.root.set(self.entity_key(), "resolution", value);
        self
    }

    pub fn changed_resolution(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "resolution")
    }

    pub fn eval_resolution(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("resolution") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "resolution".to_string(), attempted_path: "resolution".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.resolution())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
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
            .dashcam_footage_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("DashcamFootage"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

