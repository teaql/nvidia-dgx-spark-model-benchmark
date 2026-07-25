// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/merchant
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Merchant", table = "merchant_data", data_service = "sqlite", audit_mask_fields = "contact_email,contact_phone")]
pub struct Merchant {
#[teaql(id)]
    id: u64,

// @source module_0.xml:6
    tax_identifier: String,

// @source module_0.xml:6
    status: String,

// @source module_0.xml:6
    category: String,

// @source module_0.xml:6
    contact_email: String,

// @source module_0.xml:6
    contact_phone: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Merchant {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            tax_identifier: String::new(),
            status: String::new(),
            category: String::new(),
            contact_email: String::new(),
            contact_phone: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Merchant", self.id)
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

    pub fn tax_identifier(&self) -> String {
        self.changed_tax_identifier().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.tax_identifier.clone())
    }

    pub fn update_tax_identifier(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.tax_identifier = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.tax_identifier.clone());
        self.root.set(self.entity_key(), "tax_identifier", value);
        self
    }

    pub fn changed_tax_identifier(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "tax_identifier")
    }

    pub fn eval_tax_identifier(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("tax_identifier") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "tax_identifier".to_string(), attempted_path: "tax_identifier".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.tax_identifier())
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

    pub fn category(&self) -> String {
        self.changed_category().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.category.clone())
    }

    pub fn update_category(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.category = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.category.clone());
        self.root.set(self.entity_key(), "category", value);
        self
    }

    pub fn changed_category(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "category")
    }

    pub fn eval_category(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("category") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "category".to_string(), attempted_path: "category".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.category())
                }}

    pub fn contact_email(&self) -> String {
        self.changed_contact_email().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.contact_email.clone())
    }

    pub fn update_contact_email(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.contact_email = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.contact_email.clone());
        self.root.set(self.entity_key(), "contact_email", value);
        self
    }

    pub fn changed_contact_email(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "contact_email")
    }

    pub fn eval_contact_email(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("contact_email") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "contact_email".to_string(), attempted_path: "contact_email".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.contact_email())
                }}

    pub fn contact_phone(&self) -> String {
        self.changed_contact_phone().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.contact_phone.clone())
    }

    pub fn update_contact_phone(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.contact_phone = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.contact_phone.clone());
        self.root.set(self.entity_key(), "contact_phone", value);
        self
    }

    pub fn changed_contact_phone(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "contact_phone")
    }

    pub fn eval_contact_phone(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("contact_phone") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "contact_phone".to_string(), attempted_path: "contact_phone".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.contact_phone())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::MerchantRepository<'a>>>
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
            .merchant_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Merchant"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

