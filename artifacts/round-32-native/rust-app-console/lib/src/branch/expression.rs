#[derive(Clone)]
pub struct BranchExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Branch>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> BranchExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Branch>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Branch> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Branch> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Branch {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_branch_code(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("branch_code", |entity| entity.eval_branch_code());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_operating_status(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("operating_status", |entity| entity.eval_operating_status());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_time_zone(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("time_zone", |entity| entity.eval_time_zone());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_contact_phone(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("contact_phone", |entity| entity.eval_contact_phone());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct BranchListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Branch>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> BranchListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Branch>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Branch>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Branch>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Branch> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::BranchExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::BranchExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::BranchExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::BranchExpression::new(next, self.root_desc.clone())
    }
}