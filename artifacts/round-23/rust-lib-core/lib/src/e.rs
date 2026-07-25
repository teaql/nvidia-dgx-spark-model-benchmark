// The `E` expression wrapper provides zero-cost AST traversal
// and will automatically panic if it encounters a NotLoaded error.
pub struct E;

impl E {
    pub fn customer_profile<'a>(value: &'a crate::CustomerProfile) -> crate::CustomerProfileExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerProfile(id={})", value.id()));
        crate::CustomerProfileExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_contact<'a>(value: &'a crate::CustomerContact) -> crate::CustomerContactExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerContact(id={})", value.id()));
        crate::CustomerContactExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_address<'a>(value: &'a crate::CustomerAddress) -> crate::CustomerAddressExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerAddress(id={})", value.id()));
        crate::CustomerAddressExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_preference<'a>(value: &'a crate::CustomerPreference) -> crate::CustomerPreferenceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerPreference(id={})", value.id()));
        crate::CustomerPreferenceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn loyalty_program<'a>(value: &'a crate::LoyaltyProgram) -> crate::LoyaltyProgramExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoyaltyProgram(id={})", value.id()));
        crate::LoyaltyProgramExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_feedback<'a>(value: &'a crate::CustomerFeedback) -> crate::CustomerFeedbackExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerFeedback(id={})", value.id()));
        crate::CustomerFeedbackExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_segment<'a>(value: &'a crate::CustomerSegment) -> crate::CustomerSegmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerSegment(id={})", value.id()));
        crate::CustomerSegmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_account<'a>(value: &'a crate::CustomerAccount) -> crate::CustomerAccountExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerAccount(id={})", value.id()));
        crate::CustomerAccountExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payment_method<'a>(value: &'a crate::PaymentMethod) -> crate::PaymentMethodExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PaymentMethod(id={})", value.id()));
        crate::PaymentMethodExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn invoice_history<'a>(value: &'a crate::InvoiceHistory) -> crate::InvoiceHistoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InvoiceHistory(id={})", value.id()));
        crate::InvoiceHistoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn dispute_record<'a>(value: &'a crate::DisputeRecord) -> crate::DisputeRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DisputeRecord(id={})", value.id()));
        crate::DisputeRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_agreement<'a>(value: &'a crate::ServiceAgreement) -> crate::ServiceAgreementExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceAgreement(id={})", value.id()));
        crate::ServiceAgreementExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn contract_terms<'a>(value: &'a crate::ContractTerms) -> crate::ContractTermsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ContractTerms(id={})", value.id()));
        crate::ContractTermsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn renewal_notice<'a>(value: &'a crate::RenewalNotice) -> crate::RenewalNoticeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RenewalNotice(id={})", value.id()));
        crate::RenewalNoticeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn cancellation_request<'a>(value: &'a crate::CancellationRequest) -> crate::CancellationRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CancellationRequest(id={})", value.id()));
        crate::CancellationRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn referral_code<'a>(value: &'a crate::ReferralCode) -> crate::ReferralCodeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ReferralCode(id={})", value.id()));
        crate::ReferralCodeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn marketing_campaign<'a>(value: &'a crate::MarketingCampaign) -> crate::MarketingCampaignExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MarketingCampaign(id={})", value.id()));
        crate::MarketingCampaignExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn lead_source<'a>(value: &'a crate::LeadSource) -> crate::LeadSourceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LeadSource(id={})", value.id()));
        crate::LeadSourceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_registry<'a>(value: &'a crate::VehicleRegistry) -> crate::VehicleRegistryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleRegistry(id={})", value.id()));
        crate::VehicleRegistryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_spec<'a>(value: &'a crate::VehicleSpec) -> crate::VehicleSpecExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleSpec(id={})", value.id()));
        crate::VehicleSpecExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn maintenance_log<'a>(value: &'a crate::MaintenanceLog) -> crate::MaintenanceLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MaintenanceLog(id={})", value.id()));
        crate::MaintenanceLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fuel_record<'a>(value: &'a crate::FuelRecord) -> crate::FuelRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FuelRecord(id={})", value.id()));
        crate::FuelRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tire_inventory<'a>(value: &'a crate::TireInventory) -> crate::TireInventoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TireInventory(id={})", value.id()));
        crate::TireInventoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver_assignment<'a>(value: &'a crate::DriverAssignment) -> crate::DriverAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DriverAssignment(id={})", value.id()));
        crate::DriverAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver_license<'a>(value: &'a crate::DriverLicense) -> crate::DriverLicenseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DriverLicense(id={})", value.id()));
        crate::DriverLicenseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver_training<'a>(value: &'a crate::DriverTraining) -> crate::DriverTrainingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DriverTraining(id={})", value.id()));
        crate::DriverTrainingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn route_plan<'a>(value: &'a crate::RoutePlan) -> crate::RoutePlanExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RoutePlan(id={})", value.id()));
        crate::RoutePlanExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn load_capacity<'a>(value: &'a crate::LoadCapacity) -> crate::LoadCapacityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoadCapacity(id={})", value.id()));
        crate::LoadCapacityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn cargo_securement<'a>(value: &'a crate::CargoSecurement) -> crate::CargoSecurementExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CargoSecurement(id={})", value.id()));
        crate::CargoSecurementExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn gps_tracking<'a>(value: &'a crate::GpsTracking) -> crate::GpsTrackingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("GpsTracking(id={})", value.id()));
        crate::GpsTrackingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn telematics_data<'a>(value: &'a crate::TelematicsData) -> crate::TelematicsDataExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TelematicsData(id={})", value.id()));
        crate::TelematicsDataExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn incident_report<'a>(value: &'a crate::IncidentReport) -> crate::IncidentReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("IncidentReport(id={})", value.id()));
        crate::IncidentReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn inspection_checklist<'a>(value: &'a crate::InspectionChecklist) -> crate::InspectionChecklistExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InspectionChecklist(id={})", value.id()));
        crate::InspectionChecklistExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_schedule<'a>(value: &'a crate::ServiceSchedule) -> crate::ServiceScheduleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceSchedule(id={})", value.id()));
        crate::ServiceScheduleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn warranty_info<'a>(value: &'a crate::WarrantyInfo) -> crate::WarrantyInfoExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("WarrantyInfo(id={})", value.id()));
        crate::WarrantyInfoExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn decommission_record<'a>(value: &'a crate::DecommissionRecord) -> crate::DecommissionRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DecommissionRecord(id={})", value.id()));
        crate::DecommissionRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn invoice_template<'a>(value: &'a crate::InvoiceTemplate) -> crate::InvoiceTemplateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InvoiceTemplate(id={})", value.id()));
        crate::InvoiceTemplateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn billing_cycle<'a>(value: &'a crate::BillingCycle) -> crate::BillingCycleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BillingCycle(id={})", value.id()));
        crate::BillingCycleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tax_jurisdiction<'a>(value: &'a crate::TaxJurisdiction) -> crate::TaxJurisdictionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TaxJurisdiction(id={})", value.id()));
        crate::TaxJurisdictionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tax_rate<'a>(value: &'a crate::TaxRate) -> crate::TaxRateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TaxRate(id={})", value.id()));
        crate::TaxRateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn discount_policy<'a>(value: &'a crate::DiscountPolicy) -> crate::DiscountPolicyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DiscountPolicy(id={})", value.id()));
        crate::DiscountPolicyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payment_gateway<'a>(value: &'a crate::PaymentGateway) -> crate::PaymentGatewayExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PaymentGateway(id={})", value.id()));
        crate::PaymentGatewayExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn transaction_log<'a>(value: &'a crate::TransactionLog) -> crate::TransactionLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TransactionLog(id={})", value.id()));
        crate::TransactionLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn refund_process<'a>(value: &'a crate::RefundProcess) -> crate::RefundProcessExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RefundProcess(id={})", value.id()));
        crate::RefundProcessExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn credit_note<'a>(value: &'a crate::CreditNote) -> crate::CreditNoteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CreditNote(id={})", value.id()));
        crate::CreditNoteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn debit_note<'a>(value: &'a crate::DebitNote) -> crate::DebitNoteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DebitNote(id={})", value.id()));
        crate::DebitNoteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn expense_category<'a>(value: &'a crate::ExpenseCategory) -> crate::ExpenseCategoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExpenseCategory(id={})", value.id()));
        crate::ExpenseCategoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn cost_center<'a>(value: &'a crate::CostCenter) -> crate::CostCenterExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CostCenter(id={})", value.id()));
        crate::CostCenterExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn budget_allocation<'a>(value: &'a crate::BudgetAllocation) -> crate::BudgetAllocationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BudgetAllocation(id={})", value.id()));
        crate::BudgetAllocationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn financial_report<'a>(value: &'a crate::FinancialReport) -> crate::FinancialReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FinancialReport(id={})", value.id()));
        crate::FinancialReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn audit_trail<'a>(value: &'a crate::AuditTrail) -> crate::AuditTrailExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AuditTrail(id={})", value.id()));
        crate::AuditTrailExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn reconciliation_entry<'a>(value: &'a crate::ReconciliationEntry) -> crate::ReconciliationEntryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ReconciliationEntry(id={})", value.id()));
        crate::ReconciliationEntryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn currency_conversion<'a>(value: &'a crate::CurrencyConversion) -> crate::CurrencyConversionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CurrencyConversion(id={})", value.id()));
        crate::CurrencyConversionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fiscal_period<'a>(value: &'a crate::FiscalPeriod) -> crate::FiscalPeriodExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FiscalPeriod(id={})", value.id()));
        crate::FiscalPeriodExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn job_order<'a>(value: &'a crate::JobOrder) -> crate::JobOrderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("JobOrder(id={})", value.id()));
        crate::JobOrderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_schedule<'a>(value: &'a crate::MoveSchedule) -> crate::MoveScheduleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveSchedule(id={})", value.id()));
        crate::MoveScheduleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn crew_assignment<'a>(value: &'a crate::CrewAssignment) -> crate::CrewAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CrewAssignment(id={})", value.id()));
        crate::CrewAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn equipment_allocation<'a>(value: &'a crate::EquipmentAllocation) -> crate::EquipmentAllocationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EquipmentAllocation(id={})", value.id()));
        crate::EquipmentAllocationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn pickup_location<'a>(value: &'a crate::PickupLocation) -> crate::PickupLocationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PickupLocation(id={})", value.id()));
        crate::PickupLocationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn delivery_location<'a>(value: &'a crate::DeliveryLocation) -> crate::DeliveryLocationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DeliveryLocation(id={})", value.id()));
        crate::DeliveryLocationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn transit_time_estimate<'a>(value: &'a crate::TransitTimeEstimate) -> crate::TransitTimeEstimateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TransitTimeEstimate(id={})", value.id()));
        crate::TransitTimeEstimateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn loading_dock<'a>(value: &'a crate::LoadingDock) -> crate::LoadingDockExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoadingDock(id={})", value.id()));
        crate::LoadingDockExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn unloading_dock<'a>(value: &'a crate::UnloadingDock) -> crate::UnloadingDockExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UnloadingDock(id={})", value.id()));
        crate::UnloadingDockExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customs_documentation<'a>(value: &'a crate::CustomsDocumentation) -> crate::CustomsDocumentationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomsDocumentation(id={})", value.id()));
        crate::CustomsDocumentationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn permit_required<'a>(value: &'a crate::PermitRequired) -> crate::PermitRequiredExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PermitRequired(id={})", value.id()));
        crate::PermitRequiredExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn insurance_coverage<'a>(value: &'a crate::InsuranceCoverage) -> crate::InsuranceCoverageExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InsuranceCoverage(id={})", value.id()));
        crate::InsuranceCoverageExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn liability_waiver<'a>(value: &'a crate::LiabilityWaiver) -> crate::LiabilityWaiverExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LiabilityWaiver(id={})", value.id()));
        crate::LiabilityWaiverExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tracking_number<'a>(value: &'a crate::TrackingNumber) -> crate::TrackingNumberExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TrackingNumber(id={})", value.id()));
        crate::TrackingNumberExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn status_update<'a>(value: &'a crate::StatusUpdate) -> crate::StatusUpdateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("StatusUpdate(id={})", value.id()));
        crate::StatusUpdateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn notification_template<'a>(value: &'a crate::NotificationTemplate) -> crate::NotificationTemplateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("NotificationTemplate(id={})", value.id()));
        crate::NotificationTemplateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn sla_metric<'a>(value: &'a crate::SlaMetric) -> crate::SlaMetricExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SlaMetric(id={})", value.id()));
        crate::SlaMetricExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn performance_kpi<'a>(value: &'a crate::PerformanceKpi) -> crate::PerformanceKpiExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PerformanceKpi(id={})", value.id()));
        crate::PerformanceKpiExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn employee_record<'a>(value: &'a crate::EmployeeRecord) -> crate::EmployeeRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EmployeeRecord(id={})", value.id()));
        crate::EmployeeRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payroll_info<'a>(value: &'a crate::PayrollInfo) -> crate::PayrollInfoExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PayrollInfo(id={})", value.id()));
        crate::PayrollInfoExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn benefits_plan<'a>(value: &'a crate::BenefitsPlan) -> crate::BenefitsPlanExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BenefitsPlan(id={})", value.id()));
        crate::BenefitsPlanExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn time_off_request<'a>(value: &'a crate::TimeOffRequest) -> crate::TimeOffRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TimeOffRequest(id={})", value.id()));
        crate::TimeOffRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn shift_schedule<'a>(value: &'a crate::ShiftSchedule) -> crate::ShiftScheduleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ShiftSchedule(id={})", value.id()));
        crate::ShiftScheduleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn performance_review<'a>(value: &'a crate::PerformanceReview) -> crate::PerformanceReviewExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PerformanceReview(id={})", value.id()));
        crate::PerformanceReviewExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn competency_matrix<'a>(value: &'a crate::CompetencyMatrix) -> crate::CompetencyMatrixExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CompetencyMatrix(id={})", value.id()));
        crate::CompetencyMatrixExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn training_course<'a>(value: &'a crate::TrainingCourse) -> crate::TrainingCourseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TrainingCourse(id={})", value.id()));
        crate::TrainingCourseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn certification_record<'a>(value: &'a crate::CertificationRecord) -> crate::CertificationRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CertificationRecord(id={})", value.id()));
        crate::CertificationRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn safety_incident<'a>(value: &'a crate::SafetyIncident) -> crate::SafetyIncidentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SafetyIncident(id={})", value.id()));
        crate::SafetyIncidentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn hazard_assessment<'a>(value: &'a crate::HazardAssessment) -> crate::HazardAssessmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("HazardAssessment(id={})", value.id()));
        crate::HazardAssessmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn policy_acknowledgment<'a>(value: &'a crate::PolicyAcknowledgment) -> crate::PolicyAcknowledgmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PolicyAcknowledgment(id={})", value.id()));
        crate::PolicyAcknowledgmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn grievance_log<'a>(value: &'a crate::GrievanceLog) -> crate::GrievanceLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("GrievanceLog(id={})", value.id()));
        crate::GrievanceLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn disciplinary_action<'a>(value: &'a crate::DisciplinaryAction) -> crate::DisciplinaryActionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DisciplinaryAction(id={})", value.id()));
        crate::DisciplinaryActionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn exit_interview<'a>(value: &'a crate::ExitInterview) -> crate::ExitInterviewExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExitInterview(id={})", value.id()));
        crate::ExitInterviewExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn onboarding_checklist<'a>(value: &'a crate::OnboardingChecklist) -> crate::OnboardingChecklistExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OnboardingChecklist(id={})", value.id()));
        crate::OnboardingChecklistExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn offboarding_checklist<'a>(value: &'a crate::OffboardingChecklist) -> crate::OffboardingChecklistExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OffboardingChecklist(id={})", value.id()));
        crate::OffboardingChecklistExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn compliance_audit<'a>(value: &'a crate::ComplianceAudit) -> crate::ComplianceAuditExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ComplianceAudit(id={})", value.id()));
        crate::ComplianceAuditExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

