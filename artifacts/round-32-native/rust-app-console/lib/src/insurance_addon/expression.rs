#[derive(Clone)]
pub struct InsuranceAddonExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::InsuranceAddon>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InsuranceAddonExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::InsuranceAddon>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::InsuranceAddon> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::InsuranceAddon> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::InsuranceAddon {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_coverage_limit(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("coverage_limit", |entity| entity.eval_coverage_limit());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_premium_amount(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("premium_amount", |entity| entity.eval_premium_amount());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct InsuranceAddonListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::InsuranceAddon>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InsuranceAddonListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::InsuranceAddon>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::InsuranceAddon>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::InsuranceAddon>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::InsuranceAddon> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::InsuranceAddonExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InsuranceAddonExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::InsuranceAddonExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InsuranceAddonExpression::new(next, self.root_desc.clone())
    }
}