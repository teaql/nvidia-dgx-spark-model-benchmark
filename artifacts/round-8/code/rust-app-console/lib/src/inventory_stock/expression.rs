#[derive(Clone)]
pub struct InventoryStockExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::InventoryStock>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InventoryStockExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::InventoryStock>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::InventoryStock> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::InventoryStock> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::InventoryStock {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_stock_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("stock_id", |entity| entity.eval_stock_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_quantity(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("quantity", |entity| entity.eval_quantity());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_consumable_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("consumable_ref_id", |entity| entity.eval_consumable_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_consumable_ref(self) -> crate::ConsumableExpression<'a> {
        let next = self.result.and_then("consumable_ref", |entity| entity.eval_consumable_ref());
        crate::ConsumableExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct InventoryStockListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::InventoryStock>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InventoryStockListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::InventoryStock>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::InventoryStock>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::InventoryStock>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::InventoryStock> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::InventoryStockExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InventoryStockExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::InventoryStockExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InventoryStockExpression::new(next, self.root_desc.clone())
    }
}