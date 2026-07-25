#[derive(Clone)]
pub struct CustomEntity216Expression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::CustomEntity216>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> CustomEntity216Expression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::CustomEntity216>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::CustomEntity216> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::CustomEntity216> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::CustomEntity216 {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_category(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("category", |entity| entity.eval_category());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_rank(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("rank", |entity| entity.eval_rank());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_score(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("score", |entity| entity.eval_score());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_note(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("note", |entity| entity.eval_note());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct CustomEntity216ListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::CustomEntity216>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> CustomEntity216ListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::CustomEntity216>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::CustomEntity216>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::CustomEntity216>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::CustomEntity216> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::CustomEntity216Expression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::CustomEntity216Expression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::CustomEntity216Expression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::CustomEntity216Expression::new(next, self.root_desc.clone())
    }
}