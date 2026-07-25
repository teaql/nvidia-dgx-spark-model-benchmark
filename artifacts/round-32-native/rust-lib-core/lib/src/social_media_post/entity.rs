// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/social_media_post
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "SocialMediaPost", table = "social_media_post_data", data_service = "sqlite")]
pub struct SocialMediaPost {
#[teaql(id)]
    id: u64,

// @source module_6.xml:7
    platform: String,

// @source module_6.xml:7
    content: String,

// @source module_6.xml:7
    post_date: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl SocialMediaPost {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            platform: String::new(),
            content: String::new(),
            post_date: chrono::Utc::now(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("SocialMediaPost", self.id)
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

    pub fn platform(&self) -> String {
        self.changed_platform().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.platform.clone())
    }

    pub fn update_platform(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.platform = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.platform.clone());
        self.root.set(self.entity_key(), "platform", value);
        self
    }

    pub fn changed_platform(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "platform")
    }

    pub fn eval_platform(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("platform") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform".to_string(), attempted_path: "platform".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.platform())
                }}

    pub fn content(&self) -> String {
        self.changed_content().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.content.clone())
    }

    pub fn update_content(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.content = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.content.clone());
        self.root.set(self.entity_key(), "content", value);
        self
    }

    pub fn changed_content(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "content")
    }

    pub fn eval_content(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("content") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "content".to_string(), attempted_path: "content".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.content())
                }}

    pub fn post_date(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_post_date().and_then(|value| value.try_timestamp()).unwrap_or(self.post_date)
    }

    pub fn update_post_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.post_date = value.try_timestamp().unwrap_or(self.post_date.clone());
        self.root.set(self.entity_key(), "post_date", value);
        self
    }

    pub fn changed_post_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "post_date")
    }

    pub fn eval_post_date(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("post_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "post_date".to_string(), attempted_path: "post_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.post_date())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::SocialMediaPostRepository<'a>>>
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
            .social_media_post_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("SocialMediaPost"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

