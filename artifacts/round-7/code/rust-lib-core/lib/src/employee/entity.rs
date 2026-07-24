// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/employee
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
#[teaql(entity = "Employee", table = "employee_data", data_service = "sqlite")]
pub struct Employee {
#[teaql(id)]
    id: u64,

// @source model.xml:79
    first_name: String,

// @source model.xml:79
    last_name: String,

// @source model.xml:79
    email: String,

// @source model.xml:79
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:79
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:79
#[teaql(column = "status")]
    status_id: u64,

// @source model.xml:79
#[teaql(column = "position")]
    position_id: u64,

// @source model.xml:79
#[teaql(column = "merchant")]
    merchant_id: u64,
// @source model.xml:79
#[teaql(relation(target = "EmployeeStatus", local_key = "status_id", foreign_key = "id"))]
    status: Option<crate::EmployeeStatus>,

// @source model.xml:79
#[teaql(relation(target = "Position", local_key = "position_id", foreign_key = "id"))]
    position: Option<crate::Position>,

// @source model.xml:79
#[teaql(relation(target = "Merchant", local_key = "merchant_id", foreign_key = "id"))]
    merchant: Option<crate::Merchant>,
    #[teaql(boxed_relations)]
    pub _relations: Box<EmployeeReverseRelations>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Employee {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            status_id: 0_u64,
            position_id: 0_u64,
            merchant_id: 0_u64,
            status: None,
            position: None,
            merchant: None,
            _relations: Box::new(EmployeeReverseRelations::new()),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Employee", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.status {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.position {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.merchant {
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

    pub fn email(&self) -> String {
        self.changed_email().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.email.clone())
    }

    pub fn update_email(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.email = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.email.clone());
        self.root.set(self.entity_key(), "email", value);
        self
    }

    pub fn changed_email(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "email")
    }

    pub fn eval_email(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("email") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "email".to_string(), attempted_path: "email".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.email())
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
    pub fn status_id(&self) -> u64 {
        self.changed_status_id().and_then(|value| value.try_u64()).unwrap_or(self.status_id)
    }

    pub(crate) fn update_status_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.status_id = value.try_u64().unwrap_or(self.status_id.clone());
        self.root.set(self.entity_key(), "status_id", value);
        self
    }

    pub fn changed_status_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "status_id")
    }

    pub fn eval_status_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("status_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.status_id())
                }}

    pub fn position_id(&self) -> u64 {
        self.changed_position_id().and_then(|value| value.try_u64()).unwrap_or(self.position_id)
    }

    pub fn update_position_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.position_id = value.try_u64().unwrap_or(self.position_id.clone());
        self.root.set(self.entity_key(), "position_id", value);
        self
    }

    pub fn changed_position_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "position_id")
    }

    pub fn eval_position_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("position_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "position_id".to_string(), attempted_path: "position_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.position_id())
                }}

    pub fn merchant_id(&self) -> u64 {
        self.changed_merchant_id().and_then(|value| value.try_u64()).unwrap_or(self.merchant_id)
    }

    pub fn update_merchant_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.merchant_id = value.try_u64().unwrap_or(self.merchant_id.clone());
        self.root.set(self.entity_key(), "merchant_id", value);
        self
    }

    pub fn changed_merchant_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "merchant_id")
    }

    pub fn eval_merchant_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("merchant_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_id".to_string(), attempted_path: "merchant_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.merchant_id())
                }}
    pub fn update_status_to_active(&mut self) -> &mut Self {
        self.update_status_id(1001_u64)
    }

    pub fn status_is_active(&self) -> bool {
        self.status_id() == 1001_u64
    }
    pub fn update_status_to_probation(&mut self) -> &mut Self {
        self.update_status_id(1002_u64)
    }

    pub fn status_is_probation(&self) -> bool {
        self.status_id() == 1002_u64
    }
    pub fn update_status_to_terminated(&mut self) -> &mut Self {
        self.update_status_id(1003_u64)
    }

    pub fn status_is_terminated(&self) -> bool {
        self.status_id() == 1003_u64
    }
    pub fn status(&self) -> Option<&crate::EmployeeStatus> {
        self.status.as_ref()
    }

    pub fn eval_status(&self) -> teaql_core::eval::EvalResult<&crate::EmployeeStatus> {
        if !self.is_loaded("status") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "status".to_string(), attempted_path: "status".to_string() }
        } else {
            match &self.status {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn position(&self) -> Option<&crate::Position> {
        self.position.as_ref()
    }

    pub fn eval_position(&self) -> teaql_core::eval::EvalResult<&crate::Position> {
        if !self.is_loaded("position") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "position".to_string(), attempted_path: "position".to_string() }
        } else {
            match &self.position {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn merchant(&self) -> Option<&crate::Merchant> {
        self.merchant.as_ref()
    }

    pub fn eval_merchant(&self) -> teaql_core::eval::EvalResult<&crate::Merchant> {
        if !self.is_loaded("merchant") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant".to_string(), attempted_path: "merchant".to_string() }
        } else {
            match &self.merchant {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::EmployeeRepository<'a>>>
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
            .employee_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Employee"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

#[derive(Clone, Debug, PartialEq, teaql_macros::TeaqlReverseRelations)]
pub struct EmployeeReverseRelations {
#[teaql(relation(target = "SalaryRecord", local_key = "id", foreign_key = "employee_id", many))]
    salary_record_list: SmartList<crate::SalaryRecord>,
#[teaql(relation(target = "AttendanceLog", local_key = "id", foreign_key = "employee_id", many))]
    attendance_log_list: SmartList<crate::AttendanceLog>,
#[teaql(relation(target = "LeaveRequest", local_key = "id", foreign_key = "employee_id", many))]
    leave_request_list: SmartList<crate::LeaveRequest>,
#[teaql(relation(target = "PerformanceReview", local_key = "id", foreign_key = "employee_id", many))]
    performance_review_list: SmartList<crate::PerformanceReview>,
#[teaql(relation(target = "TrainingRecord", local_key = "id", foreign_key = "employee_id", many))]
    training_record_list: SmartList<crate::TrainingRecord>,
#[teaql(relation(target = "BenefitPlan", local_key = "id", foreign_key = "employee_id", many))]
    benefit_plan_list: SmartList<crate::BenefitPlan>,
#[teaql(relation(target = "ExpenseClaim", local_key = "id", foreign_key = "employee_id", many))]
    expense_claim_list: SmartList<crate::ExpenseClaim>,
#[teaql(relation(target = "TaxForm", local_key = "id", foreign_key = "employee_id", many))]
    tax_form_list: SmartList<crate::TaxForm>,
#[teaql(relation(target = "Contract", local_key = "id", foreign_key = "employee_id", many))]
    contract_list: SmartList<crate::Contract>,
#[teaql(relation(target = "Resignation", local_key = "id", foreign_key = "employee_id", many))]
    resignation_list: SmartList<crate::Resignation>,
#[teaql(relation(target = "WarningLetter", local_key = "id", foreign_key = "employee_id", many))]
    warning_letter_list: SmartList<crate::WarningLetter>,
#[teaql(relation(target = "BonusRecord", local_key = "id", foreign_key = "employee_id", many))]
    bonus_record_list: SmartList<crate::BonusRecord>,
#[teaql(relation(target = "ShiftSchedule", local_key = "id", foreign_key = "employee_id", many))]
    shift_schedule_list: SmartList<crate::ShiftSchedule>,
#[teaql(relation(target = "TimeOffBalance", local_key = "id", foreign_key = "employee_id", many))]
    time_off_balance_list: SmartList<crate::TimeOffBalance>,
#[teaql(relation(target = "OnboardingChecklist", local_key = "id", foreign_key = "employee_id", many))]
    onboarding_checklist_list: SmartList<crate::OnboardingChecklist>,
#[teaql(relation(target = "OffboardingChecklist", local_key = "id", foreign_key = "employee_id", many))]
    offboarding_checklist_list: SmartList<crate::OffboardingChecklist>,
}

impl EmployeeReverseRelations {
    pub fn new() -> Self {
        Self {
            salary_record_list: Default::default(),
            attendance_log_list: Default::default(),
            leave_request_list: Default::default(),
            performance_review_list: Default::default(),
            training_record_list: Default::default(),
            benefit_plan_list: Default::default(),
            expense_claim_list: Default::default(),
            tax_form_list: Default::default(),
            contract_list: Default::default(),
            resignation_list: Default::default(),
            warning_letter_list: Default::default(),
            bonus_record_list: Default::default(),
            shift_schedule_list: Default::default(),
            time_off_balance_list: Default::default(),
            onboarding_checklist_list: Default::default(),
            offboarding_checklist_list: Default::default(),
        }
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
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
        for entity in &mut self.onboarding_checklist_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.offboarding_checklist_list {
            entity.attach_root_recursive(root.clone());
        }
    }
}
