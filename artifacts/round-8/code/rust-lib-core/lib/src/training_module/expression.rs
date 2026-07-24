#[derive(Clone)]
pub struct TrainingModuleExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::TrainingModule>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TrainingModuleExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::TrainingModule>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::TrainingModule> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::TrainingModule> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::TrainingModule {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_module_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("module_id", |entity| entity.eval_module_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_duration_hrs(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("duration_hrs", |entity| entity.eval_duration_hrs());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_employee_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("employee_ref_id", |entity| entity.eval_employee_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_employee_ref(self) -> crate::EmployeeExpression<'a> {
        let next = self.result.and_then("employee_ref", |entity| entity.eval_employee_ref());
        crate::EmployeeExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct TrainingModuleListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TrainingModule>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TrainingModuleListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TrainingModule>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::TrainingModule>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::TrainingModule>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::TrainingModule> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::TrainingModuleExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TrainingModuleExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::TrainingModuleExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TrainingModuleExpression::new(next, self.root_desc.clone())
    }
}