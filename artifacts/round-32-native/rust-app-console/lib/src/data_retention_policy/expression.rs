#[derive(Clone)]
pub struct DataRetentionPolicyExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::DataRetentionPolicy>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DataRetentionPolicyExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::DataRetentionPolicy>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::DataRetentionPolicy> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::DataRetentionPolicy> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::DataRetentionPolicy {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_retention_period_days(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("retention_period_days", |entity| entity.eval_retention_period_days());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_data_category(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("data_category", |entity| entity.eval_data_category());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct DataRetentionPolicyListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DataRetentionPolicy>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DataRetentionPolicyListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DataRetentionPolicy>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::DataRetentionPolicy>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::DataRetentionPolicy>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::DataRetentionPolicy> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::DataRetentionPolicyExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DataRetentionPolicyExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::DataRetentionPolicyExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DataRetentionPolicyExpression::new(next, self.root_desc.clone())
    }
}