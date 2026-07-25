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
    pub fn customer_profiles() -> CustomerProfileRequest {
        CustomerProfileRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_profiles_minimal() -> CustomerProfileRequest {
        CustomerProfileRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_profiles_with_children() -> CustomerProfileRequest {
        CustomerProfileRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_contacts() -> CustomerContactRequest {
        CustomerContactRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_contacts_minimal() -> CustomerContactRequest {
        CustomerContactRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_contacts_with_children() -> CustomerContactRequest {
        CustomerContactRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_addresses() -> CustomerAddressRequest {
        CustomerAddressRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_addresses_minimal() -> CustomerAddressRequest {
        CustomerAddressRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_addresses_with_children() -> CustomerAddressRequest {
        CustomerAddressRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_preferences() -> CustomerPreferenceRequest {
        CustomerPreferenceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_preferences_minimal() -> CustomerPreferenceRequest {
        CustomerPreferenceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_preferences_with_children() -> CustomerPreferenceRequest {
        CustomerPreferenceRequest::new()
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

    pub fn customer_feedback() -> CustomerFeedbackRequest {
        CustomerFeedbackRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_feedback_minimal() -> CustomerFeedbackRequest {
        CustomerFeedbackRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_feedback_with_children() -> CustomerFeedbackRequest {
        CustomerFeedbackRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_segments() -> CustomerSegmentRequest {
        CustomerSegmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_segments_minimal() -> CustomerSegmentRequest {
        CustomerSegmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_segments_with_children() -> CustomerSegmentRequest {
        CustomerSegmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_accounts() -> CustomerAccountRequest {
        CustomerAccountRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_accounts_minimal() -> CustomerAccountRequest {
        CustomerAccountRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_accounts_with_children() -> CustomerAccountRequest {
        CustomerAccountRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payment_methods() -> PaymentMethodRequest {
        PaymentMethodRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payment_methods_minimal() -> PaymentMethodRequest {
        PaymentMethodRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payment_methods_with_children() -> PaymentMethodRequest {
        PaymentMethodRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn invoice_histories() -> InvoiceHistoryRequest {
        InvoiceHistoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoice_histories_minimal() -> InvoiceHistoryRequest {
        InvoiceHistoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoice_histories_with_children() -> InvoiceHistoryRequest {
        InvoiceHistoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn dispute_records() -> DisputeRecordRequest {
        DisputeRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dispute_records_minimal() -> DisputeRecordRequest {
        DisputeRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dispute_records_with_children() -> DisputeRecordRequest {
        DisputeRecordRequest::new()
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

    pub fn contract_termses() -> ContractTermsRequest {
        ContractTermsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contract_termses_minimal() -> ContractTermsRequest {
        ContractTermsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contract_termses_with_children() -> ContractTermsRequest {
        ContractTermsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn renewal_notices() -> RenewalNoticeRequest {
        RenewalNoticeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn renewal_notices_minimal() -> RenewalNoticeRequest {
        RenewalNoticeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn renewal_notices_with_children() -> RenewalNoticeRequest {
        RenewalNoticeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn cancellation_requests() -> CancellationRequestRequest {
        CancellationRequestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cancellation_requests_minimal() -> CancellationRequestRequest {
        CancellationRequestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cancellation_requests_with_children() -> CancellationRequestRequest {
        CancellationRequestRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn referral_codes() -> ReferralCodeRequest {
        ReferralCodeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn referral_codes_minimal() -> ReferralCodeRequest {
        ReferralCodeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn referral_codes_with_children() -> ReferralCodeRequest {
        ReferralCodeRequest::new()
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

    pub fn lead_sources() -> LeadSourceRequest {
        LeadSourceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn lead_sources_minimal() -> LeadSourceRequest {
        LeadSourceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn lead_sources_with_children() -> LeadSourceRequest {
        LeadSourceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicle_registries() -> VehicleRegistryRequest {
        VehicleRegistryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_registries_minimal() -> VehicleRegistryRequest {
        VehicleRegistryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_registries_with_children() -> VehicleRegistryRequest {
        VehicleRegistryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicle_specs() -> VehicleSpecRequest {
        VehicleSpecRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_specs_minimal() -> VehicleSpecRequest {
        VehicleSpecRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_specs_with_children() -> VehicleSpecRequest {
        VehicleSpecRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn maintenance_logs() -> MaintenanceLogRequest {
        MaintenanceLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenance_logs_minimal() -> MaintenanceLogRequest {
        MaintenanceLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenance_logs_with_children() -> MaintenanceLogRequest {
        MaintenanceLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn fuel_records() -> FuelRecordRequest {
        FuelRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fuel_records_minimal() -> FuelRecordRequest {
        FuelRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fuel_records_with_children() -> FuelRecordRequest {
        FuelRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tire_inventory() -> TireInventoryRequest {
        TireInventoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tire_inventory_minimal() -> TireInventoryRequest {
        TireInventoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tire_inventory_with_children() -> TireInventoryRequest {
        TireInventoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn driver_assignments() -> DriverAssignmentRequest {
        DriverAssignmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_assignments_minimal() -> DriverAssignmentRequest {
        DriverAssignmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_assignments_with_children() -> DriverAssignmentRequest {
        DriverAssignmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn driver_licenses() -> DriverLicenseRequest {
        DriverLicenseRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_licenses_minimal() -> DriverLicenseRequest {
        DriverLicenseRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_licenses_with_children() -> DriverLicenseRequest {
        DriverLicenseRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn driver_trainings() -> DriverTrainingRequest {
        DriverTrainingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_trainings_minimal() -> DriverTrainingRequest {
        DriverTrainingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_trainings_with_children() -> DriverTrainingRequest {
        DriverTrainingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn route_plans() -> RoutePlanRequest {
        RoutePlanRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn route_plans_minimal() -> RoutePlanRequest {
        RoutePlanRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn route_plans_with_children() -> RoutePlanRequest {
        RoutePlanRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn load_capacities() -> LoadCapacityRequest {
        LoadCapacityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn load_capacities_minimal() -> LoadCapacityRequest {
        LoadCapacityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn load_capacities_with_children() -> LoadCapacityRequest {
        LoadCapacityRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn cargo_securements() -> CargoSecurementRequest {
        CargoSecurementRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cargo_securements_minimal() -> CargoSecurementRequest {
        CargoSecurementRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cargo_securements_with_children() -> CargoSecurementRequest {
        CargoSecurementRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn gps_trackings() -> GpsTrackingRequest {
        GpsTrackingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn gps_trackings_minimal() -> GpsTrackingRequest {
        GpsTrackingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn gps_trackings_with_children() -> GpsTrackingRequest {
        GpsTrackingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn telematics_data() -> TelematicsDataRequest {
        TelematicsDataRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn telematics_data_minimal() -> TelematicsDataRequest {
        TelematicsDataRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn telematics_data_with_children() -> TelematicsDataRequest {
        TelematicsDataRequest::new()
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

    pub fn inspection_checklists() -> InspectionChecklistRequest {
        InspectionChecklistRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inspection_checklists_minimal() -> InspectionChecklistRequest {
        InspectionChecklistRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inspection_checklists_with_children() -> InspectionChecklistRequest {
        InspectionChecklistRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn service_schedules() -> ServiceScheduleRequest {
        ServiceScheduleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_schedules_minimal() -> ServiceScheduleRequest {
        ServiceScheduleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_schedules_with_children() -> ServiceScheduleRequest {
        ServiceScheduleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn warranty_info() -> WarrantyInfoRequest {
        WarrantyInfoRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warranty_info_minimal() -> WarrantyInfoRequest {
        WarrantyInfoRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warranty_info_with_children() -> WarrantyInfoRequest {
        WarrantyInfoRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn decommission_records() -> DecommissionRecordRequest {
        DecommissionRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn decommission_records_minimal() -> DecommissionRecordRequest {
        DecommissionRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn decommission_records_with_children() -> DecommissionRecordRequest {
        DecommissionRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn invoice_templates() -> InvoiceTemplateRequest {
        InvoiceTemplateRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoice_templates_minimal() -> InvoiceTemplateRequest {
        InvoiceTemplateRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoice_templates_with_children() -> InvoiceTemplateRequest {
        InvoiceTemplateRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn billing_cycles() -> BillingCycleRequest {
        BillingCycleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_cycles_minimal() -> BillingCycleRequest {
        BillingCycleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_cycles_with_children() -> BillingCycleRequest {
        BillingCycleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tax_jurisdictions() -> TaxJurisdictionRequest {
        TaxJurisdictionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_jurisdictions_minimal() -> TaxJurisdictionRequest {
        TaxJurisdictionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_jurisdictions_with_children() -> TaxJurisdictionRequest {
        TaxJurisdictionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tax_rates() -> TaxRateRequest {
        TaxRateRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_rates_minimal() -> TaxRateRequest {
        TaxRateRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_rates_with_children() -> TaxRateRequest {
        TaxRateRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn discount_policies() -> DiscountPolicyRequest {
        DiscountPolicyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn discount_policies_minimal() -> DiscountPolicyRequest {
        DiscountPolicyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn discount_policies_with_children() -> DiscountPolicyRequest {
        DiscountPolicyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payment_gateways() -> PaymentGatewayRequest {
        PaymentGatewayRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payment_gateways_minimal() -> PaymentGatewayRequest {
        PaymentGatewayRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payment_gateways_with_children() -> PaymentGatewayRequest {
        PaymentGatewayRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn transaction_logs() -> TransactionLogRequest {
        TransactionLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn transaction_logs_minimal() -> TransactionLogRequest {
        TransactionLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn transaction_logs_with_children() -> TransactionLogRequest {
        TransactionLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn refund_processes() -> RefundProcessRequest {
        RefundProcessRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn refund_processes_minimal() -> RefundProcessRequest {
        RefundProcessRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn refund_processes_with_children() -> RefundProcessRequest {
        RefundProcessRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn credit_notes() -> CreditNoteRequest {
        CreditNoteRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn credit_notes_minimal() -> CreditNoteRequest {
        CreditNoteRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn credit_notes_with_children() -> CreditNoteRequest {
        CreditNoteRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn debit_notes() -> DebitNoteRequest {
        DebitNoteRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn debit_notes_minimal() -> DebitNoteRequest {
        DebitNoteRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn debit_notes_with_children() -> DebitNoteRequest {
        DebitNoteRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn expense_categories() -> ExpenseCategoryRequest {
        ExpenseCategoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expense_categories_minimal() -> ExpenseCategoryRequest {
        ExpenseCategoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expense_categories_with_children() -> ExpenseCategoryRequest {
        ExpenseCategoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn cost_centers() -> CostCenterRequest {
        CostCenterRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cost_centers_minimal() -> CostCenterRequest {
        CostCenterRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cost_centers_with_children() -> CostCenterRequest {
        CostCenterRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn budget_allocations() -> BudgetAllocationRequest {
        BudgetAllocationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn budget_allocations_minimal() -> BudgetAllocationRequest {
        BudgetAllocationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn budget_allocations_with_children() -> BudgetAllocationRequest {
        BudgetAllocationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn financial_reports() -> FinancialReportRequest {
        FinancialReportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn financial_reports_minimal() -> FinancialReportRequest {
        FinancialReportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn financial_reports_with_children() -> FinancialReportRequest {
        FinancialReportRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn audit_trails() -> AuditTrailRequest {
        AuditTrailRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audit_trails_minimal() -> AuditTrailRequest {
        AuditTrailRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audit_trails_with_children() -> AuditTrailRequest {
        AuditTrailRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn reconciliation_entries() -> ReconciliationEntryRequest {
        ReconciliationEntryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn reconciliation_entries_minimal() -> ReconciliationEntryRequest {
        ReconciliationEntryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn reconciliation_entries_with_children() -> ReconciliationEntryRequest {
        ReconciliationEntryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn currency_conversions() -> CurrencyConversionRequest {
        CurrencyConversionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn currency_conversions_minimal() -> CurrencyConversionRequest {
        CurrencyConversionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn currency_conversions_with_children() -> CurrencyConversionRequest {
        CurrencyConversionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn fiscal_periods() -> FiscalPeriodRequest {
        FiscalPeriodRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fiscal_periods_minimal() -> FiscalPeriodRequest {
        FiscalPeriodRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fiscal_periods_with_children() -> FiscalPeriodRequest {
        FiscalPeriodRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn job_orders() -> JobOrderRequest {
        JobOrderRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn job_orders_minimal() -> JobOrderRequest {
        JobOrderRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn job_orders_with_children() -> JobOrderRequest {
        JobOrderRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn move_schedules() -> MoveScheduleRequest {
        MoveScheduleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_schedules_minimal() -> MoveScheduleRequest {
        MoveScheduleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_schedules_with_children() -> MoveScheduleRequest {
        MoveScheduleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn crew_assignments() -> CrewAssignmentRequest {
        CrewAssignmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn crew_assignments_minimal() -> CrewAssignmentRequest {
        CrewAssignmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn crew_assignments_with_children() -> CrewAssignmentRequest {
        CrewAssignmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn equipment_allocations() -> EquipmentAllocationRequest {
        EquipmentAllocationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_allocations_minimal() -> EquipmentAllocationRequest {
        EquipmentAllocationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_allocations_with_children() -> EquipmentAllocationRequest {
        EquipmentAllocationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn pickup_locations() -> PickupLocationRequest {
        PickupLocationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pickup_locations_minimal() -> PickupLocationRequest {
        PickupLocationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pickup_locations_with_children() -> PickupLocationRequest {
        PickupLocationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn delivery_locations() -> DeliveryLocationRequest {
        DeliveryLocationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn delivery_locations_minimal() -> DeliveryLocationRequest {
        DeliveryLocationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn delivery_locations_with_children() -> DeliveryLocationRequest {
        DeliveryLocationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn transit_time_estimates() -> TransitTimeEstimateRequest {
        TransitTimeEstimateRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn transit_time_estimates_minimal() -> TransitTimeEstimateRequest {
        TransitTimeEstimateRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn transit_time_estimates_with_children() -> TransitTimeEstimateRequest {
        TransitTimeEstimateRequest::new()
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

    pub fn customs_documentations() -> CustomsDocumentationRequest {
        CustomsDocumentationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customs_documentations_minimal() -> CustomsDocumentationRequest {
        CustomsDocumentationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customs_documentations_with_children() -> CustomsDocumentationRequest {
        CustomsDocumentationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn permit_requireds() -> PermitRequiredRequest {
        PermitRequiredRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn permit_requireds_minimal() -> PermitRequiredRequest {
        PermitRequiredRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn permit_requireds_with_children() -> PermitRequiredRequest {
        PermitRequiredRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn insurance_coverages() -> InsuranceCoverageRequest {
        InsuranceCoverageRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn insurance_coverages_minimal() -> InsuranceCoverageRequest {
        InsuranceCoverageRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn insurance_coverages_with_children() -> InsuranceCoverageRequest {
        InsuranceCoverageRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn liability_waivers() -> LiabilityWaiverRequest {
        LiabilityWaiverRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn liability_waivers_minimal() -> LiabilityWaiverRequest {
        LiabilityWaiverRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn liability_waivers_with_children() -> LiabilityWaiverRequest {
        LiabilityWaiverRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tracking_numbers() -> TrackingNumberRequest {
        TrackingNumberRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tracking_numbers_minimal() -> TrackingNumberRequest {
        TrackingNumberRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tracking_numbers_with_children() -> TrackingNumberRequest {
        TrackingNumberRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn status_updates() -> StatusUpdateRequest {
        StatusUpdateRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn status_updates_minimal() -> StatusUpdateRequest {
        StatusUpdateRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn status_updates_with_children() -> StatusUpdateRequest {
        StatusUpdateRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn notification_templates() -> NotificationTemplateRequest {
        NotificationTemplateRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notification_templates_minimal() -> NotificationTemplateRequest {
        NotificationTemplateRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notification_templates_with_children() -> NotificationTemplateRequest {
        NotificationTemplateRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn sla_metrics() -> SlaMetricRequest {
        SlaMetricRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sla_metrics_minimal() -> SlaMetricRequest {
        SlaMetricRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sla_metrics_with_children() -> SlaMetricRequest {
        SlaMetricRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn performance_kpis() -> PerformanceKpiRequest {
        PerformanceKpiRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn performance_kpis_minimal() -> PerformanceKpiRequest {
        PerformanceKpiRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn performance_kpis_with_children() -> PerformanceKpiRequest {
        PerformanceKpiRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn employee_records() -> EmployeeRecordRequest {
        EmployeeRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employee_records_minimal() -> EmployeeRecordRequest {
        EmployeeRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employee_records_with_children() -> EmployeeRecordRequest {
        EmployeeRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payroll_info() -> PayrollInfoRequest {
        PayrollInfoRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_info_minimal() -> PayrollInfoRequest {
        PayrollInfoRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_info_with_children() -> PayrollInfoRequest {
        PayrollInfoRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn benefits_plans() -> BenefitsPlanRequest {
        BenefitsPlanRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn benefits_plans_minimal() -> BenefitsPlanRequest {
        BenefitsPlanRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn benefits_plans_with_children() -> BenefitsPlanRequest {
        BenefitsPlanRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn time_off_requests() -> TimeOffRequestRequest {
        TimeOffRequestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn time_off_requests_minimal() -> TimeOffRequestRequest {
        TimeOffRequestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn time_off_requests_with_children() -> TimeOffRequestRequest {
        TimeOffRequestRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn shift_schedules() -> ShiftScheduleRequest {
        ShiftScheduleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shift_schedules_minimal() -> ShiftScheduleRequest {
        ShiftScheduleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shift_schedules_with_children() -> ShiftScheduleRequest {
        ShiftScheduleRequest::new()
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

    pub fn competency_matrixes() -> CompetencyMatrixRequest {
        CompetencyMatrixRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn competency_matrixes_minimal() -> CompetencyMatrixRequest {
        CompetencyMatrixRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn competency_matrixes_with_children() -> CompetencyMatrixRequest {
        CompetencyMatrixRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn training_courses() -> TrainingCourseRequest {
        TrainingCourseRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn training_courses_minimal() -> TrainingCourseRequest {
        TrainingCourseRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn training_courses_with_children() -> TrainingCourseRequest {
        TrainingCourseRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn certification_records() -> CertificationRecordRequest {
        CertificationRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn certification_records_minimal() -> CertificationRecordRequest {
        CertificationRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn certification_records_with_children() -> CertificationRecordRequest {
        CertificationRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn safety_incidents() -> SafetyIncidentRequest {
        SafetyIncidentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn safety_incidents_minimal() -> SafetyIncidentRequest {
        SafetyIncidentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn safety_incidents_with_children() -> SafetyIncidentRequest {
        SafetyIncidentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn hazard_assessments() -> HazardAssessmentRequest {
        HazardAssessmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn hazard_assessments_minimal() -> HazardAssessmentRequest {
        HazardAssessmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn hazard_assessments_with_children() -> HazardAssessmentRequest {
        HazardAssessmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn policy_acknowledgments() -> PolicyAcknowledgmentRequest {
        PolicyAcknowledgmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn policy_acknowledgments_minimal() -> PolicyAcknowledgmentRequest {
        PolicyAcknowledgmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn policy_acknowledgments_with_children() -> PolicyAcknowledgmentRequest {
        PolicyAcknowledgmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn grievance_logs() -> GrievanceLogRequest {
        GrievanceLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn grievance_logs_minimal() -> GrievanceLogRequest {
        GrievanceLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn grievance_logs_with_children() -> GrievanceLogRequest {
        GrievanceLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn disciplinary_actions() -> DisciplinaryActionRequest {
        DisciplinaryActionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn disciplinary_actions_minimal() -> DisciplinaryActionRequest {
        DisciplinaryActionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn disciplinary_actions_with_children() -> DisciplinaryActionRequest {
        DisciplinaryActionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn exit_interviews() -> ExitInterviewRequest {
        ExitInterviewRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn exit_interviews_minimal() -> ExitInterviewRequest {
        ExitInterviewRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn exit_interviews_with_children() -> ExitInterviewRequest {
        ExitInterviewRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn onboarding_checklists() -> OnboardingChecklistRequest {
        OnboardingChecklistRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn onboarding_checklists_minimal() -> OnboardingChecklistRequest {
        OnboardingChecklistRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn onboarding_checklists_with_children() -> OnboardingChecklistRequest {
        OnboardingChecklistRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn offboarding_checklists() -> OffboardingChecklistRequest {
        OffboardingChecklistRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn offboarding_checklists_minimal() -> OffboardingChecklistRequest {
        OffboardingChecklistRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn offboarding_checklists_with_children() -> OffboardingChecklistRequest {
        OffboardingChecklistRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn compliance_audits() -> ComplianceAuditRequest {
        ComplianceAuditRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn compliance_audits_minimal() -> ComplianceAuditRequest {
        ComplianceAuditRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn compliance_audits_with_children() -> ComplianceAuditRequest {
        ComplianceAuditRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}