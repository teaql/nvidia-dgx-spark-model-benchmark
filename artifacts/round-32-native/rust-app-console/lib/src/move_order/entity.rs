// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/move_order
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "MoveOrder", table = "move_order_data", data_service = "sqlite")]
pub struct MoveOrder {
#[teaql(id)]
    id: u64,

// @source module_0.xml:9
    order_number: String,

// @source module_0.xml:9
    status: String,

// @source module_0.xml:9
    category: String,

// @source module_0.xml:9
    priority: i64,

// @source module_0.xml:9
    scheduled_date: chrono::NaiveDate,

// @source module_0.xml:9
    total_amount: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl MoveOrder {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            order_number: String::new(),
            status: String::new(),
            category: String::new(),
            priority: 0_i64,
            scheduled_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            total_amount: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("MoveOrder", self.id)
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

    pub fn order_number(&self) -> String {
        self.changed_order_number().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.order_number.clone())
    }

    pub fn update_order_number(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.order_number = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.order_number.clone());
        self.root.set(self.entity_key(), "order_number", value);
        self
    }

    pub fn changed_order_number(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "order_number")
    }

    pub fn eval_order_number(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("order_number") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "order_number".to_string(), attempted_path: "order_number".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.order_number())
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

    pub fn priority(&self) -> i64 {
        self.changed_priority().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.priority)
    }

    pub fn update_priority(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.priority = value.try_i64().map(|value| value as i64).unwrap_or(self.priority.clone());
        self.root.set(self.entity_key(), "priority", value);
        self
    }

    pub fn changed_priority(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "priority")
    }

    pub fn eval_priority(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("priority") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "priority".to_string(), attempted_path: "priority".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.priority())
                }}

    pub fn scheduled_date(&self) -> chrono::NaiveDate {
        self.changed_scheduled_date().and_then(|value| value.try_date()).unwrap_or(self.scheduled_date)
    }

    pub fn update_scheduled_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.scheduled_date = value.try_date().unwrap_or(self.scheduled_date.clone());
        self.root.set(self.entity_key(), "scheduled_date", value);
        self
    }

    pub fn changed_scheduled_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "scheduled_date")
    }

    pub fn eval_scheduled_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("scheduled_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "scheduled_date".to_string(), attempted_path: "scheduled_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.scheduled_date())
                }}

    pub fn total_amount(&self) -> String {
        self.changed_total_amount().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.total_amount.clone())
    }

    pub fn update_total_amount(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.total_amount = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.total_amount.clone());
        self.root.set(self.entity_key(), "total_amount", value);
        self
    }

    pub fn changed_total_amount(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "total_amount")
    }

    pub fn eval_total_amount(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("total_amount") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "total_amount".to_string(), attempted_path: "total_amount".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.total_amount())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
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
            .move_order_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("MoveOrder"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

