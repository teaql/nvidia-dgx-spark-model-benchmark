// The `E` expression wrapper provides zero-cost AST traversal
// and will automatically panic if it encounters a NotLoaded error.
pub struct E;

impl E {
    pub fn customer<'a>(value: &'a crate::Customer) -> crate::CustomerExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Customer(id={})", value.id()));
        crate::CustomerExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn lead<'a>(value: &'a crate::Lead) -> crate::LeadExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Lead(id={})", value.id()));
        crate::LeadExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn quote<'a>(value: &'a crate::Quote) -> crate::QuoteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Quote(id={})", value.id()));
        crate::QuoteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn contract<'a>(value: &'a crate::Contract) -> crate::ContractExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Contract(id={})", value.id()));
        crate::ContractExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn invoice<'a>(value: &'a crate::Invoice) -> crate::InvoiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Invoice(id={})", value.id()));
        crate::InvoiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payment<'a>(value: &'a crate::Payment) -> crate::PaymentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Payment(id={})", value.id()));
        crate::PaymentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn sales_order<'a>(value: &'a crate::SalesOrder) -> crate::SalesOrderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SalesOrder(id={})", value.id()));
        crate::SalesOrderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn sales_rep<'a>(value: &'a crate::SalesRep) -> crate::SalesRepExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SalesRep(id={})", value.id()));
        crate::SalesRepExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn territory<'a>(value: &'a crate::Territory) -> crate::TerritoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Territory(id={})", value.id()));
        crate::TerritoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn pricing<'a>(value: &'a crate::Pricing) -> crate::PricingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Pricing(id={})", value.id()));
        crate::PricingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn discount<'a>(value: &'a crate::Discount) -> crate::DiscountExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Discount(id={})", value.id()));
        crate::DiscountExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn promotion<'a>(value: &'a crate::Promotion) -> crate::PromotionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Promotion(id={})", value.id()));
        crate::PromotionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn campaign<'a>(value: &'a crate::Campaign) -> crate::CampaignExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Campaign(id={})", value.id()));
        crate::CampaignExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn feedback<'a>(value: &'a crate::Feedback) -> crate::FeedbackExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Feedback(id={})", value.id()));
        crate::FeedbackExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn complaint<'a>(value: &'a crate::Complaint) -> crate::ComplaintExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Complaint(id={})", value.id()));
        crate::ComplaintExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_request<'a>(value: &'a crate::ServiceRequest) -> crate::ServiceRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceRequest(id={})", value.id()));
        crate::ServiceRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn warranty<'a>(value: &'a crate::Warranty) -> crate::WarrantyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Warranty(id={})", value.id()));
        crate::WarrantyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn renewal<'a>(value: &'a crate::Renewal) -> crate::RenewalExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Renewal(id={})", value.id()));
        crate::RenewalExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn upsell<'a>(value: &'a crate::Upsell) -> crate::UpsellExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Upsell(id={})", value.id()));
        crate::UpsellExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn cross_sell<'a>(value: &'a crate::CrossSell) -> crate::CrossSellExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CrossSell(id={})", value.id()));
        crate::CrossSellExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn truck<'a>(value: &'a crate::Truck) -> crate::TruckExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Truck(id={})", value.id()));
        crate::TruckExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn trailer<'a>(value: &'a crate::Trailer) -> crate::TrailerExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Trailer(id={})", value.id()));
        crate::TrailerExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver<'a>(value: &'a crate::Driver) -> crate::DriverExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Driver(id={})", value.id()));
        crate::DriverExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_order<'a>(value: &'a crate::MoveOrder) -> crate::MoveOrderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveOrder(id={})", value.id()));
        crate::MoveOrderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn route<'a>(value: &'a crate::Route) -> crate::RouteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Route(id={})", value.id()));
        crate::RouteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn schedule<'a>(value: &'a crate::Schedule) -> crate::ScheduleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Schedule(id={})", value.id()));
        crate::ScheduleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn load<'a>(value: &'a crate::Load) -> crate::LoadExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Load(id={})", value.id()));
        crate::LoadExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn unload<'a>(value: &'a crate::Unload) -> crate::UnloadExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Unload(id={})", value.id()));
        crate::UnloadExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn warehouse<'a>(value: &'a crate::Warehouse) -> crate::WarehouseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Warehouse(id={})", value.id()));
        crate::WarehouseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn inventory<'a>(value: &'a crate::Inventory) -> crate::InventoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Inventory(id={})", value.id()));
        crate::InventoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn pallet<'a>(value: &'a crate::Pallet) -> crate::PalletExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Pallet(id={})", value.id()));
        crate::PalletExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn shipment<'a>(value: &'a crate::Shipment) -> crate::ShipmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Shipment(id={})", value.id()));
        crate::ShipmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tracking<'a>(value: &'a crate::Tracking) -> crate::TrackingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Tracking(id={})", value.id()));
        crate::TrackingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fuel_log<'a>(value: &'a crate::FuelLog) -> crate::FuelLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FuelLog(id={})", value.id()));
        crate::FuelLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn maintenance<'a>(value: &'a crate::Maintenance) -> crate::MaintenanceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Maintenance(id={})", value.id()));
        crate::MaintenanceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn inspection<'a>(value: &'a crate::Inspection) -> crate::InspectionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Inspection(id={})", value.id()));
        crate::InspectionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn safety_report<'a>(value: &'a crate::SafetyReport) -> crate::SafetyReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SafetyReport(id={})", value.id()));
        crate::SafetyReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn crew<'a>(value: &'a crate::Crew) -> crate::CrewExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Crew(id={})", value.id()));
        crate::CrewExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn equipment<'a>(value: &'a crate::Equipment) -> crate::EquipmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Equipment(id={})", value.id()));
        crate::EquipmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn facility<'a>(value: &'a crate::Facility) -> crate::FacilityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Facility(id={})", value.id()));
        crate::FacilityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn budget<'a>(value: &'a crate::Budget) -> crate::BudgetExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Budget(id={})", value.id()));
        crate::BudgetExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn expense<'a>(value: &'a crate::Expense) -> crate::ExpenseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Expense(id={})", value.id()));
        crate::ExpenseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn revenue<'a>(value: &'a crate::Revenue) -> crate::RevenueExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Revenue(id={})", value.id()));
        crate::RevenueExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn profit<'a>(value: &'a crate::Profit) -> crate::ProfitExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Profit(id={})", value.id()));
        crate::ProfitExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn loss<'a>(value: &'a crate::Loss) -> crate::LossExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Loss(id={})", value.id()));
        crate::LossExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tax<'a>(value: &'a crate::Tax) -> crate::TaxExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Tax(id={})", value.id()));
        crate::TaxExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn audit<'a>(value: &'a crate::Audit) -> crate::AuditExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Audit(id={})", value.id()));
        crate::AuditExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn ledger<'a>(value: &'a crate::Ledger) -> crate::LedgerExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Ledger(id={})", value.id()));
        crate::LedgerExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn journal<'a>(value: &'a crate::Journal) -> crate::JournalExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Journal(id={})", value.id()));
        crate::JournalExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn accounts_payable<'a>(value: &'a crate::AccountsPayable) -> crate::AccountsPayableExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AccountsPayable(id={})", value.id()));
        crate::AccountsPayableExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn accounts_receivable<'a>(value: &'a crate::AccountsReceivable) -> crate::AccountsReceivableExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AccountsReceivable(id={})", value.id()));
        crate::AccountsReceivableExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payroll<'a>(value: &'a crate::Payroll) -> crate::PayrollExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Payroll(id={})", value.id()));
        crate::PayrollExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn expense_report<'a>(value: &'a crate::ExpenseReport) -> crate::ExpenseReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExpenseReport(id={})", value.id()));
        crate::ExpenseReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn budget_forecast<'a>(value: &'a crate::BudgetForecast) -> crate::BudgetForecastExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BudgetForecast(id={})", value.id()));
        crate::BudgetForecastExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn cash_flow<'a>(value: &'a crate::CashFlow) -> crate::CashFlowExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CashFlow(id={})", value.id()));
        crate::CashFlowExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn investment<'a>(value: &'a crate::Investment) -> crate::InvestmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Investment(id={})", value.id()));
        crate::InvestmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn asset<'a>(value: &'a crate::Asset) -> crate::AssetExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Asset(id={})", value.id()));
        crate::AssetExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn liability<'a>(value: &'a crate::Liability) -> crate::LiabilityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Liability(id={})", value.id()));
        crate::LiabilityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn equity<'a>(value: &'a crate::Equity) -> crate::EquityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Equity(id={})", value.id()));
        crate::EquityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn financial_statement<'a>(value: &'a crate::FinancialStatement) -> crate::FinancialStatementExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FinancialStatement(id={})", value.id()));
        crate::FinancialStatementExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn employee<'a>(value: &'a crate::Employee) -> crate::EmployeeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Employee(id={})", value.id()));
        crate::EmployeeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn contractor<'a>(value: &'a crate::Contractor) -> crate::ContractorExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Contractor(id={})", value.id()));
        crate::ContractorExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn beneficiary<'a>(value: &'a crate::Beneficiary) -> crate::BeneficiaryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Beneficiary(id={})", value.id()));
        crate::BeneficiaryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn dependent<'a>(value: &'a crate::Dependent) -> crate::DependentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Dependent(id={})", value.id()));
        crate::DependentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn training<'a>(value: &'a crate::Training) -> crate::TrainingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Training(id={})", value.id()));
        crate::TrainingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn certification<'a>(value: &'a crate::Certification) -> crate::CertificationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Certification(id={})", value.id()));
        crate::CertificationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn performance_review<'a>(value: &'a crate::PerformanceReview) -> crate::PerformanceReviewExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PerformanceReview(id={})", value.id()));
        crate::PerformanceReviewExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn termination<'a>(value: &'a crate::Termination) -> crate::TerminationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Termination(id={})", value.id()));
        crate::TerminationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn onboarding<'a>(value: &'a crate::Onboarding) -> crate::OnboardingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Onboarding(id={})", value.id()));
        crate::OnboardingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

