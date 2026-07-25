// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/magic_link
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "MagicLink", table = "magic_link_data", data_service = "sqlite", audit_mask_fields = "token")]
pub struct MagicLink {
#[teaql(id)]
    id: u64,

// @source module_10.xml:7
    user_account_ref: String,

// @source module_10.xml:7
    token: String,

// @source module_10.xml:7
    expires_at: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl MagicLink {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            user_account_ref: String::new(),
            token: String::new(),
            expires_at: chrono::Utc::now(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("MagicLink", self.id)
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

    pub fn user_account_ref(&self) -> String {
        self.changed_user_account_ref().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.user_account_ref.clone())
    }

    pub fn update_user_account_ref(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.user_account_ref = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.user_account_ref.clone());
        self.root.set(self.entity_key(), "user_account_ref", value);
        self
    }

    pub fn changed_user_account_ref(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "user_account_ref")
    }

    pub fn eval_user_account_ref(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("user_account_ref") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "user_account_ref".to_string(), attempted_path: "user_account_ref".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.user_account_ref())
                }}

    pub fn token(&self) -> String {
        self.changed_token().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.token.clone())
    }

    pub fn update_token(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.token = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.token.clone());
        self.root.set(self.entity_key(), "token", value);
        self
    }

    pub fn changed_token(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "token")
    }

    pub fn eval_token(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("token") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "token".to_string(), attempted_path: "token".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.token())
                }}

    pub fn expires_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_expires_at().and_then(|value| value.try_timestamp()).unwrap_or(self.expires_at)
    }

    pub fn update_expires_at(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.expires_at = value.try_timestamp().unwrap_or(self.expires_at.clone());
        self.root.set(self.entity_key(), "expires_at", value);
        self
    }

    pub fn changed_expires_at(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "expires_at")
    }

    pub fn eval_expires_at(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("expires_at") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "expires_at".to_string(), attempted_path: "expires_at".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.expires_at())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::MagicLinkRepository<'a>>>
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
            .magic_link_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("MagicLink"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

