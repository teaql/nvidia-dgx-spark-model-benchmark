// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/onboarding_checklist
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "OnboardingChecklist", table = "onboarding_checklist_data", data_service = "sqlite")]
pub struct OnboardingChecklist {
#[teaql(id)]
    id: u64,

// @source hr_payroll.xml:83
    task_description: String,

// @source hr_payroll.xml:83
    due_date: chrono::NaiveDate,

// @source hr_payroll.xml:83
    completed: String,
#[teaql(version)]
    version: i64,
// @source hr_payroll.xml:83
#[teaql(column = "employee")]
    employee_id: u64,
// @source hr_payroll.xml:83
#[teaql(relation(target = "EmployeeRecord", local_key = "employee_id", foreign_key = "id"))]
    employee: Option<crate::EmployeeRecord>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl OnboardingChecklist {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            task_description: String::new(),
            due_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            completed: String::new(),
            version: 0_i64,
            employee_id: 0_u64,
            employee: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("OnboardingChecklist", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.employee {
            entity.attach_root_recursive(root.clone());
        }
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

    pub fn task_description(&self) -> String {
        self.changed_task_description().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.task_description.clone())
    }

    pub fn update_task_description(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.task_description = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.task_description.clone());
        self.root.set(self.entity_key(), "task_description", value);
        self
    }

    pub fn changed_task_description(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "task_description")
    }

    pub fn eval_task_description(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("task_description") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "task_description".to_string(), attempted_path: "task_description".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.task_description())
                }}

    pub fn due_date(&self) -> chrono::NaiveDate {
        self.changed_due_date().and_then(|value| value.try_date()).unwrap_or(self.due_date)
    }

    pub fn update_due_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.due_date = value.try_date().unwrap_or(self.due_date.clone());
        self.root.set(self.entity_key(), "due_date", value);
        self
    }

    pub fn changed_due_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "due_date")
    }

    pub fn eval_due_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("due_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "due_date".to_string(), attempted_path: "due_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.due_date())
                }}

    pub fn completed(&self) -> String {
        self.changed_completed().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.completed.clone())
    }

    pub fn update_completed(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.completed = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.completed.clone());
        self.root.set(self.entity_key(), "completed", value);
        self
    }

    pub fn changed_completed(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "completed")
    }

    pub fn eval_completed(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("completed") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "completed".to_string(), attempted_path: "completed".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.completed())
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
    pub fn employee_id(&self) -> u64 {
        self.changed_employee_id().and_then(|value| value.try_u64()).unwrap_or(self.employee_id)
    }

    pub fn update_employee_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.employee_id = value.try_u64().unwrap_or(self.employee_id.clone());
        self.root.set(self.entity_key(), "employee_id", value);
        self
    }

    pub fn changed_employee_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "employee_id")
    }

    pub fn eval_employee_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("employee_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_id".to_string(), attempted_path: "employee_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.employee_id())
                }}
    pub fn employee(&self) -> Option<&crate::EmployeeRecord> {
        self.employee.as_ref()
    }

    pub fn eval_employee(&self) -> teaql_core::eval::EvalResult<&crate::EmployeeRecord> {
        if !self.is_loaded("employee") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee".to_string(), attempted_path: "employee".to_string() }
        } else {
            match &self.employee {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::OnboardingChecklistRepository<'a>>>
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
            .onboarding_checklist_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("OnboardingChecklist"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

