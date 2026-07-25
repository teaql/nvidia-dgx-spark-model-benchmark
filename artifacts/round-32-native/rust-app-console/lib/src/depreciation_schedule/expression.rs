#[derive(Clone)]
pub struct DepreciationScheduleExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::DepreciationSchedule>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DepreciationScheduleExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::DepreciationSchedule>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::DepreciationSchedule> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::DepreciationSchedule> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::DepreciationSchedule {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_fiscal_year(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("fiscal_year", |entity| entity.eval_fiscal_year());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_depreciation_amount(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("depreciation_amount", |entity| entity.eval_depreciation_amount());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_book_value(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("book_value", |entity| entity.eval_book_value());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_method(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("method", |entity| entity.eval_method());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct DepreciationScheduleListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DepreciationSchedule>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DepreciationScheduleListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DepreciationSchedule>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::DepreciationSchedule>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::DepreciationSchedule>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::DepreciationSchedule> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::DepreciationScheduleExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DepreciationScheduleExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::DepreciationScheduleExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DepreciationScheduleExpression::new(next, self.root_desc.clone())
    }
}