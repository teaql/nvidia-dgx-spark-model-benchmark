#[derive(Clone)]
pub struct MoveInventoryExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::MoveInventory>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MoveInventoryExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::MoveInventory>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::MoveInventory> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::MoveInventory> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::MoveInventory {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_item_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("item_name", |entity| entity.eval_item_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_create_time(self) -> crate::ValueExpression<'a, chrono::DateTime<chrono::Utc>> {
        let next = self.result.and_then("create_time", |entity| entity.eval_create_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_update_time(self) -> crate::ValueExpression<'a, chrono::DateTime<chrono::Utc>> {
        let next = self.result.and_then("update_time", |entity| entity.eval_update_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_condition_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("condition_id", |entity| entity.eval_condition_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_order_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("move_order_id", |entity| entity.eval_move_order_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_id", |entity| entity.eval_merchant_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_condition(self) -> crate::InventoryConditionTypeExpression<'a> {
        let next = self.result.and_then("condition", |entity| entity.eval_condition());
        crate::InventoryConditionTypeExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_order(self) -> crate::MoveOrderExpression<'a> {
        let next = self.result.and_then("move_order", |entity| entity.eval_move_order());
        crate::MoveOrderExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant", |entity| entity.eval_merchant());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn condition_is_new(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("condition_id", |entity| {
            if !entity.is_loaded("condition_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "condition_id".to_string(), attempted_path: "condition_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.condition_is_new())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn condition_is_used(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("condition_id", |entity| {
            if !entity.is_loaded("condition_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "condition_id".to_string(), attempted_path: "condition_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.condition_is_used())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn condition_is_damaged(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("condition_id", |entity| {
            if !entity.is_loaded("condition_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "condition_id".to_string(), attempted_path: "condition_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.condition_is_damaged())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct MoveInventoryListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MoveInventory>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MoveInventoryListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MoveInventory>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::MoveInventory>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::MoveInventory>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::MoveInventory> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::MoveInventoryExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MoveInventoryExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::MoveInventoryExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MoveInventoryExpression::new(next, self.root_desc.clone())
    }
}