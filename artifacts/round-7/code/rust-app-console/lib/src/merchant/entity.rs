// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/merchant
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
#[teaql(entity = "Merchant", table = "merchant_data", data_service = "sqlite")]
pub struct Merchant {
#[teaql(id)]
    id: u64,

// @source model.xml:53
    name: String,

// @source model.xml:53
    tax_number: String,

// @source model.xml:53
    address: String,

// @source model.xml:53
    external_id: String,

// @source model.xml:53
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:53
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:53
#[teaql(column = "platform")]
    platform_id: u64,
// @source model.xml:53
#[teaql(relation(target = "Platform", local_key = "platform_id", foreign_key = "id"))]
    platform: Option<crate::Platform>,
    #[teaql(boxed_relations)]
    pub _relations: Box<MerchantReverseRelations>,
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
            name: String::new(),
            tax_number: String::new(),
            address: String::new(),
            external_id: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            platform_id: 0_u64,
            platform: None,
            _relations: Box::new(MerchantReverseRelations::new()),
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
        if let Some(entity) = &mut self.platform {
            entity.attach_root_recursive(root.clone());
        }
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

    pub fn name(&self) -> String {
        self.changed_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.name.clone())
    }

    pub fn update_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.name.clone());
        self.root.set(self.entity_key(), "name", value);
        self
    }

    pub fn changed_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "name")
    }

    pub fn eval_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "name".to_string(), attempted_path: "name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.name())
                }}

    pub fn tax_number(&self) -> String {
        self.changed_tax_number().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.tax_number.clone())
    }

    pub fn update_tax_number(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.tax_number = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.tax_number.clone());
        self.root.set(self.entity_key(), "tax_number", value);
        self
    }

    pub fn changed_tax_number(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "tax_number")
    }

    pub fn eval_tax_number(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("tax_number") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "tax_number".to_string(), attempted_path: "tax_number".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.tax_number())
                }}

    pub fn address(&self) -> String {
        self.changed_address().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.address.clone())
    }

    pub fn update_address(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.address = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.address.clone());
        self.root.set(self.entity_key(), "address", value);
        self
    }

    pub fn changed_address(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "address")
    }

    pub fn eval_address(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("address") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "address".to_string(), attempted_path: "address".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.address())
                }}

    pub fn external_id(&self) -> String {
        self.changed_external_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.external_id.clone())
    }

    pub fn update_external_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.external_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.external_id.clone());
        self.root.set(self.entity_key(), "external_id", value);
        self
    }

    pub fn changed_external_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "external_id")
    }

    pub fn eval_external_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("external_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "external_id".to_string(), attempted_path: "external_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.external_id())
                }}

    pub fn create_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_create_time().and_then(|value| value.try_timestamp()).unwrap_or(self.create_time)
    }

    pub fn update_create_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.create_time = value.try_timestamp().unwrap_or(self.create_time.clone());
        self.root.set(self.entity_key(), "create_time", value);
        self
    }

    pub fn changed_create_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "create_time")
    }

    pub fn eval_create_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("create_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "create_time".to_string(), attempted_path: "create_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.create_time())
                }}

    pub fn update_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_update_time().and_then(|value| value.try_timestamp()).unwrap_or(self.update_time)
    }

    pub fn update_update_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.update_time = value.try_timestamp().unwrap_or(self.update_time.clone());
        self.root.set(self.entity_key(), "update_time", value);
        self
    }

    pub fn changed_update_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "update_time")
    }

    pub fn eval_update_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
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
    pub fn platform_id(&self) -> u64 {
        self.changed_platform_id().and_then(|value| value.try_u64()).unwrap_or(self.platform_id)
    }

    pub fn update_platform_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.platform_id = value.try_u64().unwrap_or(self.platform_id.clone());
        self.root.set(self.entity_key(), "platform_id", value);
        self
    }

    pub fn changed_platform_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "platform_id")
    }

    pub fn eval_platform_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("platform_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform_id".to_string(), attempted_path: "platform_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.platform_id())
                }}
    pub fn platform(&self) -> Option<&crate::Platform> {
        self.platform.as_ref()
    }

    pub fn eval_platform(&self) -> teaql_core::eval::EvalResult<&crate::Platform> {
        if !self.is_loaded("platform") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform".to_string(), attempted_path: "platform".to_string() }
        } else {
            match &self.platform {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn department_list(&self) -> &SmartList<crate::Department> {
        &self._relations.department_list
    }

    pub fn department_list_mut(&mut self) -> &mut SmartList<crate::Department> {
        &mut self._relations.department_list
    }

    pub fn eval_department_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Department>> {
        if !self.is_loaded("department_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "department_list".to_string(), attempted_path: "department_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.department_list)
        }
    }

    pub fn position_list(&self) -> &SmartList<crate::Position> {
        &self._relations.position_list
    }

    pub fn position_list_mut(&mut self) -> &mut SmartList<crate::Position> {
        &mut self._relations.position_list
    }

    pub fn eval_position_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Position>> {
        if !self.is_loaded("position_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "position_list".to_string(), attempted_path: "position_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.position_list)
        }
    }

    pub fn employee_list(&self) -> &SmartList<crate::Employee> {
        &self._relations.employee_list
    }

    pub fn employee_list_mut(&mut self) -> &mut SmartList<crate::Employee> {
        &mut self._relations.employee_list
    }

    pub fn eval_employee_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Employee>> {
        if !self.is_loaded("employee_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_list".to_string(), attempted_path: "employee_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.employee_list)
        }
    }

    pub fn salary_record_list(&self) -> &SmartList<crate::SalaryRecord> {
        &self._relations.salary_record_list
    }

    pub fn salary_record_list_mut(&mut self) -> &mut SmartList<crate::SalaryRecord> {
        &mut self._relations.salary_record_list
    }

    pub fn eval_salary_record_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::SalaryRecord>> {
        if !self.is_loaded("salary_record_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "salary_record_list".to_string(), attempted_path: "salary_record_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.salary_record_list)
        }
    }

    pub fn attendance_log_list(&self) -> &SmartList<crate::AttendanceLog> {
        &self._relations.attendance_log_list
    }

    pub fn attendance_log_list_mut(&mut self) -> &mut SmartList<crate::AttendanceLog> {
        &mut self._relations.attendance_log_list
    }

    pub fn eval_attendance_log_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AttendanceLog>> {
        if !self.is_loaded("attendance_log_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "attendance_log_list".to_string(), attempted_path: "attendance_log_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.attendance_log_list)
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

    pub fn performance_review_list(&self) -> &SmartList<crate::PerformanceReview> {
        &self._relations.performance_review_list
    }

    pub fn performance_review_list_mut(&mut self) -> &mut SmartList<crate::PerformanceReview> {
        &mut self._relations.performance_review_list
    }

    pub fn eval_performance_review_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PerformanceReview>> {
        if !self.is_loaded("performance_review_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "performance_review_list".to_string(), attempted_path: "performance_review_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.performance_review_list)
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

    pub fn benefit_plan_list(&self) -> &SmartList<crate::BenefitPlan> {
        &self._relations.benefit_plan_list
    }

    pub fn benefit_plan_list_mut(&mut self) -> &mut SmartList<crate::BenefitPlan> {
        &mut self._relations.benefit_plan_list
    }

    pub fn eval_benefit_plan_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::BenefitPlan>> {
        if !self.is_loaded("benefit_plan_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "benefit_plan_list".to_string(), attempted_path: "benefit_plan_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.benefit_plan_list)
        }
    }

    pub fn expense_claim_list(&self) -> &SmartList<crate::ExpenseClaim> {
        &self._relations.expense_claim_list
    }

    pub fn expense_claim_list_mut(&mut self) -> &mut SmartList<crate::ExpenseClaim> {
        &mut self._relations.expense_claim_list
    }

    pub fn eval_expense_claim_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExpenseClaim>> {
        if !self.is_loaded("expense_claim_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "expense_claim_list".to_string(), attempted_path: "expense_claim_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.expense_claim_list)
        }
    }

    pub fn payroll_run_list(&self) -> &SmartList<crate::PayrollRun> {
        &self._relations.payroll_run_list
    }

    pub fn payroll_run_list_mut(&mut self) -> &mut SmartList<crate::PayrollRun> {
        &mut self._relations.payroll_run_list
    }

    pub fn eval_payroll_run_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PayrollRun>> {
        if !self.is_loaded("payroll_run_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payroll_run_list".to_string(), attempted_path: "payroll_run_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.payroll_run_list)
        }
    }

    pub fn tax_form_list(&self) -> &SmartList<crate::TaxForm> {
        &self._relations.tax_form_list
    }

    pub fn tax_form_list_mut(&mut self) -> &mut SmartList<crate::TaxForm> {
        &mut self._relations.tax_form_list
    }

    pub fn eval_tax_form_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TaxForm>> {
        if !self.is_loaded("tax_form_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "tax_form_list".to_string(), attempted_path: "tax_form_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.tax_form_list)
        }
    }

    pub fn contract_list(&self) -> &SmartList<crate::Contract> {
        &self._relations.contract_list
    }

    pub fn contract_list_mut(&mut self) -> &mut SmartList<crate::Contract> {
        &mut self._relations.contract_list
    }

    pub fn eval_contract_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Contract>> {
        if !self.is_loaded("contract_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "contract_list".to_string(), attempted_path: "contract_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.contract_list)
        }
    }

    pub fn resignation_list(&self) -> &SmartList<crate::Resignation> {
        &self._relations.resignation_list
    }

    pub fn resignation_list_mut(&mut self) -> &mut SmartList<crate::Resignation> {
        &mut self._relations.resignation_list
    }

    pub fn eval_resignation_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Resignation>> {
        if !self.is_loaded("resignation_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "resignation_list".to_string(), attempted_path: "resignation_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.resignation_list)
        }
    }

    pub fn warning_letter_list(&self) -> &SmartList<crate::WarningLetter> {
        &self._relations.warning_letter_list
    }

    pub fn warning_letter_list_mut(&mut self) -> &mut SmartList<crate::WarningLetter> {
        &mut self._relations.warning_letter_list
    }

    pub fn eval_warning_letter_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::WarningLetter>> {
        if !self.is_loaded("warning_letter_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "warning_letter_list".to_string(), attempted_path: "warning_letter_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.warning_letter_list)
        }
    }

    pub fn bonus_record_list(&self) -> &SmartList<crate::BonusRecord> {
        &self._relations.bonus_record_list
    }

    pub fn bonus_record_list_mut(&mut self) -> &mut SmartList<crate::BonusRecord> {
        &mut self._relations.bonus_record_list
    }

    pub fn eval_bonus_record_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::BonusRecord>> {
        if !self.is_loaded("bonus_record_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "bonus_record_list".to_string(), attempted_path: "bonus_record_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.bonus_record_list)
        }
    }

    pub fn shift_schedule_list(&self) -> &SmartList<crate::ShiftSchedule> {
        &self._relations.shift_schedule_list
    }

    pub fn shift_schedule_list_mut(&mut self) -> &mut SmartList<crate::ShiftSchedule> {
        &mut self._relations.shift_schedule_list
    }

    pub fn eval_shift_schedule_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ShiftSchedule>> {
        if !self.is_loaded("shift_schedule_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "shift_schedule_list".to_string(), attempted_path: "shift_schedule_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.shift_schedule_list)
        }
    }

    pub fn time_off_balance_list(&self) -> &SmartList<crate::TimeOffBalance> {
        &self._relations.time_off_balance_list
    }

    pub fn time_off_balance_list_mut(&mut self) -> &mut SmartList<crate::TimeOffBalance> {
        &mut self._relations.time_off_balance_list
    }

    pub fn eval_time_off_balance_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TimeOffBalance>> {
        if !self.is_loaded("time_off_balance_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "time_off_balance_list".to_string(), attempted_path: "time_off_balance_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.time_off_balance_list)
        }
    }

    pub fn recruitment_post_list(&self) -> &SmartList<crate::RecruitmentPost> {
        &self._relations.recruitment_post_list
    }

    pub fn recruitment_post_list_mut(&mut self) -> &mut SmartList<crate::RecruitmentPost> {
        &mut self._relations.recruitment_post_list
    }

    pub fn eval_recruitment_post_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::RecruitmentPost>> {
        if !self.is_loaded("recruitment_post_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "recruitment_post_list".to_string(), attempted_path: "recruitment_post_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.recruitment_post_list)
        }
    }

    pub fn job_application_list(&self) -> &SmartList<crate::JobApplication> {
        &self._relations.job_application_list
    }

    pub fn job_application_list_mut(&mut self) -> &mut SmartList<crate::JobApplication> {
        &mut self._relations.job_application_list
    }

    pub fn eval_job_application_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::JobApplication>> {
        if !self.is_loaded("job_application_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "job_application_list".to_string(), attempted_path: "job_application_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.job_application_list)
        }
    }

    pub fn interview_list(&self) -> &SmartList<crate::Interview> {
        &self._relations.interview_list
    }

    pub fn interview_list_mut(&mut self) -> &mut SmartList<crate::Interview> {
        &mut self._relations.interview_list
    }

    pub fn eval_interview_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Interview>> {
        if !self.is_loaded("interview_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "interview_list".to_string(), attempted_path: "interview_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.interview_list)
        }
    }

    pub fn offer_letter_list(&self) -> &SmartList<crate::OfferLetter> {
        &self._relations.offer_letter_list
    }

    pub fn offer_letter_list_mut(&mut self) -> &mut SmartList<crate::OfferLetter> {
        &mut self._relations.offer_letter_list
    }

    pub fn eval_offer_letter_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::OfferLetter>> {
        if !self.is_loaded("offer_letter_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "offer_letter_list".to_string(), attempted_path: "offer_letter_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.offer_letter_list)
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

    pub fn offboarding_checklist_list(&self) -> &SmartList<crate::OffboardingChecklist> {
        &self._relations.offboarding_checklist_list
    }

    pub fn offboarding_checklist_list_mut(&mut self) -> &mut SmartList<crate::OffboardingChecklist> {
        &mut self._relations.offboarding_checklist_list
    }

    pub fn eval_offboarding_checklist_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::OffboardingChecklist>> {
        if !self.is_loaded("offboarding_checklist_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "offboarding_checklist_list".to_string(), attempted_path: "offboarding_checklist_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.offboarding_checklist_list)
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

#[derive(Clone, Debug, PartialEq, teaql_macros::TeaqlReverseRelations)]
pub struct MerchantReverseRelations {
#[teaql(relation(target = "Department", local_key = "id", foreign_key = "merchant_id", many))]
    department_list: SmartList<crate::Department>,
#[teaql(relation(target = "Position", local_key = "id", foreign_key = "merchant_id", many))]
    position_list: SmartList<crate::Position>,
#[teaql(relation(target = "Employee", local_key = "id", foreign_key = "merchant_id", many))]
    employee_list: SmartList<crate::Employee>,
#[teaql(relation(target = "SalaryRecord", local_key = "id", foreign_key = "merchant_id", many))]
    salary_record_list: SmartList<crate::SalaryRecord>,
#[teaql(relation(target = "AttendanceLog", local_key = "id", foreign_key = "merchant_id", many))]
    attendance_log_list: SmartList<crate::AttendanceLog>,
#[teaql(relation(target = "LeaveRequest", local_key = "id", foreign_key = "merchant_id", many))]
    leave_request_list: SmartList<crate::LeaveRequest>,
#[teaql(relation(target = "PerformanceReview", local_key = "id", foreign_key = "merchant_id", many))]
    performance_review_list: SmartList<crate::PerformanceReview>,
#[teaql(relation(target = "TrainingRecord", local_key = "id", foreign_key = "merchant_id", many))]
    training_record_list: SmartList<crate::TrainingRecord>,
#[teaql(relation(target = "BenefitPlan", local_key = "id", foreign_key = "merchant_id", many))]
    benefit_plan_list: SmartList<crate::BenefitPlan>,
#[teaql(relation(target = "ExpenseClaim", local_key = "id", foreign_key = "merchant_id", many))]
    expense_claim_list: SmartList<crate::ExpenseClaim>,
#[teaql(relation(target = "PayrollRun", local_key = "id", foreign_key = "merchant_id", many))]
    payroll_run_list: SmartList<crate::PayrollRun>,
#[teaql(relation(target = "TaxForm", local_key = "id", foreign_key = "merchant_id", many))]
    tax_form_list: SmartList<crate::TaxForm>,
#[teaql(relation(target = "Contract", local_key = "id", foreign_key = "merchant_id", many))]
    contract_list: SmartList<crate::Contract>,
#[teaql(relation(target = "Resignation", local_key = "id", foreign_key = "merchant_id", many))]
    resignation_list: SmartList<crate::Resignation>,
#[teaql(relation(target = "WarningLetter", local_key = "id", foreign_key = "merchant_id", many))]
    warning_letter_list: SmartList<crate::WarningLetter>,
#[teaql(relation(target = "BonusRecord", local_key = "id", foreign_key = "merchant_id", many))]
    bonus_record_list: SmartList<crate::BonusRecord>,
#[teaql(relation(target = "ShiftSchedule", local_key = "id", foreign_key = "merchant_id", many))]
    shift_schedule_list: SmartList<crate::ShiftSchedule>,
#[teaql(relation(target = "TimeOffBalance", local_key = "id", foreign_key = "merchant_id", many))]
    time_off_balance_list: SmartList<crate::TimeOffBalance>,
#[teaql(relation(target = "RecruitmentPost", local_key = "id", foreign_key = "merchant_id", many))]
    recruitment_post_list: SmartList<crate::RecruitmentPost>,
#[teaql(relation(target = "JobApplication", local_key = "id", foreign_key = "merchant_id", many))]
    job_application_list: SmartList<crate::JobApplication>,
#[teaql(relation(target = "Interview", local_key = "id", foreign_key = "merchant_id", many))]
    interview_list: SmartList<crate::Interview>,
#[teaql(relation(target = "OfferLetter", local_key = "id", foreign_key = "merchant_id", many))]
    offer_letter_list: SmartList<crate::OfferLetter>,
#[teaql(relation(target = "OnboardingChecklist", local_key = "id", foreign_key = "merchant_id", many))]
    onboarding_checklist_list: SmartList<crate::OnboardingChecklist>,
#[teaql(relation(target = "OffboardingChecklist", local_key = "id", foreign_key = "merchant_id", many))]
    offboarding_checklist_list: SmartList<crate::OffboardingChecklist>,
}

impl MerchantReverseRelations {
    pub fn new() -> Self {
        Self {
            department_list: Default::default(),
            position_list: Default::default(),
            employee_list: Default::default(),
            salary_record_list: Default::default(),
            attendance_log_list: Default::default(),
            leave_request_list: Default::default(),
            performance_review_list: Default::default(),
            training_record_list: Default::default(),
            benefit_plan_list: Default::default(),
            expense_claim_list: Default::default(),
            payroll_run_list: Default::default(),
            tax_form_list: Default::default(),
            contract_list: Default::default(),
            resignation_list: Default::default(),
            warning_letter_list: Default::default(),
            bonus_record_list: Default::default(),
            shift_schedule_list: Default::default(),
            time_off_balance_list: Default::default(),
            recruitment_post_list: Default::default(),
            job_application_list: Default::default(),
            interview_list: Default::default(),
            offer_letter_list: Default::default(),
            onboarding_checklist_list: Default::default(),
            offboarding_checklist_list: Default::default(),
        }
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        for entity in &mut self.department_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.position_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.employee_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.salary_record_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.attendance_log_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.leave_request_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.performance_review_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.training_record_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.benefit_plan_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.expense_claim_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payroll_run_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.tax_form_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.contract_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.resignation_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.warning_letter_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.bonus_record_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.shift_schedule_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.time_off_balance_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.recruitment_post_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.job_application_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.interview_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.offer_letter_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.onboarding_checklist_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.offboarding_checklist_list {
            entity.attach_root_recursive(root.clone());
        }
    }
}
