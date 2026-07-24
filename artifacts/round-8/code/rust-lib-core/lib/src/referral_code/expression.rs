#[derive(Clone)]
pub struct ReferralCodeExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::ReferralCode>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ReferralCodeExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::ReferralCode>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::ReferralCode> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::ReferralCode> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::ReferralCode {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_code(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("code", |entity| entity.eval_code());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_uses(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("uses", |entity| entity.eval_uses());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_customer_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("customer_ref_id", |entity| entity.eval_customer_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_customer_ref(self) -> crate::CustomerExpression<'a> {
        let next = self.result.and_then("customer_ref", |entity| entity.eval_customer_ref());
        crate::CustomerExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct ReferralCodeListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ReferralCode>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ReferralCodeListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ReferralCode>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::ReferralCode>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::ReferralCode>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::ReferralCode> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::ReferralCodeExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ReferralCodeExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::ReferralCodeExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ReferralCodeExpression::new(next, self.root_desc.clone())
    }
}