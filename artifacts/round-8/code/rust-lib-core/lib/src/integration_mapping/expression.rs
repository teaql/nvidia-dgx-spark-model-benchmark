#[derive(Clone)]
pub struct IntegrationMappingExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::IntegrationMapping>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> IntegrationMappingExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::IntegrationMapping>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::IntegrationMapping> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::IntegrationMapping> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::IntegrationMapping {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_mapping_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("mapping_id", |entity| entity.eval_mapping_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_source(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("source", |entity| entity.eval_source());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_api_client_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("api_client_ref_id", |entity| entity.eval_api_client_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_api_client_ref(self) -> crate::ApiClientExpression<'a> {
        let next = self.result.and_then("api_client_ref", |entity| entity.eval_api_client_ref());
        crate::ApiClientExpression::new(next, self.root_desc.clone())
    }
    pub fn get_synchronization_run_list(self) -> crate::SynchronizationRunListExpression<'a> {
        let next = self.result.and_then("synchronization_run_list", |entity| entity.eval_synchronization_run_list());
        crate::SynchronizationRunListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct IntegrationMappingListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::IntegrationMapping>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> IntegrationMappingListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::IntegrationMapping>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::IntegrationMapping>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::IntegrationMapping>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::IntegrationMapping> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::IntegrationMappingExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::IntegrationMappingExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::IntegrationMappingExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::IntegrationMappingExpression::new(next, self.root_desc.clone())
    }
}