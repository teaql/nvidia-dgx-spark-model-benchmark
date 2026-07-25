#[derive(Clone)]
pub struct InventoryItemExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::InventoryItem>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InventoryItemExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::InventoryItem>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::InventoryItem> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::InventoryItem> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::InventoryItem {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_customer_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("customer_id", |entity| entity.eval_customer_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_customer(self) -> crate::CustomerExpression<'a> {
        let next = self.result.and_then("customer", |entity| entity.eval_customer());
        crate::CustomerExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct InventoryItemListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::InventoryItem>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InventoryItemListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::InventoryItem>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::InventoryItem>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::InventoryItem>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::InventoryItem> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::InventoryItemExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InventoryItemExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::InventoryItemExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InventoryItemExpression::new(next, self.root_desc.clone())
    }
}