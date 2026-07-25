// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/terms_of_service
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "TermsOfService", table = "terms_of_service_data", data_service = "sqlite")]
pub struct TermsOfService {
#[teaql(id)]
    id: u64,

// @source module_9.xml:12
    version_string: String,

// @source module_9.xml:12
    effective_date: chrono::NaiveDate,

// @source module_9.xml:12
    content_url: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl TermsOfService {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            version_string: String::new(),
            effective_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            content_url: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("TermsOfService", self.id)
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

    pub fn version_string(&self) -> String {
        self.changed_version_string().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.version_string.clone())
    }

    pub fn update_version_string(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.version_string = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.version_string.clone());
        self.root.set(self.entity_key(), "version_string", value);
        self
    }

    pub fn changed_version_string(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "version_string")
    }

    pub fn eval_version_string(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("version_string") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "version_string".to_string(), attempted_path: "version_string".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.version_string())
                }}

    pub fn effective_date(&self) -> chrono::NaiveDate {
        self.changed_effective_date().and_then(|value| value.try_date()).unwrap_or(self.effective_date)
    }

    pub fn update_effective_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.effective_date = value.try_date().unwrap_or(self.effective_date.clone());
        self.root.set(self.entity_key(), "effective_date", value);
        self
    }

    pub fn changed_effective_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "effective_date")
    }

    pub fn eval_effective_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("effective_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "effective_date".to_string(), attempted_path: "effective_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.effective_date())
                }}

    pub fn content_url(&self) -> String {
        self.changed_content_url().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.content_url.clone())
    }

    pub fn update_content_url(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.content_url = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.content_url.clone());
        self.root.set(self.entity_key(), "content_url", value);
        self
    }

    pub fn changed_content_url(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "content_url")
    }

    pub fn eval_content_url(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("content_url") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "content_url".to_string(), attempted_path: "content_url".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.content_url())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::TermsOfServiceRepository<'a>>>
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
            .terms_of_service_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("TermsOfService"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

