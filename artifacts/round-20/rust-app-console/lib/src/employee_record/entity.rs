// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/employee_record
use std::collections::BTreeMap;

use teaql_core::SmartList;
use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "EmployeeRecord", table = "employee_record_data", data_service = "sqlite")]
pub struct EmployeeRecord {
#[teaql(id)]
    id: u64,

// @source hr_payroll.xml:17
    employee_number: String,

// @source hr_payroll.xml:17
    first_name: String,

// @source hr_payroll.xml:17
    last_name: String,

// @source hr_payroll.xml:17
    hire_date: chrono::NaiveDate,

// @source hr_payroll.xml:17
    department: String,

// @source hr_payroll.xml:17
    employment_status: String,
#[teaql(version)]
    version: i64,
    #[teaql(boxed_relations)]
    pub _relations: Box<EmployeeRecordReverseRelations>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl EmployeeRecord {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            employee_number: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            hire_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            department: String::new(),
            employment_status: String::new(),
            version: 0_i64,
            _relations: Box::new(EmployeeRecordReverseRelations::new()),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("EmployeeRecord", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        self._relations.attach_root_recursive(root.clone());
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

    pub fn employee_number(&self) -> String {
        self.changed_employee_number().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.employee_number.clone())
    }

    pub fn update_employee_number(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.employee_number = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.employee_number.clone());
        self.root.set(self.entity_key(), "employee_number", value);
        self
    }

    pub fn changed_employee_number(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "employee_number")
    }

    pub fn eval_employee_number(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("employee_number") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_number".to_string(), attempted_path: "employee_number".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.employee_number())
                }}

    pub fn first_name(&self) -> String {
        self.changed_first_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.first_name.clone())
    }

    pub fn update_first_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.first_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.first_name.clone());
        self.root.set(self.entity_key(), "first_name", value);
        self
    }

    pub fn changed_first_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "first_name")
    }

    pub fn eval_first_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("first_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "first_name".to_string(), attempted_path: "first_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.first_name())
                }}

    pub fn last_name(&self) -> String {
        self.changed_last_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.last_name.clone())
    }

    pub fn update_last_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.last_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.last_name.clone());
        self.root.set(self.entity_key(), "last_name", value);
        self
    }

    pub fn changed_last_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "last_name")
    }

    pub fn eval_last_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("last_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "last_name".to_string(), attempted_path: "last_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.last_name())
                }}

    pub fn hire_date(&self) -> chrono::NaiveDate {
        self.changed_hire_date().and_then(|value| value.try_date()).unwrap_or(self.hire_date)
    }

    pub fn update_hire_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.hire_date = value.try_date().unwrap_or(self.hire_date.clone());
        self.root.set(self.entity_key(), "hire_date", value);
        self
    }

    pub fn changed_hire_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "hire_date")
    }

    pub fn eval_hire_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("hire_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "hire_date".to_string(), attempted_path: "hire_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.hire_date())
                }}

    pub fn department(&self) -> String {
        self.changed_department().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.department.clone())
    }

    pub fn update_department(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.department = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.department.clone());
        self.root.set(self.entity_key(), "department", value);
        self
    }

    pub fn changed_department(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "department")
    }

    pub fn eval_department(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("department") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "department".to_string(), attempted_path: "department".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.department())
                }}

    pub fn employment_status(&self) -> String {
        self.changed_employment_status().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.employment_status.clone())
    }

    pub fn update_employment_status(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.employment_status = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.employment_status.clone());
        self.root.set(self.entity_key(), "employment_status", value);
        self
    }

    pub fn changed_employment_status(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "employment_status")
    }

    pub fn eval_employment_status(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("employment_status") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "employment_status".to_string(), attempted_path: "employment_status".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.employment_status())
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
    pub fn timesheet_entry_list(&self) -> &SmartList<crate::TimesheetEntry> {
        &self._relations.timesheet_entry_list
    }

    pub fn timesheet_entry_list_mut(&mut self) -> &mut SmartList<crate::TimesheetEntry> {
        &mut self._relations.timesheet_entry_list
    }

    pub fn eval_timesheet_entry_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TimesheetEntry>> {
        if !self.is_loaded("timesheet_entry_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "timesheet_entry_list".to_string(), attempted_path: "timesheet_entry_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.timesheet_entry_list)
        }
    }

    pub fn tax_withholding_list(&self) -> &SmartList<crate::TaxWithholding> {
        &self._relations.tax_withholding_list
    }

    pub fn tax_withholding_list_mut(&mut self) -> &mut SmartList<crate::TaxWithholding> {
        &mut self._relations.tax_withholding_list
    }

    pub fn eval_tax_withholding_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TaxWithholding>> {
        if !self.is_loaded("tax_withholding_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "tax_withholding_list".to_string(), attempted_path: "tax_withholding_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.tax_withholding_list)
        }
    }

    pub fn leave_request_list(&self) -> &SmartList<crate::LeaveRequest> {
        &self._relations.leave_request_list
    }

    pub fn leave_request_list_mut(&mut self) -> &mut SmartList<crate::LeaveRequest> {
        &mut self._relations.leave_request_list
    }

    pub fn eval_leave_request_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LeaveRequest>> {
        if !self.is_loaded("leave_request_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "leave_request_list".to_string(), attempted_path: "leave_request_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.leave_request_list)
        }
    }

    pub fn training_record_list(&self) -> &SmartList<crate::TrainingRecord> {
        &self._relations.training_record_list
    }

    pub fn training_record_list_mut(&mut self) -> &mut SmartList<crate::TrainingRecord> {
        &mut self._relations.training_record_list
    }

    pub fn eval_training_record_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TrainingRecord>> {
        if !self.is_loaded("training_record_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "training_record_list".to_string(), attempted_path: "training_record_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.training_record_list)
        }
    }

    pub fn performance_review_list_as_employee(&self) -> &SmartList<crate::PerformanceReview> {
        &self._relations.performance_review_list_as_employee
    }

    pub fn performance_review_list_as_employee_mut(&mut self) -> &mut SmartList<crate::PerformanceReview> {
        &mut self._relations.performance_review_list_as_employee
    }

    pub fn eval_performance_review_list_as_employee(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PerformanceReview>> {
        if !self.is_loaded("performance_review_list_as_employee") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "performance_review_list_as_employee".to_string(), attempted_path: "performance_review_list_as_employee".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.performance_review_list_as_employee)
        }
    }

    pub fn performance_review_list_as_reviewer(&self) -> &SmartList<crate::PerformanceReview> {
        &self._relations.performance_review_list_as_reviewer
    }

    pub fn performance_review_list_as_reviewer_mut(&mut self) -> &mut SmartList<crate::PerformanceReview> {
        &mut self._relations.performance_review_list_as_reviewer
    }

    pub fn eval_performance_review_list_as_reviewer(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PerformanceReview>> {
        if !self.is_loaded("performance_review_list_as_reviewer") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "performance_review_list_as_reviewer".to_string(), attempted_path: "performance_review_list_as_reviewer".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.performance_review_list_as_reviewer)
        }
    }

    pub fn compensation_adjustment_list_as_employee(&self) -> &SmartList<crate::CompensationAdjustment> {
        &self._relations.compensation_adjustment_list_as_employee
    }

    pub fn compensation_adjustment_list_as_employee_mut(&mut self) -> &mut SmartList<crate::CompensationAdjustment> {
        &mut self._relations.compensation_adjustment_list_as_employee
    }

    pub fn eval_compensation_adjustment_list_as_employee(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CompensationAdjustment>> {
        if !self.is_loaded("compensation_adjustment_list_as_employee") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "compensation_adjustment_list_as_employee".to_string(), attempted_path: "compensation_adjustment_list_as_employee".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.compensation_adjustment_list_as_employee)
        }
    }

    pub fn compensation_adjustment_list_as_approved_by(&self) -> &SmartList<crate::CompensationAdjustment> {
        &self._relations.compensation_adjustment_list_as_approved_by
    }

    pub fn compensation_adjustment_list_as_approved_by_mut(&mut self) -> &mut SmartList<crate::CompensationAdjustment> {
        &mut self._relations.compensation_adjustment_list_as_approved_by
    }

    pub fn eval_compensation_adjustment_list_as_approved_by(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CompensationAdjustment>> {
        if !self.is_loaded("compensation_adjustment_list_as_approved_by") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "compensation_adjustment_list_as_approved_by".to_string(), attempted_path: "compensation_adjustment_list_as_approved_by".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.compensation_adjustment_list_as_approved_by)
        }
    }

    pub fn onboarding_checklist_list(&self) -> &SmartList<crate::OnboardingChecklist> {
        &self._relations.onboarding_checklist_list
    }

    pub fn onboarding_checklist_list_mut(&mut self) -> &mut SmartList<crate::OnboardingChecklist> {
        &mut self._relations.onboarding_checklist_list
    }

    pub fn eval_onboarding_checklist_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::OnboardingChecklist>> {
        if !self.is_loaded("onboarding_checklist_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "onboarding_checklist_list".to_string(), attempted_path: "onboarding_checklist_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.onboarding_checklist_list)
        }
    }

    pub fn offboarding_process_list(&self) -> &SmartList<crate::OffboardingProcess> {
        &self._relations.offboarding_process_list
    }

    pub fn offboarding_process_list_mut(&mut self) -> &mut SmartList<crate::OffboardingProcess> {
        &mut self._relations.offboarding_process_list
    }

    pub fn eval_offboarding_process_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::OffboardingProcess>> {
        if !self.is_loaded("offboarding_process_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "offboarding_process_list".to_string(), attempted_path: "offboarding_process_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.offboarding_process_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
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
            .employee_record_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("EmployeeRecord"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

#[derive(Clone, Debug, PartialEq, teaql_macros::TeaqlReverseRelations)]
pub struct EmployeeRecordReverseRelations {
#[teaql(relation(target = "TimesheetEntry", local_key = "id", foreign_key = "employee_id", many))]
    timesheet_entry_list: SmartList<crate::TimesheetEntry>,
#[teaql(relation(target = "TaxWithholding", local_key = "id", foreign_key = "employee_id", many))]
    tax_withholding_list: SmartList<crate::TaxWithholding>,
#[teaql(relation(target = "LeaveRequest", local_key = "id", foreign_key = "employee_id", many))]
    leave_request_list: SmartList<crate::LeaveRequest>,
#[teaql(relation(target = "TrainingRecord", local_key = "id", foreign_key = "employee_id", many))]
    training_record_list: SmartList<crate::TrainingRecord>,
#[teaql(relation(target = "PerformanceReview", local_key = "id", foreign_key = "employee_id", many))]
    performance_review_list_as_employee: SmartList<crate::PerformanceReview>,
#[teaql(relation(target = "PerformanceReview", local_key = "id", foreign_key = "reviewer_id", many))]
    performance_review_list_as_reviewer: SmartList<crate::PerformanceReview>,
#[teaql(relation(target = "CompensationAdjustment", local_key = "id", foreign_key = "employee_id", many))]
    compensation_adjustment_list_as_employee: SmartList<crate::CompensationAdjustment>,
#[teaql(relation(target = "CompensationAdjustment", local_key = "id", foreign_key = "approved_by_id", many))]
    compensation_adjustment_list_as_approved_by: SmartList<crate::CompensationAdjustment>,
#[teaql(relation(target = "OnboardingChecklist", local_key = "id", foreign_key = "employee_id", many))]
    onboarding_checklist_list: SmartList<crate::OnboardingChecklist>,
#[teaql(relation(target = "OffboardingProcess", local_key = "id", foreign_key = "employee_id", many))]
    offboarding_process_list: SmartList<crate::OffboardingProcess>,
}

impl EmployeeRecordReverseRelations {
    pub fn new() -> Self {
        Self {
            timesheet_entry_list: Default::default(),
            tax_withholding_list: Default::default(),
            leave_request_list: Default::default(),
            training_record_list: Default::default(),
            performance_review_list_as_employee: Default::default(),
            performance_review_list_as_reviewer: Default::default(),
            compensation_adjustment_list_as_employee: Default::default(),
            compensation_adjustment_list_as_approved_by: Default::default(),
            onboarding_checklist_list: Default::default(),
            offboarding_process_list: Default::default(),
        }
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        for entity in &mut self.timesheet_entry_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.tax_withholding_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.leave_request_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.training_record_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.performance_review_list_as_employee {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.performance_review_list_as_reviewer {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.compensation_adjustment_list_as_employee {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.compensation_adjustment_list_as_approved_by {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.onboarding_checklist_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.offboarding_process_list {
            entity.attach_root_recursive(root.clone());
        }
    }
}
