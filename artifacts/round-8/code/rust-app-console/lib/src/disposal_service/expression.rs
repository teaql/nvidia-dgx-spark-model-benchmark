#[derive(Clone)]
pub struct DisposalServiceExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::DisposalService>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DisposalServiceExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::DisposalService>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::DisposalService> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::DisposalService> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::DisposalService {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("service_id", |entity| entity.eval_service_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_record_type(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("record_type", |entity| entity.eval_record_type());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_service_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("service_ref_id", |entity| entity.eval_service_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_service_ref(self) -> crate::ServiceExpression<'a> {
        let next = self.result.and_then("service_ref", |entity| entity.eval_service_ref());
        crate::ServiceExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct DisposalServiceListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DisposalService>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DisposalServiceListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DisposalService>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::DisposalService>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::DisposalService>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::DisposalService> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::DisposalServiceExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DisposalServiceExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::DisposalServiceExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DisposalServiceExpression::new(next, self.root_desc.clone())
    }
}