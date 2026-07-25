#[derive(Clone)]
pub struct OilChangeLogExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::OilChangeLog>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> OilChangeLogExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::OilChangeLog>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::OilChangeLog> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::OilChangeLog> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::OilChangeLog {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("service_date", |entity| entity.eval_service_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_mileage(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("mileage", |entity| entity.eval_mileage());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_oil_brand(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("oil_brand", |entity| entity.eval_oil_brand());
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
pub struct OilChangeLogListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::OilChangeLog>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> OilChangeLogListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::OilChangeLog>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::OilChangeLog>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::OilChangeLog>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::OilChangeLog> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::OilChangeLogExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::OilChangeLogExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::OilChangeLogExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::OilChangeLogExpression::new(next, self.root_desc.clone())
    }
}