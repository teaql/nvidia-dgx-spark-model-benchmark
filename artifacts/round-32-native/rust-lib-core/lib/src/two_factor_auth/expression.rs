#[derive(Clone)]
pub struct TwoFactorAuthExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::TwoFactorAuth>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TwoFactorAuthExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::TwoFactorAuth>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::TwoFactorAuth> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::TwoFactorAuth> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::TwoFactorAuth {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_user_account_ref(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("user_account_ref", |entity| entity.eval_user_account_ref());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_secret_key(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("secret_key", |entity| entity.eval_secret_key());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_is_enabled(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("is_enabled", |entity| entity.eval_is_enabled());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct TwoFactorAuthListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TwoFactorAuth>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TwoFactorAuthListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TwoFactorAuth>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::TwoFactorAuth>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::TwoFactorAuth>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::TwoFactorAuth> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::TwoFactorAuthExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TwoFactorAuthExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::TwoFactorAuthExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TwoFactorAuthExpression::new(next, self.root_desc.clone())
    }
}