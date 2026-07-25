// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/webhook_delivery
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "WebhookDelivery", table = "webhook_delivery_data", data_service = "sqlite")]
pub struct WebhookDelivery {
#[teaql(id)]
    id: u64,

// @source module_11.xml:13
    delivery_status: String,

// @source module_11.xml:13
    response_code: i64,

// @source module_11.xml:13
    attempted_at: chrono::DateTime<chrono::Utc>,

// @source module_11.xml:13
    response_body: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl WebhookDelivery {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            delivery_status: String::new(),
            response_code: 0_i64,
            attempted_at: chrono::Utc::now(),
            response_body: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("WebhookDelivery", self.id)
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

    pub fn delivery_status(&self) -> String {
        self.changed_delivery_status().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.delivery_status.clone())
    }

    pub fn update_delivery_status(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.delivery_status = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.delivery_status.clone());
        self.root.set(self.entity_key(), "delivery_status", value);
        self
    }

    pub fn changed_delivery_status(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "delivery_status")
    }

    pub fn eval_delivery_status(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("delivery_status") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "delivery_status".to_string(), attempted_path: "delivery_status".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.delivery_status())
                }}

    pub fn response_code(&self) -> i64 {
        self.changed_response_code().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.response_code)
    }

    pub fn update_response_code(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.response_code = value.try_i64().map(|value| value as i64).unwrap_or(self.response_code.clone());
        self.root.set(self.entity_key(), "response_code", value);
        self
    }

    pub fn changed_response_code(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "response_code")
    }

    pub fn eval_response_code(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("response_code") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "response_code".to_string(), attempted_path: "response_code".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.response_code())
                }}

    pub fn attempted_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_attempted_at().and_then(|value| value.try_timestamp()).unwrap_or(self.attempted_at)
    }

    pub fn update_attempted_at(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.attempted_at = value.try_timestamp().unwrap_or(self.attempted_at.clone());
        self.root.set(self.entity_key(), "attempted_at", value);
        self
    }

    pub fn changed_attempted_at(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "attempted_at")
    }

    pub fn eval_attempted_at(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("attempted_at") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "attempted_at".to_string(), attempted_path: "attempted_at".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.attempted_at())
                }}

    pub fn response_body(&self) -> String {
        self.changed_response_body().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.response_body.clone())
    }

    pub fn update_response_body(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.response_body = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.response_body.clone());
        self.root.set(self.entity_key(), "response_body", value);
        self
    }

    pub fn changed_response_body(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "response_body")
    }

    pub fn eval_response_body(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("response_body") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "response_body".to_string(), attempted_path: "response_body".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.response_body())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::WebhookDeliveryRepository<'a>>>
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
            .webhook_delivery_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("WebhookDelivery"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

