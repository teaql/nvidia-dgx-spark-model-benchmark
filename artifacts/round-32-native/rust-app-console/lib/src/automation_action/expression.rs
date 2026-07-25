#[derive(Clone)]
pub struct AutomationActionExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::AutomationAction>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> AutomationActionExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::AutomationAction>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::AutomationAction> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::AutomationAction> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::AutomationAction {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_action_kind(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("action_kind", |entity| entity.eval_action_kind());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_target_system(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("target_system", |entity| entity.eval_target_system());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payload(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("payload", |entity| entity.eval_payload());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_retry_count(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("retry_count", |entity| entity.eval_retry_count());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct AutomationActionListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::AutomationAction>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> AutomationActionListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::AutomationAction>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::AutomationAction>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::AutomationAction>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::AutomationAction> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::AutomationActionExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::AutomationActionExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::AutomationActionExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::AutomationActionExpression::new(next, self.root_desc.clone())
    }
}