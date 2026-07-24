#[derive(Clone)]
pub struct LeaveRequestExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::LeaveRequest>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> LeaveRequestExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::LeaveRequest>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::LeaveRequest> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::LeaveRequest> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::LeaveRequest {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_start_date(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("start_date", |entity| entity.eval_start_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_end_date(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("end_date", |entity| entity.eval_end_date());
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
    pub fn get_leave_type_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("leave_type_id", |entity| entity.eval_leave_type_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_employee_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("employee_id", |entity| entity.eval_employee_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_id", |entity| entity.eval_merchant_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_leave_type(self) -> crate::LeaveTypeExpression<'a> {
        let next = self.result.and_then("leave_type", |entity| entity.eval_leave_type());
        crate::LeaveTypeExpression::new(next, self.root_desc.clone())
    }

    pub fn get_employee(self) -> crate::EmployeeExpression<'a> {
        let next = self.result.and_then("employee", |entity| entity.eval_employee());
        crate::EmployeeExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant", |entity| entity.eval_merchant());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn leave_type_is_annual(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("leave_type_id", |entity| {
            if !entity.is_loaded("leave_type_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "leave_type_id".to_string(), attempted_path: "leave_type_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.leave_type_is_annual())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn leave_type_is_sick(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("leave_type_id", |entity| {
            if !entity.is_loaded("leave_type_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "leave_type_id".to_string(), attempted_path: "leave_type_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.leave_type_is_sick())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn leave_type_is_unpaid(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("leave_type_id", |entity| {
            if !entity.is_loaded("leave_type_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "leave_type_id".to_string(), attempted_path: "leave_type_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.leave_type_is_unpaid())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct LeaveRequestListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::LeaveRequest>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> LeaveRequestListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::LeaveRequest>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::LeaveRequest>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::LeaveRequest>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::LeaveRequest> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::LeaveRequestExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::LeaveRequestExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::LeaveRequestExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::LeaveRequestExpression::new(next, self.root_desc.clone())
    }
}