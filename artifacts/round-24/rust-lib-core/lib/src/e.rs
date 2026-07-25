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

    pub fn customer_contract<'a>(value: &'a crate::CustomerContract) -> crate::CustomerContractExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerContract(id={})", value.id()));
        crate::CustomerContractExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_feedback<'a>(value: &'a crate::CustomerFeedback) -> crate::CustomerFeedbackExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerFeedback(id={})", value.id()));
        crate::CustomerFeedbackExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_segment<'a>(value: &'a crate::CustomerSegment) -> crate::CustomerSegmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerSegment(id={})", value.id()));
        crate::CustomerSegmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_loyalty<'a>(value: &'a crate::CustomerLoyalty) -> crate::CustomerLoyaltyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerLoyalty(id={})", value.id()));
        crate::CustomerLoyaltyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_invoice<'a>(value: &'a crate::CustomerInvoice) -> crate::CustomerInvoiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerInvoice(id={})", value.id()));
        crate::CustomerInvoiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_payment<'a>(value: &'a crate::CustomerPayment) -> crate::CustomerPaymentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerPayment(id={})", value.id()));
        crate::CustomerPaymentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_claim<'a>(value: &'a crate::CustomerClaim) -> crate::CustomerClaimExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerClaim(id={})", value.id()));
        crate::CustomerClaimExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_notification<'a>(value: &'a crate::CustomerNotification) -> crate::CustomerNotificationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerNotification(id={})", value.id()));
        crate::CustomerNotificationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_account<'a>(value: &'a crate::CustomerAccount) -> crate::CustomerAccountExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerAccount(id={})", value.id()));
        crate::CustomerAccountExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_lead<'a>(value: &'a crate::CustomerLead) -> crate::CustomerLeadExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerLead(id={})", value.id()));
        crate::CustomerLeadExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_quote<'a>(value: &'a crate::CustomerQuote) -> crate::CustomerQuoteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerQuote(id={})", value.id()));
        crate::CustomerQuoteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_service<'a>(value: &'a crate::CustomerService) -> crate::CustomerServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerService(id={})", value.id()));
        crate::CustomerServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_support_ticket<'a>(value: &'a crate::CustomerSupportTicket) -> crate::CustomerSupportTicketExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerSupportTicket(id={})", value.id()));
        crate::CustomerSupportTicketExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_vehicle<'a>(value: &'a crate::CustomerVehicle) -> crate::CustomerVehicleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerVehicle(id={})", value.id()));
        crate::CustomerVehicleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_move_history<'a>(value: &'a crate::CustomerMoveHistory) -> crate::CustomerMoveHistoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerMoveHistory(id={})", value.id()));
        crate::CustomerMoveHistoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_preferred_time<'a>(value: &'a crate::CustomerPreferredTime) -> crate::CustomerPreferredTimeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerPreferredTime(id={})", value.id()));
        crate::CustomerPreferredTimeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fleet_vehicle<'a>(value: &'a crate::FleetVehicle) -> crate::FleetVehicleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FleetVehicle(id={})", value.id()));
        crate::FleetVehicleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_spec<'a>(value: &'a crate::VehicleSpec) -> crate::VehicleSpecExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleSpec(id={})", value.id()));
        crate::VehicleSpecExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_maintenance<'a>(value: &'a crate::VehicleMaintenance) -> crate::VehicleMaintenanceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleMaintenance(id={})", value.id()));
        crate::VehicleMaintenanceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_inspection<'a>(value: &'a crate::VehicleInspection) -> crate::VehicleInspectionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleInspection(id={})", value.id()));
        crate::VehicleInspectionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_assignment<'a>(value: &'a crate::VehicleAssignment) -> crate::VehicleAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleAssignment(id={})", value.id()));
        crate::VehicleAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_utilization<'a>(value: &'a crate::VehicleUtilization) -> crate::VehicleUtilizationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleUtilization(id={})", value.id()));
        crate::VehicleUtilizationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_fuel_log<'a>(value: &'a crate::VehicleFuelLog) -> crate::VehicleFuelLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleFuelLog(id={})", value.id()));
        crate::VehicleFuelLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_odometer<'a>(value: &'a crate::VehicleOdometer) -> crate::VehicleOdometerExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleOdometer(id={})", value.id()));
        crate::VehicleOdometerExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_insurance<'a>(value: &'a crate::VehicleInsurance) -> crate::VehicleInsuranceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleInsurance(id={})", value.id()));
        crate::VehicleInsuranceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_registration<'a>(value: &'a crate::VehicleRegistration) -> crate::VehicleRegistrationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleRegistration(id={})", value.id()));
        crate::VehicleRegistrationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_damage_report<'a>(value: &'a crate::VehicleDamageReport) -> crate::VehicleDamageReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleDamageReport(id={})", value.id()));
        crate::VehicleDamageReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_cleaning_schedule<'a>(value: &'a crate::VehicleCleaningSchedule) -> crate::VehicleCleaningScheduleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleCleaningSchedule(id={})", value.id()));
        crate::VehicleCleaningScheduleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fleet_operator<'a>(value: &'a crate::FleetOperator) -> crate::FleetOperatorExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FleetOperator(id={})", value.id()));
        crate::FleetOperatorExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver_profile<'a>(value: &'a crate::DriverProfile) -> crate::DriverProfileExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DriverProfile(id={})", value.id()));
        crate::DriverProfileExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver_license<'a>(value: &'a crate::DriverLicense) -> crate::DriverLicenseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DriverLicense(id={})", value.id()));
        crate::DriverLicenseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver_certification<'a>(value: &'a crate::DriverCertification) -> crate::DriverCertificationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DriverCertification(id={})", value.id()));
        crate::DriverCertificationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver_availability<'a>(value: &'a crate::DriverAvailability) -> crate::DriverAvailabilityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DriverAvailability(id={})", value.id()));
        crate::DriverAvailabilityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver_performance<'a>(value: &'a crate::DriverPerformance) -> crate::DriverPerformanceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DriverPerformance(id={})", value.id()));
        crate::DriverPerformanceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver_training<'a>(value: &'a crate::DriverTraining) -> crate::DriverTrainingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DriverTraining(id={})", value.id()));
        crate::DriverTrainingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fleet_dispatch<'a>(value: &'a crate::FleetDispatch) -> crate::FleetDispatchExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FleetDispatch(id={})", value.id()));
        crate::FleetDispatchExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn invoice_header<'a>(value: &'a crate::InvoiceHeader) -> crate::InvoiceHeaderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InvoiceHeader(id={})", value.id()));
        crate::InvoiceHeaderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn invoice_line_item<'a>(value: &'a crate::InvoiceLineItem) -> crate::InvoiceLineItemExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InvoiceLineItem(id={})", value.id()));
        crate::InvoiceLineItemExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payment_method<'a>(value: &'a crate::PaymentMethod) -> crate::PaymentMethodExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PaymentMethod(id={})", value.id()));
        crate::PaymentMethodExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payment_transaction<'a>(value: &'a crate::PaymentTransaction) -> crate::PaymentTransactionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PaymentTransaction(id={})", value.id()));
        crate::PaymentTransactionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn billing_cycle<'a>(value: &'a crate::BillingCycle) -> crate::BillingCycleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BillingCycle(id={})", value.id()));
        crate::BillingCycleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tax_code<'a>(value: &'a crate::TaxCode) -> crate::TaxCodeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TaxCode(id={})", value.id()));
        crate::TaxCodeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn discount_rule<'a>(value: &'a crate::DiscountRule) -> crate::DiscountRuleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DiscountRule(id={})", value.id()));
        crate::DiscountRuleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn credit_note<'a>(value: &'a crate::CreditNote) -> crate::CreditNoteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CreditNote(id={})", value.id()));
        crate::CreditNoteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn debit_note<'a>(value: &'a crate::DebitNote) -> crate::DebitNoteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DebitNote(id={})", value.id()));
        crate::DebitNoteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn billing_address<'a>(value: &'a crate::BillingAddress) -> crate::BillingAddressExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BillingAddress(id={})", value.id()));
        crate::BillingAddressExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn outstanding_balance<'a>(value: &'a crate::OutstandingBalance) -> crate::OutstandingBalanceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OutstandingBalance(id={})", value.id()));
        crate::OutstandingBalanceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn aging_report<'a>(value: &'a crate::AgingReport) -> crate::AgingReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AgingReport(id={})", value.id()));
        crate::AgingReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payment_reminder<'a>(value: &'a crate::PaymentReminder) -> crate::PaymentReminderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PaymentReminder(id={})", value.id()));
        crate::PaymentReminderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn refund_request<'a>(value: &'a crate::RefundRequest) -> crate::RefundRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RefundRequest(id={})", value.id()));
        crate::RefundRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn billing_adjustment<'a>(value: &'a crate::BillingAdjustment) -> crate::BillingAdjustmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BillingAdjustment(id={})", value.id()));
        crate::BillingAdjustmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn revenue_recognition<'a>(value: &'a crate::RevenueRecognition) -> crate::RevenueRecognitionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RevenueRecognition(id={})", value.id()));
        crate::RevenueRecognitionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn financial_period<'a>(value: &'a crate::FinancialPeriod) -> crate::FinancialPeriodExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FinancialPeriod(id={})", value.id()));
        crate::FinancialPeriodExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn audit_trail<'a>(value: &'a crate::AuditTrail) -> crate::AuditTrailExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AuditTrail(id={})", value.id()));
        crate::AuditTrailExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn currency_rate<'a>(value: &'a crate::CurrencyRate) -> crate::CurrencyRateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CurrencyRate(id={})", value.id()));
        crate::CurrencyRateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn billing_approval<'a>(value: &'a crate::BillingApproval) -> crate::BillingApprovalExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BillingApproval(id={})", value.id()));
        crate::BillingApprovalExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_order<'a>(value: &'a crate::MoveOrder) -> crate::MoveOrderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveOrder(id={})", value.id()));
        crate::MoveOrderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_schedule<'a>(value: &'a crate::MoveSchedule) -> crate::MoveScheduleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveSchedule(id={})", value.id()));
        crate::MoveScheduleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn route_plan<'a>(value: &'a crate::RoutePlan) -> crate::RoutePlanExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RoutePlan(id={})", value.id()));
        crate::RoutePlanExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn load_plan<'a>(value: &'a crate::LoadPlan) -> crate::LoadPlanExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoadPlan(id={})", value.id()));
        crate::LoadPlanExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn crew_assignment<'a>(value: &'a crate::CrewAssignment) -> crate::CrewAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CrewAssignment(id={})", value.id()));
        crate::CrewAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn equipment_checklist<'a>(value: &'a crate::EquipmentChecklist) -> crate::EquipmentChecklistExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EquipmentChecklist(id={})", value.id()));
        crate::EquipmentChecklistExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn loading_procedure<'a>(value: &'a crate::LoadingProcedure) -> crate::LoadingProcedureExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoadingProcedure(id={})", value.id()));
        crate::LoadingProcedureExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn unloading_procedure<'a>(value: &'a crate::UnloadingProcedure) -> crate::UnloadingProcedureExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UnloadingProcedure(id={})", value.id()));
        crate::UnloadingProcedureExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn transit_monitoring<'a>(value: &'a crate::TransitMonitoring) -> crate::TransitMonitoringExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TransitMonitoring(id={})", value.id()));
        crate::TransitMonitoringExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn delivery_confirmation<'a>(value: &'a crate::DeliveryConfirmation) -> crate::DeliveryConfirmationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DeliveryConfirmation(id={})", value.id()));
        crate::DeliveryConfirmationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn exception_handling<'a>(value: &'a crate::ExceptionHandling) -> crate::ExceptionHandlingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExceptionHandling(id={})", value.id()));
        crate::ExceptionHandlingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customs_documentation<'a>(value: &'a crate::CustomsDocumentation) -> crate::CustomsDocumentationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomsDocumentation(id={})", value.id()));
        crate::CustomsDocumentationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn inventory_snapshot<'a>(value: &'a crate::InventorySnapshot) -> crate::InventorySnapshotExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InventorySnapshot(id={})", value.id()));
        crate::InventorySnapshotExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn warehouse_allocation<'a>(value: &'a crate::WarehouseAllocation) -> crate::WarehouseAllocationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("WarehouseAllocation(id={})", value.id()));
        crate::WarehouseAllocationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn dock_scheduling<'a>(value: &'a crate::DockScheduling) -> crate::DockSchedulingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DockScheduling(id={})", value.id()));
        crate::DockSchedulingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn yard_management<'a>(value: &'a crate::YardManagement) -> crate::YardManagementExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("YardManagement(id={})", value.id()));
        crate::YardManagementExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn safety_incident<'a>(value: &'a crate::SafetyIncident) -> crate::SafetyIncidentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SafetyIncident(id={})", value.id()));
        crate::SafetyIncidentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn compliance_check<'a>(value: &'a crate::ComplianceCheck) -> crate::ComplianceCheckExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ComplianceCheck(id={})", value.id()));
        crate::ComplianceCheckExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn performance_metric<'a>(value: &'a crate::PerformanceMetric) -> crate::PerformanceMetricExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PerformanceMetric(id={})", value.id()));
        crate::PerformanceMetricExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn operations_dashboard<'a>(value: &'a crate::OperationsDashboard) -> crate::OperationsDashboardExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OperationsDashboard(id={})", value.id()));
        crate::OperationsDashboardExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn daily_summary<'a>(value: &'a crate::DailySummary) -> crate::DailySummaryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DailySummary(id={})", value.id()));
        crate::DailySummaryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn weekly_report<'a>(value: &'a crate::WeeklyReport) -> crate::WeeklyReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("WeeklyReport(id={})", value.id()));
        crate::WeeklyReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn monthly_kpi<'a>(value: &'a crate::MonthlyKpi) -> crate::MonthlyKpiExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MonthlyKpi(id={})", value.id()));
        crate::MonthlyKpiExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn annual_performance<'a>(value: &'a crate::AnnualPerformance) -> crate::AnnualPerformanceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AnnualPerformance(id={})", value.id()));
        crate::AnnualPerformanceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn utilization_report<'a>(value: &'a crate::UtilizationReport) -> crate::UtilizationReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UtilizationReport(id={})", value.id()));
        crate::UtilizationReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn cost_analysis<'a>(value: &'a crate::CostAnalysis) -> crate::CostAnalysisExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CostAnalysis(id={})", value.id()));
        crate::CostAnalysisExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn profit_margin<'a>(value: &'a crate::ProfitMargin) -> crate::ProfitMarginExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ProfitMargin(id={})", value.id()));
        crate::ProfitMarginExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_satisfaction<'a>(value: &'a crate::CustomerSatisfaction) -> crate::CustomerSatisfactionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerSatisfaction(id={})", value.id()));
        crate::CustomerSatisfactionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn on_time_delivery<'a>(value: &'a crate::OnTimeDelivery) -> crate::OnTimeDeliveryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OnTimeDelivery(id={})", value.id()));
        crate::OnTimeDeliveryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn claim_rate<'a>(value: &'a crate::ClaimRate) -> crate::ClaimRateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ClaimRate(id={})", value.id()));
        crate::ClaimRateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fleet_efficiency<'a>(value: &'a crate::FleetEfficiency) -> crate::FleetEfficiencyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FleetEfficiency(id={})", value.id()));
        crate::FleetEfficiencyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver_productivity<'a>(value: &'a crate::DriverProductivity) -> crate::DriverProductivityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DriverProductivity(id={})", value.id()));
        crate::DriverProductivityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn billing_accuracy<'a>(value: &'a crate::BillingAccuracy) -> crate::BillingAccuracyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BillingAccuracy(id={})", value.id()));
        crate::BillingAccuracyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn invoice_aging<'a>(value: &'a crate::InvoiceAging) -> crate::InvoiceAgingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InvoiceAging(id={})", value.id()));
        crate::InvoiceAgingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_volume_trend<'a>(value: &'a crate::MoveVolumeTrend) -> crate::MoveVolumeTrendExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveVolumeTrend(id={})", value.id()));
        crate::MoveVolumeTrendExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn geographic_distribution<'a>(value: &'a crate::GeographicDistribution) -> crate::GeographicDistributionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("GeographicDistribution(id={})", value.id()));
        crate::GeographicDistributionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_line_performance<'a>(value: &'a crate::ServiceLinePerformance) -> crate::ServiceLinePerformanceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceLinePerformance(id={})", value.id()));
        crate::ServiceLinePerformanceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn expense_variance<'a>(value: &'a crate::ExpenseVariance) -> crate::ExpenseVarianceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExpenseVariance(id={})", value.id()));
        crate::ExpenseVarianceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn forecast_vs_actual<'a>(value: &'a crate::ForecastVsActual) -> crate::ForecastVsActualExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ForecastVsActual(id={})", value.id()));
        crate::ForecastVsActualExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn executive_dashboard<'a>(value: &'a crate::ExecutiveDashboard) -> crate::ExecutiveDashboardExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExecutiveDashboard(id={})", value.id()));
        crate::ExecutiveDashboardExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

