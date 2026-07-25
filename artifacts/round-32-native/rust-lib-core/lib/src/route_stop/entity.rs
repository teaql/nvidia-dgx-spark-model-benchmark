// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/route_stop
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "RouteStop", table = "route_stop_data", data_service = "sqlite")]
pub struct RouteStop {
#[teaql(id)]
    id: u64,

// @source module_0.xml:12
    stop_sequence: i64,

// @source module_0.xml:12
    arrival_window_start: chrono::DateTime<chrono::Utc>,

// @source module_0.xml:12
    arrival_window_end: chrono::DateTime<chrono::Utc>,

// @source module_0.xml:12
    status: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl RouteStop {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            stop_sequence: 0_i64,
            arrival_window_start: chrono::Utc::now(),
            arrival_window_end: chrono::Utc::now(),
            status: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("RouteStop", self.id)
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

    pub fn stop_sequence(&self) -> i64 {
        self.changed_stop_sequence().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.stop_sequence)
    }

    pub fn update_stop_sequence(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.stop_sequence = value.try_i64().map(|value| value as i64).unwrap_or(self.stop_sequence.clone());
        self.root.set(self.entity_key(), "stop_sequence", value);
        self
    }

    pub fn changed_stop_sequence(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "stop_sequence")
    }

    pub fn eval_stop_sequence(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("stop_sequence") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "stop_sequence".to_string(), attempted_path: "stop_sequence".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.stop_sequence())
                }}

    pub fn arrival_window_start(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_arrival_window_start().and_then(|value| value.try_timestamp()).unwrap_or(self.arrival_window_start)
    }

    pub fn update_arrival_window_start(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.arrival_window_start = value.try_timestamp().unwrap_or(self.arrival_window_start.clone());
        self.root.set(self.entity_key(), "arrival_window_start", value);
        self
    }

    pub fn changed_arrival_window_start(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "arrival_window_start")
    }

    pub fn eval_arrival_window_start(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("arrival_window_start") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "arrival_window_start".to_string(), attempted_path: "arrival_window_start".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.arrival_window_start())
                }}

    pub fn arrival_window_end(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_arrival_window_end().and_then(|value| value.try_timestamp()).unwrap_or(self.arrival_window_end)
    }

    pub fn update_arrival_window_end(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.arrival_window_end = value.try_timestamp().unwrap_or(self.arrival_window_end.clone());
        self.root.set(self.entity_key(), "arrival_window_end", value);
        self
    }

    pub fn changed_arrival_window_end(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "arrival_window_end")
    }

    pub fn eval_arrival_window_end(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("arrival_window_end") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "arrival_window_end".to_string(), attempted_path: "arrival_window_end".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.arrival_window_end())
                }}

    pub fn status(&self) -> String {
        self.changed_status().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.status.clone())
    }

    pub fn update_status(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.status = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.status.clone());
        self.root.set(self.entity_key(), "status", value);
        self
    }

    pub fn changed_status(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "status")
    }

    pub fn eval_status(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("status") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "status".to_string(), attempted_path: "status".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.status())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::RouteStopRepository<'a>>>
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
            .route_stop_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("RouteStop"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

