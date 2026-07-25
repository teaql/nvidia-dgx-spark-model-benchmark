// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/automation_trigger
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "AutomationTrigger", table = "automation_trigger_data", data_service = "sqlite")]
pub struct AutomationTrigger {
#[teaql(id)]
    id: u64,

// @source module_11.xml:6
    trigger_event: String,

// @source module_11.xml:6
    condition_expression: String,

// @source module_11.xml:6
    is_active: String,

// @source module_11.xml:6
    execution_order: i64,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl AutomationTrigger {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            trigger_event: String::new(),
            condition_expression: String::new(),
            is_active: String::new(),
            execution_order: 0_i64,
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("AutomationTrigger", self.id)
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

    pub fn trigger_event(&self) -> String {
        self.changed_trigger_event().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.trigger_event.clone())
    }

    pub fn update_trigger_event(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.trigger_event = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.trigger_event.clone());
        self.root.set(self.entity_key(), "trigger_event", value);
        self
    }

    pub fn changed_trigger_event(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "trigger_event")
    }

    pub fn eval_trigger_event(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("trigger_event") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "trigger_event".to_string(), attempted_path: "trigger_event".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.trigger_event())
                }}

    pub fn condition_expression(&self) -> String {
        self.changed_condition_expression().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.condition_expression.clone())
    }

    pub fn update_condition_expression(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.condition_expression = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.condition_expression.clone());
        self.root.set(self.entity_key(), "condition_expression", value);
        self
    }

    pub fn changed_condition_expression(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "condition_expression")
    }

    pub fn eval_condition_expression(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("condition_expression") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "condition_expression".to_string(), attempted_path: "condition_expression".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.condition_expression())
                }}

    pub fn is_active(&self) -> String {
        self.changed_is_active().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.is_active.clone())
    }

    pub fn update_is_active(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.is_active = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.is_active.clone());
        self.root.set(self.entity_key(), "is_active", value);
        self
    }

    pub fn changed_is_active(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "is_active")
    }

    pub fn eval_is_active(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("is_active") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "is_active".to_string(), attempted_path: "is_active".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.is_active())
                }}

    pub fn execution_order(&self) -> i64 {
        self.changed_execution_order().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.execution_order)
    }

    pub fn update_execution_order(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.execution_order = value.try_i64().map(|value| value as i64).unwrap_or(self.execution_order.clone());
        self.root.set(self.entity_key(), "execution_order", value);
        self
    }

    pub fn changed_execution_order(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "execution_order")
    }

    pub fn eval_execution_order(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("execution_order") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "execution_order".to_string(), attempted_path: "execution_order".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.execution_order())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
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
            .automation_trigger_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("AutomationTrigger"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

