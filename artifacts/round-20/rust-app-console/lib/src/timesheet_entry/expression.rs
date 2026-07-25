#[derive(Clone)]
pub struct TimesheetEntryExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::TimesheetEntry>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TimesheetEntryExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::TimesheetEntry>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::TimesheetEntry> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::TimesheetEntry> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::TimesheetEntry {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_work_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("work_date", |entity| entity.eval_work_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_hours_worked(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("hours_worked", |entity| entity.eval_hours_worked());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_description(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("description", |entity| entity.eval_description());
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
pub struct TimesheetEntryListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TimesheetEntry>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TimesheetEntryListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TimesheetEntry>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::TimesheetEntry>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::TimesheetEntry>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::TimesheetEntry> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::TimesheetEntryExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TimesheetEntryExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::TimesheetEntryExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TimesheetEntryExpression::new(next, self.root_desc.clone())
    }
}