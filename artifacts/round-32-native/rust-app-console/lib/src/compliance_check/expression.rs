#[derive(Clone)]
pub struct ComplianceCheckExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::ComplianceCheck>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ComplianceCheckExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::ComplianceCheck>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::ComplianceCheck> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::ComplianceCheck> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::ComplianceCheck {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_check_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("check_date", |entity| entity.eval_check_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_standard(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("standard", |entity| entity.eval_standard());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_result(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("result", |entity| entity.eval_result());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_inspector(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("inspector", |entity| entity.eval_inspector());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct ComplianceCheckListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ComplianceCheck>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ComplianceCheckListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ComplianceCheck>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::ComplianceCheck>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::ComplianceCheck>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::ComplianceCheck> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::ComplianceCheckExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ComplianceCheckExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::ComplianceCheckExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ComplianceCheckExpression::new(next, self.root_desc.clone())
    }
}