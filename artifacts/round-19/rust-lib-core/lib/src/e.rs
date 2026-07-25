// The `E` expression wrapper provides zero-cost AST traversal
// and will automatically panic if it encounters a NotLoaded error.
pub struct E;

impl E {
    pub fn trucks<'a>(value: &'a crate::Trucks) -> crate::TrucksExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Trucks(id={})", value.id()));
        crate::TrucksExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicles<'a>(value: &'a crate::Vehicles) -> crate::VehiclesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Vehicles(id={})", value.id()));
        crate::VehiclesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn drivers<'a>(value: &'a crate::Drivers) -> crate::DriversExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Drivers(id={})", value.id()));
        crate::DriversExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn routes<'a>(value: &'a crate::Routes) -> crate::RoutesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Routes(id={})", value.id()));
        crate::RoutesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn locations<'a>(value: &'a crate::Locations) -> crate::LocationsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Locations(id={})", value.id()));
        crate::LocationsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn addresses<'a>(value: &'a crate::Addresses) -> crate::AddressesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Addresses(id={})", value.id()));
        crate::AddressesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn dispatches<'a>(value: &'a crate::Dispatches) -> crate::DispatchesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Dispatches(id={})", value.id()));
        crate::DispatchesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn jobs<'a>(value: &'a crate::Jobs) -> crate::JobsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Jobs(id={})", value.id()));
        crate::JobsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn schedules<'a>(value: &'a crate::Schedules) -> crate::SchedulesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Schedules(id={})", value.id()));
        crate::SchedulesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn shifts<'a>(value: &'a crate::Shifts) -> crate::ShiftsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Shifts(id={})", value.id()));
        crate::ShiftsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn timesheets<'a>(value: &'a crate::Timesheets) -> crate::TimesheetsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Timesheets(id={})", value.id()));
        crate::TimesheetsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tracking<'a>(value: &'a crate::Tracking) -> crate::TrackingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Tracking(id={})", value.id()));
        crate::TrackingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn geofence<'a>(value: &'a crate::Geofence) -> crate::GeofenceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Geofence(id={})", value.id()));
        crate::GeofenceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fuel<'a>(value: &'a crate::Fuel) -> crate::FuelExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Fuel(id={})", value.id()));
        crate::FuelExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn maintenance<'a>(value: &'a crate::Maintenance) -> crate::MaintenanceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Maintenance(id={})", value.id()));
        crate::MaintenanceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn repairs<'a>(value: &'a crate::Repairs) -> crate::RepairsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Repairs(id={})", value.id()));
        crate::RepairsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn inspections<'a>(value: &'a crate::Inspections) -> crate::InspectionsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Inspections(id={})", value.id()));
        crate::InspectionsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn equipment<'a>(value: &'a crate::Equipment) -> crate::EquipmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Equipment(id={})", value.id()));
        crate::EquipmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn warehouse<'a>(value: &'a crate::Warehouse) -> crate::WarehouseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Warehouse(id={})", value.id()));
        crate::WarehouseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn inventory<'a>(value: &'a crate::Inventory) -> crate::InventoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Inventory(id={})", value.id()));
        crate::InventoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn invoices<'a>(value: &'a crate::Invoices) -> crate::InvoicesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Invoices(id={})", value.id()));
        crate::InvoicesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payments<'a>(value: &'a crate::Payments) -> crate::PaymentsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Payments(id={})", value.id()));
        crate::PaymentsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn expenses<'a>(value: &'a crate::Expenses) -> crate::ExpensesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Expenses(id={})", value.id()));
        crate::ExpensesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn accounts<'a>(value: &'a crate::Accounts) -> crate::AccountsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Accounts(id={})", value.id()));
        crate::AccountsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn ledgers<'a>(value: &'a crate::Ledgers) -> crate::LedgersExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Ledgers(id={})", value.id()));
        crate::LedgersExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn taxes<'a>(value: &'a crate::Taxes) -> crate::TaxesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Taxes(id={})", value.id()));
        crate::TaxesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn quotes<'a>(value: &'a crate::Quotes) -> crate::QuotesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Quotes(id={})", value.id()));
        crate::QuotesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn estimates<'a>(value: &'a crate::Estimates) -> crate::EstimatesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Estimates(id={})", value.id()));
        crate::EstimatesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn audit<'a>(value: &'a crate::Audit) -> crate::AuditExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Audit(id={})", value.id()));
        crate::AuditExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn security<'a>(value: &'a crate::Security) -> crate::SecurityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Security(id={})", value.id()));
        crate::SecurityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn budget<'a>(value: &'a crate::Budget) -> crate::BudgetExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Budget(id={})", value.id()));
        crate::BudgetExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payroll<'a>(value: &'a crate::Payroll) -> crate::PayrollExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Payroll(id={})", value.id()));
        crate::PayrollExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn reimbursements<'a>(value: &'a crate::Reimbursements) -> crate::ReimbursementsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Reimbursements(id={})", value.id()));
        crate::ReimbursementsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn financial_reports<'a>(value: &'a crate::FinancialReports) -> crate::FinancialReportsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FinancialReports(id={})", value.id()));
        crate::FinancialReportsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn cash_flow<'a>(value: &'a crate::CashFlow) -> crate::CashFlowExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CashFlow(id={})", value.id()));
        crate::CashFlowExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customers<'a>(value: &'a crate::Customers) -> crate::CustomersExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Customers(id={})", value.id()));
        crate::CustomersExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn employees<'a>(value: &'a crate::Employees) -> crate::EmployeesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Employees(id={})", value.id()));
        crate::EmployeesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn contacts<'a>(value: &'a crate::Contacts) -> crate::ContactsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Contacts(id={})", value.id()));
        crate::ContactsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn documents<'a>(value: &'a crate::Documents) -> crate::DocumentsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Documents(id={})", value.id()));
        crate::DocumentsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn contracts<'a>(value: &'a crate::Contracts) -> crate::ContractsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Contracts(id={})", value.id()));
        crate::ContractsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn signatures<'a>(value: &'a crate::Signatures) -> crate::SignaturesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Signatures(id={})", value.id()));
        crate::SignaturesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn feedback<'a>(value: &'a crate::Feedback) -> crate::FeedbackExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Feedback(id={})", value.id()));
        crate::FeedbackExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn reviews<'a>(value: &'a crate::Reviews) -> crate::ReviewsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Reviews(id={})", value.id()));
        crate::ReviewsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn ratings<'a>(value: &'a crate::Ratings) -> crate::RatingsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Ratings(id={})", value.id()));
        crate::RatingsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn notifications<'a>(value: &'a crate::Notifications) -> crate::NotificationsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Notifications(id={})", value.id()));
        crate::NotificationsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn alerts<'a>(value: &'a crate::Alerts) -> crate::AlertsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Alerts(id={})", value.id()));
        crate::AlertsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn calendars<'a>(value: &'a crate::Calendars) -> crate::CalendarsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Calendars(id={})", value.id()));
        crate::CalendarsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn users<'a>(value: &'a crate::Users) -> crate::UsersExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Users(id={})", value.id()));
        crate::UsersExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn roles<'a>(value: &'a crate::Roles) -> crate::RolesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Roles(id={})", value.id()));
        crate::RolesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn permissions<'a>(value: &'a crate::Permissions) -> crate::PermissionsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Permissions(id={})", value.id()));
        crate::PermissionsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

