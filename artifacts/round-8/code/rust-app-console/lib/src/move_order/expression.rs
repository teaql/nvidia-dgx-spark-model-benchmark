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

    pub fn get_order_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("order_id", |entity| entity.eval_order_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_status(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("status", |entity| entity.eval_status());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_merchant_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_ref_id", |entity| entity.eval_merchant_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("customer_ref_id", |entity| entity.eval_customer_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_merchant_ref(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant_ref", |entity| entity.eval_merchant_ref());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_ref(self) -> crate::CustomerExpression<'a> {
        let next = self.result.and_then("customer_ref", |entity| entity.eval_customer_ref());
        crate::CustomerExpression::new(next, self.root_desc.clone())
    }
    pub fn get_move_quote_list(self) -> crate::MoveQuoteListExpression<'a> {
        let next = self.result.and_then("move_quote_list", |entity| entity.eval_move_quote_list());
        crate::MoveQuoteListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_route_list(self) -> crate::RouteListExpression<'a> {
        let next = self.result.and_then("route_list", |entity| entity.eval_route_list());
        crate::RouteListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_time_slot_list(self) -> crate::TimeSlotListExpression<'a> {
        let next = self.result.and_then("time_slot_list", |entity| entity.eval_time_slot_list());
        crate::TimeSlotListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_fulfillment_event_list(self) -> crate::FulfillmentEventListExpression<'a> {
        let next = self.result.and_then("fulfillment_event_list", |entity| entity.eval_fulfillment_event_list());
        crate::FulfillmentEventListExpression::new(next, self.root_desc.clone())
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

    pub fn get_move_item_list(self) -> crate::MoveItemListExpression<'a> {
        let next = self.result.and_then("move_item_list", |entity| entity.eval_move_item_list());
        crate::MoveItemListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_inventory_list_list(self) -> crate::InventoryListListExpression<'a> {
        let next = self.result.and_then("inventory_list_list", |entity| entity.eval_inventory_list_list());
        crate::InventoryListListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_transit_log_list(self) -> crate::TransitLogListExpression<'a> {
        let next = self.result.and_then("transit_log_list", |entity| entity.eval_transit_log_list());
        crate::TransitLogListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_delay_record_list(self) -> crate::DelayRecordListExpression<'a> {
        let next = self.result.and_then("delay_record_list", |entity| entity.eval_delay_record_list());
        crate::DelayRecordListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_vehicle_assignment_list(self) -> crate::VehicleAssignmentListExpression<'a> {
        let next = self.result.and_then("vehicle_assignment_list", |entity| entity.eval_vehicle_assignment_list());
        crate::VehicleAssignmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_cargo_weight_record_list(self) -> crate::CargoWeightRecordListExpression<'a> {
        let next = self.result.and_then("cargo_weight_record_list", |entity| entity.eval_cargo_weight_record_list());
        crate::CargoWeightRecordListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_special_handling_instruction_list(self) -> crate::SpecialHandlingInstructionListExpression<'a> {
        let next = self.result.and_then("special_handling_instruction_list", |entity| entity.eval_special_handling_instruction_list());
        crate::SpecialHandlingInstructionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_delivery_window_list(self) -> crate::DeliveryWindowListExpression<'a> {
        let next = self.result.and_then("delivery_window_list", |entity| entity.eval_delivery_window_list());
        crate::DeliveryWindowListExpression::new(next, self.root_desc.clone())
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