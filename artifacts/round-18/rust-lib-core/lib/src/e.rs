// The `E` expression wrapper provides zero-cost AST traversal
// and will automatically panic if it encounters a NotLoaded error.
pub struct E;

impl E {
    pub fn customer<'a>(value: &'a crate::Customer) -> crate::CustomerExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Customer(id={})", value.id()));
        crate::CustomerExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn address<'a>(value: &'a crate::Address) -> crate::AddressExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Address(id={})", value.id()));
        crate::AddressExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn truck<'a>(value: &'a crate::Truck) -> crate::TruckExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Truck(id={})", value.id()));
        crate::TruckExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver<'a>(value: &'a crate::Driver) -> crate::DriverExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Driver(id={})", value.id()));
        crate::DriverExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_order<'a>(value: &'a crate::MoveOrder) -> crate::MoveOrderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveOrder(id={})", value.id()));
        crate::MoveOrderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn inventory_item<'a>(value: &'a crate::InventoryItem) -> crate::InventoryItemExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InventoryItem(id={})", value.id()));
        crate::InventoryItemExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn packing_material<'a>(value: &'a crate::PackingMaterial) -> crate::PackingMaterialExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PackingMaterial(id={})", value.id()));
        crate::PackingMaterialExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn route<'a>(value: &'a crate::Route) -> crate::RouteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Route(id={})", value.id()));
        crate::RouteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn schedule<'a>(value: &'a crate::Schedule) -> crate::ScheduleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Schedule(id={})", value.id()));
        crate::ScheduleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn loading_unloading<'a>(value: &'a crate::LoadingUnloading) -> crate::LoadingUnloadingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoadingUnloading(id={})", value.id()));
        crate::LoadingUnloadingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn equipment<'a>(value: &'a crate::Equipment) -> crate::EquipmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Equipment(id={})", value.id()));
        crate::EquipmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tool<'a>(value: &'a crate::Tool) -> crate::ToolExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Tool(id={})", value.id()));
        crate::ToolExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn storage_facility<'a>(value: &'a crate::StorageFacility) -> crate::StorageFacilityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("StorageFacility(id={})", value.id()));
        crate::StorageFacilityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn warehouse<'a>(value: &'a crate::Warehouse) -> crate::WarehouseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Warehouse(id={})", value.id()));
        crate::WarehouseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn container<'a>(value: &'a crate::Container) -> crate::ContainerExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Container(id={})", value.id()));
        crate::ContainerExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn pallet<'a>(value: &'a crate::Pallet) -> crate::PalletExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Pallet(id={})", value.id()));
        crate::PalletExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn label<'a>(value: &'a crate::Label) -> crate::LabelExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Label(id={})", value.id()));
        crate::LabelExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn barcode<'a>(value: &'a crate::Barcode) -> crate::BarcodeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Barcode(id={})", value.id()));
        crate::BarcodeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tracking_number<'a>(value: &'a crate::TrackingNumber) -> crate::TrackingNumberExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TrackingNumber(id={})", value.id()));
        crate::TrackingNumberExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn notification<'a>(value: &'a crate::Notification) -> crate::NotificationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Notification(id={})", value.id()));
        crate::NotificationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payment<'a>(value: &'a crate::Payment) -> crate::PaymentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Payment(id={})", value.id()));
        crate::PaymentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn invoice<'a>(value: &'a crate::Invoice) -> crate::InvoiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Invoice(id={})", value.id()));
        crate::InvoiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn claim<'a>(value: &'a crate::Claim) -> crate::ClaimExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Claim(id={})", value.id()));
        crate::ClaimExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn feedback<'a>(value: &'a crate::Feedback) -> crate::FeedbackExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Feedback(id={})", value.id()));
        crate::FeedbackExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn employee<'a>(value: &'a crate::Employee) -> crate::EmployeeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Employee(id={})", value.id()));
        crate::EmployeeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn branch<'a>(value: &'a crate::Branch) -> crate::BranchExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Branch(id={})", value.id()));
        crate::BranchExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_maintenance<'a>(value: &'a crate::VehicleMaintenance) -> crate::VehicleMaintenanceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleMaintenance(id={})", value.id()));
        crate::VehicleMaintenanceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fuel_log<'a>(value: &'a crate::FuelLog) -> crate::FuelLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FuelLog(id={})", value.id()));
        crate::FuelLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn insurance_policy<'a>(value: &'a crate::InsurancePolicy) -> crate::InsurancePolicyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InsurancePolicy(id={})", value.id()));
        crate::InsurancePolicyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn license<'a>(value: &'a crate::License) -> crate::LicenseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("License(id={})", value.id()));
        crate::LicenseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn permit<'a>(value: &'a crate::Permit) -> crate::PermitExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Permit(id={})", value.id()));
        crate::PermitExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customs_document<'a>(value: &'a crate::CustomsDocument) -> crate::CustomsDocumentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomsDocument(id={})", value.id()));
        crate::CustomsDocumentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn communication_log<'a>(value: &'a crate::CommunicationLog) -> crate::CommunicationLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CommunicationLog(id={})", value.id()));
        crate::CommunicationLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn audit_trail<'a>(value: &'a crate::AuditTrail) -> crate::AuditTrailExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AuditTrail(id={})", value.id()));
        crate::AuditTrailExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn report<'a>(value: &'a crate::Report) -> crate::ReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Report(id={})", value.id()));
        crate::ReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn dashboard<'a>(value: &'a crate::Dashboard) -> crate::DashboardExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Dashboard(id={})", value.id()));
        crate::DashboardExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn settings<'a>(value: &'a crate::Settings) -> crate::SettingsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Settings(id={})", value.id()));
        crate::SettingsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn user_role<'a>(value: &'a crate::UserRole) -> crate::UserRoleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UserRole(id={})", value.id()));
        crate::UserRoleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn permission<'a>(value: &'a crate::Permission) -> crate::PermissionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Permission(id={})", value.id()));
        crate::PermissionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn api_key<'a>(value: &'a crate::ApiKey) -> crate::ApiKeyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ApiKey(id={})", value.id()));
        crate::ApiKeyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

