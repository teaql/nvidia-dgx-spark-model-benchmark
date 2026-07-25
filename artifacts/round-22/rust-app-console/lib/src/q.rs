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

    pub fn bills() -> BillRequest {
        BillRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn bills_minimal() -> BillRequest {
        BillRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn bills_with_children() -> BillRequest {
        BillRequest::new()
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

    pub fn forecasts() -> ForecastRequest {
        ForecastRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn forecasts_minimal() -> ForecastRequest {
        ForecastRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn forecasts_with_children() -> ForecastRequest {
        ForecastRequest::new()
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

    pub fn credits() -> CreditRequest {
        CreditRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn credits_minimal() -> CreditRequest {
        CreditRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn credits_with_children() -> CreditRequest {
        CreditRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn debits() -> DebitRequest {
        DebitRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn debits_minimal() -> DebitRequest {
        DebitRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn debits_with_children() -> DebitRequest {
        DebitRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn balances() -> BalanceRequest {
        BalanceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn balances_minimal() -> BalanceRequest {
        BalanceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn balances_with_children() -> BalanceRequest {
        BalanceRequest::new()
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

    pub fn vehicles() -> VehicleRequest {
        VehicleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicles_minimal() -> VehicleRequest {
        VehicleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicles_with_children() -> VehicleRequest {
        VehicleRequest::new()
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

    pub fn capacities() -> CapacityRequest {
        CapacityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn capacities_minimal() -> CapacityRequest {
        CapacityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn capacities_with_children() -> CapacityRequest {
        CapacityRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn manifests() -> ManifestRequest {
        ManifestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn manifests_minimal() -> ManifestRequest {
        ManifestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn manifests_with_children() -> ManifestRequest {
        ManifestRequest::new()
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

    pub fn dispatches() -> DispatchRequest {
        DispatchRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dispatches_minimal() -> DispatchRequest {
        DispatchRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dispatches_with_children() -> DispatchRequest {
        DispatchRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn freights() -> FreightRequest {
        FreightRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn freights_minimal() -> FreightRequest {
        FreightRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn freights_with_children() -> FreightRequest {
        FreightRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn carriers() -> CarrierRequest {
        CarrierRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn carriers_minimal() -> CarrierRequest {
        CarrierRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn carriers_with_children() -> CarrierRequest {
        CarrierRequest::new()
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

    pub fn loading_docks() -> LoadingDockRequest {
        LoadingDockRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loading_docks_minimal() -> LoadingDockRequest {
        LoadingDockRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loading_docks_with_children() -> LoadingDockRequest {
        LoadingDockRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn unloading_docks() -> UnloadingDockRequest {
        UnloadingDockRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn unloading_docks_minimal() -> UnloadingDockRequest {
        UnloadingDockRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn unloading_docks_with_children() -> UnloadingDockRequest {
        UnloadingDockRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn freight_forwarders() -> FreightForwarderRequest {
        FreightForwarderRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn freight_forwarders_minimal() -> FreightForwarderRequest {
        FreightForwarderRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn freight_forwarders_with_children() -> FreightForwarderRequest {
        FreightForwarderRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customses() -> CustomsRequest {
        CustomsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customses_minimal() -> CustomsRequest {
        CustomsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customses_with_children() -> CustomsRequest {
        CustomsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn documentations() -> DocumentationRequest {
        DocumentationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn documentations_minimal() -> DocumentationRequest {
        DocumentationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn documentations_with_children() -> DocumentationRequest {
        DocumentationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tolls() -> TollRequest {
        TollRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tolls_minimal() -> TollRequest {
        TollRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tolls_with_children() -> TollRequest {
        TollRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn fuels() -> FuelRequest {
        FuelRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fuels_minimal() -> FuelRequest {
        FuelRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fuels_with_children() -> FuelRequest {
        FuelRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

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

    pub fn clients() -> ClientRequest {
        ClientRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn clients_minimal() -> ClientRequest {
        ClientRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn clients_with_children() -> ClientRequest {
        ClientRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn contacts() -> ContactRequest {
        ContactRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contacts_minimal() -> ContactRequest {
        ContactRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contacts_with_children() -> ContactRequest {
        ContactRequest::new()
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

    pub fn prospects() -> ProspectRequest {
        ProspectRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn prospects_minimal() -> ProspectRequest {
        ProspectRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn prospects_with_children() -> ProspectRequest {
        ProspectRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn accounts() -> AccountRequest {
        AccountRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn accounts_minimal() -> AccountRequest {
        AccountRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn accounts_with_children() -> AccountRequest {
        AccountRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn service_agreements() -> ServiceAgreementRequest {
        ServiceAgreementRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_agreements_minimal() -> ServiceAgreementRequest {
        ServiceAgreementRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_agreements_with_children() -> ServiceAgreementRequest {
        ServiceAgreementRequest::new()
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

    pub fn support_tickets() -> SupportTicketRequest {
        SupportTicketRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn support_tickets_minimal() -> SupportTicketRequest {
        SupportTicketRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn support_tickets_with_children() -> SupportTicketRequest {
        SupportTicketRequest::new()
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

    pub fn surveys() -> SurveyRequest {
        SurveyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn surveys_minimal() -> SurveyRequest {
        SurveyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn surveys_with_children() -> SurveyRequest {
        SurveyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn loyalty_programs() -> LoyaltyProgramRequest {
        LoyaltyProgramRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loyalty_programs_minimal() -> LoyaltyProgramRequest {
        LoyaltyProgramRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loyalty_programs_with_children() -> LoyaltyProgramRequest {
        LoyaltyProgramRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn referrals() -> ReferralRequest {
        ReferralRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn referrals_minimal() -> ReferralRequest {
        ReferralRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn referrals_with_children() -> ReferralRequest {
        ReferralRequest::new()
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

    pub fn marketing_campaigns() -> MarketingCampaignRequest {
        MarketingCampaignRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn marketing_campaigns_minimal() -> MarketingCampaignRequest {
        MarketingCampaignRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn marketing_campaigns_with_children() -> MarketingCampaignRequest {
        MarketingCampaignRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn newsletters() -> NewsletterRequest {
        NewsletterRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn newsletters_minimal() -> NewsletterRequest {
        NewsletterRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn newsletters_with_children() -> NewsletterRequest {
        NewsletterRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn communication_preferences() -> CommunicationPreferenceRequest {
        CommunicationPreferenceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn communication_preferences_minimal() -> CommunicationPreferenceRequest {
        CommunicationPreferenceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn communication_preferences_with_children() -> CommunicationPreferenceRequest {
        CommunicationPreferenceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn profiles() -> ProfileRequest {
        ProfileRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn profiles_minimal() -> ProfileRequest {
        ProfileRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn profiles_with_children() -> ProfileRequest {
        ProfileRequest::new()
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

    pub fn repairs() -> RepairRequest {
        RepairRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn repairs_minimal() -> RepairRequest {
        RepairRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn repairs_with_children() -> RepairRequest {
        RepairRequest::new()
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

    pub fn safety_checks() -> SafetyCheckRequest {
        SafetyCheckRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn safety_checks_minimal() -> SafetyCheckRequest {
        SafetyCheckRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn safety_checks_with_children() -> SafetyCheckRequest {
        SafetyCheckRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn incident_reports() -> IncidentReportRequest {
        IncidentReportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn incident_reports_minimal() -> IncidentReportRequest {
        IncidentReportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn incident_reports_with_children() -> IncidentReportRequest {
        IncidentReportRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn claims() -> ClaimRequest {
        ClaimRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn claims_minimal() -> ClaimRequest {
        ClaimRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn claims_with_children() -> ClaimRequest {
        ClaimRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn parts_inventory() -> PartsInventoryRequest {
        PartsInventoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn parts_inventory_minimal() -> PartsInventoryRequest {
        PartsInventoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn parts_inventory_with_children() -> PartsInventoryRequest {
        PartsInventoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn stock_levels() -> StockLevelRequest {
        StockLevelRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn stock_levels_minimal() -> StockLevelRequest {
        StockLevelRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn stock_levels_with_children() -> StockLevelRequest {
        StockLevelRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn reorder_points() -> ReorderPointRequest {
        ReorderPointRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn reorder_points_minimal() -> ReorderPointRequest {
        ReorderPointRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn reorder_points_with_children() -> ReorderPointRequest {
        ReorderPointRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn suppliers() -> SupplierRequest {
        SupplierRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn suppliers_minimal() -> SupplierRequest {
        SupplierRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn suppliers_with_children() -> SupplierRequest {
        SupplierRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vendors() -> VendorRequest {
        VendorRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vendors_minimal() -> VendorRequest {
        VendorRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vendors_with_children() -> VendorRequest {
        VendorRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn purchase_orders() -> PurchaseOrderRequest {
        PurchaseOrderRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn purchase_orders_minimal() -> PurchaseOrderRequest {
        PurchaseOrderRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn purchase_orders_with_children() -> PurchaseOrderRequest {
        PurchaseOrderRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn receivings() -> ReceivingRequest {
        ReceivingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn receivings_minimal() -> ReceivingRequest {
        ReceivingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn receivings_with_children() -> ReceivingRequest {
        ReceivingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn putaways() -> PutawayRequest {
        PutawayRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn putaways_minimal() -> PutawayRequest {
        PutawayRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn putaways_with_children() -> PutawayRequest {
        PutawayRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn pickings() -> PickingRequest {
        PickingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pickings_minimal() -> PickingRequest {
        PickingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pickings_with_children() -> PickingRequest {
        PickingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn packings() -> PackingRequest {
        PackingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packings_minimal() -> PackingRequest {
        PackingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packings_with_children() -> PackingRequest {
        PackingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn shippings() -> ShippingRequest {
        ShippingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shippings_minimal() -> ShippingRequest {
        ShippingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shippings_with_children() -> ShippingRequest {
        ShippingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn returns_processes() -> ReturnsProcessRequest {
        ReturnsProcessRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn returns_processes_minimal() -> ReturnsProcessRequest {
        ReturnsProcessRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn returns_processes_with_children() -> ReturnsProcessRequest {
        ReturnsProcessRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn quality_controls() -> QualityControlRequest {
        QualityControlRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn quality_controls_minimal() -> QualityControlRequest {
        QualityControlRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn quality_controls_with_children() -> QualityControlRequest {
        QualityControlRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn performance_metrics() -> PerformanceMetricRequest {
        PerformanceMetricRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn performance_metrics_minimal() -> PerformanceMetricRequest {
        PerformanceMetricRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn performance_metrics_with_children() -> PerformanceMetricRequest {
        PerformanceMetricRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}