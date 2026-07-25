#[derive(Clone)]
pub struct ApiEndpointExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::ApiEndpoint>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ApiEndpointExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::ApiEndpoint>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::ApiEndpoint> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::ApiEndpoint> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::ApiEndpoint {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_path_pattern(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("path_pattern", |entity| entity.eval_path_pattern());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_http_method(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("http_method", |entity| entity.eval_http_method());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version_tag(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("version_tag", |entity| entity.eval_version_tag());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_is_deprecated(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("is_deprecated", |entity| entity.eval_is_deprecated());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct ApiEndpointListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ApiEndpoint>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ApiEndpointListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ApiEndpoint>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::ApiEndpoint>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::ApiEndpoint>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::ApiEndpoint> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::ApiEndpointExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ApiEndpointExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::ApiEndpointExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ApiEndpointExpression::new(next, self.root_desc.clone())
    }
}