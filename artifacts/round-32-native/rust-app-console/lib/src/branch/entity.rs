// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/branch
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Branch", table = "branch_data", data_service = "sqlite", audit_mask_fields = "contact_phone")]
pub struct Branch {
#[teaql(id)]
    id: u64,

// @source module_0.xml:7
    branch_code: String,

// @source module_0.xml:7
    operating_status: String,

// @source module_0.xml:7
    time_zone: String,

// @source module_0.xml:7
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

impl Branch {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            branch_code: String::new(),
            operating_status: String::new(),
            time_zone: String::new(),
            contact_phone: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Branch", self.id)
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

    pub fn branch_code(&self) -> String {
        self.changed_branch_code().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.branch_code.clone())
    }

    pub fn update_branch_code(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.branch_code = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.branch_code.clone());
        self.root.set(self.entity_key(), "branch_code", value);
        self
    }

    pub fn changed_branch_code(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "branch_code")
    }

    pub fn eval_branch_code(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("branch_code") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "branch_code".to_string(), attempted_path: "branch_code".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.branch_code())
                }}

    pub fn operating_status(&self) -> String {
        self.changed_operating_status().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.operating_status.clone())
    }

    pub fn update_operating_status(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.operating_status = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.operating_status.clone());
        self.root.set(self.entity_key(), "operating_status", value);
        self
    }

    pub fn changed_operating_status(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "operating_status")
    }

    pub fn eval_operating_status(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("operating_status") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "operating_status".to_string(), attempted_path: "operating_status".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.operating_status())
                }}

    pub fn time_zone(&self) -> String {
        self.changed_time_zone().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.time_zone.clone())
    }

    pub fn update_time_zone(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.time_zone = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.time_zone.clone());
        self.root.set(self.entity_key(), "time_zone", value);
        self
    }

    pub fn changed_time_zone(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "time_zone")
    }

    pub fn eval_time_zone(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("time_zone") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "time_zone".to_string(), attempted_path: "time_zone".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.time_zone())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::BranchRepository<'a>>>
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
            .branch_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Branch"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

