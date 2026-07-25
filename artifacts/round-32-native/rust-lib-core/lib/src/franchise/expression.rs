#[derive(Clone)]
pub struct FranchiseExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Franchise>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> FranchiseExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Franchise>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Franchise> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Franchise> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Franchise {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_franchise_code(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("franchise_code", |entity| entity.eval_franchise_code());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_territory_code(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("territory_code", |entity| entity.eval_territory_code());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_royalty_rate(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("royalty_rate", |entity| entity.eval_royalty_rate());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_status(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("status", |entity| entity.eval_status());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_contact_email(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("contact_email", |entity| entity.eval_contact_email());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct FranchiseListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Franchise>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> FranchiseListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Franchise>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Franchise>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Franchise>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Franchise> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::FranchiseExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::FranchiseExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::FranchiseExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::FranchiseExpression::new(next, self.root_desc.clone())
    }
}