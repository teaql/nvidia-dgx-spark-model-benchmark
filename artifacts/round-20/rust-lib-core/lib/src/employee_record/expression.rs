#[derive(Clone)]
pub struct EmployeeRecordExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::EmployeeRecord>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> EmployeeRecordExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::EmployeeRecord>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::EmployeeRecord> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::EmployeeRecord> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::EmployeeRecord {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_employee_number(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("employee_number", |entity| entity.eval_employee_number());
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

    pub fn get_hire_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("hire_date", |entity| entity.eval_hire_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_department(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("department", |entity| entity.eval_department());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_employment_status(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("employment_status", |entity| entity.eval_employment_status());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_timesheet_entry_list(self) -> crate::TimesheetEntryListExpression<'a> {
        let next = self.result.and_then("timesheet_entry_list", |entity| entity.eval_timesheet_entry_list());
        crate::TimesheetEntryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tax_withholding_list(self) -> crate::TaxWithholdingListExpression<'a> {
        let next = self.result.and_then("tax_withholding_list", |entity| entity.eval_tax_withholding_list());
        crate::TaxWithholdingListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_leave_request_list(self) -> crate::LeaveRequestListExpression<'a> {
        let next = self.result.and_then("leave_request_list", |entity| entity.eval_leave_request_list());
        crate::LeaveRequestListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_training_record_list(self) -> crate::TrainingRecordListExpression<'a> {
        let next = self.result.and_then("training_record_list", |entity| entity.eval_training_record_list());
        crate::TrainingRecordListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_performance_review_list_as_employee(self) -> crate::PerformanceReviewListExpression<'a> {
        let next = self.result.and_then("performance_review_list_as_employee", |entity| entity.eval_performance_review_list_as_employee());
        crate::PerformanceReviewListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_performance_review_list_as_reviewer(self) -> crate::PerformanceReviewListExpression<'a> {
        let next = self.result.and_then("performance_review_list_as_reviewer", |entity| entity.eval_performance_review_list_as_reviewer());
        crate::PerformanceReviewListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_compensation_adjustment_list_as_employee(self) -> crate::CompensationAdjustmentListExpression<'a> {
        let next = self.result.and_then("compensation_adjustment_list_as_employee", |entity| entity.eval_compensation_adjustment_list_as_employee());
        crate::CompensationAdjustmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_compensation_adjustment_list_as_approved_by(self) -> crate::CompensationAdjustmentListExpression<'a> {
        let next = self.result.and_then("compensation_adjustment_list_as_approved_by", |entity| entity.eval_compensation_adjustment_list_as_approved_by());
        crate::CompensationAdjustmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_onboarding_checklist_list(self) -> crate::OnboardingChecklistListExpression<'a> {
        let next = self.result.and_then("onboarding_checklist_list", |entity| entity.eval_onboarding_checklist_list());
        crate::OnboardingChecklistListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_offboarding_process_list(self) -> crate::OffboardingProcessListExpression<'a> {
        let next = self.result.and_then("offboarding_process_list", |entity| entity.eval_offboarding_process_list());
        crate::OffboardingProcessListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct EmployeeRecordListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::EmployeeRecord>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> EmployeeRecordListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::EmployeeRecord>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::EmployeeRecord>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::EmployeeRecord>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::EmployeeRecord> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::EmployeeRecordExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::EmployeeRecordExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::EmployeeRecordExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::EmployeeRecordExpression::new(next, self.root_desc.clone())
    }
}