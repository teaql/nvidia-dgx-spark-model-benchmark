#[derive(Clone)]
pub struct MerchantExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Merchant>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MerchantExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Merchant>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Merchant> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Merchant> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Merchant {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tax_number(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("tax_number", |entity| entity.eval_tax_number());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_address(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("address", |entity| entity.eval_address());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_external_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("external_id", |entity| entity.eval_external_id());
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
    pub fn get_platform_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("platform_id", |entity| entity.eval_platform_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_platform(self) -> crate::PlatformExpression<'a> {
        let next = self.result.and_then("platform", |entity| entity.eval_platform());
        crate::PlatformExpression::new(next, self.root_desc.clone())
    }
    pub fn get_department_list(self) -> crate::DepartmentListExpression<'a> {
        let next = self.result.and_then("department_list", |entity| entity.eval_department_list());
        crate::DepartmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_position_list(self) -> crate::PositionListExpression<'a> {
        let next = self.result.and_then("position_list", |entity| entity.eval_position_list());
        crate::PositionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_employee_list(self) -> crate::EmployeeListExpression<'a> {
        let next = self.result.and_then("employee_list", |entity| entity.eval_employee_list());
        crate::EmployeeListExpression::new(next, self.root_desc.clone())
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

    pub fn get_payroll_run_list(self) -> crate::PayrollRunListExpression<'a> {
        let next = self.result.and_then("payroll_run_list", |entity| entity.eval_payroll_run_list());
        crate::PayrollRunListExpression::new(next, self.root_desc.clone())
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

    pub fn get_recruitment_post_list(self) -> crate::RecruitmentPostListExpression<'a> {
        let next = self.result.and_then("recruitment_post_list", |entity| entity.eval_recruitment_post_list());
        crate::RecruitmentPostListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_job_application_list(self) -> crate::JobApplicationListExpression<'a> {
        let next = self.result.and_then("job_application_list", |entity| entity.eval_job_application_list());
        crate::JobApplicationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_interview_list(self) -> crate::InterviewListExpression<'a> {
        let next = self.result.and_then("interview_list", |entity| entity.eval_interview_list());
        crate::InterviewListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_offer_letter_list(self) -> crate::OfferLetterListExpression<'a> {
        let next = self.result.and_then("offer_letter_list", |entity| entity.eval_offer_letter_list());
        crate::OfferLetterListExpression::new(next, self.root_desc.clone())
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
pub struct MerchantListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Merchant>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MerchantListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Merchant>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Merchant>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Merchant>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Merchant> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::MerchantExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::MerchantExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
}