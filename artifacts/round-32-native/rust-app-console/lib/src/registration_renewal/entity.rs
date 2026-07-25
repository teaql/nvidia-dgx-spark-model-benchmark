// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/registration_renewal
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "RegistrationRenewal", table = "registration_renewal_data", data_service = "sqlite")]
pub struct RegistrationRenewal {
#[teaql(id)]
    id: u64,

// @source module_8.xml:14
    renewal_date: chrono::NaiveDate,

// @source module_8.xml:14
    expiration_date: chrono::NaiveDate,

// @source module_8.xml:14
    fee: String,

// @source module_8.xml:14
    state_code: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl RegistrationRenewal {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            renewal_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            expiration_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            fee: String::new(),
            state_code: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("RegistrationRenewal", self.id)
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

    pub fn renewal_date(&self) -> chrono::NaiveDate {
        self.changed_renewal_date().and_then(|value| value.try_date()).unwrap_or(self.renewal_date)
    }

    pub fn update_renewal_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.renewal_date = value.try_date().unwrap_or(self.renewal_date.clone());
        self.root.set(self.entity_key(), "renewal_date", value);
        self
    }

    pub fn changed_renewal_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "renewal_date")
    }

    pub fn eval_renewal_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("renewal_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "renewal_date".to_string(), attempted_path: "renewal_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.renewal_date())
                }}

    pub fn expiration_date(&self) -> chrono::NaiveDate {
        self.changed_expiration_date().and_then(|value| value.try_date()).unwrap_or(self.expiration_date)
    }

    pub fn update_expiration_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.expiration_date = value.try_date().unwrap_or(self.expiration_date.clone());
        self.root.set(self.entity_key(), "expiration_date", value);
        self
    }

    pub fn changed_expiration_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "expiration_date")
    }

    pub fn eval_expiration_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("expiration_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "expiration_date".to_string(), attempted_path: "expiration_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.expiration_date())
                }}

    pub fn fee(&self) -> String {
        self.changed_fee().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.fee.clone())
    }

    pub fn update_fee(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.fee = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.fee.clone());
        self.root.set(self.entity_key(), "fee", value);
        self
    }

    pub fn changed_fee(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "fee")
    }

    pub fn eval_fee(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("fee") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "fee".to_string(), attempted_path: "fee".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.fee())
                }}

    pub fn state_code(&self) -> String {
        self.changed_state_code().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.state_code.clone())
    }

    pub fn update_state_code(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.state_code = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.state_code.clone());
        self.root.set(self.entity_key(), "state_code", value);
        self
    }

    pub fn changed_state_code(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "state_code")
    }

    pub fn eval_state_code(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("state_code") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "state_code".to_string(), attempted_path: "state_code".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.state_code())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::RegistrationRenewalRepository<'a>>>
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
            .registration_renewal_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("RegistrationRenewal"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

