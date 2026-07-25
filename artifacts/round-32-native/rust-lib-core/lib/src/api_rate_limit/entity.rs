// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/api_rate_limit
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "ApiRateLimit", table = "api_rate_limit_data", data_service = "sqlite")]
pub struct ApiRateLimit {
#[teaql(id)]
    id: u64,

// @source module_11.xml:16
    limit_key: String,

// @source module_11.xml:16
    max_requests: i64,

// @source module_11.xml:16
    window_seconds: i64,

// @source module_11.xml:16
    current_count: i64,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl ApiRateLimit {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            limit_key: String::new(),
            max_requests: 0_i64,
            window_seconds: 0_i64,
            current_count: 0_i64,
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("ApiRateLimit", self.id)
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

    pub fn limit_key(&self) -> String {
        self.changed_limit_key().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.limit_key.clone())
    }

    pub fn update_limit_key(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.limit_key = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.limit_key.clone());
        self.root.set(self.entity_key(), "limit_key", value);
        self
    }

    pub fn changed_limit_key(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "limit_key")
    }

    pub fn eval_limit_key(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("limit_key") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "limit_key".to_string(), attempted_path: "limit_key".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.limit_key())
                }}

    pub fn max_requests(&self) -> i64 {
        self.changed_max_requests().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.max_requests)
    }

    pub fn update_max_requests(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.max_requests = value.try_i64().map(|value| value as i64).unwrap_or(self.max_requests.clone());
        self.root.set(self.entity_key(), "max_requests", value);
        self
    }

    pub fn changed_max_requests(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "max_requests")
    }

    pub fn eval_max_requests(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("max_requests") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "max_requests".to_string(), attempted_path: "max_requests".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.max_requests())
                }}

    pub fn window_seconds(&self) -> i64 {
        self.changed_window_seconds().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.window_seconds)
    }

    pub fn update_window_seconds(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.window_seconds = value.try_i64().map(|value| value as i64).unwrap_or(self.window_seconds.clone());
        self.root.set(self.entity_key(), "window_seconds", value);
        self
    }

    pub fn changed_window_seconds(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "window_seconds")
    }

    pub fn eval_window_seconds(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("window_seconds") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "window_seconds".to_string(), attempted_path: "window_seconds".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.window_seconds())
                }}

    pub fn current_count(&self) -> i64 {
        self.changed_current_count().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.current_count)
    }

    pub fn update_current_count(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.current_count = value.try_i64().map(|value| value as i64).unwrap_or(self.current_count.clone());
        self.root.set(self.entity_key(), "current_count", value);
        self
    }

    pub fn changed_current_count(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "current_count")
    }

    pub fn eval_current_count(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("current_count") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "current_count".to_string(), attempted_path: "current_count".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.current_count())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::ApiRateLimitRepository<'a>>>
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
            .api_rate_limit_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("ApiRateLimit"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

