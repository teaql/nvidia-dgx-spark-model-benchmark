#[derive(Clone)]
pub struct DocumentExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Document>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DocumentExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Document>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Document> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Document> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Document {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_document_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("document_id", |entity| entity.eval_document_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_contract_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("contract_ref_id", |entity| entity.eval_contract_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_contract_ref(self) -> crate::ContractExpression<'a> {
        let next = self.result.and_then("contract_ref", |entity| entity.eval_contract_ref());
        crate::ContractExpression::new(next, self.root_desc.clone())
    }
    pub fn get_document_version_list(self) -> crate::DocumentVersionListExpression<'a> {
        let next = self.result.and_then("document_version_list", |entity| entity.eval_document_version_list());
        crate::DocumentVersionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_recovery_request_list(self) -> crate::RecoveryRequestListExpression<'a> {
        let next = self.result.and_then("recovery_request_list", |entity| entity.eval_recovery_request_list());
        crate::RecoveryRequestListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct DocumentListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Document>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DocumentListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Document>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Document>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Document>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Document> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::DocumentExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DocumentExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::DocumentExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DocumentExpression::new(next, self.root_desc.clone())
    }
}