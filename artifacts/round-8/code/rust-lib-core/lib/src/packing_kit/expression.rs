#[derive(Clone)]
pub struct PackingKitExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::PackingKit>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PackingKitExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::PackingKit>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::PackingKit> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::PackingKit> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::PackingKit {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_kit_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("kit_id", |entity| entity.eval_kit_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_contents(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("contents", |entity| entity.eval_contents());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_product_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("product_ref_id", |entity| entity.eval_product_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_product_ref(self) -> crate::ProductExpression<'a> {
        let next = self.result.and_then("product_ref", |entity| entity.eval_product_ref());
        crate::ProductExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct PackingKitListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::PackingKit>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PackingKitListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::PackingKit>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::PackingKit>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::PackingKit>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::PackingKit> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::PackingKitExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PackingKitExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::PackingKitExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PackingKitExpression::new(next, self.root_desc.clone())
    }
}