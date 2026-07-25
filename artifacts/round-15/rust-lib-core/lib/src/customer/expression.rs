#[derive(Clone)]
pub struct CustomerExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Customer>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> CustomerExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Customer>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Customer> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Customer> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Customer {
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
    pub fn get_employee_list(self) -> crate::EmployeeListExpression<'a> {
        let next = self.result.and_then("employee_list", |entity| entity.eval_employee_list());
        crate::EmployeeListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_truck_list(self) -> crate::TruckListExpression<'a> {
        let next = self.result.and_then("truck_list", |entity| entity.eval_truck_list());
        crate::TruckListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_inventory_item_list(self) -> crate::InventoryItemListExpression<'a> {
        let next = self.result.and_then("inventory_item_list", |entity| entity.eval_inventory_item_list());
        crate::InventoryItemListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_order_list(self) -> crate::MoveOrderListExpression<'a> {
        let next = self.result.and_then("move_order_list", |entity| entity.eval_move_order_list());
        crate::MoveOrderListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_route_list(self) -> crate::RouteListExpression<'a> {
        let next = self.result.and_then("route_list", |entity| entity.eval_route_list());
        crate::RouteListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payment_list(self) -> crate::PaymentListExpression<'a> {
        let next = self.result.and_then("payment_list", |entity| entity.eval_payment_list());
        crate::PaymentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_invoice_list(self) -> crate::InvoiceListExpression<'a> {
        let next = self.result.and_then("invoice_list", |entity| entity.eval_invoice_list());
        crate::InvoiceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_feedback_list(self) -> crate::FeedbackListExpression<'a> {
        let next = self.result.and_then("feedback_list", |entity| entity.eval_feedback_list());
        crate::FeedbackListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_schedule_list(self) -> crate::ScheduleListExpression<'a> {
        let next = self.result.and_then("schedule_list", |entity| entity.eval_schedule_list());
        crate::ScheduleListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct CustomerListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Customer>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> CustomerListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Customer>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Customer>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Customer>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Customer> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::CustomerExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::CustomerExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::CustomerExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::CustomerExpression::new(next, self.root_desc.clone())
    }
}