// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/compliance_check
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "ComplianceCheck", table = "compliance_check_data", data_service = "sqlite")]
pub struct ComplianceCheck {
#[teaql(id)]
    id: u64,

// @source module_9.xml:8
    check_date: chrono::NaiveDate,

// @source module_9.xml:8
    standard: String,

// @source module_9.xml:8
    result: String,

// @source module_9.xml:8
    inspector: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl ComplianceCheck {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            check_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            standard: String::new(),
            result: String::new(),
            inspector: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("ComplianceCheck", self.id)
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

    pub fn check_date(&self) -> chrono::NaiveDate {
        self.changed_check_date().and_then(|value| value.try_date()).unwrap_or(self.check_date)
    }

    pub fn update_check_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.check_date = value.try_date().unwrap_or(self.check_date.clone());
        self.root.set(self.entity_key(), "check_date", value);
        self
    }

    pub fn changed_check_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "check_date")
    }

    pub fn eval_check_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("check_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "check_date".to_string(), attempted_path: "check_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.check_date())
                }}

    pub fn standard(&self) -> String {
        self.changed_standard().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.standard.clone())
    }

    pub fn update_standard(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.standard = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.standard.clone());
        self.root.set(self.entity_key(), "standard", value);
        self
    }

    pub fn changed_standard(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "standard")
    }

    pub fn eval_standard(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("standard") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "standard".to_string(), attempted_path: "standard".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.standard())
                }}

    pub fn result(&self) -> String {
        self.changed_result().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.result.clone())
    }

    pub fn update_result(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.result = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.result.clone());
        self.root.set(self.entity_key(), "result", value);
        self
    }

    pub fn changed_result(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "result")
    }

    pub fn eval_result(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("result") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "result".to_string(), attempted_path: "result".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.result())
                }}

    pub fn inspector(&self) -> String {
        self.changed_inspector().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.inspector.clone())
    }

    pub fn update_inspector(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.inspector = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.inspector.clone());
        self.root.set(self.entity_key(), "inspector", value);
        self
    }

    pub fn changed_inspector(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "inspector")
    }

    pub fn eval_inspector(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("inspector") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "inspector".to_string(), attempted_path: "inspector".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.inspector())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::ComplianceCheckRepository<'a>>>
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
            .compliance_check_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("ComplianceCheck"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

