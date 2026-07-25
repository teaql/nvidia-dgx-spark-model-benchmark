// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/platform_config
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "PlatformConfig", table = "platform_config_data", data_service = "sqlite", audit_mask_fields = "secret_key")]
pub struct PlatformConfig {
#[teaql(id)]
    id: u64,

// @source module_0.xml:4
    config_key: String,

// @source module_0.xml:4
    config_value: String,

// @source module_0.xml:4
    category: String,

// @source module_0.xml:4
    is_enabled: String,

// @source module_0.xml:4
    secret_key: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl PlatformConfig {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            config_key: String::new(),
            config_value: String::new(),
            category: String::new(),
            is_enabled: String::new(),
            secret_key: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("PlatformConfig", self.id)
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

    pub fn config_key(&self) -> String {
        self.changed_config_key().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.config_key.clone())
    }

    pub fn update_config_key(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.config_key = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.config_key.clone());
        self.root.set(self.entity_key(), "config_key", value);
        self
    }

    pub fn changed_config_key(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "config_key")
    }

    pub fn eval_config_key(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("config_key") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "config_key".to_string(), attempted_path: "config_key".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.config_key())
                }}

    pub fn config_value(&self) -> String {
        self.changed_config_value().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.config_value.clone())
    }

    pub fn update_config_value(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.config_value = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.config_value.clone());
        self.root.set(self.entity_key(), "config_value", value);
        self
    }

    pub fn changed_config_value(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "config_value")
    }

    pub fn eval_config_value(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("config_value") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "config_value".to_string(), attempted_path: "config_value".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.config_value())
                }}

    pub fn category(&self) -> String {
        self.changed_category().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.category.clone())
    }

    pub fn update_category(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.category = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.category.clone());
        self.root.set(self.entity_key(), "category", value);
        self
    }

    pub fn changed_category(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "category")
    }

    pub fn eval_category(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("category") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "category".to_string(), attempted_path: "category".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.category())
                }}

    pub fn is_enabled(&self) -> String {
        self.changed_is_enabled().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.is_enabled.clone())
    }

    pub fn update_is_enabled(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.is_enabled = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.is_enabled.clone());
        self.root.set(self.entity_key(), "is_enabled", value);
        self
    }

    pub fn changed_is_enabled(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "is_enabled")
    }

    pub fn eval_is_enabled(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("is_enabled") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "is_enabled".to_string(), attempted_path: "is_enabled".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.is_enabled())
                }}

    pub fn secret_key(&self) -> String {
        self.changed_secret_key().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.secret_key.clone())
    }

    pub fn update_secret_key(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.secret_key = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.secret_key.clone());
        self.root.set(self.entity_key(), "secret_key", value);
        self
    }

    pub fn changed_secret_key(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "secret_key")
    }

    pub fn eval_secret_key(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("secret_key") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "secret_key".to_string(), attempted_path: "secret_key".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.secret_key())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
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
            .platform_config_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("PlatformConfig"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

