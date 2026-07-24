#[derive(Clone)]
pub struct MerchantExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Merchant>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MerchantExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Merchant>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Merchant> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Merchant> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Merchant {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tax_number(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("tax_number", |entity| entity.eval_tax_number());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_address(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("address", |entity| entity.eval_address());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_external_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("external_id", |entity| entity.eval_external_id());
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
    pub fn get_platform_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("platform_id", |entity| entity.eval_platform_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_platform(self) -> crate::PlatformExpression<'a> {
        let next = self.result.and_then("platform", |entity| entity.eval_platform());
        crate::PlatformExpression::new(next, self.root_desc.clone())
    }
    pub fn get_move_quote_list(self) -> crate::MoveQuoteListExpression<'a> {
        let next = self.result.and_then("move_quote_list", |entity| entity.eval_move_quote_list());
        crate::MoveQuoteListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_order_list(self) -> crate::MoveOrderListExpression<'a> {
        let next = self.result.and_then("move_order_list", |entity| entity.eval_move_order_list());
        crate::MoveOrderListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_route_stop_list(self) -> crate::RouteStopListExpression<'a> {
        let next = self.result.and_then("route_stop_list", |entity| entity.eval_route_stop_list());
        crate::RouteStopListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_crew_list(self) -> crate::CrewListExpression<'a> {
        let next = self.result.and_then("crew_list", |entity| entity.eval_crew_list());
        crate::CrewListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_crew_member_assignment_list(self) -> crate::CrewMemberAssignmentListExpression<'a> {
        let next = self.result.and_then("crew_member_assignment_list", |entity| entity.eval_crew_member_assignment_list());
        crate::CrewMemberAssignmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_vehicle_list(self) -> crate::VehicleListExpression<'a> {
        let next = self.result.and_then("vehicle_list", |entity| entity.eval_vehicle_list());
        crate::VehicleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_vehicle_assignment_list(self) -> crate::VehicleAssignmentListExpression<'a> {
        let next = self.result.and_then("vehicle_assignment_list", |entity| entity.eval_vehicle_assignment_list());
        crate::VehicleAssignmentListExpression::new(next, self.root_desc.clone())
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

    pub fn get_pickup_instruction_list(self) -> crate::PickupInstructionListExpression<'a> {
        let next = self.result.and_then("pickup_instruction_list", |entity| entity.eval_pickup_instruction_list());
        crate::PickupInstructionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_delivery_instruction_list(self) -> crate::DeliveryInstructionListExpression<'a> {
        let next = self.result.and_then("delivery_instruction_list", |entity| entity.eval_delivery_instruction_list());
        crate::DeliveryInstructionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_inventory_list(self) -> crate::MoveInventoryListExpression<'a> {
        let next = self.result.and_then("move_inventory_list", |entity| entity.eval_move_inventory_list());
        crate::MoveInventoryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_packaging_item_list(self) -> crate::PackagingItemListExpression<'a> {
        let next = self.result.and_then("packaging_item_list", |entity| entity.eval_packaging_item_list());
        crate::PackagingItemListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_logistics_provider_list(self) -> crate::LogisticsProviderListExpression<'a> {
        let next = self.result.and_then("logistics_provider_list", |entity| entity.eval_logistics_provider_list());
        crate::LogisticsProviderListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_third_party_dispatch_list(self) -> crate::ThirdPartyDispatchListExpression<'a> {
        let next = self.result.and_then("third_party_dispatch_list", |entity| entity.eval_third_party_dispatch_list());
        crate::ThirdPartyDispatchListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_fuel_log_list(self) -> crate::FuelLogListExpression<'a> {
        let next = self.result.and_then("fuel_log_list", |entity| entity.eval_fuel_log_list());
        crate::FuelLogListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_maintenance_record_list(self) -> crate::MaintenanceRecordListExpression<'a> {
        let next = self.result.and_then("maintenance_record_list", |entity| entity.eval_maintenance_record_list());
        crate::MaintenanceRecordListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_toll_receipt_list(self) -> crate::TollReceiptListExpression<'a> {
        let next = self.result.and_then("toll_receipt_list", |entity| entity.eval_toll_receipt_list());
        crate::TollReceiptListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_shift_log_list(self) -> crate::ShiftLogListExpression<'a> {
        let next = self.result.and_then("shift_log_list", |entity| entity.eval_shift_log_list());
        crate::ShiftLogListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_feedback_list(self) -> crate::CustomerFeedbackListExpression<'a> {
        let next = self.result.and_then("customer_feedback_list", |entity| entity.eval_customer_feedback_list());
        crate::CustomerFeedbackListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_incident_report_list(self) -> crate::IncidentReportListExpression<'a> {
        let next = self.result.and_then("incident_report_list", |entity| entity.eval_incident_report_list());
        crate::IncidentReportListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct MerchantListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Merchant>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MerchantListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Merchant>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Merchant>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Merchant>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Merchant> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::MerchantExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::MerchantExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
}