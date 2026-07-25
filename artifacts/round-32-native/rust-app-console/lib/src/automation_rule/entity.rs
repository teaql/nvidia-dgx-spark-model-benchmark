// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/automation_rule
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "AutomationRule", table = "automation_rule_data", data_service = "sqlite")]
pub struct AutomationRule {
#[teaql(id)]
    id: u64,

// @source module_11.xml:5
    rule_code: String,

// @source module_11.xml:5
    description: String,

// @source module_11.xml:5
    category: String,

// @source module_11.xml:5
    is_enabled: String,

// @source module_11.xml:5
    created_at: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl AutomationRule {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            rule_code: String::new(),
            description: String::new(),
            category: String::new(),
            is_enabled: String::new(),
            created_at: chrono::Utc::now(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("AutomationRule", self.id)
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

    pub fn rule_code(&self) -> String {
        self.changed_rule_code().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.rule_code.clone())
    }

    pub fn update_rule_code(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.rule_code = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.rule_code.clone());
        self.root.set(self.entity_key(), "rule_code", value);
        self
    }

    pub fn changed_rule_code(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "rule_code")
    }

    pub fn eval_rule_code(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("rule_code") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "rule_code".to_string(), attempted_path: "rule_code".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.rule_code())
                }}

    pub fn description(&self) -> String {
        self.changed_description().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.description.clone())
    }

    pub fn update_description(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.description = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.description.clone());
        self.root.set(self.entity_key(), "description", value);
        self
    }

    pub fn changed_description(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "description")
    }

    pub fn eval_description(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("description") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "description".to_string(), attempted_path: "description".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.description())
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

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_created_at().and_then(|value| value.try_timestamp()).unwrap_or(self.created_at)
    }

    pub fn update_created_at(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.created_at = value.try_timestamp().unwrap_or(self.created_at.clone());
        self.root.set(self.entity_key(), "created_at", value);
        self
    }

    pub fn changed_created_at(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "created_at")
    }

    pub fn eval_created_at(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("created_at") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "created_at".to_string(), attempted_path: "created_at".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.created_at())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::AutomationRuleRepository<'a>>>
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
            .automation_rule_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("AutomationRule"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

