// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/sms_delivery_receipt
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "SmsDeliveryReceipt", table = "sms_delivery_receipt_data", data_service = "sqlite", audit_mask_fields = "phone")]
pub struct SmsDeliveryReceipt {
#[teaql(id)]
    id: u64,

// @source module_11.xml:8
    message_sid: String,

// @source module_11.xml:8
    status: String,

// @source module_11.xml:8
    error_code: String,

// @source module_11.xml:8
    delivered_at: chrono::DateTime<chrono::Utc>,

// @source module_11.xml:8
    phone: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl SmsDeliveryReceipt {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            message_sid: String::new(),
            status: String::new(),
            error_code: String::new(),
            delivered_at: chrono::Utc::now(),
            phone: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("SmsDeliveryReceipt", self.id)
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

    pub fn message_sid(&self) -> String {
        self.changed_message_sid().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.message_sid.clone())
    }

    pub fn update_message_sid(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.message_sid = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.message_sid.clone());
        self.root.set(self.entity_key(), "message_sid", value);
        self
    }

    pub fn changed_message_sid(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "message_sid")
    }

    pub fn eval_message_sid(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("message_sid") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "message_sid".to_string(), attempted_path: "message_sid".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.message_sid())
                }}

    pub fn status(&self) -> String {
        self.changed_status().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.status.clone())
    }

    pub fn update_status(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.status = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.status.clone());
        self.root.set(self.entity_key(), "status", value);
        self
    }

    pub fn changed_status(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "status")
    }

    pub fn eval_status(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("status") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "status".to_string(), attempted_path: "status".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.status())
                }}

    pub fn error_code(&self) -> String {
        self.changed_error_code().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.error_code.clone())
    }

    pub fn update_error_code(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.error_code = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.error_code.clone());
        self.root.set(self.entity_key(), "error_code", value);
        self
    }

    pub fn changed_error_code(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "error_code")
    }

    pub fn eval_error_code(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("error_code") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "error_code".to_string(), attempted_path: "error_code".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.error_code())
                }}

    pub fn delivered_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_delivered_at().and_then(|value| value.try_timestamp()).unwrap_or(self.delivered_at)
    }

    pub fn update_delivered_at(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.delivered_at = value.try_timestamp().unwrap_or(self.delivered_at.clone());
        self.root.set(self.entity_key(), "delivered_at", value);
        self
    }

    pub fn changed_delivered_at(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "delivered_at")
    }

    pub fn eval_delivered_at(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("delivered_at") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "delivered_at".to_string(), attempted_path: "delivered_at".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.delivered_at())
                }}

    pub fn phone(&self) -> String {
        self.changed_phone().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.phone.clone())
    }

    pub fn update_phone(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.phone = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.phone.clone());
        self.root.set(self.entity_key(), "phone", value);
        self
    }

    pub fn changed_phone(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "phone")
    }

    pub fn eval_phone(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("phone") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "phone".to_string(), attempted_path: "phone".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.phone())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::SmsDeliveryReceiptRepository<'a>>>
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
            .sms_delivery_receipt_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("SmsDeliveryReceipt"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

