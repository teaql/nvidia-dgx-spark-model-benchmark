// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/direct_deposit_info
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "DirectDepositInfo", table = "direct_deposit_info_data", data_service = "sqlite", audit_mask_fields = "account_number,routing_number")]
pub struct DirectDepositInfo {
#[teaql(id)]
    id: u64,

// @source module_2.xml:15
    bank_institution: String,

// @source module_2.xml:15
    account_number: String,

// @source module_2.xml:15
    routing_number: String,

// @source module_2.xml:15
    account_kind: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl DirectDepositInfo {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            bank_institution: String::new(),
            account_number: String::new(),
            routing_number: String::new(),
            account_kind: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("DirectDepositInfo", self.id)
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

    pub fn bank_institution(&self) -> String {
        self.changed_bank_institution().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.bank_institution.clone())
    }

    pub fn update_bank_institution(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.bank_institution = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.bank_institution.clone());
        self.root.set(self.entity_key(), "bank_institution", value);
        self
    }

    pub fn changed_bank_institution(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "bank_institution")
    }

    pub fn eval_bank_institution(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("bank_institution") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "bank_institution".to_string(), attempted_path: "bank_institution".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.bank_institution())
                }}

    pub fn account_number(&self) -> String {
        self.changed_account_number().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.account_number.clone())
    }

    pub fn update_account_number(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.account_number = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.account_number.clone());
        self.root.set(self.entity_key(), "account_number", value);
        self
    }

    pub fn changed_account_number(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "account_number")
    }

    pub fn eval_account_number(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("account_number") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "account_number".to_string(), attempted_path: "account_number".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.account_number())
                }}

    pub fn routing_number(&self) -> String {
        self.changed_routing_number().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.routing_number.clone())
    }

    pub fn update_routing_number(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.routing_number = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.routing_number.clone());
        self.root.set(self.entity_key(), "routing_number", value);
        self
    }

    pub fn changed_routing_number(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "routing_number")
    }

    pub fn eval_routing_number(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("routing_number") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "routing_number".to_string(), attempted_path: "routing_number".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.routing_number())
                }}

    pub fn account_kind(&self) -> String {
        self.changed_account_kind().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.account_kind.clone())
    }

    pub fn update_account_kind(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.account_kind = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.account_kind.clone());
        self.root.set(self.entity_key(), "account_kind", value);
        self
    }

    pub fn changed_account_kind(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "account_kind")
    }

    pub fn eval_account_kind(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("account_kind") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "account_kind".to_string(), attempted_path: "account_kind".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.account_kind())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
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
            .direct_deposit_info_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("DirectDepositInfo"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

