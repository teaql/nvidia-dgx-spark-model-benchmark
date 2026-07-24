#[derive(Clone)]
pub struct SynchronizationRunExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::SynchronizationRun>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> SynchronizationRunExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::SynchronizationRun>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::SynchronizationRun> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::SynchronizationRun> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::SynchronizationRun {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_run_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("run_id", |entity| entity.eval_run_id());
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
    pub fn get_integration_mapping_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("integration_mapping_ref_id", |entity| entity.eval_integration_mapping_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_integration_mapping_ref(self) -> crate::IntegrationMappingExpression<'a> {
        let next = self.result.and_then("integration_mapping_ref", |entity| entity.eval_integration_mapping_ref());
        crate::IntegrationMappingExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct SynchronizationRunListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::SynchronizationRun>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> SynchronizationRunListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::SynchronizationRun>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::SynchronizationRun>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::SynchronizationRun>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::SynchronizationRun> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::SynchronizationRunExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::SynchronizationRunExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::SynchronizationRunExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::SynchronizationRunExpression::new(next, self.root_desc.clone())
    }
}