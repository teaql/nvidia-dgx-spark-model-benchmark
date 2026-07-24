#[derive(Clone)]
pub struct EmployeeExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Employee>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> EmployeeExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Employee>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Employee> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Employee> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Employee {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_first_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("first_name", |entity| entity.eval_first_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_last_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("last_name", |entity| entity.eval_last_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_email(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("email", |entity| entity.eval_email());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_create_time(self) -> crate::ValueExpression<'a, chrono::DateTime<chrono::Utc>> {
        let next = self.result.and_then("create_time", |entity| entity.eval_create_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_update_time(self) -> crate::ValueExpression<'a, chrono::DateTime<chrono::Utc>> {
        let next = self.result.and_then("update_time", |entity| entity.eval_update_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_status_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("status_id", |entity| entity.eval_status_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_position_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("position_id", |entity| entity.eval_position_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_id", |entity| entity.eval_merchant_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_status(self) -> crate::EmployeeStatusExpression<'a> {
        let next = self.result.and_then("status", |entity| entity.eval_status());
        crate::EmployeeStatusExpression::new(next, self.root_desc.clone())
    }

    pub fn get_position(self) -> crate::PositionExpression<'a> {
        let next = self.result.and_then("position", |entity| entity.eval_position());
        crate::PositionExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant", |entity| entity.eval_merchant());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn status_is_active(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("status_id", |entity| {
            if !entity.is_loaded("status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.status_is_active())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn status_is_probation(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("status_id", |entity| {
            if !entity.is_loaded("status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.status_is_probation())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn status_is_terminated(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("status_id", |entity| {
            if !entity.is_loaded("status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.status_is_terminated())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_salary_record_list(self) -> crate::SalaryRecordListExpression<'a> {
        let next = self.result.and_then("salary_record_list", |entity| entity.eval_salary_record_list());
        crate::SalaryRecordListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_attendance_log_list(self) -> crate::AttendanceLogListExpression<'a> {
        let next = self.result.and_then("attendance_log_list", |entity| entity.eval_attendance_log_list());
        crate::AttendanceLogListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_leave_request_list(self) -> crate::LeaveRequestListExpression<'a> {
        let next = self.result.and_then("leave_request_list", |entity| entity.eval_leave_request_list());
        crate::LeaveRequestListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_performance_review_list(self) -> crate::PerformanceReviewListExpression<'a> {
        let next = self.result.and_then("performance_review_list", |entity| entity.eval_performance_review_list());
        crate::PerformanceReviewListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_training_record_list(self) -> crate::TrainingRecordListExpression<'a> {
        let next = self.result.and_then("training_record_list", |entity| entity.eval_training_record_list());
        crate::TrainingRecordListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_benefit_plan_list(self) -> crate::BenefitPlanListExpression<'a> {
        let next = self.result.and_then("benefit_plan_list", |entity| entity.eval_benefit_plan_list());
        crate::BenefitPlanListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_expense_claim_list(self) -> crate::ExpenseClaimListExpression<'a> {
        let next = self.result.and_then("expense_claim_list", |entity| entity.eval_expense_claim_list());
        crate::ExpenseClaimListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tax_form_list(self) -> crate::TaxFormListExpression<'a> {
        let next = self.result.and_then("tax_form_list", |entity| entity.eval_tax_form_list());
        crate::TaxFormListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_contract_list(self) -> crate::ContractListExpression<'a> {
        let next = self.result.and_then("contract_list", |entity| entity.eval_contract_list());
        crate::ContractListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_resignation_list(self) -> crate::ResignationListExpression<'a> {
        let next = self.result.and_then("resignation_list", |entity| entity.eval_resignation_list());
        crate::ResignationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_warning_letter_list(self) -> crate::WarningLetterListExpression<'a> {
        let next = self.result.and_then("warning_letter_list", |entity| entity.eval_warning_letter_list());
        crate::WarningLetterListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_bonus_record_list(self) -> crate::BonusRecordListExpression<'a> {
        let next = self.result.and_then("bonus_record_list", |entity| entity.eval_bonus_record_list());
        crate::BonusRecordListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_shift_schedule_list(self) -> crate::ShiftScheduleListExpression<'a> {
        let next = self.result.and_then("shift_schedule_list", |entity| entity.eval_shift_schedule_list());
        crate::ShiftScheduleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_time_off_balance_list(self) -> crate::TimeOffBalanceListExpression<'a> {
        let next = self.result.and_then("time_off_balance_list", |entity| entity.eval_time_off_balance_list());
        crate::TimeOffBalanceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_onboarding_checklist_list(self) -> crate::OnboardingChecklistListExpression<'a> {
        let next = self.result.and_then("onboarding_checklist_list", |entity| entity.eval_onboarding_checklist_list());
        crate::OnboardingChecklistListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_offboarding_checklist_list(self) -> crate::OffboardingChecklistListExpression<'a> {
        let next = self.result.and_then("offboarding_checklist_list", |entity| entity.eval_offboarding_checklist_list());
        crate::OffboardingChecklistListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct EmployeeListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Employee>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> EmployeeListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Employee>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Employee>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Employee>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Employee> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::EmployeeExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::EmployeeExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::EmployeeExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::EmployeeExpression::new(next, self.root_desc.clone())
    }
}