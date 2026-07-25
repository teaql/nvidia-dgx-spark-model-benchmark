// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/custom_entity_221
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "CustomEntity221", table = "custom_entity_221_data", data_service = "sqlite")]
pub struct CustomEntity221 {
#[teaql(id)]
    id: u64,

// @source module_14.xml:13
    code: String,

// @source module_14.xml:13
    details: String,

// @source module_14.xml:13
    priority: i64,

// @source module_14.xml:13
    date_updated: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl CustomEntity221 {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            code: String::new(),
            details: String::new(),
            priority: 0_i64,
            date_updated: chrono::Utc::now(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("CustomEntity221", self.id)
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

    pub fn code(&self) -> String {
        self.changed_code().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.code.clone())
    }

    pub fn update_code(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.code = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.code.clone());
        self.root.set(self.entity_key(), "code", value);
        self
    }

    pub fn changed_code(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "code")
    }

    pub fn eval_code(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("code") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "code".to_string(), attempted_path: "code".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.code())
                }}

    pub fn details(&self) -> String {
        self.changed_details().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.details.clone())
    }

    pub fn update_details(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.details = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.details.clone());
        self.root.set(self.entity_key(), "details", value);
        self
    }

    pub fn changed_details(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "details")
    }

    pub fn eval_details(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("details") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "details".to_string(), attempted_path: "details".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.details())
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

    pub fn date_updated(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_date_updated().and_then(|value| value.try_timestamp()).unwrap_or(self.date_updated)
    }

    pub fn update_date_updated(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.date_updated = value.try_timestamp().unwrap_or(self.date_updated.clone());
        self.root.set(self.entity_key(), "date_updated", value);
        self
    }

    pub fn changed_date_updated(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "date_updated")
    }

    pub fn eval_date_updated(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("date_updated") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "date_updated".to_string(), attempted_path: "date_updated".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.date_updated())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CustomEntity221Repository<'a>>>
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
            .custom_entity_221_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("CustomEntity221"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

