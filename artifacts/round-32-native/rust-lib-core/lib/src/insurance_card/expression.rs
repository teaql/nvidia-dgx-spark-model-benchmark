#[derive(Clone)]
pub struct InsuranceCardExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::InsuranceCard>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InsuranceCardExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::InsuranceCard>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::InsuranceCard> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::InsuranceCard> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::InsuranceCard {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_provider(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("provider", |entity| entity.eval_provider());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_policy_number(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("policy_number", |entity| entity.eval_policy_number());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_start_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("start_date", |entity| entity.eval_start_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_expiration_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("expiration_date", |entity| entity.eval_expiration_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_coverage_details(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("coverage_details", |entity| entity.eval_coverage_details());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct InsuranceCardListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::InsuranceCard>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InsuranceCardListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::InsuranceCard>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::InsuranceCard>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::InsuranceCard>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::InsuranceCard> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::InsuranceCardExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InsuranceCardExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::InsuranceCardExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InsuranceCardExpression::new(next, self.root_desc.clone())
    }
}