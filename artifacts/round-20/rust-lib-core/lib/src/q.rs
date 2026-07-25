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

    pub fn address_books() -> AddressBookRequest {
        AddressBookRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn address_books_minimal() -> AddressBookRequest {
        AddressBookRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn address_books_with_children() -> AddressBookRequest {
        AddressBookRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn contact_persons() -> ContactPersonRequest {
        ContactPersonRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contact_persons_minimal() -> ContactPersonRequest {
        ContactPersonRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contact_persons_with_children() -> ContactPersonRequest {
        ContactPersonRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn account_settingses() -> AccountSettingsRequest {
        AccountSettingsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn account_settingses_minimal() -> AccountSettingsRequest {
        AccountSettingsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn account_settingses_with_children() -> AccountSettingsRequest {
        AccountSettingsRequest::new()
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

    pub fn service_histories() -> ServiceHistoryRequest {
        ServiceHistoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_histories_minimal() -> ServiceHistoryRequest {
        ServiceHistoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_histories_with_children() -> ServiceHistoryRequest {
        ServiceHistoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn feedback_reviews() -> FeedbackReviewRequest {
        FeedbackReviewRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn feedback_reviews_minimal() -> FeedbackReviewRequest {
        FeedbackReviewRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn feedback_reviews_with_children() -> FeedbackReviewRequest {
        FeedbackReviewRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn dispute_cases() -> DisputeCaseRequest {
        DisputeCaseRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dispute_cases_minimal() -> DisputeCaseRequest {
        DisputeCaseRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dispute_cases_with_children() -> DisputeCaseRequest {
        DisputeCaseRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn document_uploads() -> DocumentUploadRequest {
        DocumentUploadRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn document_uploads_minimal() -> DocumentUploadRequest {
        DocumentUploadRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn document_uploads_with_children() -> DocumentUploadRequest {
        DocumentUploadRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn preference_centers() -> PreferenceCenterRequest {
        PreferenceCenterRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn preference_centers_minimal() -> PreferenceCenterRequest {
        PreferenceCenterRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn preference_centers_with_children() -> PreferenceCenterRequest {
        PreferenceCenterRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn notification_prefs() -> NotificationPrefRequest {
        NotificationPrefRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notification_prefs_minimal() -> NotificationPrefRequest {
        NotificationPrefRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notification_prefs_with_children() -> NotificationPrefRequest {
        NotificationPrefRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn billing_contacts() -> BillingContactRequest {
        BillingContactRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_contacts_minimal() -> BillingContactRequest {
        BillingContactRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_contacts_with_children() -> BillingContactRequest {
        BillingContactRequest::new()
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

    pub fn load_manifests() -> LoadManifestRequest {
        LoadManifestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn load_manifests_minimal() -> LoadManifestRequest {
        LoadManifestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn load_manifests_with_children() -> LoadManifestRequest {
        LoadManifestRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn equipment_inventory() -> EquipmentInventoryRequest {
        EquipmentInventoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_inventory_minimal() -> EquipmentInventoryRequest {
        EquipmentInventoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_inventory_with_children() -> EquipmentInventoryRequest {
        EquipmentInventoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn garage_assignments() -> GarageAssignmentRequest {
        GarageAssignmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn garage_assignments_minimal() -> GarageAssignmentRequest {
        GarageAssignmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn garage_assignments_with_children() -> GarageAssignmentRequest {
        GarageAssignmentRequest::new()
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

    pub fn compliance_certificates() -> ComplianceCertificateRequest {
        ComplianceCertificateRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn compliance_certificates_minimal() -> ComplianceCertificateRequest {
        ComplianceCertificateRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn compliance_certificates_with_children() -> ComplianceCertificateRequest {
        ComplianceCertificateRequest::new()
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

    pub fn tax_calculations() -> TaxCalculationRequest {
        TaxCalculationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_calculations_minimal() -> TaxCalculationRequest {
        TaxCalculationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_calculations_with_children() -> TaxCalculationRequest {
        TaxCalculationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn credit_memoes() -> CreditMemoRequest {
        CreditMemoRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn credit_memoes_minimal() -> CreditMemoRequest {
        CreditMemoRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn credit_memoes_with_children() -> CreditMemoRequest {
        CreditMemoRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn deposit_receipts() -> DepositReceiptRequest {
        DepositReceiptRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn deposit_receipts_minimal() -> DepositReceiptRequest {
        DepositReceiptRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn deposit_receipts_with_children() -> DepositReceiptRequest {
        DepositReceiptRequest::new()
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

    pub fn currency_exchanges() -> CurrencyExchangeRequest {
        CurrencyExchangeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn currency_exchanges_minimal() -> CurrencyExchangeRequest {
        CurrencyExchangeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn currency_exchanges_with_children() -> CurrencyExchangeRequest {
        CurrencyExchangeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn receivable_agings() -> ReceivableAgingRequest {
        ReceivableAgingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn receivable_agings_minimal() -> ReceivableAgingRequest {
        ReceivableAgingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn receivable_agings_with_children() -> ReceivableAgingRequest {
        ReceivableAgingRequest::new()
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

    pub fn payroll_runs() -> PayrollRunRequest {
        PayrollRunRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_runs_minimal() -> PayrollRunRequest {
        PayrollRunRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_runs_with_children() -> PayrollRunRequest {
        PayrollRunRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn timesheet_entries() -> TimesheetEntryRequest {
        TimesheetEntryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn timesheet_entries_minimal() -> TimesheetEntryRequest {
        TimesheetEntryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn timesheet_entries_with_children() -> TimesheetEntryRequest {
        TimesheetEntryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn benefit_plans() -> BenefitPlanRequest {
        BenefitPlanRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn benefit_plans_minimal() -> BenefitPlanRequest {
        BenefitPlanRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn benefit_plans_with_children() -> BenefitPlanRequest {
        BenefitPlanRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tax_withholdings() -> TaxWithholdingRequest {
        TaxWithholdingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_withholdings_minimal() -> TaxWithholdingRequest {
        TaxWithholdingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_withholdings_with_children() -> TaxWithholdingRequest {
        TaxWithholdingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn leave_requests() -> LeaveRequestRequest {
        LeaveRequestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn leave_requests_minimal() -> LeaveRequestRequest {
        LeaveRequestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn leave_requests_with_children() -> LeaveRequestRequest {
        LeaveRequestRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn training_records() -> TrainingRecordRequest {
        TrainingRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn training_records_minimal() -> TrainingRecordRequest {
        TrainingRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn training_records_with_children() -> TrainingRecordRequest {
        TrainingRecordRequest::new()
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

    pub fn compensation_adjustments() -> CompensationAdjustmentRequest {
        CompensationAdjustmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn compensation_adjustments_minimal() -> CompensationAdjustmentRequest {
        CompensationAdjustmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn compensation_adjustments_with_children() -> CompensationAdjustmentRequest {
        CompensationAdjustmentRequest::new()
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

    pub fn offboarding_processes() -> OffboardingProcessRequest {
        OffboardingProcessRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn offboarding_processes_minimal() -> OffboardingProcessRequest {
        OffboardingProcessRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn offboarding_processes_with_children() -> OffboardingProcessRequest {
        OffboardingProcessRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn employee_handbooks() -> EmployeeHandbookRequest {
        EmployeeHandbookRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employee_handbooks_minimal() -> EmployeeHandbookRequest {
        EmployeeHandbookRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employee_handbooks_with_children() -> EmployeeHandbookRequest {
        EmployeeHandbookRequest::new()
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

    pub fn job_schedules() -> JobScheduleRequest {
        JobScheduleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn job_schedules_minimal() -> JobScheduleRequest {
        JobScheduleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn job_schedules_with_children() -> JobScheduleRequest {
        JobScheduleRequest::new()
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

    pub fn time_slots() -> TimeSlotRequest {
        TimeSlotRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn time_slots_minimal() -> TimeSlotRequest {
        TimeSlotRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn time_slots_with_children() -> TimeSlotRequest {
        TimeSlotRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn service_locations() -> ServiceLocationRequest {
        ServiceLocationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_locations_minimal() -> ServiceLocationRequest {
        ServiceLocationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_locations_with_children() -> ServiceLocationRequest {
        ServiceLocationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn special_instructionses() -> SpecialInstructionsRequest {
        SpecialInstructionsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn special_instructionses_minimal() -> SpecialInstructionsRequest {
        SpecialInstructionsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn special_instructionses_with_children() -> SpecialInstructionsRequest {
        SpecialInstructionsRequest::new()
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

    pub fn cancellation_policies() -> CancellationPolicyRequest {
        CancellationPolicyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cancellation_policies_minimal() -> CancellationPolicyRequest {
        CancellationPolicyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cancellation_policies_with_children() -> CancellationPolicyRequest {
        CancellationPolicyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn reschedule_requests() -> RescheduleRequestRequest {
        RescheduleRequestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn reschedule_requests_minimal() -> RescheduleRequestRequest {
        RescheduleRequestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn reschedule_requests_with_children() -> RescheduleRequestRequest {
        RescheduleRequestRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn satisfaction_surveys() -> SatisfactionSurveyRequest {
        SatisfactionSurveyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn satisfaction_surveys_minimal() -> SatisfactionSurveyRequest {
        SatisfactionSurveyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn satisfaction_surveys_with_children() -> SatisfactionSurveyRequest {
        SatisfactionSurveyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn follow_up_tasks() -> FollowUpTaskRequest {
        FollowUpTaskRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn follow_up_tasks_minimal() -> FollowUpTaskRequest {
        FollowUpTaskRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn follow_up_tasks_with_children() -> FollowUpTaskRequest {
        FollowUpTaskRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}