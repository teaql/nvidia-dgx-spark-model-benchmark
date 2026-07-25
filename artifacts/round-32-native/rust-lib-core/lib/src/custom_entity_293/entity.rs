// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/custom_entity_293
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "CustomEntity293", table = "custom_entity_293_data", data_service = "sqlite")]
pub struct CustomEntity293 {
#[teaql(id)]
    id: u64,

// @source module_19.xml:10
    comment: String,

// @source module_19.xml:10
    author: String,

// @source module_19.xml:10
    posted_at: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl CustomEntity293 {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            comment: String::new(),
            author: String::new(),
            posted_at: chrono::Utc::now(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("CustomEntity293", self.id)
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

    pub fn comment(&self) -> String {
        self.changed_comment().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.comment.clone())
    }

    pub fn update_comment(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.comment = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.comment.clone());
        self.root.set(self.entity_key(), "comment", value);
        self
    }

    pub fn changed_comment(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "comment")
    }

    pub fn eval_comment(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("comment") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "comment".to_string(), attempted_path: "comment".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.comment())
                }}

    pub fn author(&self) -> String {
        self.changed_author().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.author.clone())
    }

    pub fn update_author(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.author = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.author.clone());
        self.root.set(self.entity_key(), "author", value);
        self
    }

    pub fn changed_author(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "author")
    }

    pub fn eval_author(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("author") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "author".to_string(), attempted_path: "author".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.author())
                }}

    pub fn posted_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_posted_at().and_then(|value| value.try_timestamp()).unwrap_or(self.posted_at)
    }

    pub fn update_posted_at(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.posted_at = value.try_timestamp().unwrap_or(self.posted_at.clone());
        self.root.set(self.entity_key(), "posted_at", value);
        self
    }

    pub fn changed_posted_at(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "posted_at")
    }

    pub fn eval_posted_at(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("posted_at") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "posted_at".to_string(), attempted_path: "posted_at".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.posted_at())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CustomEntity293Repository<'a>>>
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
            .custom_entity_293_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("CustomEntity293"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

