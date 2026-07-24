#[derive(Clone)]
pub struct AutomationTriggerExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::AutomationTrigger>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> AutomationTriggerExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::AutomationTrigger>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::AutomationTrigger> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::AutomationTrigger> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::AutomationTrigger {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_trigger_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("trigger_id", |entity| entity.eval_trigger_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_event(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("event", |entity| entity.eval_event());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_automation_rule_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("automation_rule_ref_id", |entity| entity.eval_automation_rule_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_automation_rule_ref(self) -> crate::AutomationRuleExpression<'a> {
        let next = self.result.and_then("automation_rule_ref", |entity| entity.eval_automation_rule_ref());
        crate::AutomationRuleExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct AutomationTriggerListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::AutomationTrigger>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> AutomationTriggerListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::AutomationTrigger>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::AutomationTrigger>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::AutomationTrigger>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::AutomationTrigger> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::AutomationTriggerExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::AutomationTriggerExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::AutomationTriggerExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::AutomationTriggerExpression::new(next, self.root_desc.clone())
    }
}