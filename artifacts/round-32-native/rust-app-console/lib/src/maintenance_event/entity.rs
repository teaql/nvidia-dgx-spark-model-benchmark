// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/maintenance_event
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "MaintenanceEvent", table = "maintenance_event_data", data_service = "sqlite")]
pub struct MaintenanceEvent {
#[teaql(id)]
    id: u64,

// @source module_8.xml:7
    service_date: chrono::NaiveDate,

// @source module_8.xml:7
    cost: String,

// @source module_8.xml:7
    service_provider: String,

// @source module_8.xml:7
#[teaql(large_text)]
    description: Option<String>,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl MaintenanceEvent {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            service_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            cost: String::new(),
            service_provider: String::new(),
            description: None,
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("MaintenanceEvent", self.id)
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

    pub fn service_date(&self) -> chrono::NaiveDate {
        self.changed_service_date().and_then(|value| value.try_date()).unwrap_or(self.service_date)
    }

    pub fn update_service_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.service_date = value.try_date().unwrap_or(self.service_date.clone());
        self.root.set(self.entity_key(), "service_date", value);
        self
    }

    pub fn changed_service_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "service_date")
    }

    pub fn eval_service_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("service_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "service_date".to_string(), attempted_path: "service_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.service_date())
                }}

    pub fn cost(&self) -> String {
        self.changed_cost().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.cost.clone())
    }

    pub fn update_cost(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.cost = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.cost.clone());
        self.root.set(self.entity_key(), "cost", value);
        self
    }

    pub fn changed_cost(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "cost")
    }

    pub fn eval_cost(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("cost") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "cost".to_string(), attempted_path: "cost".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.cost())
                }}

    pub fn service_provider(&self) -> String {
        self.changed_service_provider().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.service_provider.clone())
    }

    pub fn update_service_provider(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.service_provider = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.service_provider.clone());
        self.root.set(self.entity_key(), "service_provider", value);
        self
    }

    pub fn changed_service_provider(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "service_provider")
    }

    pub fn eval_service_provider(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("service_provider") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "service_provider".to_string(), attempted_path: "service_provider".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.service_provider())
                }}

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn update_description(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.description = if matches!(value, teaql_core::Value::Null) { None } else { value.try_text().map(|value| value.trim().to_owned()).map(Some).unwrap_or_else(|| self.description.clone()) };
        self.root.set(self.entity_key(), "description", value);
        self
    }

    pub fn changed_description(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "description")
    }

    pub fn eval_description(&self) -> teaql_core::eval::EvalResult<Option<String>> {
        if !self.is_loaded("description") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "description".to_string(), attempted_path: "description".to_string() }
                } else {
                    match &self.description {
                        Some(v) => teaql_core::eval::EvalResult::Value(v.clone()),
                        None => teaql_core::eval::EvalResult::Null,
                    }
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::MaintenanceEventRepository<'a>>>
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
            .maintenance_event_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("MaintenanceEvent"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

