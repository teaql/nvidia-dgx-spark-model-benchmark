use teaql_core::Expr;

use crate::*;

pub struct PurposedQuery<T> {
    pub inner: T,
    pub purpose: String,
}

impl<T> PurposedQuery<T> {
    pub fn new(inner: T, purpose: impl Into<String>) -> Self {
        Self { inner, purpose: purpose.into() }
    }
}

pub struct Q;

impl Q {
    pub fn customers() -> CustomerRequest {
        CustomerRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customers_minimal() -> CustomerRequest {
        CustomerRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customers_with_children() -> CustomerRequest {
        CustomerRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn leads() -> LeadRequest {
        LeadRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn leads_minimal() -> LeadRequest {
        LeadRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn leads_with_children() -> LeadRequest {
        LeadRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn quotes() -> QuoteRequest {
        QuoteRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn quotes_minimal() -> QuoteRequest {
        QuoteRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn quotes_with_children() -> QuoteRequest {
        QuoteRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn contracts() -> ContractRequest {
        ContractRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contracts_minimal() -> ContractRequest {
        ContractRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contracts_with_children() -> ContractRequest {
        ContractRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn invoices() -> InvoiceRequest {
        InvoiceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoices_minimal() -> InvoiceRequest {
        InvoiceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoices_with_children() -> InvoiceRequest {
        InvoiceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payments() -> PaymentRequest {
        PaymentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payments_minimal() -> PaymentRequest {
        PaymentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payments_with_children() -> PaymentRequest {
        PaymentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn sales_orders() -> SalesOrderRequest {
        SalesOrderRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sales_orders_minimal() -> SalesOrderRequest {
        SalesOrderRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sales_orders_with_children() -> SalesOrderRequest {
        SalesOrderRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn sales_reps() -> SalesRepRequest {
        SalesRepRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sales_reps_minimal() -> SalesRepRequest {
        SalesRepRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sales_reps_with_children() -> SalesRepRequest {
        SalesRepRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn territories() -> TerritoryRequest {
        TerritoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn territories_minimal() -> TerritoryRequest {
        TerritoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn territories_with_children() -> TerritoryRequest {
        TerritoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn pricings() -> PricingRequest {
        PricingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pricings_minimal() -> PricingRequest {
        PricingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pricings_with_children() -> PricingRequest {
        PricingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn discounts() -> DiscountRequest {
        DiscountRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn discounts_minimal() -> DiscountRequest {
        DiscountRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn discounts_with_children() -> DiscountRequest {
        DiscountRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn promotions() -> PromotionRequest {
        PromotionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn promotions_minimal() -> PromotionRequest {
        PromotionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn promotions_with_children() -> PromotionRequest {
        PromotionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn campaigns() -> CampaignRequest {
        CampaignRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn campaigns_minimal() -> CampaignRequest {
        CampaignRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn campaigns_with_children() -> CampaignRequest {
        CampaignRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn feedback() -> FeedbackRequest {
        FeedbackRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn feedback_minimal() -> FeedbackRequest {
        FeedbackRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn feedback_with_children() -> FeedbackRequest {
        FeedbackRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn complaints() -> ComplaintRequest {
        ComplaintRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn complaints_minimal() -> ComplaintRequest {
        ComplaintRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn complaints_with_children() -> ComplaintRequest {
        ComplaintRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn service_requests() -> ServiceRequestRequest {
        ServiceRequestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_requests_minimal() -> ServiceRequestRequest {
        ServiceRequestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_requests_with_children() -> ServiceRequestRequest {
        ServiceRequestRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn warranties() -> WarrantyRequest {
        WarrantyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warranties_minimal() -> WarrantyRequest {
        WarrantyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warranties_with_children() -> WarrantyRequest {
        WarrantyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn renewals() -> RenewalRequest {
        RenewalRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn renewals_minimal() -> RenewalRequest {
        RenewalRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn renewals_with_children() -> RenewalRequest {
        RenewalRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn upsells() -> UpsellRequest {
        UpsellRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn upsells_minimal() -> UpsellRequest {
        UpsellRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn upsells_with_children() -> UpsellRequest {
        UpsellRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn cross_sells() -> CrossSellRequest {
        CrossSellRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cross_sells_minimal() -> CrossSellRequest {
        CrossSellRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cross_sells_with_children() -> CrossSellRequest {
        CrossSellRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn trucks() -> TruckRequest {
        TruckRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn trucks_minimal() -> TruckRequest {
        TruckRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn trucks_with_children() -> TruckRequest {
        TruckRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn trailers() -> TrailerRequest {
        TrailerRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn trailers_minimal() -> TrailerRequest {
        TrailerRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn trailers_with_children() -> TrailerRequest {
        TrailerRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn drivers() -> DriverRequest {
        DriverRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn drivers_minimal() -> DriverRequest {
        DriverRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn drivers_with_children() -> DriverRequest {
        DriverRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn move_orders() -> MoveOrderRequest {
        MoveOrderRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_orders_minimal() -> MoveOrderRequest {
        MoveOrderRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_orders_with_children() -> MoveOrderRequest {
        MoveOrderRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn routes() -> RouteRequest {
        RouteRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn routes_minimal() -> RouteRequest {
        RouteRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn routes_with_children() -> RouteRequest {
        RouteRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn schedules() -> ScheduleRequest {
        ScheduleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn schedules_minimal() -> ScheduleRequest {
        ScheduleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn schedules_with_children() -> ScheduleRequest {
        ScheduleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn loads() -> LoadRequest {
        LoadRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loads_minimal() -> LoadRequest {
        LoadRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loads_with_children() -> LoadRequest {
        LoadRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn unloads() -> UnloadRequest {
        UnloadRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn unloads_minimal() -> UnloadRequest {
        UnloadRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn unloads_with_children() -> UnloadRequest {
        UnloadRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn warehouses() -> WarehouseRequest {
        WarehouseRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warehouses_minimal() -> WarehouseRequest {
        WarehouseRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warehouses_with_children() -> WarehouseRequest {
        WarehouseRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn inventory() -> InventoryRequest {
        InventoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_minimal() -> InventoryRequest {
        InventoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_with_children() -> InventoryRequest {
        InventoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn pallets() -> PalletRequest {
        PalletRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pallets_minimal() -> PalletRequest {
        PalletRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pallets_with_children() -> PalletRequest {
        PalletRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn shipments() -> ShipmentRequest {
        ShipmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shipments_minimal() -> ShipmentRequest {
        ShipmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shipments_with_children() -> ShipmentRequest {
        ShipmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn trackings() -> TrackingRequest {
        TrackingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn trackings_minimal() -> TrackingRequest {
        TrackingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn trackings_with_children() -> TrackingRequest {
        TrackingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn fuel_logs() -> FuelLogRequest {
        FuelLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fuel_logs_minimal() -> FuelLogRequest {
        FuelLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fuel_logs_with_children() -> FuelLogRequest {
        FuelLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn maintenances() -> MaintenanceRequest {
        MaintenanceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenances_minimal() -> MaintenanceRequest {
        MaintenanceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenances_with_children() -> MaintenanceRequest {
        MaintenanceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn inspections() -> InspectionRequest {
        InspectionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inspections_minimal() -> InspectionRequest {
        InspectionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inspections_with_children() -> InspectionRequest {
        InspectionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn safety_reports() -> SafetyReportRequest {
        SafetyReportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn safety_reports_minimal() -> SafetyReportRequest {
        SafetyReportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn safety_reports_with_children() -> SafetyReportRequest {
        SafetyReportRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn crews() -> CrewRequest {
        CrewRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn crews_minimal() -> CrewRequest {
        CrewRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn crews_with_children() -> CrewRequest {
        CrewRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn equipment() -> EquipmentRequest {
        EquipmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_minimal() -> EquipmentRequest {
        EquipmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_with_children() -> EquipmentRequest {
        EquipmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn facilities() -> FacilityRequest {
        FacilityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn facilities_minimal() -> FacilityRequest {
        FacilityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn facilities_with_children() -> FacilityRequest {
        FacilityRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn budgets() -> BudgetRequest {
        BudgetRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn budgets_minimal() -> BudgetRequest {
        BudgetRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn budgets_with_children() -> BudgetRequest {
        BudgetRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn expenses() -> ExpenseRequest {
        ExpenseRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expenses_minimal() -> ExpenseRequest {
        ExpenseRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expenses_with_children() -> ExpenseRequest {
        ExpenseRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn revenues() -> RevenueRequest {
        RevenueRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn revenues_minimal() -> RevenueRequest {
        RevenueRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn revenues_with_children() -> RevenueRequest {
        RevenueRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn profits() -> ProfitRequest {
        ProfitRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn profits_minimal() -> ProfitRequest {
        ProfitRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn profits_with_children() -> ProfitRequest {
        ProfitRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn losses() -> LossRequest {
        LossRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn losses_minimal() -> LossRequest {
        LossRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn losses_with_children() -> LossRequest {
        LossRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn taxes() -> TaxRequest {
        TaxRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn taxes_minimal() -> TaxRequest {
        TaxRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn taxes_with_children() -> TaxRequest {
        TaxRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn audits() -> AuditRequest {
        AuditRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audits_minimal() -> AuditRequest {
        AuditRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audits_with_children() -> AuditRequest {
        AuditRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn ledgers() -> LedgerRequest {
        LedgerRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn ledgers_minimal() -> LedgerRequest {
        LedgerRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn ledgers_with_children() -> LedgerRequest {
        LedgerRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn journals() -> JournalRequest {
        JournalRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn journals_minimal() -> JournalRequest {
        JournalRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn journals_with_children() -> JournalRequest {
        JournalRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn accounts_payables() -> AccountsPayableRequest {
        AccountsPayableRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn accounts_payables_minimal() -> AccountsPayableRequest {
        AccountsPayableRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn accounts_payables_with_children() -> AccountsPayableRequest {
        AccountsPayableRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn accounts_receivables() -> AccountsReceivableRequest {
        AccountsReceivableRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn accounts_receivables_minimal() -> AccountsReceivableRequest {
        AccountsReceivableRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn accounts_receivables_with_children() -> AccountsReceivableRequest {
        AccountsReceivableRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payrolls() -> PayrollRequest {
        PayrollRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payrolls_minimal() -> PayrollRequest {
        PayrollRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payrolls_with_children() -> PayrollRequest {
        PayrollRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn expense_reports() -> ExpenseReportRequest {
        ExpenseReportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expense_reports_minimal() -> ExpenseReportRequest {
        ExpenseReportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expense_reports_with_children() -> ExpenseReportRequest {
        ExpenseReportRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn budget_forecasts() -> BudgetForecastRequest {
        BudgetForecastRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn budget_forecasts_minimal() -> BudgetForecastRequest {
        BudgetForecastRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn budget_forecasts_with_children() -> BudgetForecastRequest {
        BudgetForecastRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn cash_flows() -> CashFlowRequest {
        CashFlowRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cash_flows_minimal() -> CashFlowRequest {
        CashFlowRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cash_flows_with_children() -> CashFlowRequest {
        CashFlowRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn investments() -> InvestmentRequest {
        InvestmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn investments_minimal() -> InvestmentRequest {
        InvestmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn investments_with_children() -> InvestmentRequest {
        InvestmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn assets() -> AssetRequest {
        AssetRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn assets_minimal() -> AssetRequest {
        AssetRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn assets_with_children() -> AssetRequest {
        AssetRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn liabilities() -> LiabilityRequest {
        LiabilityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn liabilities_minimal() -> LiabilityRequest {
        LiabilityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn liabilities_with_children() -> LiabilityRequest {
        LiabilityRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn equities() -> EquityRequest {
        EquityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equities_minimal() -> EquityRequest {
        EquityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equities_with_children() -> EquityRequest {
        EquityRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn financial_statements() -> FinancialStatementRequest {
        FinancialStatementRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn financial_statements_minimal() -> FinancialStatementRequest {
        FinancialStatementRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn financial_statements_with_children() -> FinancialStatementRequest {
        FinancialStatementRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn employees() -> EmployeeRequest {
        EmployeeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employees_minimal() -> EmployeeRequest {
        EmployeeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employees_with_children() -> EmployeeRequest {
        EmployeeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn contractors() -> ContractorRequest {
        ContractorRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contractors_minimal() -> ContractorRequest {
        ContractorRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contractors_with_children() -> ContractorRequest {
        ContractorRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn beneficiaries() -> BeneficiaryRequest {
        BeneficiaryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn beneficiaries_minimal() -> BeneficiaryRequest {
        BeneficiaryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn beneficiaries_with_children() -> BeneficiaryRequest {
        BeneficiaryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn dependents() -> DependentRequest {
        DependentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dependents_minimal() -> DependentRequest {
        DependentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dependents_with_children() -> DependentRequest {
        DependentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn trainings() -> TrainingRequest {
        TrainingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn trainings_minimal() -> TrainingRequest {
        TrainingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn trainings_with_children() -> TrainingRequest {
        TrainingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn certifications() -> CertificationRequest {
        CertificationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn certifications_minimal() -> CertificationRequest {
        CertificationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn certifications_with_children() -> CertificationRequest {
        CertificationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn performance_reviews() -> PerformanceReviewRequest {
        PerformanceReviewRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn performance_reviews_minimal() -> PerformanceReviewRequest {
        PerformanceReviewRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn performance_reviews_with_children() -> PerformanceReviewRequest {
        PerformanceReviewRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn terminations() -> TerminationRequest {
        TerminationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn terminations_minimal() -> TerminationRequest {
        TerminationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn terminations_with_children() -> TerminationRequest {
        TerminationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn onboardings() -> OnboardingRequest {
        OnboardingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn onboardings_minimal() -> OnboardingRequest {
        OnboardingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn onboardings_with_children() -> OnboardingRequest {
        OnboardingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}