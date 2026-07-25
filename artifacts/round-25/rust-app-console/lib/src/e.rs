// The `E` expression wrapper provides zero-cost AST traversal
// and will automatically panic if it encounters a NotLoaded error.
pub struct E;

impl E {
    pub fn platform<'a>(value: &'a crate::Platform) -> crate::PlatformExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Platform(id={})", value.id()));
        crate::PlatformExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn platform_config<'a>(value: &'a crate::PlatformConfig) -> crate::PlatformConfigExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PlatformConfig(id={})", value.id()));
        crate::PlatformConfigExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tenant_registry<'a>(value: &'a crate::TenantRegistry) -> crate::TenantRegistryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TenantRegistry(id={})", value.id()));
        crate::TenantRegistryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn merchant<'a>(value: &'a crate::Merchant) -> crate::MerchantExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Merchant(id={})", value.id()));
        crate::MerchantExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn branch<'a>(value: &'a crate::Branch) -> crate::BranchExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Branch(id={})", value.id()));
        crate::BranchExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn franchise<'a>(value: &'a crate::Franchise) -> crate::FranchiseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Franchise(id={})", value.id()));
        crate::FranchiseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_order<'a>(value: &'a crate::MoveOrder) -> crate::MoveOrderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveOrder(id={})", value.id()));
        crate::MoveOrderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_quote<'a>(value: &'a crate::MoveQuote) -> crate::MoveQuoteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveQuote(id={})", value.id()));
        crate::MoveQuoteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn route<'a>(value: &'a crate::Route) -> crate::RouteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Route(id={})", value.id()));
        crate::RouteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn route_stop<'a>(value: &'a crate::RouteStop) -> crate::RouteStopExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RouteStop(id={})", value.id()));
        crate::RouteStopExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn time_slot<'a>(value: &'a crate::TimeSlot) -> crate::TimeSlotExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TimeSlot(id={})", value.id()));
        crate::TimeSlotExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fulfillment_event<'a>(value: &'a crate::FulfillmentEvent) -> crate::FulfillmentEventExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FulfillmentEvent(id={})", value.id()));
        crate::FulfillmentEventExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn address<'a>(value: &'a crate::Address) -> crate::AddressExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Address(id={})", value.id()));
        crate::AddressExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn crew<'a>(value: &'a crate::Crew) -> crate::CrewExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Crew(id={})", value.id()));
        crate::CrewExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn dispatch_assignment<'a>(value: &'a crate::DispatchAssignment) -> crate::DispatchAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DispatchAssignment(id={})", value.id()));
        crate::DispatchAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn damage_report<'a>(value: &'a crate::DamageReport) -> crate::DamageReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DamageReport(id={})", value.id()));
        crate::DamageReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn proof_of_delivery<'a>(value: &'a crate::ProofOfDelivery) -> crate::ProofOfDeliveryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ProofOfDelivery(id={})", value.id()));
        crate::ProofOfDeliveryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn packing_list<'a>(value: &'a crate::PackingList) -> crate::PackingListExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PackingList(id={})", value.id()));
        crate::PackingListExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn inventory_item<'a>(value: &'a crate::InventoryItem) -> crate::InventoryItemExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InventoryItem(id={})", value.id()));
        crate::InventoryItemExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_load_plan<'a>(value: &'a crate::VehicleLoadPlan) -> crate::VehicleLoadPlanExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleLoadPlan(id={})", value.id()));
        crate::VehicleLoadPlanExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn weigh_station_ticket<'a>(value: &'a crate::WeighStationTicket) -> crate::WeighStationTicketExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("WeighStationTicket(id={})", value.id()));
        crate::WeighStationTicketExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn toll_receipt<'a>(value: &'a crate::TollReceipt) -> crate::TollReceiptExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TollReceipt(id={})", value.id()));
        crate::TollReceiptExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn parking_permit<'a>(value: &'a crate::ParkingPermit) -> crate::ParkingPermitExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ParkingPermit(id={})", value.id()));
        crate::ParkingPermitExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn traffic_violation<'a>(value: &'a crate::TrafficViolation) -> crate::TrafficViolationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TrafficViolation(id={})", value.id()));
        crate::TrafficViolationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn detour_log<'a>(value: &'a crate::DetourLog) -> crate::DetourLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DetourLog(id={})", value.id()));
        crate::DetourLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fuel_stop<'a>(value: &'a crate::FuelStop) -> crate::FuelStopExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FuelStop(id={})", value.id()));
        crate::FuelStopExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn weather_delay<'a>(value: &'a crate::WeatherDelay) -> crate::WeatherDelayExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("WeatherDelay(id={})", value.id()));
        crate::WeatherDelayExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_signature<'a>(value: &'a crate::CustomerSignature) -> crate::CustomerSignatureExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerSignature(id={})", value.id()));
        crate::CustomerSignatureExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn walkthrough_checklist<'a>(value: &'a crate::WalkthroughChecklist) -> crate::WalkthroughChecklistExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("WalkthroughChecklist(id={})", value.id()));
        crate::WalkthroughChecklistExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn post_move_survey<'a>(value: &'a crate::PostMoveSurvey) -> crate::PostMoveSurveyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PostMoveSurvey(id={})", value.id()));
        crate::PostMoveSurveyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn operations_manager_override<'a>(value: &'a crate::OperationsManagerOverride) -> crate::OperationsManagerOverrideExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OperationsManagerOverride(id={})", value.id()));
        crate::OperationsManagerOverrideExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn employee<'a>(value: &'a crate::Employee) -> crate::EmployeeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Employee(id={})", value.id()));
        crate::EmployeeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn department<'a>(value: &'a crate::Department) -> crate::DepartmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Department(id={})", value.id()));
        crate::DepartmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn job_assignment<'a>(value: &'a crate::JobAssignment) -> crate::JobAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("JobAssignment(id={})", value.id()));
        crate::JobAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn work_shift<'a>(value: &'a crate::WorkShift) -> crate::WorkShiftExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("WorkShift(id={})", value.id()));
        crate::WorkShiftExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn worked_hours<'a>(value: &'a crate::WorkedHours) -> crate::WorkedHoursExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("WorkedHours(id={})", value.id()));
        crate::WorkedHoursExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payroll_period<'a>(value: &'a crate::PayrollPeriod) -> crate::PayrollPeriodExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PayrollPeriod(id={})", value.id()));
        crate::PayrollPeriodExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payroll_calculation<'a>(value: &'a crate::PayrollCalculation) -> crate::PayrollCalculationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PayrollCalculation(id={})", value.id()));
        crate::PayrollCalculationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payslip<'a>(value: &'a crate::Payslip) -> crate::PayslipExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Payslip(id={})", value.id()));
        crate::PayslipExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn bonus<'a>(value: &'a crate::Bonus) -> crate::BonusExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Bonus(id={})", value.id()));
        crate::BonusExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn leave_request<'a>(value: &'a crate::LeaveRequest) -> crate::LeaveRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LeaveRequest(id={})", value.id()));
        crate::LeaveRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn employee_certification<'a>(value: &'a crate::EmployeeCertification) -> crate::EmployeeCertificationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EmployeeCertification(id={})", value.id()));
        crate::EmployeeCertificationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tax_withholding<'a>(value: &'a crate::TaxWithholding) -> crate::TaxWithholdingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TaxWithholding(id={})", value.id()));
        crate::TaxWithholdingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn direct_deposit_info<'a>(value: &'a crate::DirectDepositInfo) -> crate::DirectDepositInfoExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DirectDepositInfo(id={})", value.id()));
        crate::DirectDepositInfoExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn union_dues<'a>(value: &'a crate::UnionDues) -> crate::UnionDuesExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UnionDues(id={})", value.id()));
        crate::UnionDuesExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn overtime_approval<'a>(value: &'a crate::OvertimeApproval) -> crate::OvertimeApprovalExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OvertimeApproval(id={})", value.id()));
        crate::OvertimeApprovalExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn expense_reimbursement<'a>(value: &'a crate::ExpenseReimbursement) -> crate::ExpenseReimbursementExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExpenseReimbursement(id={})", value.id()));
        crate::ExpenseReimbursementExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn performance_review<'a>(value: &'a crate::PerformanceReview) -> crate::PerformanceReviewExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PerformanceReview(id={})", value.id()));
        crate::PerformanceReviewExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn warning_letter<'a>(value: &'a crate::WarningLetter) -> crate::WarningLetterExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("WarningLetter(id={})", value.id()));
        crate::WarningLetterExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn termination_record<'a>(value: &'a crate::TerminationRecord) -> crate::TerminationRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TerminationRecord(id={})", value.id()));
        crate::TerminationRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn emergency_contact<'a>(value: &'a crate::EmergencyContact) -> crate::EmergencyContactExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EmergencyContact(id={})", value.id()));
        crate::EmergencyContactExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn uniform_assignment<'a>(value: &'a crate::UniformAssignment) -> crate::UniformAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UniformAssignment(id={})", value.id()));
        crate::UniformAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn background_check<'a>(value: &'a crate::BackgroundCheck) -> crate::BackgroundCheckExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BackgroundCheck(id={})", value.id()));
        crate::BackgroundCheckExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer<'a>(value: &'a crate::Customer) -> crate::CustomerExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Customer(id={})", value.id()));
        crate::CustomerExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn private_customer_profile<'a>(value: &'a crate::PrivateCustomerProfile) -> crate::PrivateCustomerProfileExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PrivateCustomerProfile(id={})", value.id()));
        crate::PrivateCustomerProfileExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn corporate_customer_profile<'a>(value: &'a crate::CorporateCustomerProfile) -> crate::CorporateCustomerProfileExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CorporateCustomerProfile(id={})", value.id()));
        crate::CorporateCustomerProfileExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_contact<'a>(value: &'a crate::CustomerContact) -> crate::CustomerContactExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerContact(id={})", value.id()));
        crate::CustomerContactExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn billing_profile<'a>(value: &'a crate::BillingProfile) -> crate::BillingProfileExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BillingProfile(id={})", value.id()));
        crate::BillingProfileExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_history<'a>(value: &'a crate::CustomerHistory) -> crate::CustomerHistoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerHistory(id={})", value.id()));
        crate::CustomerHistoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_preference<'a>(value: &'a crate::CustomerPreference) -> crate::CustomerPreferenceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerPreference(id={})", value.id()));
        crate::CustomerPreferenceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_consent<'a>(value: &'a crate::CustomerConsent) -> crate::CustomerConsentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerConsent(id={})", value.id()));
        crate::CustomerConsentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn referral_code<'a>(value: &'a crate::ReferralCode) -> crate::ReferralCodeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ReferralCode(id={})", value.id()));
        crate::ReferralCodeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn loyalty_tier<'a>(value: &'a crate::LoyaltyTier) -> crate::LoyaltyTierExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoyaltyTier(id={})", value.id()));
        crate::LoyaltyTierExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn complaint_ticket<'a>(value: &'a crate::ComplaintTicket) -> crate::ComplaintTicketExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ComplaintTicket(id={})", value.id()));
        crate::ComplaintTicketExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn resolution_offer<'a>(value: &'a crate::ResolutionOffer) -> crate::ResolutionOfferExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ResolutionOffer(id={})", value.id()));
        crate::ResolutionOfferExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vip_status<'a>(value: &'a crate::VipStatus) -> crate::VipStatusExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VipStatus(id={})", value.id()));
        crate::VipStatusExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn do_not_contact_list<'a>(value: &'a crate::DoNotContactList) -> crate::DoNotContactListExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DoNotContactList(id={})", value.id()));
        crate::DoNotContactListExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_note<'a>(value: &'a crate::CustomerNote) -> crate::CustomerNoteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerNote(id={})", value.id()));
        crate::CustomerNoteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn communication_log<'a>(value: &'a crate::CommunicationLog) -> crate::CommunicationLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CommunicationLog(id={})", value.id()));
        crate::CommunicationLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn product<'a>(value: &'a crate::Product) -> crate::ProductExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Product(id={})", value.id()));
        crate::ProductExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service<'a>(value: &'a crate::Service) -> crate::ServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Service(id={})", value.id()));
        crate::ServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn moving_service<'a>(value: &'a crate::MovingService) -> crate::MovingServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MovingService(id={})", value.id()));
        crate::MovingServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn cleaning_service<'a>(value: &'a crate::CleaningService) -> crate::CleaningServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CleaningService(id={})", value.id()));
        crate::CleaningServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn box_rental<'a>(value: &'a crate::BoxRental) -> crate::BoxRentalExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BoxRental(id={})", value.id()));
        crate::BoxRentalExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_configuration<'a>(value: &'a crate::ServiceConfiguration) -> crate::ServiceConfigurationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceConfiguration(id={})", value.id()));
        crate::ServiceConfigurationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn price_list<'a>(value: &'a crate::PriceList) -> crate::PriceListExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PriceList(id={})", value.id()));
        crate::PriceListExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_price<'a>(value: &'a crate::ServicePrice) -> crate::ServicePriceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServicePrice(id={})", value.id()));
        crate::ServicePriceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_bundle<'a>(value: &'a crate::ServiceBundle) -> crate::ServiceBundleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceBundle(id={})", value.id()));
        crate::ServiceBundleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn storage_unit<'a>(value: &'a crate::StorageUnit) -> crate::StorageUnitExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("StorageUnit(id={})", value.id()));
        crate::StorageUnitExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn packing_material<'a>(value: &'a crate::PackingMaterial) -> crate::PackingMaterialExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PackingMaterial(id={})", value.id()));
        crate::PackingMaterialExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn insurance_addon<'a>(value: &'a crate::InsuranceAddon) -> crate::InsuranceAddonExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InsuranceAddon(id={})", value.id()));
        crate::InsuranceAddonExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn piano_handling<'a>(value: &'a crate::PianoHandling) -> crate::PianoHandlingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PianoHandling(id={})", value.id()));
        crate::PianoHandlingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn stair_fee<'a>(value: &'a crate::StairFee) -> crate::StairFeeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("StairFee(id={})", value.id()));
        crate::StairFeeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn long_carry_fee<'a>(value: &'a crate::LongCarryFee) -> crate::LongCarryFeeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LongCarryFee(id={})", value.id()));
        crate::LongCarryFeeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn hoisting_service<'a>(value: &'a crate::HoistingService) -> crate::HoistingServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("HoistingService(id={})", value.id()));
        crate::HoistingServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_transport<'a>(value: &'a crate::VehicleTransport) -> crate::VehicleTransportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleTransport(id={})", value.id()));
        crate::VehicleTransportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn pet_relocation_service<'a>(value: &'a crate::PetRelocationService) -> crate::PetRelocationServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PetRelocationService(id={})", value.id()));
        crate::PetRelocationServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn campaign<'a>(value: &'a crate::Campaign) -> crate::CampaignExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Campaign(id={})", value.id()));
        crate::CampaignExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn discount_code<'a>(value: &'a crate::DiscountCode) -> crate::DiscountCodeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DiscountCode(id={})", value.id()));
        crate::DiscountCodeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn lead<'a>(value: &'a crate::Lead) -> crate::LeadExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Lead(id={})", value.id()));
        crate::LeadExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn sales_opportunity<'a>(value: &'a crate::SalesOpportunity) -> crate::SalesOpportunityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SalesOpportunity(id={})", value.id()));
        crate::SalesOpportunityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn lead_activity<'a>(value: &'a crate::LeadActivity) -> crate::LeadActivityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LeadActivity(id={})", value.id()));
        crate::LeadActivityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn conversion_event<'a>(value: &'a crate::ConversionEvent) -> crate::ConversionEventExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ConversionEvent(id={})", value.id()));
        crate::ConversionEventExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn conversion_metric<'a>(value: &'a crate::ConversionMetric) -> crate::ConversionMetricExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ConversionMetric(id={})", value.id()));
        crate::ConversionMetricExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn ad_spend<'a>(value: &'a crate::AdSpend) -> crate::AdSpendExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AdSpend(id={})", value.id()));
        crate::AdSpendExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn social_media_post<'a>(value: &'a crate::SocialMediaPost) -> crate::SocialMediaPostExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SocialMediaPost(id={})", value.id()));
        crate::SocialMediaPostExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn email_blast<'a>(value: &'a crate::EmailBlast) -> crate::EmailBlastExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EmailBlast(id={})", value.id()));
        crate::EmailBlastExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn sms_campaign<'a>(value: &'a crate::SmsCampaign) -> crate::SmsCampaignExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SmsCampaign(id={})", value.id()));
        crate::SmsCampaignExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn sales_script<'a>(value: &'a crate::SalesScript) -> crate::SalesScriptExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SalesScript(id={})", value.id()));
        crate::SalesScriptExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn objection_handling_guide<'a>(value: &'a crate::ObjectionHandlingGuide) -> crate::ObjectionHandlingGuideExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ObjectionHandlingGuide(id={})", value.id()));
        crate::ObjectionHandlingGuideExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn competitor_analysis<'a>(value: &'a crate::CompetitorAnalysis) -> crate::CompetitorAnalysisExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CompetitorAnalysis(id={})", value.id()));
        crate::CompetitorAnalysisExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn sales_territory<'a>(value: &'a crate::SalesTerritory) -> crate::SalesTerritoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SalesTerritory(id={})", value.id()));
        crate::SalesTerritoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payment<'a>(value: &'a crate::Payment) -> crate::PaymentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Payment(id={})", value.id()));
        crate::PaymentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn invoice<'a>(value: &'a crate::Invoice) -> crate::InvoiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Invoice(id={})", value.id()));
        crate::InvoiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn invoice_line<'a>(value: &'a crate::InvoiceLine) -> crate::InvoiceLineExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InvoiceLine(id={})", value.id()));
        crate::InvoiceLineExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn refund<'a>(value: &'a crate::Refund) -> crate::RefundExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Refund(id={})", value.id()));
        crate::RefundExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn expense<'a>(value: &'a crate::Expense) -> crate::ExpenseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Expense(id={})", value.id()));
        crate::ExpenseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vat_rate<'a>(value: &'a crate::VatRate) -> crate::VatRateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VatRate(id={})", value.id()));
        crate::VatRateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn journal_entry<'a>(value: &'a crate::JournalEntry) -> crate::JournalEntryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("JournalEntry(id={})", value.id()));
        crate::JournalEntryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn account<'a>(value: &'a crate::Account) -> crate::AccountExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Account(id={})", value.id()));
        crate::AccountExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn financial_summary<'a>(value: &'a crate::FinancialSummary) -> crate::FinancialSummaryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FinancialSummary(id={})", value.id()));
        crate::FinancialSummaryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tax_document<'a>(value: &'a crate::TaxDocument) -> crate::TaxDocumentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TaxDocument(id={})", value.id()));
        crate::TaxDocumentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn bank_transaction<'a>(value: &'a crate::BankTransaction) -> crate::BankTransactionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BankTransaction(id={})", value.id()));
        crate::BankTransactionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn merchant_fee<'a>(value: &'a crate::MerchantFee) -> crate::MerchantFeeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MerchantFee(id={})", value.id()));
        crate::MerchantFeeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn chargeback_record<'a>(value: &'a crate::ChargebackRecord) -> crate::ChargebackRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ChargebackRecord(id={})", value.id()));
        crate::ChargebackRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn credit_note<'a>(value: &'a crate::CreditNote) -> crate::CreditNoteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CreditNote(id={})", value.id()));
        crate::CreditNoteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn debit_note<'a>(value: &'a crate::DebitNote) -> crate::DebitNoteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DebitNote(id={})", value.id()));
        crate::DebitNoteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn audit_adjustment<'a>(value: &'a crate::AuditAdjustment) -> crate::AuditAdjustmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AuditAdjustment(id={})", value.id()));
        crate::AuditAdjustmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fiscal_year<'a>(value: &'a crate::FiscalYear) -> crate::FiscalYearExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FiscalYear(id={})", value.id()));
        crate::FiscalYearExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle<'a>(value: &'a crate::Vehicle) -> crate::VehicleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Vehicle(id={})", value.id()));
        crate::VehicleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

