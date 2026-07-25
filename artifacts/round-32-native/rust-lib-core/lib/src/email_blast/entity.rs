// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/email_blast
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "EmailBlast", table = "email_blast_data", data_service = "sqlite", audit_mask_fields = "sender_email")]
pub struct EmailBlast {
#[teaql(id)]
    id: u64,

// @source module_6.xml:8
    subject: String,

// @source module_6.xml:8
    content: String,

// @source module_6.xml:8
    sender_email: String,

// @source module_6.xml:8
    send_date: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl EmailBlast {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            subject: String::new(),
            content: String::new(),
            sender_email: String::new(),
            send_date: chrono::Utc::now(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("EmailBlast", self.id)
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

    pub fn subject(&self) -> String {
        self.changed_subject().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.subject.clone())
    }

    pub fn update_subject(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.subject = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.subject.clone());
        self.root.set(self.entity_key(), "subject", value);
        self
    }

    pub fn changed_subject(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "subject")
    }

    pub fn eval_subject(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("subject") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "subject".to_string(), attempted_path: "subject".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.subject())
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

    pub fn sender_email(&self) -> String {
        self.changed_sender_email().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.sender_email.clone())
    }

    pub fn update_sender_email(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.sender_email = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.sender_email.clone());
        self.root.set(self.entity_key(), "sender_email", value);
        self
    }

    pub fn changed_sender_email(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "sender_email")
    }

    pub fn eval_sender_email(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("sender_email") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "sender_email".to_string(), attempted_path: "sender_email".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.sender_email())
                }}

    pub fn send_date(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_send_date().and_then(|value| value.try_timestamp()).unwrap_or(self.send_date)
    }

    pub fn update_send_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.send_date = value.try_timestamp().unwrap_or(self.send_date.clone());
        self.root.set(self.entity_key(), "send_date", value);
        self
    }

    pub fn changed_send_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "send_date")
    }

    pub fn eval_send_date(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("send_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "send_date".to_string(), attempted_path: "send_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.send_date())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::EmailBlastRepository<'a>>>
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
            .email_blast_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("EmailBlast"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

