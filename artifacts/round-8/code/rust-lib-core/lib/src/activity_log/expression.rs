#[derive(Clone)]
pub struct ActivityLogExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::ActivityLog>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ActivityLogExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::ActivityLog>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::ActivityLog> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::ActivityLog> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::ActivityLog {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_log_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("log_id", |entity| entity.eval_log_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_action(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("action", |entity| entity.eval_action());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_user_account_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("user_account_ref_id", |entity| entity.eval_user_account_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_user_account_ref(self) -> crate::UserAccountExpression<'a> {
        let next = self.result.and_then("user_account_ref", |entity| entity.eval_user_account_ref());
        crate::UserAccountExpression::new(next, self.root_desc.clone())
    }
    pub fn get_entity_change_list(self) -> crate::EntityChangeListExpression<'a> {
        let next = self.result.and_then("entity_change_list", |entity| entity.eval_entity_change_list());
        crate::EntityChangeListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct ActivityLogListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ActivityLog>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ActivityLogListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::ActivityLog>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::ActivityLog>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::ActivityLog>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::ActivityLog> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::ActivityLogExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ActivityLogExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::ActivityLogExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ActivityLogExpression::new(next, self.root_desc.clone())
    }
}