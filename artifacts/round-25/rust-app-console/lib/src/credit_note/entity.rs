// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/credit_note
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "CreditNote", table = "credit_note_data", data_service = "sqlite")]
pub struct CreditNote {
#[teaql(id)]
    id: u64,

// @source module_7.xml:107
    note_number: String,

// @source module_7.xml:107
    related_invoice: String,

// @source module_7.xml:107
    amount: rust_decimal::Decimal,

// @source module_7.xml:107
    issue_date: chrono::NaiveDate,

// @source module_7.xml:107
    create_time: chrono::NaiveDate,

// @source module_7.xml:107
    update_time: chrono::NaiveDate,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl CreditNote {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            note_number: String::new(),
            related_invoice: String::new(),
            amount: rust_decimal::Decimal::ZERO,
            issue_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            create_time: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            update_time: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("CreditNote", self.id)
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

    pub fn note_number(&self) -> String {
        self.changed_note_number().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.note_number.clone())
    }

    pub fn update_note_number(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.note_number = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.note_number.clone());
        self.root.set(self.entity_key(), "note_number", value);
        self
    }

    pub fn changed_note_number(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "note_number")
    }

    pub fn eval_note_number(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("note_number") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "note_number".to_string(), attempted_path: "note_number".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.note_number())
                }}

    pub fn related_invoice(&self) -> String {
        self.changed_related_invoice().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.related_invoice.clone())
    }

    pub fn update_related_invoice(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.related_invoice = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.related_invoice.clone());
        self.root.set(self.entity_key(), "related_invoice", value);
        self
    }

    pub fn changed_related_invoice(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "related_invoice")
    }

    pub fn eval_related_invoice(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("related_invoice") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "related_invoice".to_string(), attempted_path: "related_invoice".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.related_invoice())
                }}

    pub fn amount(&self) -> rust_decimal::Decimal {
        self.changed_amount().and_then(|value| value.try_decimal()).unwrap_or(self.amount)
    }

    pub fn update_amount(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.amount = value.try_decimal().unwrap_or(self.amount.clone());
        self.root.set(self.entity_key(), "amount", value);
        self
    }

    pub fn changed_amount(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "amount")
    }

    pub fn eval_amount(&self) -> teaql_core::eval::EvalResult<rust_decimal::Decimal> {
        if !self.is_loaded("amount") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "amount".to_string(), attempted_path: "amount".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.amount())
                }}

    pub fn issue_date(&self) -> chrono::NaiveDate {
        self.changed_issue_date().and_then(|value| value.try_date()).unwrap_or(self.issue_date)
    }

    pub fn update_issue_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.issue_date = value.try_date().unwrap_or(self.issue_date.clone());
        self.root.set(self.entity_key(), "issue_date", value);
        self
    }

    pub fn changed_issue_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "issue_date")
    }

    pub fn eval_issue_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("issue_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "issue_date".to_string(), attempted_path: "issue_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.issue_date())
                }}

    pub fn create_time(&self) -> chrono::NaiveDate {
        self.changed_create_time().and_then(|value| value.try_date()).unwrap_or(self.create_time)
    }

    pub fn update_create_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.create_time = value.try_date().unwrap_or(self.create_time.clone());
        self.root.set(self.entity_key(), "create_time", value);
        self
    }

    pub fn changed_create_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "create_time")
    }

    pub fn eval_create_time(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("create_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "create_time".to_string(), attempted_path: "create_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.create_time())
                }}

    pub fn update_time(&self) -> chrono::NaiveDate {
        self.changed_update_time().and_then(|value| value.try_date()).unwrap_or(self.update_time)
    }

    pub fn update_update_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.update_time = value.try_date().unwrap_or(self.update_time.clone());
        self.root.set(self.entity_key(), "update_time", value);
        self
    }

    pub fn changed_update_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "update_time")
    }

    pub fn eval_update_time(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("update_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "update_time".to_string(), attempted_path: "update_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.update_time())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CreditNoteRepository<'a>>>
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
            .credit_note_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("CreditNote"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

