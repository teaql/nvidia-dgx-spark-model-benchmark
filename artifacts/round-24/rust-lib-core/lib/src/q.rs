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

    pub fn customer_contracts() -> CustomerContractRequest {
        CustomerContractRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_contracts_minimal() -> CustomerContractRequest {
        CustomerContractRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_contracts_with_children() -> CustomerContractRequest {
        CustomerContractRequest::new()
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

    pub fn customer_loyalties() -> CustomerLoyaltyRequest {
        CustomerLoyaltyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_loyalties_minimal() -> CustomerLoyaltyRequest {
        CustomerLoyaltyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_loyalties_with_children() -> CustomerLoyaltyRequest {
        CustomerLoyaltyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_invoices() -> CustomerInvoiceRequest {
        CustomerInvoiceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_invoices_minimal() -> CustomerInvoiceRequest {
        CustomerInvoiceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_invoices_with_children() -> CustomerInvoiceRequest {
        CustomerInvoiceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_payments() -> CustomerPaymentRequest {
        CustomerPaymentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_payments_minimal() -> CustomerPaymentRequest {
        CustomerPaymentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_payments_with_children() -> CustomerPaymentRequest {
        CustomerPaymentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_claims() -> CustomerClaimRequest {
        CustomerClaimRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_claims_minimal() -> CustomerClaimRequest {
        CustomerClaimRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_claims_with_children() -> CustomerClaimRequest {
        CustomerClaimRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_notifications() -> CustomerNotificationRequest {
        CustomerNotificationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_notifications_minimal() -> CustomerNotificationRequest {
        CustomerNotificationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_notifications_with_children() -> CustomerNotificationRequest {
        CustomerNotificationRequest::new()
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

    pub fn customer_leads() -> CustomerLeadRequest {
        CustomerLeadRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_leads_minimal() -> CustomerLeadRequest {
        CustomerLeadRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_leads_with_children() -> CustomerLeadRequest {
        CustomerLeadRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_quotes() -> CustomerQuoteRequest {
        CustomerQuoteRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_quotes_minimal() -> CustomerQuoteRequest {
        CustomerQuoteRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_quotes_with_children() -> CustomerQuoteRequest {
        CustomerQuoteRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_services() -> CustomerServiceRequest {
        CustomerServiceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_services_minimal() -> CustomerServiceRequest {
        CustomerServiceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_services_with_children() -> CustomerServiceRequest {
        CustomerServiceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_support_tickets() -> CustomerSupportTicketRequest {
        CustomerSupportTicketRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_support_tickets_minimal() -> CustomerSupportTicketRequest {
        CustomerSupportTicketRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_support_tickets_with_children() -> CustomerSupportTicketRequest {
        CustomerSupportTicketRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_vehicles() -> CustomerVehicleRequest {
        CustomerVehicleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_vehicles_minimal() -> CustomerVehicleRequest {
        CustomerVehicleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_vehicles_with_children() -> CustomerVehicleRequest {
        CustomerVehicleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_move_histories() -> CustomerMoveHistoryRequest {
        CustomerMoveHistoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_move_histories_minimal() -> CustomerMoveHistoryRequest {
        CustomerMoveHistoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_move_histories_with_children() -> CustomerMoveHistoryRequest {
        CustomerMoveHistoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_preferred_times() -> CustomerPreferredTimeRequest {
        CustomerPreferredTimeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_preferred_times_minimal() -> CustomerPreferredTimeRequest {
        CustomerPreferredTimeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_preferred_times_with_children() -> CustomerPreferredTimeRequest {
        CustomerPreferredTimeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn fleet_vehicles() -> FleetVehicleRequest {
        FleetVehicleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fleet_vehicles_minimal() -> FleetVehicleRequest {
        FleetVehicleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fleet_vehicles_with_children() -> FleetVehicleRequest {
        FleetVehicleRequest::new()
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

    pub fn vehicle_maintenances() -> VehicleMaintenanceRequest {
        VehicleMaintenanceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_maintenances_minimal() -> VehicleMaintenanceRequest {
        VehicleMaintenanceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_maintenances_with_children() -> VehicleMaintenanceRequest {
        VehicleMaintenanceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicle_inspections() -> VehicleInspectionRequest {
        VehicleInspectionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_inspections_minimal() -> VehicleInspectionRequest {
        VehicleInspectionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_inspections_with_children() -> VehicleInspectionRequest {
        VehicleInspectionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicle_assignments() -> VehicleAssignmentRequest {
        VehicleAssignmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_assignments_minimal() -> VehicleAssignmentRequest {
        VehicleAssignmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_assignments_with_children() -> VehicleAssignmentRequest {
        VehicleAssignmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicle_utilizations() -> VehicleUtilizationRequest {
        VehicleUtilizationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_utilizations_minimal() -> VehicleUtilizationRequest {
        VehicleUtilizationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_utilizations_with_children() -> VehicleUtilizationRequest {
        VehicleUtilizationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicle_fuel_logs() -> VehicleFuelLogRequest {
        VehicleFuelLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_fuel_logs_minimal() -> VehicleFuelLogRequest {
        VehicleFuelLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_fuel_logs_with_children() -> VehicleFuelLogRequest {
        VehicleFuelLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicle_odometers() -> VehicleOdometerRequest {
        VehicleOdometerRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_odometers_minimal() -> VehicleOdometerRequest {
        VehicleOdometerRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_odometers_with_children() -> VehicleOdometerRequest {
        VehicleOdometerRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicle_insurances() -> VehicleInsuranceRequest {
        VehicleInsuranceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_insurances_minimal() -> VehicleInsuranceRequest {
        VehicleInsuranceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_insurances_with_children() -> VehicleInsuranceRequest {
        VehicleInsuranceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicle_registrations() -> VehicleRegistrationRequest {
        VehicleRegistrationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_registrations_minimal() -> VehicleRegistrationRequest {
        VehicleRegistrationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_registrations_with_children() -> VehicleRegistrationRequest {
        VehicleRegistrationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicle_damage_reports() -> VehicleDamageReportRequest {
        VehicleDamageReportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_damage_reports_minimal() -> VehicleDamageReportRequest {
        VehicleDamageReportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_damage_reports_with_children() -> VehicleDamageReportRequest {
        VehicleDamageReportRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicle_cleaning_schedules() -> VehicleCleaningScheduleRequest {
        VehicleCleaningScheduleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_cleaning_schedules_minimal() -> VehicleCleaningScheduleRequest {
        VehicleCleaningScheduleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_cleaning_schedules_with_children() -> VehicleCleaningScheduleRequest {
        VehicleCleaningScheduleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn fleet_operators() -> FleetOperatorRequest {
        FleetOperatorRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fleet_operators_minimal() -> FleetOperatorRequest {
        FleetOperatorRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fleet_operators_with_children() -> FleetOperatorRequest {
        FleetOperatorRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn driver_profiles() -> DriverProfileRequest {
        DriverProfileRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_profiles_minimal() -> DriverProfileRequest {
        DriverProfileRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_profiles_with_children() -> DriverProfileRequest {
        DriverProfileRequest::new()
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

    pub fn driver_certifications() -> DriverCertificationRequest {
        DriverCertificationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_certifications_minimal() -> DriverCertificationRequest {
        DriverCertificationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_certifications_with_children() -> DriverCertificationRequest {
        DriverCertificationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn driver_availabilities() -> DriverAvailabilityRequest {
        DriverAvailabilityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_availabilities_minimal() -> DriverAvailabilityRequest {
        DriverAvailabilityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_availabilities_with_children() -> DriverAvailabilityRequest {
        DriverAvailabilityRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn driver_performances() -> DriverPerformanceRequest {
        DriverPerformanceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_performances_minimal() -> DriverPerformanceRequest {
        DriverPerformanceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_performances_with_children() -> DriverPerformanceRequest {
        DriverPerformanceRequest::new()
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

    pub fn fleet_dispatches() -> FleetDispatchRequest {
        FleetDispatchRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fleet_dispatches_minimal() -> FleetDispatchRequest {
        FleetDispatchRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fleet_dispatches_with_children() -> FleetDispatchRequest {
        FleetDispatchRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn invoice_headers() -> InvoiceHeaderRequest {
        InvoiceHeaderRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoice_headers_minimal() -> InvoiceHeaderRequest {
        InvoiceHeaderRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoice_headers_with_children() -> InvoiceHeaderRequest {
        InvoiceHeaderRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn invoice_line_items() -> InvoiceLineItemRequest {
        InvoiceLineItemRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoice_line_items_minimal() -> InvoiceLineItemRequest {
        InvoiceLineItemRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoice_line_items_with_children() -> InvoiceLineItemRequest {
        InvoiceLineItemRequest::new()
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

    pub fn payment_transactions() -> PaymentTransactionRequest {
        PaymentTransactionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payment_transactions_minimal() -> PaymentTransactionRequest {
        PaymentTransactionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payment_transactions_with_children() -> PaymentTransactionRequest {
        PaymentTransactionRequest::new()
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

    pub fn tax_codes() -> TaxCodeRequest {
        TaxCodeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_codes_minimal() -> TaxCodeRequest {
        TaxCodeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_codes_with_children() -> TaxCodeRequest {
        TaxCodeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn discount_rules() -> DiscountRuleRequest {
        DiscountRuleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn discount_rules_minimal() -> DiscountRuleRequest {
        DiscountRuleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn discount_rules_with_children() -> DiscountRuleRequest {
        DiscountRuleRequest::new()
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

    pub fn billing_addresses() -> BillingAddressRequest {
        BillingAddressRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_addresses_minimal() -> BillingAddressRequest {
        BillingAddressRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_addresses_with_children() -> BillingAddressRequest {
        BillingAddressRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn outstanding_balances() -> OutstandingBalanceRequest {
        OutstandingBalanceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn outstanding_balances_minimal() -> OutstandingBalanceRequest {
        OutstandingBalanceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn outstanding_balances_with_children() -> OutstandingBalanceRequest {
        OutstandingBalanceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn aging_reports() -> AgingReportRequest {
        AgingReportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn aging_reports_minimal() -> AgingReportRequest {
        AgingReportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn aging_reports_with_children() -> AgingReportRequest {
        AgingReportRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payment_reminders() -> PaymentReminderRequest {
        PaymentReminderRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payment_reminders_minimal() -> PaymentReminderRequest {
        PaymentReminderRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payment_reminders_with_children() -> PaymentReminderRequest {
        PaymentReminderRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn refund_requests() -> RefundRequestRequest {
        RefundRequestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn refund_requests_minimal() -> RefundRequestRequest {
        RefundRequestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn refund_requests_with_children() -> RefundRequestRequest {
        RefundRequestRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn billing_adjustments() -> BillingAdjustmentRequest {
        BillingAdjustmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_adjustments_minimal() -> BillingAdjustmentRequest {
        BillingAdjustmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_adjustments_with_children() -> BillingAdjustmentRequest {
        BillingAdjustmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn revenue_recognitions() -> RevenueRecognitionRequest {
        RevenueRecognitionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn revenue_recognitions_minimal() -> RevenueRecognitionRequest {
        RevenueRecognitionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn revenue_recognitions_with_children() -> RevenueRecognitionRequest {
        RevenueRecognitionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn financial_periods() -> FinancialPeriodRequest {
        FinancialPeriodRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn financial_periods_minimal() -> FinancialPeriodRequest {
        FinancialPeriodRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn financial_periods_with_children() -> FinancialPeriodRequest {
        FinancialPeriodRequest::new()
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

    pub fn currency_rates() -> CurrencyRateRequest {
        CurrencyRateRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn currency_rates_minimal() -> CurrencyRateRequest {
        CurrencyRateRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn currency_rates_with_children() -> CurrencyRateRequest {
        CurrencyRateRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn billing_approvals() -> BillingApprovalRequest {
        BillingApprovalRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_approvals_minimal() -> BillingApprovalRequest {
        BillingApprovalRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_approvals_with_children() -> BillingApprovalRequest {
        BillingApprovalRequest::new()
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

    pub fn load_plans() -> LoadPlanRequest {
        LoadPlanRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn load_plans_minimal() -> LoadPlanRequest {
        LoadPlanRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn load_plans_with_children() -> LoadPlanRequest {
        LoadPlanRequest::new()
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

    pub fn equipment_checklists() -> EquipmentChecklistRequest {
        EquipmentChecklistRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_checklists_minimal() -> EquipmentChecklistRequest {
        EquipmentChecklistRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_checklists_with_children() -> EquipmentChecklistRequest {
        EquipmentChecklistRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn loading_procedures() -> LoadingProcedureRequest {
        LoadingProcedureRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loading_procedures_minimal() -> LoadingProcedureRequest {
        LoadingProcedureRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loading_procedures_with_children() -> LoadingProcedureRequest {
        LoadingProcedureRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn unloading_procedures() -> UnloadingProcedureRequest {
        UnloadingProcedureRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn unloading_procedures_minimal() -> UnloadingProcedureRequest {
        UnloadingProcedureRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn unloading_procedures_with_children() -> UnloadingProcedureRequest {
        UnloadingProcedureRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn transit_monitorings() -> TransitMonitoringRequest {
        TransitMonitoringRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn transit_monitorings_minimal() -> TransitMonitoringRequest {
        TransitMonitoringRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn transit_monitorings_with_children() -> TransitMonitoringRequest {
        TransitMonitoringRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn delivery_confirmations() -> DeliveryConfirmationRequest {
        DeliveryConfirmationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn delivery_confirmations_minimal() -> DeliveryConfirmationRequest {
        DeliveryConfirmationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn delivery_confirmations_with_children() -> DeliveryConfirmationRequest {
        DeliveryConfirmationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn exception_handlings() -> ExceptionHandlingRequest {
        ExceptionHandlingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn exception_handlings_minimal() -> ExceptionHandlingRequest {
        ExceptionHandlingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn exception_handlings_with_children() -> ExceptionHandlingRequest {
        ExceptionHandlingRequest::new()
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

    pub fn inventory_snapshots() -> InventorySnapshotRequest {
        InventorySnapshotRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_snapshots_minimal() -> InventorySnapshotRequest {
        InventorySnapshotRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_snapshots_with_children() -> InventorySnapshotRequest {
        InventorySnapshotRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn warehouse_allocations() -> WarehouseAllocationRequest {
        WarehouseAllocationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warehouse_allocations_minimal() -> WarehouseAllocationRequest {
        WarehouseAllocationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warehouse_allocations_with_children() -> WarehouseAllocationRequest {
        WarehouseAllocationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn dock_schedulings() -> DockSchedulingRequest {
        DockSchedulingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dock_schedulings_minimal() -> DockSchedulingRequest {
        DockSchedulingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dock_schedulings_with_children() -> DockSchedulingRequest {
        DockSchedulingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn yard_managements() -> YardManagementRequest {
        YardManagementRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn yard_managements_minimal() -> YardManagementRequest {
        YardManagementRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn yard_managements_with_children() -> YardManagementRequest {
        YardManagementRequest::new()
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

    pub fn compliance_checks() -> ComplianceCheckRequest {
        ComplianceCheckRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn compliance_checks_minimal() -> ComplianceCheckRequest {
        ComplianceCheckRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn compliance_checks_with_children() -> ComplianceCheckRequest {
        ComplianceCheckRequest::new()
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

    pub fn operations_dashboards() -> OperationsDashboardRequest {
        OperationsDashboardRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn operations_dashboards_minimal() -> OperationsDashboardRequest {
        OperationsDashboardRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn operations_dashboards_with_children() -> OperationsDashboardRequest {
        OperationsDashboardRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn daily_summaries() -> DailySummaryRequest {
        DailySummaryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn daily_summaries_minimal() -> DailySummaryRequest {
        DailySummaryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn daily_summaries_with_children() -> DailySummaryRequest {
        DailySummaryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn weekly_reports() -> WeeklyReportRequest {
        WeeklyReportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn weekly_reports_minimal() -> WeeklyReportRequest {
        WeeklyReportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn weekly_reports_with_children() -> WeeklyReportRequest {
        WeeklyReportRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn monthly_kpis() -> MonthlyKpiRequest {
        MonthlyKpiRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn monthly_kpis_minimal() -> MonthlyKpiRequest {
        MonthlyKpiRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn monthly_kpis_with_children() -> MonthlyKpiRequest {
        MonthlyKpiRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn annual_performances() -> AnnualPerformanceRequest {
        AnnualPerformanceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn annual_performances_minimal() -> AnnualPerformanceRequest {
        AnnualPerformanceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn annual_performances_with_children() -> AnnualPerformanceRequest {
        AnnualPerformanceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn utilization_reports() -> UtilizationReportRequest {
        UtilizationReportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn utilization_reports_minimal() -> UtilizationReportRequest {
        UtilizationReportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn utilization_reports_with_children() -> UtilizationReportRequest {
        UtilizationReportRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn cost_analyses() -> CostAnalysisRequest {
        CostAnalysisRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cost_analyses_minimal() -> CostAnalysisRequest {
        CostAnalysisRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cost_analyses_with_children() -> CostAnalysisRequest {
        CostAnalysisRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn profit_margins() -> ProfitMarginRequest {
        ProfitMarginRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn profit_margins_minimal() -> ProfitMarginRequest {
        ProfitMarginRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn profit_margins_with_children() -> ProfitMarginRequest {
        ProfitMarginRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_satisfactions() -> CustomerSatisfactionRequest {
        CustomerSatisfactionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_satisfactions_minimal() -> CustomerSatisfactionRequest {
        CustomerSatisfactionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_satisfactions_with_children() -> CustomerSatisfactionRequest {
        CustomerSatisfactionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn on_time_deliveries() -> OnTimeDeliveryRequest {
        OnTimeDeliveryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn on_time_deliveries_minimal() -> OnTimeDeliveryRequest {
        OnTimeDeliveryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn on_time_deliveries_with_children() -> OnTimeDeliveryRequest {
        OnTimeDeliveryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn claim_rates() -> ClaimRateRequest {
        ClaimRateRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn claim_rates_minimal() -> ClaimRateRequest {
        ClaimRateRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn claim_rates_with_children() -> ClaimRateRequest {
        ClaimRateRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn fleet_efficiencies() -> FleetEfficiencyRequest {
        FleetEfficiencyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fleet_efficiencies_minimal() -> FleetEfficiencyRequest {
        FleetEfficiencyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fleet_efficiencies_with_children() -> FleetEfficiencyRequest {
        FleetEfficiencyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn driver_productivities() -> DriverProductivityRequest {
        DriverProductivityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_productivities_minimal() -> DriverProductivityRequest {
        DriverProductivityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driver_productivities_with_children() -> DriverProductivityRequest {
        DriverProductivityRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn billing_accuracies() -> BillingAccuracyRequest {
        BillingAccuracyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_accuracies_minimal() -> BillingAccuracyRequest {
        BillingAccuracyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_accuracies_with_children() -> BillingAccuracyRequest {
        BillingAccuracyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn invoice_agings() -> InvoiceAgingRequest {
        InvoiceAgingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoice_agings_minimal() -> InvoiceAgingRequest {
        InvoiceAgingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoice_agings_with_children() -> InvoiceAgingRequest {
        InvoiceAgingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn move_volume_trends() -> MoveVolumeTrendRequest {
        MoveVolumeTrendRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_volume_trends_minimal() -> MoveVolumeTrendRequest {
        MoveVolumeTrendRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_volume_trends_with_children() -> MoveVolumeTrendRequest {
        MoveVolumeTrendRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn geographic_distributions() -> GeographicDistributionRequest {
        GeographicDistributionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn geographic_distributions_minimal() -> GeographicDistributionRequest {
        GeographicDistributionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn geographic_distributions_with_children() -> GeographicDistributionRequest {
        GeographicDistributionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn service_line_performances() -> ServiceLinePerformanceRequest {
        ServiceLinePerformanceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_line_performances_minimal() -> ServiceLinePerformanceRequest {
        ServiceLinePerformanceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_line_performances_with_children() -> ServiceLinePerformanceRequest {
        ServiceLinePerformanceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn expense_variances() -> ExpenseVarianceRequest {
        ExpenseVarianceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expense_variances_minimal() -> ExpenseVarianceRequest {
        ExpenseVarianceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expense_variances_with_children() -> ExpenseVarianceRequest {
        ExpenseVarianceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn forecast_vs_actuals() -> ForecastVsActualRequest {
        ForecastVsActualRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn forecast_vs_actuals_minimal() -> ForecastVsActualRequest {
        ForecastVsActualRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn forecast_vs_actuals_with_children() -> ForecastVsActualRequest {
        ForecastVsActualRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn executive_dashboards() -> ExecutiveDashboardRequest {
        ExecutiveDashboardRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn executive_dashboards_minimal() -> ExecutiveDashboardRequest {
        ExecutiveDashboardRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn executive_dashboards_with_children() -> ExecutiveDashboardRequest {
        ExecutiveDashboardRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}