// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/custom_entity_689
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "CustomEntity689", table = "custom_entity_689_data", data_service = "sqlite")]
pub struct CustomEntity689 {
#[teaql(id)]
    id: u64,

// @source module_45.xml:16
    metric: String,

// @source module_45.xml:16
    reading: String,

// @source module_45.xml:16
    recorded_at: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl CustomEntity689 {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            metric: String::new(),
            reading: String::new(),
            recorded_at: chrono::Utc::now(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("CustomEntity689", self.id)
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

    pub fn metric(&self) -> String {
        self.changed_metric().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.metric.clone())
    }

    pub fn update_metric(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.metric = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.metric.clone());
        self.root.set(self.entity_key(), "metric", value);
        self
    }

    pub fn changed_metric(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "metric")
    }

    pub fn eval_metric(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("metric") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "metric".to_string(), attempted_path: "metric".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.metric())
                }}

    pub fn reading(&self) -> String {
        self.changed_reading().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.reading.clone())
    }

    pub fn update_reading(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.reading = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.reading.clone());
        self.root.set(self.entity_key(), "reading", value);
        self
    }

    pub fn changed_reading(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "reading")
    }

    pub fn eval_reading(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("reading") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "reading".to_string(), attempted_path: "reading".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.reading())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CustomEntity689Repository<'a>>>
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
            .custom_entity_689_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("CustomEntity689"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

