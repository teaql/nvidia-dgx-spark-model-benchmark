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

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_role(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("role", |entity| entity.eval_role());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_merchant_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_ref_id", |entity| entity.eval_merchant_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_merchant_ref(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant_ref", |entity| entity.eval_merchant_ref());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn get_job_assignment_list(self) -> crate::JobAssignmentListExpression<'a> {
        let next = self.result.and_then("job_assignment_list", |entity| entity.eval_job_assignment_list());
        crate::JobAssignmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_work_shift_list(self) -> crate::WorkShiftListExpression<'a> {
        let next = self.result.and_then("work_shift_list", |entity| entity.eval_work_shift_list());
        crate::WorkShiftListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_bonus_list(self) -> crate::BonusListExpression<'a> {
        let next = self.result.and_then("bonus_list", |entity| entity.eval_bonus_list());
        crate::BonusListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_leave_request_list(self) -> crate::LeaveRequestListExpression<'a> {
        let next = self.result.and_then("leave_request_list", |entity| entity.eval_leave_request_list());
        crate::LeaveRequestListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_employee_certification_list(self) -> crate::EmployeeCertificationListExpression<'a> {
        let next = self.result.and_then("employee_certification_list", |entity| entity.eval_employee_certification_list());
        crate::EmployeeCertificationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_training_module_list(self) -> crate::TrainingModuleListExpression<'a> {
        let next = self.result.and_then("training_module_list", |entity| entity.eval_training_module_list());
        crate::TrainingModuleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_availability_schedule_list(self) -> crate::AvailabilityScheduleListExpression<'a> {
        let next = self.result.and_then("availability_schedule_list", |entity| entity.eval_availability_schedule_list());
        crate::AvailabilityScheduleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_skill_profile_list(self) -> crate::SkillProfileListExpression<'a> {
        let next = self.result.and_then("skill_profile_list", |entity| entity.eval_skill_profile_list());
        crate::SkillProfileListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_performance_review_list(self) -> crate::PerformanceReviewListExpression<'a> {
        let next = self.result.and_then("performance_review_list", |entity| entity.eval_performance_review_list());
        crate::PerformanceReviewListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_overtime_record_list(self) -> crate::OvertimeRecordListExpression<'a> {
        let next = self.result.and_then("overtime_record_list", |entity| entity.eval_overtime_record_list());
        crate::OvertimeRecordListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_benefit_enrollment_list(self) -> crate::BenefitEnrollmentListExpression<'a> {
        let next = self.result.and_then("benefit_enrollment_list", |entity| entity.eval_benefit_enrollment_list());
        crate::BenefitEnrollmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_shift_swap_request_list(self) -> crate::ShiftSwapRequestListExpression<'a> {
        let next = self.result.and_then("shift_swap_request_list", |entity| entity.eval_shift_swap_request_list());
        crate::ShiftSwapRequestListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_attendance_record_list(self) -> crate::AttendanceRecordListExpression<'a> {
        let next = self.result.and_then("attendance_record_list", |entity| entity.eval_attendance_record_list());
        crate::AttendanceRecordListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_commission_record_list(self) -> crate::CommissionRecordListExpression<'a> {
        let next = self.result.and_then("commission_record_list", |entity| entity.eval_commission_record_list());
        crate::CommissionRecordListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_user_account_list(self) -> crate::UserAccountListExpression<'a> {
        let next = self.result.and_then("user_account_list", |entity| entity.eval_user_account_list());
        crate::UserAccountListExpression::new(next, self.root_desc.clone())
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