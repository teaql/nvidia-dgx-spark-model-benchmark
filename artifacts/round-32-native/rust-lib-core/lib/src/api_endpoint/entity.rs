// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/api_endpoint
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "ApiEndpoint", table = "api_endpoint_data", data_service = "sqlite")]
pub struct ApiEndpoint {
#[teaql(id)]
    id: u64,

// @source module_11.xml:11
    path_pattern: String,

// @source module_11.xml:11
    http_method: String,

// @source module_11.xml:11
    version_tag: String,

// @source module_11.xml:11
    is_deprecated: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl ApiEndpoint {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            path_pattern: String::new(),
            http_method: String::new(),
            version_tag: String::new(),
            is_deprecated: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("ApiEndpoint", self.id)
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

    pub fn path_pattern(&self) -> String {
        self.changed_path_pattern().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.path_pattern.clone())
    }

    pub fn update_path_pattern(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.path_pattern = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.path_pattern.clone());
        self.root.set(self.entity_key(), "path_pattern", value);
        self
    }

    pub fn changed_path_pattern(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "path_pattern")
    }

    pub fn eval_path_pattern(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("path_pattern") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "path_pattern".to_string(), attempted_path: "path_pattern".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.path_pattern())
                }}

    pub fn http_method(&self) -> String {
        self.changed_http_method().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.http_method.clone())
    }

    pub fn update_http_method(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.http_method = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.http_method.clone());
        self.root.set(self.entity_key(), "http_method", value);
        self
    }

    pub fn changed_http_method(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "http_method")
    }

    pub fn eval_http_method(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("http_method") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "http_method".to_string(), attempted_path: "http_method".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.http_method())
                }}

    pub fn version_tag(&self) -> String {
        self.changed_version_tag().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.version_tag.clone())
    }

    pub fn update_version_tag(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.version_tag = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.version_tag.clone());
        self.root.set(self.entity_key(), "version_tag", value);
        self
    }

    pub fn changed_version_tag(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "version_tag")
    }

    pub fn eval_version_tag(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("version_tag") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "version_tag".to_string(), attempted_path: "version_tag".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.version_tag())
                }}

    pub fn is_deprecated(&self) -> String {
        self.changed_is_deprecated().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.is_deprecated.clone())
    }

    pub fn update_is_deprecated(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.is_deprecated = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.is_deprecated.clone());
        self.root.set(self.entity_key(), "is_deprecated", value);
        self
    }

    pub fn changed_is_deprecated(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "is_deprecated")
    }

    pub fn eval_is_deprecated(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("is_deprecated") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "is_deprecated".to_string(), attempted_path: "is_deprecated".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.is_deprecated())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::ApiEndpointRepository<'a>>>
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
            .api_endpoint_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("ApiEndpoint"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

