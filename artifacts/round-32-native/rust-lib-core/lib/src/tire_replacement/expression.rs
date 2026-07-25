#[derive(Clone)]
pub struct TireReplacementExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::TireReplacement>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TireReplacementExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::TireReplacement>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::TireReplacement> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::TireReplacement> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::TireReplacement {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_replacement_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("replacement_date", |entity| entity.eval_replacement_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_mileage(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("mileage", |entity| entity.eval_mileage());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tire_brand(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("tire_brand", |entity| entity.eval_tire_brand());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_cost(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("cost", |entity| entity.eval_cost());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct TireReplacementListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TireReplacement>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TireReplacementListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TireReplacement>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::TireReplacement>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::TireReplacement>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::TireReplacement> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::TireReplacementExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TireReplacementExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::TireReplacementExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TireReplacementExpression::new(next, self.root_desc.clone())
    }
}