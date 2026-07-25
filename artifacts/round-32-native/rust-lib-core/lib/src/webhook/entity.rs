// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/webhook
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Webhook", table = "webhook_data", data_service = "sqlite", audit_mask_fields = "secret_key")]
pub struct Webhook {
#[teaql(id)]
    id: u64,

// @source module_11.xml:12
    target_url: String,

// @source module_11.xml:12
    event_subscription: String,

// @source module_11.xml:12
    is_active: String,

// @source module_11.xml:12
    secret_key: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Webhook {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            target_url: String::new(),
            event_subscription: String::new(),
            is_active: String::new(),
            secret_key: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Webhook", self.id)
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

    pub fn target_url(&self) -> String {
        self.changed_target_url().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.target_url.clone())
    }

    pub fn update_target_url(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.target_url = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.target_url.clone());
        self.root.set(self.entity_key(), "target_url", value);
        self
    }

    pub fn changed_target_url(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "target_url")
    }

    pub fn eval_target_url(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("target_url") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "target_url".to_string(), attempted_path: "target_url".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.target_url())
                }}

    pub fn event_subscription(&self) -> String {
        self.changed_event_subscription().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.event_subscription.clone())
    }

    pub fn update_event_subscription(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.event_subscription = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.event_subscription.clone());
        self.root.set(self.entity_key(), "event_subscription", value);
        self
    }

    pub fn changed_event_subscription(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "event_subscription")
    }

    pub fn eval_event_subscription(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("event_subscription") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "event_subscription".to_string(), attempted_path: "event_subscription".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.event_subscription())
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

    pub fn secret_key(&self) -> String {
        self.changed_secret_key().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.secret_key.clone())
    }

    pub fn update_secret_key(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.secret_key = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.secret_key.clone());
        self.root.set(self.entity_key(), "secret_key", value);
        self
    }

    pub fn changed_secret_key(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "secret_key")
    }

    pub fn eval_secret_key(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("secret_key") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "secret_key".to_string(), attempted_path: "secret_key".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.secret_key())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::WebhookRepository<'a>>>
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
            .webhook_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Webhook"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

