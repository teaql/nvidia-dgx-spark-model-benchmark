// The `E` expression wrapper provides zero-cost AST traversal
// and will automatically panic if it encounters a NotLoaded error.
pub struct E;

impl E {
    pub fn invoice<'a>(value: &'a crate::Invoice) -> crate::InvoiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Invoice(id={})", value.id()));
        crate::InvoiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn bill<'a>(value: &'a crate::Bill) -> crate::BillExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Bill(id={})", value.id()));
        crate::BillExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payment<'a>(value: &'a crate::Payment) -> crate::PaymentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Payment(id={})", value.id()));
        crate::PaymentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn expense<'a>(value: &'a crate::Expense) -> crate::ExpenseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Expense(id={})", value.id()));
        crate::ExpenseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn revenue<'a>(value: &'a crate::Revenue) -> crate::RevenueExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Revenue(id={})", value.id()));
        crate::RevenueExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn ledger<'a>(value: &'a crate::Ledger) -> crate::LedgerExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Ledger(id={})", value.id()));
        crate::LedgerExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn audit<'a>(value: &'a crate::Audit) -> crate::AuditExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Audit(id={})", value.id()));
        crate::AuditExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tax<'a>(value: &'a crate::Tax) -> crate::TaxExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Tax(id={})", value.id()));
        crate::TaxExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn budget<'a>(value: &'a crate::Budget) -> crate::BudgetExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Budget(id={})", value.id()));
        crate::BudgetExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn forecast<'a>(value: &'a crate::Forecast) -> crate::ForecastExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Forecast(id={})", value.id()));
        crate::ForecastExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payroll<'a>(value: &'a crate::Payroll) -> crate::PayrollExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Payroll(id={})", value.id()));
        crate::PayrollExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn expense_report<'a>(value: &'a crate::ExpenseReport) -> crate::ExpenseReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExpenseReport(id={})", value.id()));
        crate::ExpenseReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn credit<'a>(value: &'a crate::Credit) -> crate::CreditExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Credit(id={})", value.id()));
        crate::CreditExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn debit<'a>(value: &'a crate::Debit) -> crate::DebitExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Debit(id={})", value.id()));
        crate::DebitExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn balance<'a>(value: &'a crate::Balance) -> crate::BalanceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Balance(id={})", value.id()));
        crate::BalanceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn cash_flow<'a>(value: &'a crate::CashFlow) -> crate::CashFlowExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CashFlow(id={})", value.id()));
        crate::CashFlowExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn financial_statement<'a>(value: &'a crate::FinancialStatement) -> crate::FinancialStatementExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FinancialStatement(id={})", value.id()));
        crate::FinancialStatementExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn shipment<'a>(value: &'a crate::Shipment) -> crate::ShipmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Shipment(id={})", value.id()));
        crate::ShipmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn route<'a>(value: &'a crate::Route) -> crate::RouteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Route(id={})", value.id()));
        crate::RouteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle<'a>(value: &'a crate::Vehicle) -> crate::VehicleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Vehicle(id={})", value.id()));
        crate::VehicleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver<'a>(value: &'a crate::Driver) -> crate::DriverExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Driver(id={})", value.id()));
        crate::DriverExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn load<'a>(value: &'a crate::Load) -> crate::LoadExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Load(id={})", value.id()));
        crate::LoadExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn unload<'a>(value: &'a crate::Unload) -> crate::UnloadExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Unload(id={})", value.id()));
        crate::UnloadExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn capacity<'a>(value: &'a crate::Capacity) -> crate::CapacityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Capacity(id={})", value.id()));
        crate::CapacityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn manifest<'a>(value: &'a crate::Manifest) -> crate::ManifestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Manifest(id={})", value.id()));
        crate::ManifestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tracking<'a>(value: &'a crate::Tracking) -> crate::TrackingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Tracking(id={})", value.id()));
        crate::TrackingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn dispatch<'a>(value: &'a crate::Dispatch) -> crate::DispatchExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Dispatch(id={})", value.id()));
        crate::DispatchExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn freight<'a>(value: &'a crate::Freight) -> crate::FreightExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Freight(id={})", value.id()));
        crate::FreightExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn carrier<'a>(value: &'a crate::Carrier) -> crate::CarrierExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Carrier(id={})", value.id()));
        crate::CarrierExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn warehouse<'a>(value: &'a crate::Warehouse) -> crate::WarehouseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Warehouse(id={})", value.id()));
        crate::WarehouseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn loading_dock<'a>(value: &'a crate::LoadingDock) -> crate::LoadingDockExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoadingDock(id={})", value.id()));
        crate::LoadingDockExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn unloading_dock<'a>(value: &'a crate::UnloadingDock) -> crate::UnloadingDockExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UnloadingDock(id={})", value.id()));
        crate::UnloadingDockExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn freight_forwarder<'a>(value: &'a crate::FreightForwarder) -> crate::FreightForwarderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FreightForwarder(id={})", value.id()));
        crate::FreightForwarderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customs<'a>(value: &'a crate::Customs) -> crate::CustomsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Customs(id={})", value.id()));
        crate::CustomsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn documentation<'a>(value: &'a crate::Documentation) -> crate::DocumentationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Documentation(id={})", value.id()));
        crate::DocumentationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn toll<'a>(value: &'a crate::Toll) -> crate::TollExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Toll(id={})", value.id()));
        crate::TollExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fuel<'a>(value: &'a crate::Fuel) -> crate::FuelExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Fuel(id={})", value.id()));
        crate::FuelExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer<'a>(value: &'a crate::Customer) -> crate::CustomerExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Customer(id={})", value.id()));
        crate::CustomerExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn client<'a>(value: &'a crate::Client) -> crate::ClientExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Client(id={})", value.id()));
        crate::ClientExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn contact<'a>(value: &'a crate::Contact) -> crate::ContactExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Contact(id={})", value.id()));
        crate::ContactExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn lead<'a>(value: &'a crate::Lead) -> crate::LeadExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Lead(id={})", value.id()));
        crate::LeadExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn prospect<'a>(value: &'a crate::Prospect) -> crate::ProspectExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Prospect(id={})", value.id()));
        crate::ProspectExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn account<'a>(value: &'a crate::Account) -> crate::AccountExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Account(id={})", value.id()));
        crate::AccountExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_agreement<'a>(value: &'a crate::ServiceAgreement) -> crate::ServiceAgreementExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceAgreement(id={})", value.id()));
        crate::ServiceAgreementExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn contract<'a>(value: &'a crate::Contract) -> crate::ContractExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Contract(id={})", value.id()));
        crate::ContractExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn warranty<'a>(value: &'a crate::Warranty) -> crate::WarrantyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Warranty(id={})", value.id()));
        crate::WarrantyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn support_ticket<'a>(value: &'a crate::SupportTicket) -> crate::SupportTicketExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SupportTicket(id={})", value.id()));
        crate::SupportTicketExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn feedback<'a>(value: &'a crate::Feedback) -> crate::FeedbackExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Feedback(id={})", value.id()));
        crate::FeedbackExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn survey<'a>(value: &'a crate::Survey) -> crate::SurveyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Survey(id={})", value.id()));
        crate::SurveyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn loyalty_program<'a>(value: &'a crate::LoyaltyProgram) -> crate::LoyaltyProgramExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoyaltyProgram(id={})", value.id()));
        crate::LoyaltyProgramExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn referral<'a>(value: &'a crate::Referral) -> crate::ReferralExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Referral(id={})", value.id()));
        crate::ReferralExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn discount<'a>(value: &'a crate::Discount) -> crate::DiscountExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Discount(id={})", value.id()));
        crate::DiscountExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn promotion<'a>(value: &'a crate::Promotion) -> crate::PromotionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Promotion(id={})", value.id()));
        crate::PromotionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn marketing_campaign<'a>(value: &'a crate::MarketingCampaign) -> crate::MarketingCampaignExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MarketingCampaign(id={})", value.id()));
        crate::MarketingCampaignExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn newsletter<'a>(value: &'a crate::Newsletter) -> crate::NewsletterExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Newsletter(id={})", value.id()));
        crate::NewsletterExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn communication_preference<'a>(value: &'a crate::CommunicationPreference) -> crate::CommunicationPreferenceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CommunicationPreference(id={})", value.id()));
        crate::CommunicationPreferenceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn profile<'a>(value: &'a crate::Profile) -> crate::ProfileExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Profile(id={})", value.id()));
        crate::ProfileExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn maintenance<'a>(value: &'a crate::Maintenance) -> crate::MaintenanceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Maintenance(id={})", value.id()));
        crate::MaintenanceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn repair<'a>(value: &'a crate::Repair) -> crate::RepairExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Repair(id={})", value.id()));
        crate::RepairExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn inspection<'a>(value: &'a crate::Inspection) -> crate::InspectionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Inspection(id={})", value.id()));
        crate::InspectionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn safety_check<'a>(value: &'a crate::SafetyCheck) -> crate::SafetyCheckExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SafetyCheck(id={})", value.id()));
        crate::SafetyCheckExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn incident_report<'a>(value: &'a crate::IncidentReport) -> crate::IncidentReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("IncidentReport(id={})", value.id()));
        crate::IncidentReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn claim<'a>(value: &'a crate::Claim) -> crate::ClaimExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Claim(id={})", value.id()));
        crate::ClaimExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn parts_inventory<'a>(value: &'a crate::PartsInventory) -> crate::PartsInventoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PartsInventory(id={})", value.id()));
        crate::PartsInventoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn stock_level<'a>(value: &'a crate::StockLevel) -> crate::StockLevelExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("StockLevel(id={})", value.id()));
        crate::StockLevelExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn reorder_point<'a>(value: &'a crate::ReorderPoint) -> crate::ReorderPointExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ReorderPoint(id={})", value.id()));
        crate::ReorderPointExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn supplier<'a>(value: &'a crate::Supplier) -> crate::SupplierExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Supplier(id={})", value.id()));
        crate::SupplierExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vendor<'a>(value: &'a crate::Vendor) -> crate::VendorExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Vendor(id={})", value.id()));
        crate::VendorExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn purchase_order<'a>(value: &'a crate::PurchaseOrder) -> crate::PurchaseOrderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PurchaseOrder(id={})", value.id()));
        crate::PurchaseOrderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn receiving<'a>(value: &'a crate::Receiving) -> crate::ReceivingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Receiving(id={})", value.id()));
        crate::ReceivingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn putaway<'a>(value: &'a crate::Putaway) -> crate::PutawayExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Putaway(id={})", value.id()));
        crate::PutawayExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn picking<'a>(value: &'a crate::Picking) -> crate::PickingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Picking(id={})", value.id()));
        crate::PickingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn packing<'a>(value: &'a crate::Packing) -> crate::PackingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Packing(id={})", value.id()));
        crate::PackingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn shipping<'a>(value: &'a crate::Shipping) -> crate::ShippingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Shipping(id={})", value.id()));
        crate::ShippingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn returns_process<'a>(value: &'a crate::ReturnsProcess) -> crate::ReturnsProcessExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ReturnsProcess(id={})", value.id()));
        crate::ReturnsProcessExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn quality_control<'a>(value: &'a crate::QualityControl) -> crate::QualityControlExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("QualityControl(id={})", value.id()));
        crate::QualityControlExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn performance_metric<'a>(value: &'a crate::PerformanceMetric) -> crate::PerformanceMetricExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PerformanceMetric(id={})", value.id()));
        crate::PerformanceMetricExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

