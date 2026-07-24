#[derive(Clone)]
pub struct RoleExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Role>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> RoleExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Role>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Role> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Role> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Role {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_role_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("role_id", |entity| entity.eval_role_id());
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
    pub fn get_merchant_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_ref_id", |entity| entity.eval_merchant_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_merchant_ref(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant_ref", |entity| entity.eval_merchant_ref());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn get_role_permission_list(self) -> crate::RolePermissionListExpression<'a> {
        let next = self.result.and_then("role_permission_list", |entity| entity.eval_role_permission_list());
        crate::RolePermissionListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct RoleListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Role>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> RoleListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Role>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Role>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Role>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Role> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::RoleExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::RoleExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::RoleExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::RoleExpression::new(next, self.root_desc.clone())
    }
}