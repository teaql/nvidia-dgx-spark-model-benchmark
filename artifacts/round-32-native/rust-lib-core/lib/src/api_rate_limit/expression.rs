#[derive(Clone)]
pub struct ApiRateLimitExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::ApiRateLimit>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ApiRateLimitExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::ApiRateLimit>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::ApiRateLimit> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::ApiRateLimit> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::ApiRateLimit {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_limit_key(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("limit_key", |entity| entity.eval_limit_key());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_max_requests(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("max_requests", |entity| entity.eval_max_requests());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_window_seconds(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("window_seconds", |entity| entity.eval_window_seconds());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_current_count(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("current_count", |entity| entity.eval_current_count());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct ApiRateLimitListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ApiRateLimit>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ApiRateLimitListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ApiRateLimit>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::ApiRateLimit>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::ApiRateLimit>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::ApiRateLimit> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::ApiRateLimitExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ApiRateLimitExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::ApiRateLimitExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ApiRateLimitExpression::new(next, self.root_desc.clone())
    }
}