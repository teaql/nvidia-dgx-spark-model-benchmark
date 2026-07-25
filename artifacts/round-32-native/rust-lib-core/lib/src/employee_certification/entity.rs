// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/employee_certification
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "EmployeeCertification", table = "employee_certification_data", data_service = "sqlite")]
pub struct EmployeeCertification {
#[teaql(id)]
    id: u64,

// @source module_2.xml:13
    certification_title: String,

// @source module_2.xml:13
    issuing_authority: String,

// @source module_2.xml:13
    issue_date: chrono::NaiveDate,

// @source module_2.xml:13
    expiry_date: chrono::NaiveDate,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl EmployeeCertification {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            certification_title: String::new(),
            issuing_authority: String::new(),
            issue_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            expiry_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("EmployeeCertification", self.id)
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

    pub fn certification_title(&self) -> String {
        self.changed_certification_title().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.certification_title.clone())
    }

    pub fn update_certification_title(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.certification_title = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.certification_title.clone());
        self.root.set(self.entity_key(), "certification_title", value);
        self
    }

    pub fn changed_certification_title(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "certification_title")
    }

    pub fn eval_certification_title(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("certification_title") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "certification_title".to_string(), attempted_path: "certification_title".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.certification_title())
                }}

    pub fn issuing_authority(&self) -> String {
        self.changed_issuing_authority().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.issuing_authority.clone())
    }

    pub fn update_issuing_authority(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.issuing_authority = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.issuing_authority.clone());
        self.root.set(self.entity_key(), "issuing_authority", value);
        self
    }

    pub fn changed_issuing_authority(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "issuing_authority")
    }

    pub fn eval_issuing_authority(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("issuing_authority") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "issuing_authority".to_string(), attempted_path: "issuing_authority".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.issuing_authority())
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

    pub fn expiry_date(&self) -> chrono::NaiveDate {
        self.changed_expiry_date().and_then(|value| value.try_date()).unwrap_or(self.expiry_date)
    }

    pub fn update_expiry_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.expiry_date = value.try_date().unwrap_or(self.expiry_date.clone());
        self.root.set(self.entity_key(), "expiry_date", value);
        self
    }

    pub fn changed_expiry_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "expiry_date")
    }

    pub fn eval_expiry_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("expiry_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "expiry_date".to_string(), attempted_path: "expiry_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.expiry_date())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::EmployeeCertificationRepository<'a>>>
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
            .employee_certification_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("EmployeeCertification"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

