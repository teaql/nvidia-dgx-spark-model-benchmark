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

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_department_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("department_id", |entity| entity.eval_department_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_department(self) -> crate::DepartmentExpression<'a> {
        let next = self.result.and_then("department", |entity| entity.eval_department());
        crate::DepartmentExpression::new(next, self.root_desc.clone())
    }
    pub fn get_operations_manager_override_list(self) -> crate::OperationsManagerOverrideListExpression<'a> {
        let next = self.result.and_then("operations_manager_override_list", |entity| entity.eval_operations_manager_override_list());
        crate::OperationsManagerOverrideListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_job_assignment_list(self) -> crate::JobAssignmentListExpression<'a> {
        let next = self.result.and_then("job_assignment_list", |entity| entity.eval_job_assignment_list());
        crate::JobAssignmentListExpression::new(next, self.root_desc.clone())
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

    pub fn get_tax_withholding_list(self) -> crate::TaxWithholdingListExpression<'a> {
        let next = self.result.and_then("tax_withholding_list", |entity| entity.eval_tax_withholding_list());
        crate::TaxWithholdingListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_direct_deposit_info_list(self) -> crate::DirectDepositInfoListExpression<'a> {
        let next = self.result.and_then("direct_deposit_info_list", |entity| entity.eval_direct_deposit_info_list());
        crate::DirectDepositInfoListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_union_dues_list(self) -> crate::UnionDuesListExpression<'a> {
        let next = self.result.and_then("union_dues_list", |entity| entity.eval_union_dues_list());
        crate::UnionDuesListExpression::new(next, self.root_desc.clone())
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