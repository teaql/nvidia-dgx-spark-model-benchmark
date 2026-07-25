#[derive(Clone)]
pub struct OffboardingProcessExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::OffboardingProcess>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> OffboardingProcessExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::OffboardingProcess>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::OffboardingProcess> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::OffboardingProcess> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::OffboardingProcess {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_process_step(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("process_step", |entity| entity.eval_process_step());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_due_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("due_date", |entity| entity.eval_due_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_completed(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("completed", |entity| entity.eval_completed());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_employee_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("employee_id", |entity| entity.eval_employee_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_employee(self) -> crate::EmployeeRecordExpression<'a> {
        let next = self.result.and_then("employee", |entity| entity.eval_employee());
        crate::EmployeeRecordExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct OffboardingProcessListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::OffboardingProcess>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> OffboardingProcessListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::OffboardingProcess>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::OffboardingProcess>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::OffboardingProcess>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::OffboardingProcess> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::OffboardingProcessExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::OffboardingProcessExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::OffboardingProcessExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::OffboardingProcessExpression::new(next, self.root_desc.clone())
    }
}