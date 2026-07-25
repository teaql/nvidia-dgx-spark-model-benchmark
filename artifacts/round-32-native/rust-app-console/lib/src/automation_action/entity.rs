// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/automation_action
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "AutomationAction", table = "automation_action_data", data_service = "sqlite")]
pub struct AutomationAction {
#[teaql(id)]
    id: u64,

// @source module_11.xml:7
    action_kind: String,

// @source module_11.xml:7
    target_system: String,

// @source module_11.xml:7
    payload: String,

// @source module_11.xml:7
    retry_count: i64,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl AutomationAction {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            action_kind: String::new(),
            target_system: String::new(),
            payload: String::new(),
            retry_count: 0_i64,
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("AutomationAction", self.id)
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

    pub fn action_kind(&self) -> String {
        self.changed_action_kind().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.action_kind.clone())
    }

    pub fn update_action_kind(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.action_kind = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.action_kind.clone());
        self.root.set(self.entity_key(), "action_kind", value);
        self
    }

    pub fn changed_action_kind(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "action_kind")
    }

    pub fn eval_action_kind(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("action_kind") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "action_kind".to_string(), attempted_path: "action_kind".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.action_kind())
                }}

    pub fn target_system(&self) -> String {
        self.changed_target_system().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.target_system.clone())
    }

    pub fn update_target_system(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.target_system = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.target_system.clone());
        self.root.set(self.entity_key(), "target_system", value);
        self
    }

    pub fn changed_target_system(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "target_system")
    }

    pub fn eval_target_system(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("target_system") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "target_system".to_string(), attempted_path: "target_system".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.target_system())
                }}

    pub fn payload(&self) -> String {
        self.changed_payload().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.payload.clone())
    }

    pub fn update_payload(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.payload = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.payload.clone());
        self.root.set(self.entity_key(), "payload", value);
        self
    }

    pub fn changed_payload(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "payload")
    }

    pub fn eval_payload(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("payload") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "payload".to_string(), attempted_path: "payload".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.payload())
                }}

    pub fn retry_count(&self) -> i64 {
        self.changed_retry_count().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.retry_count)
    }

    pub fn update_retry_count(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.retry_count = value.try_i64().map(|value| value as i64).unwrap_or(self.retry_count.clone());
        self.root.set(self.entity_key(), "retry_count", value);
        self
    }

    pub fn changed_retry_count(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "retry_count")
    }

    pub fn eval_retry_count(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("retry_count") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "retry_count".to_string(), attempted_path: "retry_count".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.retry_count())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::AutomationActionRepository<'a>>>
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
            .automation_action_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("AutomationAction"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

