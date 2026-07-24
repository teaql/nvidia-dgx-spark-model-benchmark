#[derive(Clone)]
pub struct MaintenanceCostExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::MaintenanceCost>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MaintenanceCostExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::MaintenanceCost>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::MaintenanceCost> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::MaintenanceCost> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::MaintenanceCost {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_cost_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("cost_id", |entity| entity.eval_cost_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_amount(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("amount", |entity| entity.eval_amount());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_maintenance_event_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("maintenance_event_ref_id", |entity| entity.eval_maintenance_event_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_maintenance_event_ref(self) -> crate::MaintenanceEventExpression<'a> {
        let next = self.result.and_then("maintenance_event_ref", |entity| entity.eval_maintenance_event_ref());
        crate::MaintenanceEventExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct MaintenanceCostListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MaintenanceCost>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MaintenanceCostListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MaintenanceCost>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::MaintenanceCost>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::MaintenanceCost>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::MaintenanceCost> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::MaintenanceCostExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MaintenanceCostExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::MaintenanceCostExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MaintenanceCostExpression::new(next, self.root_desc.clone())
    }
}