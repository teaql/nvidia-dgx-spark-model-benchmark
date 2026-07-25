#[derive(Clone)]
pub struct PayrollRunExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::PayrollRun>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PayrollRunExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::PayrollRun>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::PayrollRun> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::PayrollRun> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::PayrollRun {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payroll_period_start(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("payroll_period_start", |entity| entity.eval_payroll_period_start());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payroll_period_end(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("payroll_period_end", |entity| entity.eval_payroll_period_end());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_run_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("run_date", |entity| entity.eval_run_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_status(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("status", |entity| entity.eval_status());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct PayrollRunListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::PayrollRun>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PayrollRunListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::PayrollRun>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::PayrollRun>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::PayrollRun>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::PayrollRun> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::PayrollRunExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PayrollRunExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::PayrollRunExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PayrollRunExpression::new(next, self.root_desc.clone())
    }
}