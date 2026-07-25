#[derive(Clone)]
pub struct TrainingRecordExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::TrainingRecord>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TrainingRecordExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::TrainingRecord>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::TrainingRecord> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::TrainingRecord> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::TrainingRecord {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_training_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("training_name", |entity| entity.eval_training_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_completion_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("completion_date", |entity| entity.eval_completion_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_provider(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("provider", |entity| entity.eval_provider());
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
pub struct TrainingRecordListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TrainingRecord>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TrainingRecordListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TrainingRecord>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::TrainingRecord>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::TrainingRecord>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::TrainingRecord> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::TrainingRecordExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TrainingRecordExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::TrainingRecordExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TrainingRecordExpression::new(next, self.root_desc.clone())
    }
}