// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/entity_change
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "EntityChange", table = "entity_change_data", data_service = "sqlite")]
pub struct EntityChange {
#[teaql(id)]
    id: u64,

// @source module_10.xml:14
    activity_log_ref: String,

// @source module_10.xml:14
    field_name: String,

// @source module_10.xml:14
    old_value: String,

// @source module_10.xml:14
    new_value: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl EntityChange {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            activity_log_ref: String::new(),
            field_name: String::new(),
            old_value: String::new(),
            new_value: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("EntityChange", self.id)
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

    pub fn activity_log_ref(&self) -> String {
        self.changed_activity_log_ref().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.activity_log_ref.clone())
    }

    pub fn update_activity_log_ref(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.activity_log_ref = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.activity_log_ref.clone());
        self.root.set(self.entity_key(), "activity_log_ref", value);
        self
    }

    pub fn changed_activity_log_ref(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "activity_log_ref")
    }

    pub fn eval_activity_log_ref(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("activity_log_ref") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "activity_log_ref".to_string(), attempted_path: "activity_log_ref".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.activity_log_ref())
                }}

    pub fn field_name(&self) -> String {
        self.changed_field_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.field_name.clone())
    }

    pub fn update_field_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.field_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.field_name.clone());
        self.root.set(self.entity_key(), "field_name", value);
        self
    }

    pub fn changed_field_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "field_name")
    }

    pub fn eval_field_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("field_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "field_name".to_string(), attempted_path: "field_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.field_name())
                }}

    pub fn old_value(&self) -> String {
        self.changed_old_value().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.old_value.clone())
    }

    pub fn update_old_value(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.old_value = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.old_value.clone());
        self.root.set(self.entity_key(), "old_value", value);
        self
    }

    pub fn changed_old_value(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "old_value")
    }

    pub fn eval_old_value(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("old_value") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "old_value".to_string(), attempted_path: "old_value".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.old_value())
                }}

    pub fn new_value(&self) -> String {
        self.changed_new_value().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.new_value.clone())
    }

    pub fn update_new_value(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.new_value = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.new_value.clone());
        self.root.set(self.entity_key(), "new_value", value);
        self
    }

    pub fn changed_new_value(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "new_value")
    }

    pub fn eval_new_value(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("new_value") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "new_value".to_string(), attempted_path: "new_value".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.new_value())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::EntityChangeRepository<'a>>>
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
            .entity_change_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("EntityChange"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

