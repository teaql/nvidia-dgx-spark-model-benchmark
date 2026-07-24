// The `E` expression wrapper provides zero-cost AST traversal
// and will automatically panic if it encounters a NotLoaded error.
pub struct E;

impl E {
    pub fn route_status_type<'a>(value: &'a crate::RouteStatusType) -> crate::RouteStatusTypeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RouteStatusType(id={})", value.id()));
        crate::RouteStatusTypeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn inventory_condition_type<'a>(value: &'a crate::InventoryConditionType) -> crate::InventoryConditionTypeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InventoryConditionType(id={})", value.id()));
        crate::InventoryConditionTypeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn exception_severity<'a>(value: &'a crate::ExceptionSeverity) -> crate::ExceptionSeverityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExceptionSeverity(id={})", value.id()));
        crate::ExceptionSeverityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn order_status<'a>(value: &'a crate::OrderStatus) -> crate::OrderStatusExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OrderStatus(id={})", value.id()));
        crate::OrderStatusExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn crew_role<'a>(value: &'a crate::CrewRole) -> crate::CrewRoleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CrewRole(id={})", value.id()));
        crate::CrewRoleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn platform<'a>(value: &'a crate::Platform) -> crate::PlatformExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Platform(id={})", value.id()));
        crate::PlatformExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn merchant<'a>(value: &'a crate::Merchant) -> crate::MerchantExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Merchant(id={})", value.id()));
        crate::MerchantExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_quote<'a>(value: &'a crate::MoveQuote) -> crate::MoveQuoteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveQuote(id={})", value.id()));
        crate::MoveQuoteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_order<'a>(value: &'a crate::MoveOrder) -> crate::MoveOrderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveOrder(id={})", value.id()));
        crate::MoveOrderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn route_stop<'a>(value: &'a crate::RouteStop) -> crate::RouteStopExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RouteStop(id={})", value.id()));
        crate::RouteStopExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn crew<'a>(value: &'a crate::Crew) -> crate::CrewExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Crew(id={})", value.id()));
        crate::CrewExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn crew_member_assignment<'a>(value: &'a crate::CrewMemberAssignment) -> crate::CrewMemberAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CrewMemberAssignment(id={})", value.id()));
        crate::CrewMemberAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle<'a>(value: &'a crate::Vehicle) -> crate::VehicleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Vehicle(id={})", value.id()));
        crate::VehicleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_assignment<'a>(value: &'a crate::VehicleAssignment) -> crate::VehicleAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleAssignment(id={})", value.id()));
        crate::VehicleAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn dispatch_assignment<'a>(value: &'a crate::DispatchAssignment) -> crate::DispatchAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DispatchAssignment(id={})", value.id()));
        crate::DispatchAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn damage_report<'a>(value: &'a crate::DamageReport) -> crate::DamageReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DamageReport(id={})", value.id()));
        crate::DamageReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn proof_of_delivery<'a>(value: &'a crate::ProofOfDelivery) -> crate::ProofOfDeliveryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ProofOfDelivery(id={})", value.id()));
        crate::ProofOfDeliveryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn operational_exception<'a>(value: &'a crate::OperationalException) -> crate::OperationalExceptionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OperationalException(id={})", value.id()));
        crate::OperationalExceptionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn pickup_instruction<'a>(value: &'a crate::PickupInstruction) -> crate::PickupInstructionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PickupInstruction(id={})", value.id()));
        crate::PickupInstructionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn delivery_instruction<'a>(value: &'a crate::DeliveryInstruction) -> crate::DeliveryInstructionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DeliveryInstruction(id={})", value.id()));
        crate::DeliveryInstructionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_inventory<'a>(value: &'a crate::MoveInventory) -> crate::MoveInventoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveInventory(id={})", value.id()));
        crate::MoveInventoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn packaging_item<'a>(value: &'a crate::PackagingItem) -> crate::PackagingItemExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PackagingItem(id={})", value.id()));
        crate::PackagingItemExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn logistics_provider<'a>(value: &'a crate::LogisticsProvider) -> crate::LogisticsProviderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LogisticsProvider(id={})", value.id()));
        crate::LogisticsProviderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn third_party_dispatch<'a>(value: &'a crate::ThirdPartyDispatch) -> crate::ThirdPartyDispatchExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ThirdPartyDispatch(id={})", value.id()));
        crate::ThirdPartyDispatchExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fuel_log<'a>(value: &'a crate::FuelLog) -> crate::FuelLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FuelLog(id={})", value.id()));
        crate::FuelLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn maintenance_record<'a>(value: &'a crate::MaintenanceRecord) -> crate::MaintenanceRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MaintenanceRecord(id={})", value.id()));
        crate::MaintenanceRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn toll_receipt<'a>(value: &'a crate::TollReceipt) -> crate::TollReceiptExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TollReceipt(id={})", value.id()));
        crate::TollReceiptExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn shift_log<'a>(value: &'a crate::ShiftLog) -> crate::ShiftLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ShiftLog(id={})", value.id()));
        crate::ShiftLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_feedback<'a>(value: &'a crate::CustomerFeedback) -> crate::CustomerFeedbackExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerFeedback(id={})", value.id()));
        crate::CustomerFeedbackExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn incident_report<'a>(value: &'a crate::IncidentReport) -> crate::IncidentReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("IncidentReport(id={})", value.id()));
        crate::IncidentReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }
}


pub fn trigger_logic_bug_panic(root_desc: &str, failed_node: &str, attempted_path: &str) -> ! {
    let parts: Vec<&str> = attempted_path.split('.').collect();
    let break_idx = parts.iter().position(|&p| p == failed_node).unwrap_or(0);

    let mut nested_fix = String::new();
    if break_idx < parts.len() - 1 {
        nested_fix.push_str(&format!("\"select_{}(", failed_node));
        let mut close_parens = 1;
        for i in (break_idx + 1)..parts.len() {
            let sub_field = parts[i];
            let prev_field = parts[i-1];
            let is_last = i == parts.len() - 1;
            if is_last {
                nested_fix.push_str(&format!("Q::{}s().select_{}()", prev_field, sub_field));
            } else {
                nested_fix.push_str(&format!("Q::{}s().select_{}(", prev_field, sub_field));
                close_parens += 1;
            }
        }
        for _ in 0..close_parens {
            nested_fix.push(')');
        }
        nested_fix.push('"');
    } else {
        nested_fix = "null".to_string();
    }

    let suggested_fix = format!("\"select_{}()\"", failed_node);

    let access_path_json = format!("[{}]", parts.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(", "));
    let missing_preload_json = format!("[\"{}\"]", failed_node);

    let human_nested = if nested_fix != "null" { format!(" 或完整嵌套加载 {}", nested_fix) } else { String::new() };
    let root_name = root_desc.split('(').next().unwrap_or("Unknown");

    let mut root_snake = String::new();
    for (i, c) in root_name.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                root_snake.push('_');
            }
            root_snake.push(c.to_ascii_lowercase());
        } else {
            root_snake.push(c);
        }
    }
    let id_part = root_desc.split('(').nth(1).unwrap_or(")");
    let mut original_expr = format!("E::{}({}", root_snake, id_part);
    for p in &parts {
        original_expr.push_str(&format!(".get_{}()", p));
        if *p == failed_node {
            original_expr.push_str("<broken>");
        }
    }

    let human_message = format!("\"访问 {}.{} 时缺少预加载。请在查询中加入 {}{}\"", root_name, attempted_path, suggested_fix, human_nested);

    panic!("\n\n💥 [Coding Logic Bug]\n\noriginal_expr_with_broken_point: \"{}\"\nroot: {}\naccess_path: {}\nbreak_point: \"{}\"\nmissing_preload: {}\nsuggested_fix: {}\nnested_fix: {}\nseverity: \"error\"\nhuman_message: {}\n", 
        original_expr, root_desc, access_path_json, failed_node, missing_preload_json, suggested_fix, nested_fix, human_message);
}

#[derive(Clone)]
pub struct ValueExpression<'a, T> {
    result: teaql_core::eval::EvalResult<T>,
    root_desc: std::sync::Arc<String>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, T: Clone> ValueExpression<'a, T> {
    pub fn new(result: teaql_core::eval::EvalResult<T>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc, _phantom: std::marker::PhantomData }
    }

    fn resolve(self) -> Option<T> {
        match self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(self) -> Option<T> {
        self.resolve()
    }

    pub fn unwrap(self) -> T {
        self.resolve().expect("Value was legitimately null in database!")
    }

    pub fn or_else(self, default_value: T) -> T {
        self.eval().unwrap_or(default_value)
    }

    pub fn or_else_with(self, default_fn: impl FnOnce() -> T) -> T {
        self.eval().unwrap_or_else(default_fn)
    }

    pub fn or_default(self) -> T where T: Default {
        self.eval().unwrap_or_default()
    }
}

