#[derive(Clone)]
pub struct LeadScoreExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::LeadScore>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> LeadScoreExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::LeadScore>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::LeadScore> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::LeadScore> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::LeadScore {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_score_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("score_id", |entity| entity.eval_score_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_value(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("value", |entity| entity.eval_value());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_lead_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("lead_ref_id", |entity| entity.eval_lead_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_lead_ref(self) -> crate::LeadExpression<'a> {
        let next = self.result.and_then("lead_ref", |entity| entity.eval_lead_ref());
        crate::LeadExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct LeadScoreListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::LeadScore>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> LeadScoreListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::LeadScore>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::LeadScore>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::LeadScore>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::LeadScore> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::LeadScoreExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::LeadScoreExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::LeadScoreExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::LeadScoreExpression::new(next, self.root_desc.clone())
    }
}