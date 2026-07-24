#[derive(Clone)]
pub struct MoveOrderExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::MoveOrder>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MoveOrderExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::MoveOrder>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::MoveOrder> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::MoveOrder> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::MoveOrder {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_order_number(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("order_number", |entity| entity.eval_order_number());
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
    pub fn get_status_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("status_id", |entity| entity.eval_status_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_quote_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("quote_id", |entity| entity.eval_quote_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_id", |entity| entity.eval_merchant_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_status(self) -> crate::OrderStatusExpression<'a> {
        let next = self.result.and_then("status", |entity| entity.eval_status());
        crate::OrderStatusExpression::new(next, self.root_desc.clone())
    }

    pub fn get_quote(self) -> crate::MoveQuoteExpression<'a> {
        let next = self.result.and_then("quote", |entity| entity.eval_quote());
        crate::MoveQuoteExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant", |entity| entity.eval_merchant());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn status_is_draft(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("status_id", |entity| {
            if !entity.is_loaded("status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.status_is_draft())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn status_is_confirmed(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("status_id", |entity| {
            if !entity.is_loaded("status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.status_is_confirmed())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn status_is_completed(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("status_id", |entity| {
            if !entity.is_loaded("status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.status_is_completed())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_route_stop_list(self) -> crate::RouteStopListExpression<'a> {
        let next = self.result.and_then("route_stop_list", |entity| entity.eval_route_stop_list());
        crate::RouteStopListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_dispatch_assignment_list(self) -> crate::DispatchAssignmentListExpression<'a> {
        let next = self.result.and_then("dispatch_assignment_list", |entity| entity.eval_dispatch_assignment_list());
        crate::DispatchAssignmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_damage_report_list(self) -> crate::DamageReportListExpression<'a> {
        let next = self.result.and_then("damage_report_list", |entity| entity.eval_damage_report_list());
        crate::DamageReportListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_proof_of_delivery_list(self) -> crate::ProofOfDeliveryListExpression<'a> {
        let next = self.result.and_then("proof_of_delivery_list", |entity| entity.eval_proof_of_delivery_list());
        crate::ProofOfDeliveryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_operational_exception_list(self) -> crate::OperationalExceptionListExpression<'a> {
        let next = self.result.and_then("operational_exception_list", |entity| entity.eval_operational_exception_list());
        crate::OperationalExceptionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_inventory_list(self) -> crate::MoveInventoryListExpression<'a> {
        let next = self.result.and_then("move_inventory_list", |entity| entity.eval_move_inventory_list());
        crate::MoveInventoryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_packaging_item_list(self) -> crate::PackagingItemListExpression<'a> {
        let next = self.result.and_then("packaging_item_list", |entity| entity.eval_packaging_item_list());
        crate::PackagingItemListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_third_party_dispatch_list(self) -> crate::ThirdPartyDispatchListExpression<'a> {
        let next = self.result.and_then("third_party_dispatch_list", |entity| entity.eval_third_party_dispatch_list());
        crate::ThirdPartyDispatchListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_feedback_list(self) -> crate::CustomerFeedbackListExpression<'a> {
        let next = self.result.and_then("customer_feedback_list", |entity| entity.eval_customer_feedback_list());
        crate::CustomerFeedbackListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct MoveOrderListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MoveOrder>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MoveOrderListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MoveOrder>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::MoveOrder>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::MoveOrder>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::MoveOrder> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::MoveOrderExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MoveOrderExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::MoveOrderExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MoveOrderExpression::new(next, self.root_desc.clone())
    }
}