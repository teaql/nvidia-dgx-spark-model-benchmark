// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/cleaning_service
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "CleaningService", table = "cleaning_service_data", data_service = "sqlite")]
pub struct CleaningService {
#[teaql(id)]
    id: u64,

// @source module_4.xml:14
    hours: i64,

// @source module_4.xml:14
    number_of_cleaners: i64,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl CleaningService {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            hours: 0_i64,
            number_of_cleaners: 0_i64,
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("CleaningService", self.id)
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

    pub fn hours(&self) -> i64 {
        self.changed_hours().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.hours)
    }

    pub fn update_hours(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.hours = value.try_i64().map(|value| value as i64).unwrap_or(self.hours.clone());
        self.root.set(self.entity_key(), "hours", value);
        self
    }

    pub fn changed_hours(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "hours")
    }

    pub fn eval_hours(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("hours") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "hours".to_string(), attempted_path: "hours".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.hours())
                }}

    pub fn number_of_cleaners(&self) -> i64 {
        self.changed_number_of_cleaners().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.number_of_cleaners)
    }

    pub fn update_number_of_cleaners(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.number_of_cleaners = value.try_i64().map(|value| value as i64).unwrap_or(self.number_of_cleaners.clone());
        self.root.set(self.entity_key(), "number_of_cleaners", value);
        self
    }

    pub fn changed_number_of_cleaners(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "number_of_cleaners")
    }

    pub fn eval_number_of_cleaners(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("number_of_cleaners") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "number_of_cleaners".to_string(), attempted_path: "number_of_cleaners".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.number_of_cleaners())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CleaningServiceRepository<'a>>>
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
            .cleaning_service_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("CleaningService"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

