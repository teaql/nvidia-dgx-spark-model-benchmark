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

    pub fn equipment<'a>(value: &'a crate::Equipment) -> crate::EquipmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Equipment(id={})", value.id()));
        crate::EquipmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn consumable<'a>(value: &'a crate::Consumable) -> crate::ConsumableExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Consumable(id={})", value.id()));
        crate::ConsumableExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn asset_assignment<'a>(value: &'a crate::AssetAssignment) -> crate::AssetAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AssetAssignment(id={})", value.id()));
        crate::AssetAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn asset_inspection<'a>(value: &'a crate::AssetInspection) -> crate::AssetInspectionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AssetInspection(id={})", value.id()));
        crate::AssetInspectionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn maintenance_schedule<'a>(value: &'a crate::MaintenanceSchedule) -> crate::MaintenanceScheduleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MaintenanceSchedule(id={})", value.id()));
        crate::MaintenanceScheduleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn maintenance_event<'a>(value: &'a crate::MaintenanceEvent) -> crate::MaintenanceEventExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MaintenanceEvent(id={})", value.id()));
        crate::MaintenanceEventExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fuel_record<'a>(value: &'a crate::FuelRecord) -> crate::FuelRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FuelRecord(id={})", value.id()));
        crate::FuelRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn supplier<'a>(value: &'a crate::Supplier) -> crate::SupplierExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Supplier(id={})", value.id()));
        crate::SupplierExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn gps_tracker<'a>(value: &'a crate::GpsTracker) -> crate::GpsTrackerExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("GpsTracker(id={})", value.id()));
        crate::GpsTrackerExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn dashcam_footage<'a>(value: &'a crate::DashcamFootage) -> crate::DashcamFootageExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DashcamFootage(id={})", value.id()));
        crate::DashcamFootageExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tire_replacement<'a>(value: &'a crate::TireReplacement) -> crate::TireReplacementExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TireReplacement(id={})", value.id()));
        crate::TireReplacementExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn oil_change_log<'a>(value: &'a crate::OilChangeLog) -> crate::OilChangeLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OilChangeLog(id={})", value.id()));
        crate::OilChangeLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn registration_renewal<'a>(value: &'a crate::RegistrationRenewal) -> crate::RegistrationRenewalExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RegistrationRenewal(id={})", value.id()));
        crate::RegistrationRenewalExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn insurance_card<'a>(value: &'a crate::InsuranceCard) -> crate::InsuranceCardExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InsuranceCard(id={})", value.id()));
        crate::InsuranceCardExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn depreciation_schedule<'a>(value: &'a crate::DepreciationSchedule) -> crate::DepreciationScheduleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DepreciationSchedule(id={})", value.id()));
        crate::DepreciationScheduleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn scrap_record<'a>(value: &'a crate::ScrapRecord) -> crate::ScrapRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ScrapRecord(id={})", value.id()));
        crate::ScrapRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn contract<'a>(value: &'a crate::Contract) -> crate::ContractExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Contract(id={})", value.id()));
        crate::ContractExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn insurance_policy<'a>(value: &'a crate::InsurancePolicy) -> crate::InsurancePolicyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InsurancePolicy(id={})", value.id()));
        crate::InsurancePolicyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn insurance_claim<'a>(value: &'a crate::InsuranceClaim) -> crate::InsuranceClaimExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InsuranceClaim(id={})", value.id()));
        crate::InsuranceClaimExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn document<'a>(value: &'a crate::Document) -> crate::DocumentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Document(id={})", value.id()));
        crate::DocumentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn document_version<'a>(value: &'a crate::DocumentVersion) -> crate::DocumentVersionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DocumentVersion(id={})", value.id()));
        crate::DocumentVersionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn compliance_check<'a>(value: &'a crate::ComplianceCheck) -> crate::ComplianceCheckExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ComplianceCheck(id={})", value.id()));
        crate::ComplianceCheckExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn data_retention_policy<'a>(value: &'a crate::DataRetentionPolicy) -> crate::DataRetentionPolicyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DataRetentionPolicy(id={})", value.id()));
        crate::DataRetentionPolicyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn recovery_request<'a>(value: &'a crate::RecoveryRequest) -> crate::RecoveryRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RecoveryRequest(id={})", value.id()));
        crate::RecoveryRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn nda_agreement<'a>(value: &'a crate::NdaAgreement) -> crate::NdaAgreementExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("NdaAgreement(id={})", value.id()));
        crate::NdaAgreementExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn terms_of_service<'a>(value: &'a crate::TermsOfService) -> crate::TermsOfServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TermsOfService(id={})", value.id()));
        crate::TermsOfServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn privacy_policy<'a>(value: &'a crate::PrivacyPolicy) -> crate::PrivacyPolicyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PrivacyPolicy(id={})", value.id()));
        crate::PrivacyPolicyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn cookie_consent<'a>(value: &'a crate::CookieConsent) -> crate::CookieConsentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CookieConsent(id={})", value.id()));
        crate::CookieConsentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn gdpr_request<'a>(value: &'a crate::GdprRequest) -> crate::GdprRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("GdprRequest(id={})", value.id()));
        crate::GdprRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn osha_incident<'a>(value: &'a crate::OshaIncident) -> crate::OshaIncidentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OshaIncident(id={})", value.id()));
        crate::OshaIncidentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn user_account<'a>(value: &'a crate::UserAccount) -> crate::UserAccountExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UserAccount(id={})", value.id()));
        crate::UserAccountExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn role<'a>(value: &'a crate::Role) -> crate::RoleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Role(id={})", value.id()));
        crate::RoleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn permission<'a>(value: &'a crate::Permission) -> crate::PermissionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Permission(id={})", value.id()));
        crate::PermissionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn user_role_assignment<'a>(value: &'a crate::UserRoleAssignment) -> crate::UserRoleAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UserRoleAssignment(id={})", value.id()));
        crate::UserRoleAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn role_permission<'a>(value: &'a crate::RolePermission) -> crate::RolePermissionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RolePermission(id={})", value.id()));
        crate::RolePermissionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn magic_link<'a>(value: &'a crate::MagicLink) -> crate::MagicLinkExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MagicLink(id={})", value.id()));
        crate::MagicLinkExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn user_session<'a>(value: &'a crate::UserSession) -> crate::UserSessionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UserSession(id={})", value.id()));
        crate::UserSessionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn password_reset<'a>(value: &'a crate::PasswordReset) -> crate::PasswordResetExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PasswordReset(id={})", value.id()));
        crate::PasswordResetExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn two_factor_auth<'a>(value: &'a crate::TwoFactorAuth) -> crate::TwoFactorAuthExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TwoFactorAuth(id={})", value.id()));
        crate::TwoFactorAuthExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn access_token<'a>(value: &'a crate::AccessToken) -> crate::AccessTokenExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AccessToken(id={})", value.id()));
        crate::AccessTokenExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn activity_log<'a>(value: &'a crate::ActivityLog) -> crate::ActivityLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ActivityLog(id={})", value.id()));
        crate::ActivityLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn audit_log<'a>(value: &'a crate::AuditLog) -> crate::AuditLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AuditLog(id={})", value.id()));
        crate::AuditLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn entity_change<'a>(value: &'a crate::EntityChange) -> crate::EntityChangeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EntityChange(id={})", value.id()));
        crate::EntityChangeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn change_set<'a>(value: &'a crate::ChangeSet) -> crate::ChangeSetExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ChangeSet(id={})", value.id()));
        crate::ChangeSetExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn login_attempt<'a>(value: &'a crate::LoginAttempt) -> crate::LoginAttemptExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoginAttempt(id={})", value.id()));
        crate::LoginAttemptExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn failed_auth_log<'a>(value: &'a crate::FailedAuthLog) -> crate::FailedAuthLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FailedAuthLog(id={})", value.id()));
        crate::FailedAuthLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn notification<'a>(value: &'a crate::Notification) -> crate::NotificationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Notification(id={})", value.id()));
        crate::NotificationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn notification_template<'a>(value: &'a crate::NotificationTemplate) -> crate::NotificationTemplateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("NotificationTemplate(id={})", value.id()));
        crate::NotificationTemplateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn automation_rule<'a>(value: &'a crate::AutomationRule) -> crate::AutomationRuleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AutomationRule(id={})", value.id()));
        crate::AutomationRuleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn automation_trigger<'a>(value: &'a crate::AutomationTrigger) -> crate::AutomationTriggerExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AutomationTrigger(id={})", value.id()));
        crate::AutomationTriggerExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn automation_action<'a>(value: &'a crate::AutomationAction) -> crate::AutomationActionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AutomationAction(id={})", value.id()));
        crate::AutomationActionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn sms_delivery_receipt<'a>(value: &'a crate::SmsDeliveryReceipt) -> crate::SmsDeliveryReceiptExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SmsDeliveryReceipt(id={})", value.id()));
        crate::SmsDeliveryReceiptExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn email_bounce_log<'a>(value: &'a crate::EmailBounceLog) -> crate::EmailBounceLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EmailBounceLog(id={})", value.id()));
        crate::EmailBounceLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn api_client<'a>(value: &'a crate::ApiClient) -> crate::ApiClientExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ApiClient(id={})", value.id()));
        crate::ApiClientExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn api_endpoint<'a>(value: &'a crate::ApiEndpoint) -> crate::ApiEndpointExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ApiEndpoint(id={})", value.id()));
        crate::ApiEndpointExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn webhook<'a>(value: &'a crate::Webhook) -> crate::WebhookExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Webhook(id={})", value.id()));
        crate::WebhookExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn webhook_delivery<'a>(value: &'a crate::WebhookDelivery) -> crate::WebhookDeliveryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("WebhookDelivery(id={})", value.id()));
        crate::WebhookDeliveryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn integration_mapping<'a>(value: &'a crate::IntegrationMapping) -> crate::IntegrationMappingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("IntegrationMapping(id={})", value.id()));
        crate::IntegrationMappingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn sync_job<'a>(value: &'a crate::SyncJob) -> crate::SyncJobExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SyncJob(id={})", value.id()));
        crate::SyncJobExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn api_rate_limit<'a>(value: &'a crate::ApiRateLimit) -> crate::ApiRateLimitExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ApiRateLimit(id={})", value.id()));
        crate::ApiRateLimitExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_180<'a>(value: &'a crate::CustomEntity180) -> crate::CustomEntity180Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity180(id={})", value.id()));
        crate::CustomEntity180Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_181<'a>(value: &'a crate::CustomEntity181) -> crate::CustomEntity181Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity181(id={})", value.id()));
        crate::CustomEntity181Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_182<'a>(value: &'a crate::CustomEntity182) -> crate::CustomEntity182Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity182(id={})", value.id()));
        crate::CustomEntity182Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_183<'a>(value: &'a crate::CustomEntity183) -> crate::CustomEntity183Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity183(id={})", value.id()));
        crate::CustomEntity183Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_184<'a>(value: &'a crate::CustomEntity184) -> crate::CustomEntity184Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity184(id={})", value.id()));
        crate::CustomEntity184Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_185<'a>(value: &'a crate::CustomEntity185) -> crate::CustomEntity185Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity185(id={})", value.id()));
        crate::CustomEntity185Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_186<'a>(value: &'a crate::CustomEntity186) -> crate::CustomEntity186Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity186(id={})", value.id()));
        crate::CustomEntity186Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_187<'a>(value: &'a crate::CustomEntity187) -> crate::CustomEntity187Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity187(id={})", value.id()));
        crate::CustomEntity187Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_188<'a>(value: &'a crate::CustomEntity188) -> crate::CustomEntity188Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity188(id={})", value.id()));
        crate::CustomEntity188Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_189<'a>(value: &'a crate::CustomEntity189) -> crate::CustomEntity189Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity189(id={})", value.id()));
        crate::CustomEntity189Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_190<'a>(value: &'a crate::CustomEntity190) -> crate::CustomEntity190Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity190(id={})", value.id()));
        crate::CustomEntity190Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_191<'a>(value: &'a crate::CustomEntity191) -> crate::CustomEntity191Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity191(id={})", value.id()));
        crate::CustomEntity191Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_192<'a>(value: &'a crate::CustomEntity192) -> crate::CustomEntity192Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity192(id={})", value.id()));
        crate::CustomEntity192Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_193<'a>(value: &'a crate::CustomEntity193) -> crate::CustomEntity193Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity193(id={})", value.id()));
        crate::CustomEntity193Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_194<'a>(value: &'a crate::CustomEntity194) -> crate::CustomEntity194Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity194(id={})", value.id()));
        crate::CustomEntity194Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_195<'a>(value: &'a crate::CustomEntity195) -> crate::CustomEntity195Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity195(id={})", value.id()));
        crate::CustomEntity195Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_196<'a>(value: &'a crate::CustomEntity196) -> crate::CustomEntity196Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity196(id={})", value.id()));
        crate::CustomEntity196Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_197<'a>(value: &'a crate::CustomEntity197) -> crate::CustomEntity197Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity197(id={})", value.id()));
        crate::CustomEntity197Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_198<'a>(value: &'a crate::CustomEntity198) -> crate::CustomEntity198Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity198(id={})", value.id()));
        crate::CustomEntity198Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_199<'a>(value: &'a crate::CustomEntity199) -> crate::CustomEntity199Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity199(id={})", value.id()));
        crate::CustomEntity199Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_200<'a>(value: &'a crate::CustomEntity200) -> crate::CustomEntity200Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity200(id={})", value.id()));
        crate::CustomEntity200Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_201<'a>(value: &'a crate::CustomEntity201) -> crate::CustomEntity201Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity201(id={})", value.id()));
        crate::CustomEntity201Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_202<'a>(value: &'a crate::CustomEntity202) -> crate::CustomEntity202Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity202(id={})", value.id()));
        crate::CustomEntity202Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_203<'a>(value: &'a crate::CustomEntity203) -> crate::CustomEntity203Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity203(id={})", value.id()));
        crate::CustomEntity203Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_204<'a>(value: &'a crate::CustomEntity204) -> crate::CustomEntity204Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity204(id={})", value.id()));
        crate::CustomEntity204Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_205<'a>(value: &'a crate::CustomEntity205) -> crate::CustomEntity205Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity205(id={})", value.id()));
        crate::CustomEntity205Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_206<'a>(value: &'a crate::CustomEntity206) -> crate::CustomEntity206Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity206(id={})", value.id()));
        crate::CustomEntity206Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_207<'a>(value: &'a crate::CustomEntity207) -> crate::CustomEntity207Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity207(id={})", value.id()));
        crate::CustomEntity207Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_208<'a>(value: &'a crate::CustomEntity208) -> crate::CustomEntity208Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity208(id={})", value.id()));
        crate::CustomEntity208Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_209<'a>(value: &'a crate::CustomEntity209) -> crate::CustomEntity209Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity209(id={})", value.id()));
        crate::CustomEntity209Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_210<'a>(value: &'a crate::CustomEntity210) -> crate::CustomEntity210Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity210(id={})", value.id()));
        crate::CustomEntity210Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_211<'a>(value: &'a crate::CustomEntity211) -> crate::CustomEntity211Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity211(id={})", value.id()));
        crate::CustomEntity211Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_212<'a>(value: &'a crate::CustomEntity212) -> crate::CustomEntity212Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity212(id={})", value.id()));
        crate::CustomEntity212Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_213<'a>(value: &'a crate::CustomEntity213) -> crate::CustomEntity213Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity213(id={})", value.id()));
        crate::CustomEntity213Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_214<'a>(value: &'a crate::CustomEntity214) -> crate::CustomEntity214Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity214(id={})", value.id()));
        crate::CustomEntity214Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_215<'a>(value: &'a crate::CustomEntity215) -> crate::CustomEntity215Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity215(id={})", value.id()));
        crate::CustomEntity215Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_216<'a>(value: &'a crate::CustomEntity216) -> crate::CustomEntity216Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity216(id={})", value.id()));
        crate::CustomEntity216Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_217<'a>(value: &'a crate::CustomEntity217) -> crate::CustomEntity217Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity217(id={})", value.id()));
        crate::CustomEntity217Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_218<'a>(value: &'a crate::CustomEntity218) -> crate::CustomEntity218Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity218(id={})", value.id()));
        crate::CustomEntity218Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_219<'a>(value: &'a crate::CustomEntity219) -> crate::CustomEntity219Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity219(id={})", value.id()));
        crate::CustomEntity219Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_220<'a>(value: &'a crate::CustomEntity220) -> crate::CustomEntity220Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity220(id={})", value.id()));
        crate::CustomEntity220Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_221<'a>(value: &'a crate::CustomEntity221) -> crate::CustomEntity221Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity221(id={})", value.id()));
        crate::CustomEntity221Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_222<'a>(value: &'a crate::CustomEntity222) -> crate::CustomEntity222Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity222(id={})", value.id()));
        crate::CustomEntity222Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_223<'a>(value: &'a crate::CustomEntity223) -> crate::CustomEntity223Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity223(id={})", value.id()));
        crate::CustomEntity223Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_224<'a>(value: &'a crate::CustomEntity224) -> crate::CustomEntity224Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity224(id={})", value.id()));
        crate::CustomEntity224Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_225<'a>(value: &'a crate::CustomEntity225) -> crate::CustomEntity225Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity225(id={})", value.id()));
        crate::CustomEntity225Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_226<'a>(value: &'a crate::CustomEntity226) -> crate::CustomEntity226Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity226(id={})", value.id()));
        crate::CustomEntity226Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_227<'a>(value: &'a crate::CustomEntity227) -> crate::CustomEntity227Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity227(id={})", value.id()));
        crate::CustomEntity227Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_228<'a>(value: &'a crate::CustomEntity228) -> crate::CustomEntity228Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity228(id={})", value.id()));
        crate::CustomEntity228Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_229<'a>(value: &'a crate::CustomEntity229) -> crate::CustomEntity229Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity229(id={})", value.id()));
        crate::CustomEntity229Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_230<'a>(value: &'a crate::CustomEntity230) -> crate::CustomEntity230Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity230(id={})", value.id()));
        crate::CustomEntity230Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_231<'a>(value: &'a crate::CustomEntity231) -> crate::CustomEntity231Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity231(id={})", value.id()));
        crate::CustomEntity231Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_232<'a>(value: &'a crate::CustomEntity232) -> crate::CustomEntity232Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity232(id={})", value.id()));
        crate::CustomEntity232Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_233<'a>(value: &'a crate::CustomEntity233) -> crate::CustomEntity233Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity233(id={})", value.id()));
        crate::CustomEntity233Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_234<'a>(value: &'a crate::CustomEntity234) -> crate::CustomEntity234Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity234(id={})", value.id()));
        crate::CustomEntity234Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_235<'a>(value: &'a crate::CustomEntity235) -> crate::CustomEntity235Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity235(id={})", value.id()));
        crate::CustomEntity235Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_236<'a>(value: &'a crate::CustomEntity236) -> crate::CustomEntity236Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity236(id={})", value.id()));
        crate::CustomEntity236Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_237<'a>(value: &'a crate::CustomEntity237) -> crate::CustomEntity237Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity237(id={})", value.id()));
        crate::CustomEntity237Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_238<'a>(value: &'a crate::CustomEntity238) -> crate::CustomEntity238Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity238(id={})", value.id()));
        crate::CustomEntity238Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_239<'a>(value: &'a crate::CustomEntity239) -> crate::CustomEntity239Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity239(id={})", value.id()));
        crate::CustomEntity239Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_240<'a>(value: &'a crate::CustomEntity240) -> crate::CustomEntity240Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity240(id={})", value.id()));
        crate::CustomEntity240Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_241<'a>(value: &'a crate::CustomEntity241) -> crate::CustomEntity241Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity241(id={})", value.id()));
        crate::CustomEntity241Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_242<'a>(value: &'a crate::CustomEntity242) -> crate::CustomEntity242Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity242(id={})", value.id()));
        crate::CustomEntity242Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_243<'a>(value: &'a crate::CustomEntity243) -> crate::CustomEntity243Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity243(id={})", value.id()));
        crate::CustomEntity243Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_244<'a>(value: &'a crate::CustomEntity244) -> crate::CustomEntity244Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity244(id={})", value.id()));
        crate::CustomEntity244Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_245<'a>(value: &'a crate::CustomEntity245) -> crate::CustomEntity245Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity245(id={})", value.id()));
        crate::CustomEntity245Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_246<'a>(value: &'a crate::CustomEntity246) -> crate::CustomEntity246Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity246(id={})", value.id()));
        crate::CustomEntity246Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_247<'a>(value: &'a crate::CustomEntity247) -> crate::CustomEntity247Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity247(id={})", value.id()));
        crate::CustomEntity247Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_248<'a>(value: &'a crate::CustomEntity248) -> crate::CustomEntity248Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity248(id={})", value.id()));
        crate::CustomEntity248Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_249<'a>(value: &'a crate::CustomEntity249) -> crate::CustomEntity249Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity249(id={})", value.id()));
        crate::CustomEntity249Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_250<'a>(value: &'a crate::CustomEntity250) -> crate::CustomEntity250Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity250(id={})", value.id()));
        crate::CustomEntity250Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_251<'a>(value: &'a crate::CustomEntity251) -> crate::CustomEntity251Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity251(id={})", value.id()));
        crate::CustomEntity251Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_252<'a>(value: &'a crate::CustomEntity252) -> crate::CustomEntity252Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity252(id={})", value.id()));
        crate::CustomEntity252Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_253<'a>(value: &'a crate::CustomEntity253) -> crate::CustomEntity253Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity253(id={})", value.id()));
        crate::CustomEntity253Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_254<'a>(value: &'a crate::CustomEntity254) -> crate::CustomEntity254Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity254(id={})", value.id()));
        crate::CustomEntity254Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_255<'a>(value: &'a crate::CustomEntity255) -> crate::CustomEntity255Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity255(id={})", value.id()));
        crate::CustomEntity255Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_256<'a>(value: &'a crate::CustomEntity256) -> crate::CustomEntity256Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity256(id={})", value.id()));
        crate::CustomEntity256Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_257<'a>(value: &'a crate::CustomEntity257) -> crate::CustomEntity257Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity257(id={})", value.id()));
        crate::CustomEntity257Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_258<'a>(value: &'a crate::CustomEntity258) -> crate::CustomEntity258Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity258(id={})", value.id()));
        crate::CustomEntity258Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_259<'a>(value: &'a crate::CustomEntity259) -> crate::CustomEntity259Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity259(id={})", value.id()));
        crate::CustomEntity259Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_260<'a>(value: &'a crate::CustomEntity260) -> crate::CustomEntity260Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity260(id={})", value.id()));
        crate::CustomEntity260Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_261<'a>(value: &'a crate::CustomEntity261) -> crate::CustomEntity261Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity261(id={})", value.id()));
        crate::CustomEntity261Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_262<'a>(value: &'a crate::CustomEntity262) -> crate::CustomEntity262Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity262(id={})", value.id()));
        crate::CustomEntity262Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_263<'a>(value: &'a crate::CustomEntity263) -> crate::CustomEntity263Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity263(id={})", value.id()));
        crate::CustomEntity263Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_264<'a>(value: &'a crate::CustomEntity264) -> crate::CustomEntity264Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity264(id={})", value.id()));
        crate::CustomEntity264Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_265<'a>(value: &'a crate::CustomEntity265) -> crate::CustomEntity265Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity265(id={})", value.id()));
        crate::CustomEntity265Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_266<'a>(value: &'a crate::CustomEntity266) -> crate::CustomEntity266Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity266(id={})", value.id()));
        crate::CustomEntity266Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_267<'a>(value: &'a crate::CustomEntity267) -> crate::CustomEntity267Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity267(id={})", value.id()));
        crate::CustomEntity267Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_268<'a>(value: &'a crate::CustomEntity268) -> crate::CustomEntity268Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity268(id={})", value.id()));
        crate::CustomEntity268Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_269<'a>(value: &'a crate::CustomEntity269) -> crate::CustomEntity269Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity269(id={})", value.id()));
        crate::CustomEntity269Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_270<'a>(value: &'a crate::CustomEntity270) -> crate::CustomEntity270Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity270(id={})", value.id()));
        crate::CustomEntity270Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_271<'a>(value: &'a crate::CustomEntity271) -> crate::CustomEntity271Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity271(id={})", value.id()));
        crate::CustomEntity271Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_272<'a>(value: &'a crate::CustomEntity272) -> crate::CustomEntity272Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity272(id={})", value.id()));
        crate::CustomEntity272Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_273<'a>(value: &'a crate::CustomEntity273) -> crate::CustomEntity273Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity273(id={})", value.id()));
        crate::CustomEntity273Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_274<'a>(value: &'a crate::CustomEntity274) -> crate::CustomEntity274Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity274(id={})", value.id()));
        crate::CustomEntity274Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_275<'a>(value: &'a crate::CustomEntity275) -> crate::CustomEntity275Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity275(id={})", value.id()));
        crate::CustomEntity275Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_276<'a>(value: &'a crate::CustomEntity276) -> crate::CustomEntity276Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity276(id={})", value.id()));
        crate::CustomEntity276Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_277<'a>(value: &'a crate::CustomEntity277) -> crate::CustomEntity277Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity277(id={})", value.id()));
        crate::CustomEntity277Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_278<'a>(value: &'a crate::CustomEntity278) -> crate::CustomEntity278Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity278(id={})", value.id()));
        crate::CustomEntity278Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_279<'a>(value: &'a crate::CustomEntity279) -> crate::CustomEntity279Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity279(id={})", value.id()));
        crate::CustomEntity279Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_280<'a>(value: &'a crate::CustomEntity280) -> crate::CustomEntity280Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity280(id={})", value.id()));
        crate::CustomEntity280Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_281<'a>(value: &'a crate::CustomEntity281) -> crate::CustomEntity281Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity281(id={})", value.id()));
        crate::CustomEntity281Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_282<'a>(value: &'a crate::CustomEntity282) -> crate::CustomEntity282Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity282(id={})", value.id()));
        crate::CustomEntity282Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_283<'a>(value: &'a crate::CustomEntity283) -> crate::CustomEntity283Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity283(id={})", value.id()));
        crate::CustomEntity283Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_284<'a>(value: &'a crate::CustomEntity284) -> crate::CustomEntity284Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity284(id={})", value.id()));
        crate::CustomEntity284Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_285<'a>(value: &'a crate::CustomEntity285) -> crate::CustomEntity285Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity285(id={})", value.id()));
        crate::CustomEntity285Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_286<'a>(value: &'a crate::CustomEntity286) -> crate::CustomEntity286Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity286(id={})", value.id()));
        crate::CustomEntity286Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_287<'a>(value: &'a crate::CustomEntity287) -> crate::CustomEntity287Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity287(id={})", value.id()));
        crate::CustomEntity287Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_288<'a>(value: &'a crate::CustomEntity288) -> crate::CustomEntity288Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity288(id={})", value.id()));
        crate::CustomEntity288Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_289<'a>(value: &'a crate::CustomEntity289) -> crate::CustomEntity289Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity289(id={})", value.id()));
        crate::CustomEntity289Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_290<'a>(value: &'a crate::CustomEntity290) -> crate::CustomEntity290Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity290(id={})", value.id()));
        crate::CustomEntity290Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_291<'a>(value: &'a crate::CustomEntity291) -> crate::CustomEntity291Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity291(id={})", value.id()));
        crate::CustomEntity291Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_292<'a>(value: &'a crate::CustomEntity292) -> crate::CustomEntity292Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity292(id={})", value.id()));
        crate::CustomEntity292Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_293<'a>(value: &'a crate::CustomEntity293) -> crate::CustomEntity293Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity293(id={})", value.id()));
        crate::CustomEntity293Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_294<'a>(value: &'a crate::CustomEntity294) -> crate::CustomEntity294Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity294(id={})", value.id()));
        crate::CustomEntity294Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_295<'a>(value: &'a crate::CustomEntity295) -> crate::CustomEntity295Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity295(id={})", value.id()));
        crate::CustomEntity295Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_296<'a>(value: &'a crate::CustomEntity296) -> crate::CustomEntity296Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity296(id={})", value.id()));
        crate::CustomEntity296Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_297<'a>(value: &'a crate::CustomEntity297) -> crate::CustomEntity297Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity297(id={})", value.id()));
        crate::CustomEntity297Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_298<'a>(value: &'a crate::CustomEntity298) -> crate::CustomEntity298Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity298(id={})", value.id()));
        crate::CustomEntity298Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_299<'a>(value: &'a crate::CustomEntity299) -> crate::CustomEntity299Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity299(id={})", value.id()));
        crate::CustomEntity299Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_300<'a>(value: &'a crate::CustomEntity300) -> crate::CustomEntity300Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity300(id={})", value.id()));
        crate::CustomEntity300Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_301<'a>(value: &'a crate::CustomEntity301) -> crate::CustomEntity301Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity301(id={})", value.id()));
        crate::CustomEntity301Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_302<'a>(value: &'a crate::CustomEntity302) -> crate::CustomEntity302Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity302(id={})", value.id()));
        crate::CustomEntity302Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_303<'a>(value: &'a crate::CustomEntity303) -> crate::CustomEntity303Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity303(id={})", value.id()));
        crate::CustomEntity303Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_304<'a>(value: &'a crate::CustomEntity304) -> crate::CustomEntity304Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity304(id={})", value.id()));
        crate::CustomEntity304Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_305<'a>(value: &'a crate::CustomEntity305) -> crate::CustomEntity305Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity305(id={})", value.id()));
        crate::CustomEntity305Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_306<'a>(value: &'a crate::CustomEntity306) -> crate::CustomEntity306Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity306(id={})", value.id()));
        crate::CustomEntity306Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_307<'a>(value: &'a crate::CustomEntity307) -> crate::CustomEntity307Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity307(id={})", value.id()));
        crate::CustomEntity307Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_308<'a>(value: &'a crate::CustomEntity308) -> crate::CustomEntity308Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity308(id={})", value.id()));
        crate::CustomEntity308Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_309<'a>(value: &'a crate::CustomEntity309) -> crate::CustomEntity309Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity309(id={})", value.id()));
        crate::CustomEntity309Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_310<'a>(value: &'a crate::CustomEntity310) -> crate::CustomEntity310Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity310(id={})", value.id()));
        crate::CustomEntity310Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_311<'a>(value: &'a crate::CustomEntity311) -> crate::CustomEntity311Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity311(id={})", value.id()));
        crate::CustomEntity311Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_312<'a>(value: &'a crate::CustomEntity312) -> crate::CustomEntity312Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity312(id={})", value.id()));
        crate::CustomEntity312Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_313<'a>(value: &'a crate::CustomEntity313) -> crate::CustomEntity313Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity313(id={})", value.id()));
        crate::CustomEntity313Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_314<'a>(value: &'a crate::CustomEntity314) -> crate::CustomEntity314Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity314(id={})", value.id()));
        crate::CustomEntity314Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_315<'a>(value: &'a crate::CustomEntity315) -> crate::CustomEntity315Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity315(id={})", value.id()));
        crate::CustomEntity315Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_316<'a>(value: &'a crate::CustomEntity316) -> crate::CustomEntity316Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity316(id={})", value.id()));
        crate::CustomEntity316Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_317<'a>(value: &'a crate::CustomEntity317) -> crate::CustomEntity317Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity317(id={})", value.id()));
        crate::CustomEntity317Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_318<'a>(value: &'a crate::CustomEntity318) -> crate::CustomEntity318Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity318(id={})", value.id()));
        crate::CustomEntity318Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_319<'a>(value: &'a crate::CustomEntity319) -> crate::CustomEntity319Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity319(id={})", value.id()));
        crate::CustomEntity319Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_320<'a>(value: &'a crate::CustomEntity320) -> crate::CustomEntity320Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity320(id={})", value.id()));
        crate::CustomEntity320Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_321<'a>(value: &'a crate::CustomEntity321) -> crate::CustomEntity321Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity321(id={})", value.id()));
        crate::CustomEntity321Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_322<'a>(value: &'a crate::CustomEntity322) -> crate::CustomEntity322Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity322(id={})", value.id()));
        crate::CustomEntity322Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_323<'a>(value: &'a crate::CustomEntity323) -> crate::CustomEntity323Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity323(id={})", value.id()));
        crate::CustomEntity323Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_324<'a>(value: &'a crate::CustomEntity324) -> crate::CustomEntity324Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity324(id={})", value.id()));
        crate::CustomEntity324Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_325<'a>(value: &'a crate::CustomEntity325) -> crate::CustomEntity325Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity325(id={})", value.id()));
        crate::CustomEntity325Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_326<'a>(value: &'a crate::CustomEntity326) -> crate::CustomEntity326Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity326(id={})", value.id()));
        crate::CustomEntity326Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_327<'a>(value: &'a crate::CustomEntity327) -> crate::CustomEntity327Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity327(id={})", value.id()));
        crate::CustomEntity327Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_328<'a>(value: &'a crate::CustomEntity328) -> crate::CustomEntity328Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity328(id={})", value.id()));
        crate::CustomEntity328Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_329<'a>(value: &'a crate::CustomEntity329) -> crate::CustomEntity329Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity329(id={})", value.id()));
        crate::CustomEntity329Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_330<'a>(value: &'a crate::CustomEntity330) -> crate::CustomEntity330Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity330(id={})", value.id()));
        crate::CustomEntity330Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_331<'a>(value: &'a crate::CustomEntity331) -> crate::CustomEntity331Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity331(id={})", value.id()));
        crate::CustomEntity331Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_332<'a>(value: &'a crate::CustomEntity332) -> crate::CustomEntity332Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity332(id={})", value.id()));
        crate::CustomEntity332Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_333<'a>(value: &'a crate::CustomEntity333) -> crate::CustomEntity333Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity333(id={})", value.id()));
        crate::CustomEntity333Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_334<'a>(value: &'a crate::CustomEntity334) -> crate::CustomEntity334Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity334(id={})", value.id()));
        crate::CustomEntity334Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_335<'a>(value: &'a crate::CustomEntity335) -> crate::CustomEntity335Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity335(id={})", value.id()));
        crate::CustomEntity335Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_336<'a>(value: &'a crate::CustomEntity336) -> crate::CustomEntity336Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity336(id={})", value.id()));
        crate::CustomEntity336Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_337<'a>(value: &'a crate::CustomEntity337) -> crate::CustomEntity337Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity337(id={})", value.id()));
        crate::CustomEntity337Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_338<'a>(value: &'a crate::CustomEntity338) -> crate::CustomEntity338Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity338(id={})", value.id()));
        crate::CustomEntity338Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_339<'a>(value: &'a crate::CustomEntity339) -> crate::CustomEntity339Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity339(id={})", value.id()));
        crate::CustomEntity339Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_340<'a>(value: &'a crate::CustomEntity340) -> crate::CustomEntity340Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity340(id={})", value.id()));
        crate::CustomEntity340Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_341<'a>(value: &'a crate::CustomEntity341) -> crate::CustomEntity341Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity341(id={})", value.id()));
        crate::CustomEntity341Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_342<'a>(value: &'a crate::CustomEntity342) -> crate::CustomEntity342Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity342(id={})", value.id()));
        crate::CustomEntity342Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_343<'a>(value: &'a crate::CustomEntity343) -> crate::CustomEntity343Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity343(id={})", value.id()));
        crate::CustomEntity343Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_344<'a>(value: &'a crate::CustomEntity344) -> crate::CustomEntity344Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity344(id={})", value.id()));
        crate::CustomEntity344Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_345<'a>(value: &'a crate::CustomEntity345) -> crate::CustomEntity345Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity345(id={})", value.id()));
        crate::CustomEntity345Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_346<'a>(value: &'a crate::CustomEntity346) -> crate::CustomEntity346Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity346(id={})", value.id()));
        crate::CustomEntity346Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_347<'a>(value: &'a crate::CustomEntity347) -> crate::CustomEntity347Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity347(id={})", value.id()));
        crate::CustomEntity347Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_348<'a>(value: &'a crate::CustomEntity348) -> crate::CustomEntity348Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity348(id={})", value.id()));
        crate::CustomEntity348Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_349<'a>(value: &'a crate::CustomEntity349) -> crate::CustomEntity349Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity349(id={})", value.id()));
        crate::CustomEntity349Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_350<'a>(value: &'a crate::CustomEntity350) -> crate::CustomEntity350Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity350(id={})", value.id()));
        crate::CustomEntity350Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_351<'a>(value: &'a crate::CustomEntity351) -> crate::CustomEntity351Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity351(id={})", value.id()));
        crate::CustomEntity351Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_352<'a>(value: &'a crate::CustomEntity352) -> crate::CustomEntity352Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity352(id={})", value.id()));
        crate::CustomEntity352Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_353<'a>(value: &'a crate::CustomEntity353) -> crate::CustomEntity353Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity353(id={})", value.id()));
        crate::CustomEntity353Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_354<'a>(value: &'a crate::CustomEntity354) -> crate::CustomEntity354Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity354(id={})", value.id()));
        crate::CustomEntity354Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_355<'a>(value: &'a crate::CustomEntity355) -> crate::CustomEntity355Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity355(id={})", value.id()));
        crate::CustomEntity355Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_356<'a>(value: &'a crate::CustomEntity356) -> crate::CustomEntity356Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity356(id={})", value.id()));
        crate::CustomEntity356Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_357<'a>(value: &'a crate::CustomEntity357) -> crate::CustomEntity357Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity357(id={})", value.id()));
        crate::CustomEntity357Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_358<'a>(value: &'a crate::CustomEntity358) -> crate::CustomEntity358Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity358(id={})", value.id()));
        crate::CustomEntity358Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_359<'a>(value: &'a crate::CustomEntity359) -> crate::CustomEntity359Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity359(id={})", value.id()));
        crate::CustomEntity359Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_360<'a>(value: &'a crate::CustomEntity360) -> crate::CustomEntity360Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity360(id={})", value.id()));
        crate::CustomEntity360Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_361<'a>(value: &'a crate::CustomEntity361) -> crate::CustomEntity361Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity361(id={})", value.id()));
        crate::CustomEntity361Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_362<'a>(value: &'a crate::CustomEntity362) -> crate::CustomEntity362Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity362(id={})", value.id()));
        crate::CustomEntity362Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_363<'a>(value: &'a crate::CustomEntity363) -> crate::CustomEntity363Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity363(id={})", value.id()));
        crate::CustomEntity363Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_364<'a>(value: &'a crate::CustomEntity364) -> crate::CustomEntity364Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity364(id={})", value.id()));
        crate::CustomEntity364Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_365<'a>(value: &'a crate::CustomEntity365) -> crate::CustomEntity365Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity365(id={})", value.id()));
        crate::CustomEntity365Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_366<'a>(value: &'a crate::CustomEntity366) -> crate::CustomEntity366Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity366(id={})", value.id()));
        crate::CustomEntity366Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_367<'a>(value: &'a crate::CustomEntity367) -> crate::CustomEntity367Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity367(id={})", value.id()));
        crate::CustomEntity367Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_368<'a>(value: &'a crate::CustomEntity368) -> crate::CustomEntity368Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity368(id={})", value.id()));
        crate::CustomEntity368Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_369<'a>(value: &'a crate::CustomEntity369) -> crate::CustomEntity369Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity369(id={})", value.id()));
        crate::CustomEntity369Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_370<'a>(value: &'a crate::CustomEntity370) -> crate::CustomEntity370Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity370(id={})", value.id()));
        crate::CustomEntity370Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_371<'a>(value: &'a crate::CustomEntity371) -> crate::CustomEntity371Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity371(id={})", value.id()));
        crate::CustomEntity371Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_372<'a>(value: &'a crate::CustomEntity372) -> crate::CustomEntity372Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity372(id={})", value.id()));
        crate::CustomEntity372Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_373<'a>(value: &'a crate::CustomEntity373) -> crate::CustomEntity373Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity373(id={})", value.id()));
        crate::CustomEntity373Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_374<'a>(value: &'a crate::CustomEntity374) -> crate::CustomEntity374Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity374(id={})", value.id()));
        crate::CustomEntity374Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_375<'a>(value: &'a crate::CustomEntity375) -> crate::CustomEntity375Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity375(id={})", value.id()));
        crate::CustomEntity375Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_376<'a>(value: &'a crate::CustomEntity376) -> crate::CustomEntity376Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity376(id={})", value.id()));
        crate::CustomEntity376Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_377<'a>(value: &'a crate::CustomEntity377) -> crate::CustomEntity377Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity377(id={})", value.id()));
        crate::CustomEntity377Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_378<'a>(value: &'a crate::CustomEntity378) -> crate::CustomEntity378Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity378(id={})", value.id()));
        crate::CustomEntity378Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_379<'a>(value: &'a crate::CustomEntity379) -> crate::CustomEntity379Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity379(id={})", value.id()));
        crate::CustomEntity379Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_380<'a>(value: &'a crate::CustomEntity380) -> crate::CustomEntity380Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity380(id={})", value.id()));
        crate::CustomEntity380Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_381<'a>(value: &'a crate::CustomEntity381) -> crate::CustomEntity381Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity381(id={})", value.id()));
        crate::CustomEntity381Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_382<'a>(value: &'a crate::CustomEntity382) -> crate::CustomEntity382Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity382(id={})", value.id()));
        crate::CustomEntity382Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_383<'a>(value: &'a crate::CustomEntity383) -> crate::CustomEntity383Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity383(id={})", value.id()));
        crate::CustomEntity383Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_384<'a>(value: &'a crate::CustomEntity384) -> crate::CustomEntity384Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity384(id={})", value.id()));
        crate::CustomEntity384Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_385<'a>(value: &'a crate::CustomEntity385) -> crate::CustomEntity385Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity385(id={})", value.id()));
        crate::CustomEntity385Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_386<'a>(value: &'a crate::CustomEntity386) -> crate::CustomEntity386Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity386(id={})", value.id()));
        crate::CustomEntity386Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_387<'a>(value: &'a crate::CustomEntity387) -> crate::CustomEntity387Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity387(id={})", value.id()));
        crate::CustomEntity387Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_388<'a>(value: &'a crate::CustomEntity388) -> crate::CustomEntity388Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity388(id={})", value.id()));
        crate::CustomEntity388Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_389<'a>(value: &'a crate::CustomEntity389) -> crate::CustomEntity389Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity389(id={})", value.id()));
        crate::CustomEntity389Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_390<'a>(value: &'a crate::CustomEntity390) -> crate::CustomEntity390Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity390(id={})", value.id()));
        crate::CustomEntity390Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_391<'a>(value: &'a crate::CustomEntity391) -> crate::CustomEntity391Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity391(id={})", value.id()));
        crate::CustomEntity391Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_392<'a>(value: &'a crate::CustomEntity392) -> crate::CustomEntity392Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity392(id={})", value.id()));
        crate::CustomEntity392Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_393<'a>(value: &'a crate::CustomEntity393) -> crate::CustomEntity393Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity393(id={})", value.id()));
        crate::CustomEntity393Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_394<'a>(value: &'a crate::CustomEntity394) -> crate::CustomEntity394Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity394(id={})", value.id()));
        crate::CustomEntity394Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_395<'a>(value: &'a crate::CustomEntity395) -> crate::CustomEntity395Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity395(id={})", value.id()));
        crate::CustomEntity395Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_396<'a>(value: &'a crate::CustomEntity396) -> crate::CustomEntity396Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity396(id={})", value.id()));
        crate::CustomEntity396Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_397<'a>(value: &'a crate::CustomEntity397) -> crate::CustomEntity397Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity397(id={})", value.id()));
        crate::CustomEntity397Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_398<'a>(value: &'a crate::CustomEntity398) -> crate::CustomEntity398Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity398(id={})", value.id()));
        crate::CustomEntity398Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_399<'a>(value: &'a crate::CustomEntity399) -> crate::CustomEntity399Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity399(id={})", value.id()));
        crate::CustomEntity399Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_400<'a>(value: &'a crate::CustomEntity400) -> crate::CustomEntity400Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity400(id={})", value.id()));
        crate::CustomEntity400Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_401<'a>(value: &'a crate::CustomEntity401) -> crate::CustomEntity401Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity401(id={})", value.id()));
        crate::CustomEntity401Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_402<'a>(value: &'a crate::CustomEntity402) -> crate::CustomEntity402Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity402(id={})", value.id()));
        crate::CustomEntity402Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_403<'a>(value: &'a crate::CustomEntity403) -> crate::CustomEntity403Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity403(id={})", value.id()));
        crate::CustomEntity403Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_404<'a>(value: &'a crate::CustomEntity404) -> crate::CustomEntity404Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity404(id={})", value.id()));
        crate::CustomEntity404Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_405<'a>(value: &'a crate::CustomEntity405) -> crate::CustomEntity405Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity405(id={})", value.id()));
        crate::CustomEntity405Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_406<'a>(value: &'a crate::CustomEntity406) -> crate::CustomEntity406Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity406(id={})", value.id()));
        crate::CustomEntity406Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_407<'a>(value: &'a crate::CustomEntity407) -> crate::CustomEntity407Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity407(id={})", value.id()));
        crate::CustomEntity407Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_408<'a>(value: &'a crate::CustomEntity408) -> crate::CustomEntity408Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity408(id={})", value.id()));
        crate::CustomEntity408Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_409<'a>(value: &'a crate::CustomEntity409) -> crate::CustomEntity409Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity409(id={})", value.id()));
        crate::CustomEntity409Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_410<'a>(value: &'a crate::CustomEntity410) -> crate::CustomEntity410Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity410(id={})", value.id()));
        crate::CustomEntity410Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_411<'a>(value: &'a crate::CustomEntity411) -> crate::CustomEntity411Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity411(id={})", value.id()));
        crate::CustomEntity411Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_412<'a>(value: &'a crate::CustomEntity412) -> crate::CustomEntity412Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity412(id={})", value.id()));
        crate::CustomEntity412Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_413<'a>(value: &'a crate::CustomEntity413) -> crate::CustomEntity413Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity413(id={})", value.id()));
        crate::CustomEntity413Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_414<'a>(value: &'a crate::CustomEntity414) -> crate::CustomEntity414Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity414(id={})", value.id()));
        crate::CustomEntity414Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_415<'a>(value: &'a crate::CustomEntity415) -> crate::CustomEntity415Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity415(id={})", value.id()));
        crate::CustomEntity415Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_416<'a>(value: &'a crate::CustomEntity416) -> crate::CustomEntity416Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity416(id={})", value.id()));
        crate::CustomEntity416Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_417<'a>(value: &'a crate::CustomEntity417) -> crate::CustomEntity417Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity417(id={})", value.id()));
        crate::CustomEntity417Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_418<'a>(value: &'a crate::CustomEntity418) -> crate::CustomEntity418Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity418(id={})", value.id()));
        crate::CustomEntity418Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_419<'a>(value: &'a crate::CustomEntity419) -> crate::CustomEntity419Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity419(id={})", value.id()));
        crate::CustomEntity419Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_420<'a>(value: &'a crate::CustomEntity420) -> crate::CustomEntity420Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity420(id={})", value.id()));
        crate::CustomEntity420Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_421<'a>(value: &'a crate::CustomEntity421) -> crate::CustomEntity421Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity421(id={})", value.id()));
        crate::CustomEntity421Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_422<'a>(value: &'a crate::CustomEntity422) -> crate::CustomEntity422Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity422(id={})", value.id()));
        crate::CustomEntity422Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_423<'a>(value: &'a crate::CustomEntity423) -> crate::CustomEntity423Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity423(id={})", value.id()));
        crate::CustomEntity423Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_424<'a>(value: &'a crate::CustomEntity424) -> crate::CustomEntity424Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity424(id={})", value.id()));
        crate::CustomEntity424Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_425<'a>(value: &'a crate::CustomEntity425) -> crate::CustomEntity425Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity425(id={})", value.id()));
        crate::CustomEntity425Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_426<'a>(value: &'a crate::CustomEntity426) -> crate::CustomEntity426Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity426(id={})", value.id()));
        crate::CustomEntity426Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_427<'a>(value: &'a crate::CustomEntity427) -> crate::CustomEntity427Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity427(id={})", value.id()));
        crate::CustomEntity427Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_428<'a>(value: &'a crate::CustomEntity428) -> crate::CustomEntity428Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity428(id={})", value.id()));
        crate::CustomEntity428Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_429<'a>(value: &'a crate::CustomEntity429) -> crate::CustomEntity429Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity429(id={})", value.id()));
        crate::CustomEntity429Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_430<'a>(value: &'a crate::CustomEntity430) -> crate::CustomEntity430Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity430(id={})", value.id()));
        crate::CustomEntity430Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_431<'a>(value: &'a crate::CustomEntity431) -> crate::CustomEntity431Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity431(id={})", value.id()));
        crate::CustomEntity431Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_432<'a>(value: &'a crate::CustomEntity432) -> crate::CustomEntity432Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity432(id={})", value.id()));
        crate::CustomEntity432Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_433<'a>(value: &'a crate::CustomEntity433) -> crate::CustomEntity433Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity433(id={})", value.id()));
        crate::CustomEntity433Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_434<'a>(value: &'a crate::CustomEntity434) -> crate::CustomEntity434Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity434(id={})", value.id()));
        crate::CustomEntity434Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_435<'a>(value: &'a crate::CustomEntity435) -> crate::CustomEntity435Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity435(id={})", value.id()));
        crate::CustomEntity435Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_436<'a>(value: &'a crate::CustomEntity436) -> crate::CustomEntity436Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity436(id={})", value.id()));
        crate::CustomEntity436Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_437<'a>(value: &'a crate::CustomEntity437) -> crate::CustomEntity437Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity437(id={})", value.id()));
        crate::CustomEntity437Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_438<'a>(value: &'a crate::CustomEntity438) -> crate::CustomEntity438Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity438(id={})", value.id()));
        crate::CustomEntity438Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_439<'a>(value: &'a crate::CustomEntity439) -> crate::CustomEntity439Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity439(id={})", value.id()));
        crate::CustomEntity439Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_440<'a>(value: &'a crate::CustomEntity440) -> crate::CustomEntity440Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity440(id={})", value.id()));
        crate::CustomEntity440Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_441<'a>(value: &'a crate::CustomEntity441) -> crate::CustomEntity441Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity441(id={})", value.id()));
        crate::CustomEntity441Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_442<'a>(value: &'a crate::CustomEntity442) -> crate::CustomEntity442Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity442(id={})", value.id()));
        crate::CustomEntity442Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_443<'a>(value: &'a crate::CustomEntity443) -> crate::CustomEntity443Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity443(id={})", value.id()));
        crate::CustomEntity443Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_444<'a>(value: &'a crate::CustomEntity444) -> crate::CustomEntity444Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity444(id={})", value.id()));
        crate::CustomEntity444Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_445<'a>(value: &'a crate::CustomEntity445) -> crate::CustomEntity445Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity445(id={})", value.id()));
        crate::CustomEntity445Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_446<'a>(value: &'a crate::CustomEntity446) -> crate::CustomEntity446Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity446(id={})", value.id()));
        crate::CustomEntity446Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_447<'a>(value: &'a crate::CustomEntity447) -> crate::CustomEntity447Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity447(id={})", value.id()));
        crate::CustomEntity447Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_448<'a>(value: &'a crate::CustomEntity448) -> crate::CustomEntity448Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity448(id={})", value.id()));
        crate::CustomEntity448Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_449<'a>(value: &'a crate::CustomEntity449) -> crate::CustomEntity449Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity449(id={})", value.id()));
        crate::CustomEntity449Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_450<'a>(value: &'a crate::CustomEntity450) -> crate::CustomEntity450Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity450(id={})", value.id()));
        crate::CustomEntity450Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_451<'a>(value: &'a crate::CustomEntity451) -> crate::CustomEntity451Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity451(id={})", value.id()));
        crate::CustomEntity451Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_452<'a>(value: &'a crate::CustomEntity452) -> crate::CustomEntity452Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity452(id={})", value.id()));
        crate::CustomEntity452Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_453<'a>(value: &'a crate::CustomEntity453) -> crate::CustomEntity453Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity453(id={})", value.id()));
        crate::CustomEntity453Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_454<'a>(value: &'a crate::CustomEntity454) -> crate::CustomEntity454Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity454(id={})", value.id()));
        crate::CustomEntity454Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_455<'a>(value: &'a crate::CustomEntity455) -> crate::CustomEntity455Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity455(id={})", value.id()));
        crate::CustomEntity455Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_456<'a>(value: &'a crate::CustomEntity456) -> crate::CustomEntity456Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity456(id={})", value.id()));
        crate::CustomEntity456Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_457<'a>(value: &'a crate::CustomEntity457) -> crate::CustomEntity457Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity457(id={})", value.id()));
        crate::CustomEntity457Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_458<'a>(value: &'a crate::CustomEntity458) -> crate::CustomEntity458Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity458(id={})", value.id()));
        crate::CustomEntity458Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_459<'a>(value: &'a crate::CustomEntity459) -> crate::CustomEntity459Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity459(id={})", value.id()));
        crate::CustomEntity459Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_460<'a>(value: &'a crate::CustomEntity460) -> crate::CustomEntity460Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity460(id={})", value.id()));
        crate::CustomEntity460Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_461<'a>(value: &'a crate::CustomEntity461) -> crate::CustomEntity461Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity461(id={})", value.id()));
        crate::CustomEntity461Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_462<'a>(value: &'a crate::CustomEntity462) -> crate::CustomEntity462Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity462(id={})", value.id()));
        crate::CustomEntity462Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_463<'a>(value: &'a crate::CustomEntity463) -> crate::CustomEntity463Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity463(id={})", value.id()));
        crate::CustomEntity463Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_464<'a>(value: &'a crate::CustomEntity464) -> crate::CustomEntity464Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity464(id={})", value.id()));
        crate::CustomEntity464Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_465<'a>(value: &'a crate::CustomEntity465) -> crate::CustomEntity465Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity465(id={})", value.id()));
        crate::CustomEntity465Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_466<'a>(value: &'a crate::CustomEntity466) -> crate::CustomEntity466Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity466(id={})", value.id()));
        crate::CustomEntity466Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_467<'a>(value: &'a crate::CustomEntity467) -> crate::CustomEntity467Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity467(id={})", value.id()));
        crate::CustomEntity467Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_468<'a>(value: &'a crate::CustomEntity468) -> crate::CustomEntity468Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity468(id={})", value.id()));
        crate::CustomEntity468Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_469<'a>(value: &'a crate::CustomEntity469) -> crate::CustomEntity469Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity469(id={})", value.id()));
        crate::CustomEntity469Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_470<'a>(value: &'a crate::CustomEntity470) -> crate::CustomEntity470Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity470(id={})", value.id()));
        crate::CustomEntity470Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_471<'a>(value: &'a crate::CustomEntity471) -> crate::CustomEntity471Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity471(id={})", value.id()));
        crate::CustomEntity471Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_472<'a>(value: &'a crate::CustomEntity472) -> crate::CustomEntity472Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity472(id={})", value.id()));
        crate::CustomEntity472Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_473<'a>(value: &'a crate::CustomEntity473) -> crate::CustomEntity473Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity473(id={})", value.id()));
        crate::CustomEntity473Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_474<'a>(value: &'a crate::CustomEntity474) -> crate::CustomEntity474Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity474(id={})", value.id()));
        crate::CustomEntity474Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_475<'a>(value: &'a crate::CustomEntity475) -> crate::CustomEntity475Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity475(id={})", value.id()));
        crate::CustomEntity475Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_476<'a>(value: &'a crate::CustomEntity476) -> crate::CustomEntity476Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity476(id={})", value.id()));
        crate::CustomEntity476Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_477<'a>(value: &'a crate::CustomEntity477) -> crate::CustomEntity477Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity477(id={})", value.id()));
        crate::CustomEntity477Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_478<'a>(value: &'a crate::CustomEntity478) -> crate::CustomEntity478Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity478(id={})", value.id()));
        crate::CustomEntity478Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_479<'a>(value: &'a crate::CustomEntity479) -> crate::CustomEntity479Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity479(id={})", value.id()));
        crate::CustomEntity479Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_480<'a>(value: &'a crate::CustomEntity480) -> crate::CustomEntity480Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity480(id={})", value.id()));
        crate::CustomEntity480Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_481<'a>(value: &'a crate::CustomEntity481) -> crate::CustomEntity481Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity481(id={})", value.id()));
        crate::CustomEntity481Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_482<'a>(value: &'a crate::CustomEntity482) -> crate::CustomEntity482Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity482(id={})", value.id()));
        crate::CustomEntity482Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_483<'a>(value: &'a crate::CustomEntity483) -> crate::CustomEntity483Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity483(id={})", value.id()));
        crate::CustomEntity483Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_484<'a>(value: &'a crate::CustomEntity484) -> crate::CustomEntity484Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity484(id={})", value.id()));
        crate::CustomEntity484Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_485<'a>(value: &'a crate::CustomEntity485) -> crate::CustomEntity485Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity485(id={})", value.id()));
        crate::CustomEntity485Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_486<'a>(value: &'a crate::CustomEntity486) -> crate::CustomEntity486Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity486(id={})", value.id()));
        crate::CustomEntity486Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_487<'a>(value: &'a crate::CustomEntity487) -> crate::CustomEntity487Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity487(id={})", value.id()));
        crate::CustomEntity487Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_488<'a>(value: &'a crate::CustomEntity488) -> crate::CustomEntity488Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity488(id={})", value.id()));
        crate::CustomEntity488Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_489<'a>(value: &'a crate::CustomEntity489) -> crate::CustomEntity489Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity489(id={})", value.id()));
        crate::CustomEntity489Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_490<'a>(value: &'a crate::CustomEntity490) -> crate::CustomEntity490Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity490(id={})", value.id()));
        crate::CustomEntity490Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_491<'a>(value: &'a crate::CustomEntity491) -> crate::CustomEntity491Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity491(id={})", value.id()));
        crate::CustomEntity491Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_492<'a>(value: &'a crate::CustomEntity492) -> crate::CustomEntity492Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity492(id={})", value.id()));
        crate::CustomEntity492Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_493<'a>(value: &'a crate::CustomEntity493) -> crate::CustomEntity493Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity493(id={})", value.id()));
        crate::CustomEntity493Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_494<'a>(value: &'a crate::CustomEntity494) -> crate::CustomEntity494Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity494(id={})", value.id()));
        crate::CustomEntity494Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_495<'a>(value: &'a crate::CustomEntity495) -> crate::CustomEntity495Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity495(id={})", value.id()));
        crate::CustomEntity495Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_496<'a>(value: &'a crate::CustomEntity496) -> crate::CustomEntity496Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity496(id={})", value.id()));
        crate::CustomEntity496Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_497<'a>(value: &'a crate::CustomEntity497) -> crate::CustomEntity497Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity497(id={})", value.id()));
        crate::CustomEntity497Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_498<'a>(value: &'a crate::CustomEntity498) -> crate::CustomEntity498Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity498(id={})", value.id()));
        crate::CustomEntity498Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_499<'a>(value: &'a crate::CustomEntity499) -> crate::CustomEntity499Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity499(id={})", value.id()));
        crate::CustomEntity499Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_500<'a>(value: &'a crate::CustomEntity500) -> crate::CustomEntity500Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity500(id={})", value.id()));
        crate::CustomEntity500Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_501<'a>(value: &'a crate::CustomEntity501) -> crate::CustomEntity501Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity501(id={})", value.id()));
        crate::CustomEntity501Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_502<'a>(value: &'a crate::CustomEntity502) -> crate::CustomEntity502Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity502(id={})", value.id()));
        crate::CustomEntity502Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_503<'a>(value: &'a crate::CustomEntity503) -> crate::CustomEntity503Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity503(id={})", value.id()));
        crate::CustomEntity503Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_504<'a>(value: &'a crate::CustomEntity504) -> crate::CustomEntity504Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity504(id={})", value.id()));
        crate::CustomEntity504Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_505<'a>(value: &'a crate::CustomEntity505) -> crate::CustomEntity505Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity505(id={})", value.id()));
        crate::CustomEntity505Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_506<'a>(value: &'a crate::CustomEntity506) -> crate::CustomEntity506Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity506(id={})", value.id()));
        crate::CustomEntity506Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_507<'a>(value: &'a crate::CustomEntity507) -> crate::CustomEntity507Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity507(id={})", value.id()));
        crate::CustomEntity507Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_508<'a>(value: &'a crate::CustomEntity508) -> crate::CustomEntity508Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity508(id={})", value.id()));
        crate::CustomEntity508Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_509<'a>(value: &'a crate::CustomEntity509) -> crate::CustomEntity509Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity509(id={})", value.id()));
        crate::CustomEntity509Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_510<'a>(value: &'a crate::CustomEntity510) -> crate::CustomEntity510Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity510(id={})", value.id()));
        crate::CustomEntity510Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_511<'a>(value: &'a crate::CustomEntity511) -> crate::CustomEntity511Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity511(id={})", value.id()));
        crate::CustomEntity511Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_512<'a>(value: &'a crate::CustomEntity512) -> crate::CustomEntity512Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity512(id={})", value.id()));
        crate::CustomEntity512Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_513<'a>(value: &'a crate::CustomEntity513) -> crate::CustomEntity513Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity513(id={})", value.id()));
        crate::CustomEntity513Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_514<'a>(value: &'a crate::CustomEntity514) -> crate::CustomEntity514Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity514(id={})", value.id()));
        crate::CustomEntity514Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_515<'a>(value: &'a crate::CustomEntity515) -> crate::CustomEntity515Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity515(id={})", value.id()));
        crate::CustomEntity515Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_516<'a>(value: &'a crate::CustomEntity516) -> crate::CustomEntity516Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity516(id={})", value.id()));
        crate::CustomEntity516Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_517<'a>(value: &'a crate::CustomEntity517) -> crate::CustomEntity517Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity517(id={})", value.id()));
        crate::CustomEntity517Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_518<'a>(value: &'a crate::CustomEntity518) -> crate::CustomEntity518Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity518(id={})", value.id()));
        crate::CustomEntity518Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_519<'a>(value: &'a crate::CustomEntity519) -> crate::CustomEntity519Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity519(id={})", value.id()));
        crate::CustomEntity519Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_520<'a>(value: &'a crate::CustomEntity520) -> crate::CustomEntity520Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity520(id={})", value.id()));
        crate::CustomEntity520Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_521<'a>(value: &'a crate::CustomEntity521) -> crate::CustomEntity521Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity521(id={})", value.id()));
        crate::CustomEntity521Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_522<'a>(value: &'a crate::CustomEntity522) -> crate::CustomEntity522Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity522(id={})", value.id()));
        crate::CustomEntity522Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_523<'a>(value: &'a crate::CustomEntity523) -> crate::CustomEntity523Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity523(id={})", value.id()));
        crate::CustomEntity523Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_524<'a>(value: &'a crate::CustomEntity524) -> crate::CustomEntity524Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity524(id={})", value.id()));
        crate::CustomEntity524Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_525<'a>(value: &'a crate::CustomEntity525) -> crate::CustomEntity525Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity525(id={})", value.id()));
        crate::CustomEntity525Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_526<'a>(value: &'a crate::CustomEntity526) -> crate::CustomEntity526Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity526(id={})", value.id()));
        crate::CustomEntity526Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_527<'a>(value: &'a crate::CustomEntity527) -> crate::CustomEntity527Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity527(id={})", value.id()));
        crate::CustomEntity527Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_528<'a>(value: &'a crate::CustomEntity528) -> crate::CustomEntity528Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity528(id={})", value.id()));
        crate::CustomEntity528Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_529<'a>(value: &'a crate::CustomEntity529) -> crate::CustomEntity529Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity529(id={})", value.id()));
        crate::CustomEntity529Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_530<'a>(value: &'a crate::CustomEntity530) -> crate::CustomEntity530Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity530(id={})", value.id()));
        crate::CustomEntity530Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_531<'a>(value: &'a crate::CustomEntity531) -> crate::CustomEntity531Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity531(id={})", value.id()));
        crate::CustomEntity531Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_532<'a>(value: &'a crate::CustomEntity532) -> crate::CustomEntity532Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity532(id={})", value.id()));
        crate::CustomEntity532Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_533<'a>(value: &'a crate::CustomEntity533) -> crate::CustomEntity533Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity533(id={})", value.id()));
        crate::CustomEntity533Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_534<'a>(value: &'a crate::CustomEntity534) -> crate::CustomEntity534Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity534(id={})", value.id()));
        crate::CustomEntity534Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_535<'a>(value: &'a crate::CustomEntity535) -> crate::CustomEntity535Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity535(id={})", value.id()));
        crate::CustomEntity535Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_536<'a>(value: &'a crate::CustomEntity536) -> crate::CustomEntity536Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity536(id={})", value.id()));
        crate::CustomEntity536Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_537<'a>(value: &'a crate::CustomEntity537) -> crate::CustomEntity537Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity537(id={})", value.id()));
        crate::CustomEntity537Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_538<'a>(value: &'a crate::CustomEntity538) -> crate::CustomEntity538Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity538(id={})", value.id()));
        crate::CustomEntity538Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_539<'a>(value: &'a crate::CustomEntity539) -> crate::CustomEntity539Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity539(id={})", value.id()));
        crate::CustomEntity539Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_540<'a>(value: &'a crate::CustomEntity540) -> crate::CustomEntity540Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity540(id={})", value.id()));
        crate::CustomEntity540Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_541<'a>(value: &'a crate::CustomEntity541) -> crate::CustomEntity541Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity541(id={})", value.id()));
        crate::CustomEntity541Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_542<'a>(value: &'a crate::CustomEntity542) -> crate::CustomEntity542Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity542(id={})", value.id()));
        crate::CustomEntity542Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_543<'a>(value: &'a crate::CustomEntity543) -> crate::CustomEntity543Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity543(id={})", value.id()));
        crate::CustomEntity543Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_544<'a>(value: &'a crate::CustomEntity544) -> crate::CustomEntity544Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity544(id={})", value.id()));
        crate::CustomEntity544Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_545<'a>(value: &'a crate::CustomEntity545) -> crate::CustomEntity545Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity545(id={})", value.id()));
        crate::CustomEntity545Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_546<'a>(value: &'a crate::CustomEntity546) -> crate::CustomEntity546Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity546(id={})", value.id()));
        crate::CustomEntity546Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_547<'a>(value: &'a crate::CustomEntity547) -> crate::CustomEntity547Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity547(id={})", value.id()));
        crate::CustomEntity547Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_548<'a>(value: &'a crate::CustomEntity548) -> crate::CustomEntity548Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity548(id={})", value.id()));
        crate::CustomEntity548Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_549<'a>(value: &'a crate::CustomEntity549) -> crate::CustomEntity549Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity549(id={})", value.id()));
        crate::CustomEntity549Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_550<'a>(value: &'a crate::CustomEntity550) -> crate::CustomEntity550Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity550(id={})", value.id()));
        crate::CustomEntity550Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_551<'a>(value: &'a crate::CustomEntity551) -> crate::CustomEntity551Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity551(id={})", value.id()));
        crate::CustomEntity551Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_552<'a>(value: &'a crate::CustomEntity552) -> crate::CustomEntity552Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity552(id={})", value.id()));
        crate::CustomEntity552Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_553<'a>(value: &'a crate::CustomEntity553) -> crate::CustomEntity553Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity553(id={})", value.id()));
        crate::CustomEntity553Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_554<'a>(value: &'a crate::CustomEntity554) -> crate::CustomEntity554Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity554(id={})", value.id()));
        crate::CustomEntity554Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_555<'a>(value: &'a crate::CustomEntity555) -> crate::CustomEntity555Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity555(id={})", value.id()));
        crate::CustomEntity555Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_556<'a>(value: &'a crate::CustomEntity556) -> crate::CustomEntity556Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity556(id={})", value.id()));
        crate::CustomEntity556Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_557<'a>(value: &'a crate::CustomEntity557) -> crate::CustomEntity557Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity557(id={})", value.id()));
        crate::CustomEntity557Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_558<'a>(value: &'a crate::CustomEntity558) -> crate::CustomEntity558Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity558(id={})", value.id()));
        crate::CustomEntity558Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_559<'a>(value: &'a crate::CustomEntity559) -> crate::CustomEntity559Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity559(id={})", value.id()));
        crate::CustomEntity559Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_560<'a>(value: &'a crate::CustomEntity560) -> crate::CustomEntity560Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity560(id={})", value.id()));
        crate::CustomEntity560Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_561<'a>(value: &'a crate::CustomEntity561) -> crate::CustomEntity561Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity561(id={})", value.id()));
        crate::CustomEntity561Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_562<'a>(value: &'a crate::CustomEntity562) -> crate::CustomEntity562Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity562(id={})", value.id()));
        crate::CustomEntity562Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_563<'a>(value: &'a crate::CustomEntity563) -> crate::CustomEntity563Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity563(id={})", value.id()));
        crate::CustomEntity563Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_564<'a>(value: &'a crate::CustomEntity564) -> crate::CustomEntity564Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity564(id={})", value.id()));
        crate::CustomEntity564Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_565<'a>(value: &'a crate::CustomEntity565) -> crate::CustomEntity565Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity565(id={})", value.id()));
        crate::CustomEntity565Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_566<'a>(value: &'a crate::CustomEntity566) -> crate::CustomEntity566Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity566(id={})", value.id()));
        crate::CustomEntity566Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_567<'a>(value: &'a crate::CustomEntity567) -> crate::CustomEntity567Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity567(id={})", value.id()));
        crate::CustomEntity567Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_568<'a>(value: &'a crate::CustomEntity568) -> crate::CustomEntity568Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity568(id={})", value.id()));
        crate::CustomEntity568Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_569<'a>(value: &'a crate::CustomEntity569) -> crate::CustomEntity569Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity569(id={})", value.id()));
        crate::CustomEntity569Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_570<'a>(value: &'a crate::CustomEntity570) -> crate::CustomEntity570Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity570(id={})", value.id()));
        crate::CustomEntity570Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_571<'a>(value: &'a crate::CustomEntity571) -> crate::CustomEntity571Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity571(id={})", value.id()));
        crate::CustomEntity571Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_572<'a>(value: &'a crate::CustomEntity572) -> crate::CustomEntity572Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity572(id={})", value.id()));
        crate::CustomEntity572Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_573<'a>(value: &'a crate::CustomEntity573) -> crate::CustomEntity573Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity573(id={})", value.id()));
        crate::CustomEntity573Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_574<'a>(value: &'a crate::CustomEntity574) -> crate::CustomEntity574Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity574(id={})", value.id()));
        crate::CustomEntity574Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_575<'a>(value: &'a crate::CustomEntity575) -> crate::CustomEntity575Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity575(id={})", value.id()));
        crate::CustomEntity575Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_576<'a>(value: &'a crate::CustomEntity576) -> crate::CustomEntity576Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity576(id={})", value.id()));
        crate::CustomEntity576Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_577<'a>(value: &'a crate::CustomEntity577) -> crate::CustomEntity577Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity577(id={})", value.id()));
        crate::CustomEntity577Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_578<'a>(value: &'a crate::CustomEntity578) -> crate::CustomEntity578Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity578(id={})", value.id()));
        crate::CustomEntity578Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_579<'a>(value: &'a crate::CustomEntity579) -> crate::CustomEntity579Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity579(id={})", value.id()));
        crate::CustomEntity579Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_580<'a>(value: &'a crate::CustomEntity580) -> crate::CustomEntity580Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity580(id={})", value.id()));
        crate::CustomEntity580Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_581<'a>(value: &'a crate::CustomEntity581) -> crate::CustomEntity581Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity581(id={})", value.id()));
        crate::CustomEntity581Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_582<'a>(value: &'a crate::CustomEntity582) -> crate::CustomEntity582Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity582(id={})", value.id()));
        crate::CustomEntity582Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_583<'a>(value: &'a crate::CustomEntity583) -> crate::CustomEntity583Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity583(id={})", value.id()));
        crate::CustomEntity583Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_584<'a>(value: &'a crate::CustomEntity584) -> crate::CustomEntity584Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity584(id={})", value.id()));
        crate::CustomEntity584Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_585<'a>(value: &'a crate::CustomEntity585) -> crate::CustomEntity585Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity585(id={})", value.id()));
        crate::CustomEntity585Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_586<'a>(value: &'a crate::CustomEntity586) -> crate::CustomEntity586Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity586(id={})", value.id()));
        crate::CustomEntity586Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_587<'a>(value: &'a crate::CustomEntity587) -> crate::CustomEntity587Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity587(id={})", value.id()));
        crate::CustomEntity587Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_588<'a>(value: &'a crate::CustomEntity588) -> crate::CustomEntity588Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity588(id={})", value.id()));
        crate::CustomEntity588Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_589<'a>(value: &'a crate::CustomEntity589) -> crate::CustomEntity589Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity589(id={})", value.id()));
        crate::CustomEntity589Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_590<'a>(value: &'a crate::CustomEntity590) -> crate::CustomEntity590Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity590(id={})", value.id()));
        crate::CustomEntity590Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_591<'a>(value: &'a crate::CustomEntity591) -> crate::CustomEntity591Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity591(id={})", value.id()));
        crate::CustomEntity591Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_592<'a>(value: &'a crate::CustomEntity592) -> crate::CustomEntity592Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity592(id={})", value.id()));
        crate::CustomEntity592Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_593<'a>(value: &'a crate::CustomEntity593) -> crate::CustomEntity593Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity593(id={})", value.id()));
        crate::CustomEntity593Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_594<'a>(value: &'a crate::CustomEntity594) -> crate::CustomEntity594Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity594(id={})", value.id()));
        crate::CustomEntity594Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_595<'a>(value: &'a crate::CustomEntity595) -> crate::CustomEntity595Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity595(id={})", value.id()));
        crate::CustomEntity595Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_596<'a>(value: &'a crate::CustomEntity596) -> crate::CustomEntity596Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity596(id={})", value.id()));
        crate::CustomEntity596Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_597<'a>(value: &'a crate::CustomEntity597) -> crate::CustomEntity597Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity597(id={})", value.id()));
        crate::CustomEntity597Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_598<'a>(value: &'a crate::CustomEntity598) -> crate::CustomEntity598Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity598(id={})", value.id()));
        crate::CustomEntity598Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_599<'a>(value: &'a crate::CustomEntity599) -> crate::CustomEntity599Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity599(id={})", value.id()));
        crate::CustomEntity599Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_600<'a>(value: &'a crate::CustomEntity600) -> crate::CustomEntity600Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity600(id={})", value.id()));
        crate::CustomEntity600Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_601<'a>(value: &'a crate::CustomEntity601) -> crate::CustomEntity601Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity601(id={})", value.id()));
        crate::CustomEntity601Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_602<'a>(value: &'a crate::CustomEntity602) -> crate::CustomEntity602Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity602(id={})", value.id()));
        crate::CustomEntity602Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_603<'a>(value: &'a crate::CustomEntity603) -> crate::CustomEntity603Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity603(id={})", value.id()));
        crate::CustomEntity603Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_604<'a>(value: &'a crate::CustomEntity604) -> crate::CustomEntity604Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity604(id={})", value.id()));
        crate::CustomEntity604Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_605<'a>(value: &'a crate::CustomEntity605) -> crate::CustomEntity605Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity605(id={})", value.id()));
        crate::CustomEntity605Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_606<'a>(value: &'a crate::CustomEntity606) -> crate::CustomEntity606Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity606(id={})", value.id()));
        crate::CustomEntity606Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_607<'a>(value: &'a crate::CustomEntity607) -> crate::CustomEntity607Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity607(id={})", value.id()));
        crate::CustomEntity607Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_608<'a>(value: &'a crate::CustomEntity608) -> crate::CustomEntity608Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity608(id={})", value.id()));
        crate::CustomEntity608Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_609<'a>(value: &'a crate::CustomEntity609) -> crate::CustomEntity609Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity609(id={})", value.id()));
        crate::CustomEntity609Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_610<'a>(value: &'a crate::CustomEntity610) -> crate::CustomEntity610Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity610(id={})", value.id()));
        crate::CustomEntity610Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_611<'a>(value: &'a crate::CustomEntity611) -> crate::CustomEntity611Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity611(id={})", value.id()));
        crate::CustomEntity611Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_612<'a>(value: &'a crate::CustomEntity612) -> crate::CustomEntity612Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity612(id={})", value.id()));
        crate::CustomEntity612Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_613<'a>(value: &'a crate::CustomEntity613) -> crate::CustomEntity613Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity613(id={})", value.id()));
        crate::CustomEntity613Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_614<'a>(value: &'a crate::CustomEntity614) -> crate::CustomEntity614Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity614(id={})", value.id()));
        crate::CustomEntity614Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_615<'a>(value: &'a crate::CustomEntity615) -> crate::CustomEntity615Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity615(id={})", value.id()));
        crate::CustomEntity615Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_616<'a>(value: &'a crate::CustomEntity616) -> crate::CustomEntity616Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity616(id={})", value.id()));
        crate::CustomEntity616Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_617<'a>(value: &'a crate::CustomEntity617) -> crate::CustomEntity617Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity617(id={})", value.id()));
        crate::CustomEntity617Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_618<'a>(value: &'a crate::CustomEntity618) -> crate::CustomEntity618Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity618(id={})", value.id()));
        crate::CustomEntity618Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_619<'a>(value: &'a crate::CustomEntity619) -> crate::CustomEntity619Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity619(id={})", value.id()));
        crate::CustomEntity619Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_620<'a>(value: &'a crate::CustomEntity620) -> crate::CustomEntity620Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity620(id={})", value.id()));
        crate::CustomEntity620Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_621<'a>(value: &'a crate::CustomEntity621) -> crate::CustomEntity621Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity621(id={})", value.id()));
        crate::CustomEntity621Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_622<'a>(value: &'a crate::CustomEntity622) -> crate::CustomEntity622Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity622(id={})", value.id()));
        crate::CustomEntity622Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_623<'a>(value: &'a crate::CustomEntity623) -> crate::CustomEntity623Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity623(id={})", value.id()));
        crate::CustomEntity623Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_624<'a>(value: &'a crate::CustomEntity624) -> crate::CustomEntity624Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity624(id={})", value.id()));
        crate::CustomEntity624Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_625<'a>(value: &'a crate::CustomEntity625) -> crate::CustomEntity625Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity625(id={})", value.id()));
        crate::CustomEntity625Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_626<'a>(value: &'a crate::CustomEntity626) -> crate::CustomEntity626Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity626(id={})", value.id()));
        crate::CustomEntity626Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_627<'a>(value: &'a crate::CustomEntity627) -> crate::CustomEntity627Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity627(id={})", value.id()));
        crate::CustomEntity627Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_628<'a>(value: &'a crate::CustomEntity628) -> crate::CustomEntity628Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity628(id={})", value.id()));
        crate::CustomEntity628Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_629<'a>(value: &'a crate::CustomEntity629) -> crate::CustomEntity629Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity629(id={})", value.id()));
        crate::CustomEntity629Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_630<'a>(value: &'a crate::CustomEntity630) -> crate::CustomEntity630Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity630(id={})", value.id()));
        crate::CustomEntity630Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_631<'a>(value: &'a crate::CustomEntity631) -> crate::CustomEntity631Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity631(id={})", value.id()));
        crate::CustomEntity631Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_632<'a>(value: &'a crate::CustomEntity632) -> crate::CustomEntity632Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity632(id={})", value.id()));
        crate::CustomEntity632Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_633<'a>(value: &'a crate::CustomEntity633) -> crate::CustomEntity633Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity633(id={})", value.id()));
        crate::CustomEntity633Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_634<'a>(value: &'a crate::CustomEntity634) -> crate::CustomEntity634Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity634(id={})", value.id()));
        crate::CustomEntity634Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_635<'a>(value: &'a crate::CustomEntity635) -> crate::CustomEntity635Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity635(id={})", value.id()));
        crate::CustomEntity635Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_636<'a>(value: &'a crate::CustomEntity636) -> crate::CustomEntity636Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity636(id={})", value.id()));
        crate::CustomEntity636Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_637<'a>(value: &'a crate::CustomEntity637) -> crate::CustomEntity637Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity637(id={})", value.id()));
        crate::CustomEntity637Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_638<'a>(value: &'a crate::CustomEntity638) -> crate::CustomEntity638Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity638(id={})", value.id()));
        crate::CustomEntity638Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_639<'a>(value: &'a crate::CustomEntity639) -> crate::CustomEntity639Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity639(id={})", value.id()));
        crate::CustomEntity639Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_640<'a>(value: &'a crate::CustomEntity640) -> crate::CustomEntity640Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity640(id={})", value.id()));
        crate::CustomEntity640Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_641<'a>(value: &'a crate::CustomEntity641) -> crate::CustomEntity641Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity641(id={})", value.id()));
        crate::CustomEntity641Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_642<'a>(value: &'a crate::CustomEntity642) -> crate::CustomEntity642Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity642(id={})", value.id()));
        crate::CustomEntity642Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_643<'a>(value: &'a crate::CustomEntity643) -> crate::CustomEntity643Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity643(id={})", value.id()));
        crate::CustomEntity643Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_644<'a>(value: &'a crate::CustomEntity644) -> crate::CustomEntity644Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity644(id={})", value.id()));
        crate::CustomEntity644Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_645<'a>(value: &'a crate::CustomEntity645) -> crate::CustomEntity645Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity645(id={})", value.id()));
        crate::CustomEntity645Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_646<'a>(value: &'a crate::CustomEntity646) -> crate::CustomEntity646Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity646(id={})", value.id()));
        crate::CustomEntity646Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_647<'a>(value: &'a crate::CustomEntity647) -> crate::CustomEntity647Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity647(id={})", value.id()));
        crate::CustomEntity647Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_648<'a>(value: &'a crate::CustomEntity648) -> crate::CustomEntity648Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity648(id={})", value.id()));
        crate::CustomEntity648Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_649<'a>(value: &'a crate::CustomEntity649) -> crate::CustomEntity649Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity649(id={})", value.id()));
        crate::CustomEntity649Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_650<'a>(value: &'a crate::CustomEntity650) -> crate::CustomEntity650Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity650(id={})", value.id()));
        crate::CustomEntity650Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_651<'a>(value: &'a crate::CustomEntity651) -> crate::CustomEntity651Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity651(id={})", value.id()));
        crate::CustomEntity651Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_652<'a>(value: &'a crate::CustomEntity652) -> crate::CustomEntity652Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity652(id={})", value.id()));
        crate::CustomEntity652Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_653<'a>(value: &'a crate::CustomEntity653) -> crate::CustomEntity653Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity653(id={})", value.id()));
        crate::CustomEntity653Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_654<'a>(value: &'a crate::CustomEntity654) -> crate::CustomEntity654Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity654(id={})", value.id()));
        crate::CustomEntity654Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_655<'a>(value: &'a crate::CustomEntity655) -> crate::CustomEntity655Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity655(id={})", value.id()));
        crate::CustomEntity655Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_656<'a>(value: &'a crate::CustomEntity656) -> crate::CustomEntity656Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity656(id={})", value.id()));
        crate::CustomEntity656Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_657<'a>(value: &'a crate::CustomEntity657) -> crate::CustomEntity657Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity657(id={})", value.id()));
        crate::CustomEntity657Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_658<'a>(value: &'a crate::CustomEntity658) -> crate::CustomEntity658Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity658(id={})", value.id()));
        crate::CustomEntity658Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_659<'a>(value: &'a crate::CustomEntity659) -> crate::CustomEntity659Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity659(id={})", value.id()));
        crate::CustomEntity659Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_660<'a>(value: &'a crate::CustomEntity660) -> crate::CustomEntity660Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity660(id={})", value.id()));
        crate::CustomEntity660Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_661<'a>(value: &'a crate::CustomEntity661) -> crate::CustomEntity661Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity661(id={})", value.id()));
        crate::CustomEntity661Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_662<'a>(value: &'a crate::CustomEntity662) -> crate::CustomEntity662Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity662(id={})", value.id()));
        crate::CustomEntity662Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_663<'a>(value: &'a crate::CustomEntity663) -> crate::CustomEntity663Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity663(id={})", value.id()));
        crate::CustomEntity663Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_664<'a>(value: &'a crate::CustomEntity664) -> crate::CustomEntity664Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity664(id={})", value.id()));
        crate::CustomEntity664Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_665<'a>(value: &'a crate::CustomEntity665) -> crate::CustomEntity665Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity665(id={})", value.id()));
        crate::CustomEntity665Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_666<'a>(value: &'a crate::CustomEntity666) -> crate::CustomEntity666Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity666(id={})", value.id()));
        crate::CustomEntity666Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_667<'a>(value: &'a crate::CustomEntity667) -> crate::CustomEntity667Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity667(id={})", value.id()));
        crate::CustomEntity667Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_668<'a>(value: &'a crate::CustomEntity668) -> crate::CustomEntity668Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity668(id={})", value.id()));
        crate::CustomEntity668Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_669<'a>(value: &'a crate::CustomEntity669) -> crate::CustomEntity669Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity669(id={})", value.id()));
        crate::CustomEntity669Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_670<'a>(value: &'a crate::CustomEntity670) -> crate::CustomEntity670Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity670(id={})", value.id()));
        crate::CustomEntity670Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_671<'a>(value: &'a crate::CustomEntity671) -> crate::CustomEntity671Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity671(id={})", value.id()));
        crate::CustomEntity671Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_672<'a>(value: &'a crate::CustomEntity672) -> crate::CustomEntity672Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity672(id={})", value.id()));
        crate::CustomEntity672Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_673<'a>(value: &'a crate::CustomEntity673) -> crate::CustomEntity673Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity673(id={})", value.id()));
        crate::CustomEntity673Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_674<'a>(value: &'a crate::CustomEntity674) -> crate::CustomEntity674Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity674(id={})", value.id()));
        crate::CustomEntity674Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_675<'a>(value: &'a crate::CustomEntity675) -> crate::CustomEntity675Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity675(id={})", value.id()));
        crate::CustomEntity675Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_676<'a>(value: &'a crate::CustomEntity676) -> crate::CustomEntity676Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity676(id={})", value.id()));
        crate::CustomEntity676Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_677<'a>(value: &'a crate::CustomEntity677) -> crate::CustomEntity677Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity677(id={})", value.id()));
        crate::CustomEntity677Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_678<'a>(value: &'a crate::CustomEntity678) -> crate::CustomEntity678Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity678(id={})", value.id()));
        crate::CustomEntity678Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_679<'a>(value: &'a crate::CustomEntity679) -> crate::CustomEntity679Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity679(id={})", value.id()));
        crate::CustomEntity679Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_680<'a>(value: &'a crate::CustomEntity680) -> crate::CustomEntity680Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity680(id={})", value.id()));
        crate::CustomEntity680Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_681<'a>(value: &'a crate::CustomEntity681) -> crate::CustomEntity681Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity681(id={})", value.id()));
        crate::CustomEntity681Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_682<'a>(value: &'a crate::CustomEntity682) -> crate::CustomEntity682Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity682(id={})", value.id()));
        crate::CustomEntity682Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_683<'a>(value: &'a crate::CustomEntity683) -> crate::CustomEntity683Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity683(id={})", value.id()));
        crate::CustomEntity683Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_684<'a>(value: &'a crate::CustomEntity684) -> crate::CustomEntity684Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity684(id={})", value.id()));
        crate::CustomEntity684Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_685<'a>(value: &'a crate::CustomEntity685) -> crate::CustomEntity685Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity685(id={})", value.id()));
        crate::CustomEntity685Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_686<'a>(value: &'a crate::CustomEntity686) -> crate::CustomEntity686Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity686(id={})", value.id()));
        crate::CustomEntity686Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_687<'a>(value: &'a crate::CustomEntity687) -> crate::CustomEntity687Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity687(id={})", value.id()));
        crate::CustomEntity687Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_688<'a>(value: &'a crate::CustomEntity688) -> crate::CustomEntity688Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity688(id={})", value.id()));
        crate::CustomEntity688Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_689<'a>(value: &'a crate::CustomEntity689) -> crate::CustomEntity689Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity689(id={})", value.id()));
        crate::CustomEntity689Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_690<'a>(value: &'a crate::CustomEntity690) -> crate::CustomEntity690Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity690(id={})", value.id()));
        crate::CustomEntity690Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_691<'a>(value: &'a crate::CustomEntity691) -> crate::CustomEntity691Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity691(id={})", value.id()));
        crate::CustomEntity691Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_692<'a>(value: &'a crate::CustomEntity692) -> crate::CustomEntity692Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity692(id={})", value.id()));
        crate::CustomEntity692Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_693<'a>(value: &'a crate::CustomEntity693) -> crate::CustomEntity693Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity693(id={})", value.id()));
        crate::CustomEntity693Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_694<'a>(value: &'a crate::CustomEntity694) -> crate::CustomEntity694Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity694(id={})", value.id()));
        crate::CustomEntity694Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_695<'a>(value: &'a crate::CustomEntity695) -> crate::CustomEntity695Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity695(id={})", value.id()));
        crate::CustomEntity695Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_696<'a>(value: &'a crate::CustomEntity696) -> crate::CustomEntity696Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity696(id={})", value.id()));
        crate::CustomEntity696Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_697<'a>(value: &'a crate::CustomEntity697) -> crate::CustomEntity697Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity697(id={})", value.id()));
        crate::CustomEntity697Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_698<'a>(value: &'a crate::CustomEntity698) -> crate::CustomEntity698Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity698(id={})", value.id()));
        crate::CustomEntity698Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_699<'a>(value: &'a crate::CustomEntity699) -> crate::CustomEntity699Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity699(id={})", value.id()));
        crate::CustomEntity699Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_700<'a>(value: &'a crate::CustomEntity700) -> crate::CustomEntity700Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity700(id={})", value.id()));
        crate::CustomEntity700Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_701<'a>(value: &'a crate::CustomEntity701) -> crate::CustomEntity701Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity701(id={})", value.id()));
        crate::CustomEntity701Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_702<'a>(value: &'a crate::CustomEntity702) -> crate::CustomEntity702Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity702(id={})", value.id()));
        crate::CustomEntity702Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_703<'a>(value: &'a crate::CustomEntity703) -> crate::CustomEntity703Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity703(id={})", value.id()));
        crate::CustomEntity703Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_704<'a>(value: &'a crate::CustomEntity704) -> crate::CustomEntity704Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity704(id={})", value.id()));
        crate::CustomEntity704Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_705<'a>(value: &'a crate::CustomEntity705) -> crate::CustomEntity705Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity705(id={})", value.id()));
        crate::CustomEntity705Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_706<'a>(value: &'a crate::CustomEntity706) -> crate::CustomEntity706Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity706(id={})", value.id()));
        crate::CustomEntity706Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_707<'a>(value: &'a crate::CustomEntity707) -> crate::CustomEntity707Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity707(id={})", value.id()));
        crate::CustomEntity707Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_708<'a>(value: &'a crate::CustomEntity708) -> crate::CustomEntity708Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity708(id={})", value.id()));
        crate::CustomEntity708Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_709<'a>(value: &'a crate::CustomEntity709) -> crate::CustomEntity709Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity709(id={})", value.id()));
        crate::CustomEntity709Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_710<'a>(value: &'a crate::CustomEntity710) -> crate::CustomEntity710Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity710(id={})", value.id()));
        crate::CustomEntity710Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_711<'a>(value: &'a crate::CustomEntity711) -> crate::CustomEntity711Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity711(id={})", value.id()));
        crate::CustomEntity711Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_712<'a>(value: &'a crate::CustomEntity712) -> crate::CustomEntity712Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity712(id={})", value.id()));
        crate::CustomEntity712Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_713<'a>(value: &'a crate::CustomEntity713) -> crate::CustomEntity713Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity713(id={})", value.id()));
        crate::CustomEntity713Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_714<'a>(value: &'a crate::CustomEntity714) -> crate::CustomEntity714Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity714(id={})", value.id()));
        crate::CustomEntity714Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_715<'a>(value: &'a crate::CustomEntity715) -> crate::CustomEntity715Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity715(id={})", value.id()));
        crate::CustomEntity715Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_716<'a>(value: &'a crate::CustomEntity716) -> crate::CustomEntity716Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity716(id={})", value.id()));
        crate::CustomEntity716Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_717<'a>(value: &'a crate::CustomEntity717) -> crate::CustomEntity717Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity717(id={})", value.id()));
        crate::CustomEntity717Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_718<'a>(value: &'a crate::CustomEntity718) -> crate::CustomEntity718Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity718(id={})", value.id()));
        crate::CustomEntity718Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_719<'a>(value: &'a crate::CustomEntity719) -> crate::CustomEntity719Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity719(id={})", value.id()));
        crate::CustomEntity719Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_720<'a>(value: &'a crate::CustomEntity720) -> crate::CustomEntity720Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity720(id={})", value.id()));
        crate::CustomEntity720Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_721<'a>(value: &'a crate::CustomEntity721) -> crate::CustomEntity721Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity721(id={})", value.id()));
        crate::CustomEntity721Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_722<'a>(value: &'a crate::CustomEntity722) -> crate::CustomEntity722Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity722(id={})", value.id()));
        crate::CustomEntity722Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_723<'a>(value: &'a crate::CustomEntity723) -> crate::CustomEntity723Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity723(id={})", value.id()));
        crate::CustomEntity723Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_724<'a>(value: &'a crate::CustomEntity724) -> crate::CustomEntity724Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity724(id={})", value.id()));
        crate::CustomEntity724Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_725<'a>(value: &'a crate::CustomEntity725) -> crate::CustomEntity725Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity725(id={})", value.id()));
        crate::CustomEntity725Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_726<'a>(value: &'a crate::CustomEntity726) -> crate::CustomEntity726Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity726(id={})", value.id()));
        crate::CustomEntity726Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_727<'a>(value: &'a crate::CustomEntity727) -> crate::CustomEntity727Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity727(id={})", value.id()));
        crate::CustomEntity727Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_728<'a>(value: &'a crate::CustomEntity728) -> crate::CustomEntity728Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity728(id={})", value.id()));
        crate::CustomEntity728Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_729<'a>(value: &'a crate::CustomEntity729) -> crate::CustomEntity729Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity729(id={})", value.id()));
        crate::CustomEntity729Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_730<'a>(value: &'a crate::CustomEntity730) -> crate::CustomEntity730Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity730(id={})", value.id()));
        crate::CustomEntity730Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_731<'a>(value: &'a crate::CustomEntity731) -> crate::CustomEntity731Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity731(id={})", value.id()));
        crate::CustomEntity731Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_732<'a>(value: &'a crate::CustomEntity732) -> crate::CustomEntity732Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity732(id={})", value.id()));
        crate::CustomEntity732Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_733<'a>(value: &'a crate::CustomEntity733) -> crate::CustomEntity733Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity733(id={})", value.id()));
        crate::CustomEntity733Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_734<'a>(value: &'a crate::CustomEntity734) -> crate::CustomEntity734Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity734(id={})", value.id()));
        crate::CustomEntity734Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_735<'a>(value: &'a crate::CustomEntity735) -> crate::CustomEntity735Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity735(id={})", value.id()));
        crate::CustomEntity735Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_736<'a>(value: &'a crate::CustomEntity736) -> crate::CustomEntity736Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity736(id={})", value.id()));
        crate::CustomEntity736Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_737<'a>(value: &'a crate::CustomEntity737) -> crate::CustomEntity737Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity737(id={})", value.id()));
        crate::CustomEntity737Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_738<'a>(value: &'a crate::CustomEntity738) -> crate::CustomEntity738Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity738(id={})", value.id()));
        crate::CustomEntity738Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_739<'a>(value: &'a crate::CustomEntity739) -> crate::CustomEntity739Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity739(id={})", value.id()));
        crate::CustomEntity739Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_740<'a>(value: &'a crate::CustomEntity740) -> crate::CustomEntity740Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity740(id={})", value.id()));
        crate::CustomEntity740Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_741<'a>(value: &'a crate::CustomEntity741) -> crate::CustomEntity741Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity741(id={})", value.id()));
        crate::CustomEntity741Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_742<'a>(value: &'a crate::CustomEntity742) -> crate::CustomEntity742Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity742(id={})", value.id()));
        crate::CustomEntity742Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_743<'a>(value: &'a crate::CustomEntity743) -> crate::CustomEntity743Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity743(id={})", value.id()));
        crate::CustomEntity743Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_744<'a>(value: &'a crate::CustomEntity744) -> crate::CustomEntity744Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity744(id={})", value.id()));
        crate::CustomEntity744Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_745<'a>(value: &'a crate::CustomEntity745) -> crate::CustomEntity745Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity745(id={})", value.id()));
        crate::CustomEntity745Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_746<'a>(value: &'a crate::CustomEntity746) -> crate::CustomEntity746Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity746(id={})", value.id()));
        crate::CustomEntity746Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_747<'a>(value: &'a crate::CustomEntity747) -> crate::CustomEntity747Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity747(id={})", value.id()));
        crate::CustomEntity747Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_748<'a>(value: &'a crate::CustomEntity748) -> crate::CustomEntity748Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity748(id={})", value.id()));
        crate::CustomEntity748Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_749<'a>(value: &'a crate::CustomEntity749) -> crate::CustomEntity749Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity749(id={})", value.id()));
        crate::CustomEntity749Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_750<'a>(value: &'a crate::CustomEntity750) -> crate::CustomEntity750Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity750(id={})", value.id()));
        crate::CustomEntity750Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_751<'a>(value: &'a crate::CustomEntity751) -> crate::CustomEntity751Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity751(id={})", value.id()));
        crate::CustomEntity751Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_752<'a>(value: &'a crate::CustomEntity752) -> crate::CustomEntity752Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity752(id={})", value.id()));
        crate::CustomEntity752Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_753<'a>(value: &'a crate::CustomEntity753) -> crate::CustomEntity753Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity753(id={})", value.id()));
        crate::CustomEntity753Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_754<'a>(value: &'a crate::CustomEntity754) -> crate::CustomEntity754Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity754(id={})", value.id()));
        crate::CustomEntity754Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_755<'a>(value: &'a crate::CustomEntity755) -> crate::CustomEntity755Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity755(id={})", value.id()));
        crate::CustomEntity755Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_756<'a>(value: &'a crate::CustomEntity756) -> crate::CustomEntity756Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity756(id={})", value.id()));
        crate::CustomEntity756Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_757<'a>(value: &'a crate::CustomEntity757) -> crate::CustomEntity757Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity757(id={})", value.id()));
        crate::CustomEntity757Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_758<'a>(value: &'a crate::CustomEntity758) -> crate::CustomEntity758Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity758(id={})", value.id()));
        crate::CustomEntity758Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_759<'a>(value: &'a crate::CustomEntity759) -> crate::CustomEntity759Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity759(id={})", value.id()));
        crate::CustomEntity759Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_760<'a>(value: &'a crate::CustomEntity760) -> crate::CustomEntity760Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity760(id={})", value.id()));
        crate::CustomEntity760Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_761<'a>(value: &'a crate::CustomEntity761) -> crate::CustomEntity761Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity761(id={})", value.id()));
        crate::CustomEntity761Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_762<'a>(value: &'a crate::CustomEntity762) -> crate::CustomEntity762Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity762(id={})", value.id()));
        crate::CustomEntity762Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_763<'a>(value: &'a crate::CustomEntity763) -> crate::CustomEntity763Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity763(id={})", value.id()));
        crate::CustomEntity763Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_764<'a>(value: &'a crate::CustomEntity764) -> crate::CustomEntity764Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity764(id={})", value.id()));
        crate::CustomEntity764Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_765<'a>(value: &'a crate::CustomEntity765) -> crate::CustomEntity765Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity765(id={})", value.id()));
        crate::CustomEntity765Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_766<'a>(value: &'a crate::CustomEntity766) -> crate::CustomEntity766Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity766(id={})", value.id()));
        crate::CustomEntity766Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_767<'a>(value: &'a crate::CustomEntity767) -> crate::CustomEntity767Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity767(id={})", value.id()));
        crate::CustomEntity767Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_768<'a>(value: &'a crate::CustomEntity768) -> crate::CustomEntity768Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity768(id={})", value.id()));
        crate::CustomEntity768Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_769<'a>(value: &'a crate::CustomEntity769) -> crate::CustomEntity769Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity769(id={})", value.id()));
        crate::CustomEntity769Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_770<'a>(value: &'a crate::CustomEntity770) -> crate::CustomEntity770Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity770(id={})", value.id()));
        crate::CustomEntity770Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_771<'a>(value: &'a crate::CustomEntity771) -> crate::CustomEntity771Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity771(id={})", value.id()));
        crate::CustomEntity771Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_772<'a>(value: &'a crate::CustomEntity772) -> crate::CustomEntity772Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity772(id={})", value.id()));
        crate::CustomEntity772Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_773<'a>(value: &'a crate::CustomEntity773) -> crate::CustomEntity773Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity773(id={})", value.id()));
        crate::CustomEntity773Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_774<'a>(value: &'a crate::CustomEntity774) -> crate::CustomEntity774Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity774(id={})", value.id()));
        crate::CustomEntity774Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_775<'a>(value: &'a crate::CustomEntity775) -> crate::CustomEntity775Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity775(id={})", value.id()));
        crate::CustomEntity775Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_776<'a>(value: &'a crate::CustomEntity776) -> crate::CustomEntity776Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity776(id={})", value.id()));
        crate::CustomEntity776Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_777<'a>(value: &'a crate::CustomEntity777) -> crate::CustomEntity777Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity777(id={})", value.id()));
        crate::CustomEntity777Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_778<'a>(value: &'a crate::CustomEntity778) -> crate::CustomEntity778Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity778(id={})", value.id()));
        crate::CustomEntity778Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_779<'a>(value: &'a crate::CustomEntity779) -> crate::CustomEntity779Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity779(id={})", value.id()));
        crate::CustomEntity779Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_780<'a>(value: &'a crate::CustomEntity780) -> crate::CustomEntity780Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity780(id={})", value.id()));
        crate::CustomEntity780Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_781<'a>(value: &'a crate::CustomEntity781) -> crate::CustomEntity781Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity781(id={})", value.id()));
        crate::CustomEntity781Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_782<'a>(value: &'a crate::CustomEntity782) -> crate::CustomEntity782Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity782(id={})", value.id()));
        crate::CustomEntity782Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_783<'a>(value: &'a crate::CustomEntity783) -> crate::CustomEntity783Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity783(id={})", value.id()));
        crate::CustomEntity783Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_784<'a>(value: &'a crate::CustomEntity784) -> crate::CustomEntity784Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity784(id={})", value.id()));
        crate::CustomEntity784Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_785<'a>(value: &'a crate::CustomEntity785) -> crate::CustomEntity785Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity785(id={})", value.id()));
        crate::CustomEntity785Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_786<'a>(value: &'a crate::CustomEntity786) -> crate::CustomEntity786Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity786(id={})", value.id()));
        crate::CustomEntity786Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_787<'a>(value: &'a crate::CustomEntity787) -> crate::CustomEntity787Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity787(id={})", value.id()));
        crate::CustomEntity787Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_788<'a>(value: &'a crate::CustomEntity788) -> crate::CustomEntity788Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity788(id={})", value.id()));
        crate::CustomEntity788Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_789<'a>(value: &'a crate::CustomEntity789) -> crate::CustomEntity789Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity789(id={})", value.id()));
        crate::CustomEntity789Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_790<'a>(value: &'a crate::CustomEntity790) -> crate::CustomEntity790Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity790(id={})", value.id()));
        crate::CustomEntity790Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_791<'a>(value: &'a crate::CustomEntity791) -> crate::CustomEntity791Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity791(id={})", value.id()));
        crate::CustomEntity791Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_792<'a>(value: &'a crate::CustomEntity792) -> crate::CustomEntity792Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity792(id={})", value.id()));
        crate::CustomEntity792Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_793<'a>(value: &'a crate::CustomEntity793) -> crate::CustomEntity793Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity793(id={})", value.id()));
        crate::CustomEntity793Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_794<'a>(value: &'a crate::CustomEntity794) -> crate::CustomEntity794Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity794(id={})", value.id()));
        crate::CustomEntity794Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_795<'a>(value: &'a crate::CustomEntity795) -> crate::CustomEntity795Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity795(id={})", value.id()));
        crate::CustomEntity795Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_796<'a>(value: &'a crate::CustomEntity796) -> crate::CustomEntity796Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity796(id={})", value.id()));
        crate::CustomEntity796Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_797<'a>(value: &'a crate::CustomEntity797) -> crate::CustomEntity797Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity797(id={})", value.id()));
        crate::CustomEntity797Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_798<'a>(value: &'a crate::CustomEntity798) -> crate::CustomEntity798Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity798(id={})", value.id()));
        crate::CustomEntity798Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_799<'a>(value: &'a crate::CustomEntity799) -> crate::CustomEntity799Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity799(id={})", value.id()));
        crate::CustomEntity799Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_800<'a>(value: &'a crate::CustomEntity800) -> crate::CustomEntity800Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity800(id={})", value.id()));
        crate::CustomEntity800Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_801<'a>(value: &'a crate::CustomEntity801) -> crate::CustomEntity801Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity801(id={})", value.id()));
        crate::CustomEntity801Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_802<'a>(value: &'a crate::CustomEntity802) -> crate::CustomEntity802Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity802(id={})", value.id()));
        crate::CustomEntity802Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_803<'a>(value: &'a crate::CustomEntity803) -> crate::CustomEntity803Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity803(id={})", value.id()));
        crate::CustomEntity803Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_804<'a>(value: &'a crate::CustomEntity804) -> crate::CustomEntity804Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity804(id={})", value.id()));
        crate::CustomEntity804Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_805<'a>(value: &'a crate::CustomEntity805) -> crate::CustomEntity805Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity805(id={})", value.id()));
        crate::CustomEntity805Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_806<'a>(value: &'a crate::CustomEntity806) -> crate::CustomEntity806Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity806(id={})", value.id()));
        crate::CustomEntity806Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_807<'a>(value: &'a crate::CustomEntity807) -> crate::CustomEntity807Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity807(id={})", value.id()));
        crate::CustomEntity807Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_808<'a>(value: &'a crate::CustomEntity808) -> crate::CustomEntity808Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity808(id={})", value.id()));
        crate::CustomEntity808Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_809<'a>(value: &'a crate::CustomEntity809) -> crate::CustomEntity809Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity809(id={})", value.id()));
        crate::CustomEntity809Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_810<'a>(value: &'a crate::CustomEntity810) -> crate::CustomEntity810Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity810(id={})", value.id()));
        crate::CustomEntity810Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_811<'a>(value: &'a crate::CustomEntity811) -> crate::CustomEntity811Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity811(id={})", value.id()));
        crate::CustomEntity811Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_812<'a>(value: &'a crate::CustomEntity812) -> crate::CustomEntity812Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity812(id={})", value.id()));
        crate::CustomEntity812Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_813<'a>(value: &'a crate::CustomEntity813) -> crate::CustomEntity813Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity813(id={})", value.id()));
        crate::CustomEntity813Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_814<'a>(value: &'a crate::CustomEntity814) -> crate::CustomEntity814Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity814(id={})", value.id()));
        crate::CustomEntity814Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_815<'a>(value: &'a crate::CustomEntity815) -> crate::CustomEntity815Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity815(id={})", value.id()));
        crate::CustomEntity815Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_816<'a>(value: &'a crate::CustomEntity816) -> crate::CustomEntity816Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity816(id={})", value.id()));
        crate::CustomEntity816Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_817<'a>(value: &'a crate::CustomEntity817) -> crate::CustomEntity817Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity817(id={})", value.id()));
        crate::CustomEntity817Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_818<'a>(value: &'a crate::CustomEntity818) -> crate::CustomEntity818Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity818(id={})", value.id()));
        crate::CustomEntity818Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_819<'a>(value: &'a crate::CustomEntity819) -> crate::CustomEntity819Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity819(id={})", value.id()));
        crate::CustomEntity819Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_820<'a>(value: &'a crate::CustomEntity820) -> crate::CustomEntity820Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity820(id={})", value.id()));
        crate::CustomEntity820Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_821<'a>(value: &'a crate::CustomEntity821) -> crate::CustomEntity821Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity821(id={})", value.id()));
        crate::CustomEntity821Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_822<'a>(value: &'a crate::CustomEntity822) -> crate::CustomEntity822Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity822(id={})", value.id()));
        crate::CustomEntity822Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_823<'a>(value: &'a crate::CustomEntity823) -> crate::CustomEntity823Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity823(id={})", value.id()));
        crate::CustomEntity823Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_824<'a>(value: &'a crate::CustomEntity824) -> crate::CustomEntity824Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity824(id={})", value.id()));
        crate::CustomEntity824Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_825<'a>(value: &'a crate::CustomEntity825) -> crate::CustomEntity825Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity825(id={})", value.id()));
        crate::CustomEntity825Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_826<'a>(value: &'a crate::CustomEntity826) -> crate::CustomEntity826Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity826(id={})", value.id()));
        crate::CustomEntity826Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_827<'a>(value: &'a crate::CustomEntity827) -> crate::CustomEntity827Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity827(id={})", value.id()));
        crate::CustomEntity827Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_828<'a>(value: &'a crate::CustomEntity828) -> crate::CustomEntity828Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity828(id={})", value.id()));
        crate::CustomEntity828Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_829<'a>(value: &'a crate::CustomEntity829) -> crate::CustomEntity829Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity829(id={})", value.id()));
        crate::CustomEntity829Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_830<'a>(value: &'a crate::CustomEntity830) -> crate::CustomEntity830Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity830(id={})", value.id()));
        crate::CustomEntity830Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_831<'a>(value: &'a crate::CustomEntity831) -> crate::CustomEntity831Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity831(id={})", value.id()));
        crate::CustomEntity831Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_832<'a>(value: &'a crate::CustomEntity832) -> crate::CustomEntity832Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity832(id={})", value.id()));
        crate::CustomEntity832Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_833<'a>(value: &'a crate::CustomEntity833) -> crate::CustomEntity833Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity833(id={})", value.id()));
        crate::CustomEntity833Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_834<'a>(value: &'a crate::CustomEntity834) -> crate::CustomEntity834Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity834(id={})", value.id()));
        crate::CustomEntity834Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_835<'a>(value: &'a crate::CustomEntity835) -> crate::CustomEntity835Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity835(id={})", value.id()));
        crate::CustomEntity835Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_836<'a>(value: &'a crate::CustomEntity836) -> crate::CustomEntity836Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity836(id={})", value.id()));
        crate::CustomEntity836Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_837<'a>(value: &'a crate::CustomEntity837) -> crate::CustomEntity837Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity837(id={})", value.id()));
        crate::CustomEntity837Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_838<'a>(value: &'a crate::CustomEntity838) -> crate::CustomEntity838Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity838(id={})", value.id()));
        crate::CustomEntity838Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_839<'a>(value: &'a crate::CustomEntity839) -> crate::CustomEntity839Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity839(id={})", value.id()));
        crate::CustomEntity839Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_840<'a>(value: &'a crate::CustomEntity840) -> crate::CustomEntity840Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity840(id={})", value.id()));
        crate::CustomEntity840Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_841<'a>(value: &'a crate::CustomEntity841) -> crate::CustomEntity841Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity841(id={})", value.id()));
        crate::CustomEntity841Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_842<'a>(value: &'a crate::CustomEntity842) -> crate::CustomEntity842Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity842(id={})", value.id()));
        crate::CustomEntity842Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_843<'a>(value: &'a crate::CustomEntity843) -> crate::CustomEntity843Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity843(id={})", value.id()));
        crate::CustomEntity843Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_844<'a>(value: &'a crate::CustomEntity844) -> crate::CustomEntity844Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity844(id={})", value.id()));
        crate::CustomEntity844Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_845<'a>(value: &'a crate::CustomEntity845) -> crate::CustomEntity845Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity845(id={})", value.id()));
        crate::CustomEntity845Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_846<'a>(value: &'a crate::CustomEntity846) -> crate::CustomEntity846Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity846(id={})", value.id()));
        crate::CustomEntity846Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_847<'a>(value: &'a crate::CustomEntity847) -> crate::CustomEntity847Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity847(id={})", value.id()));
        crate::CustomEntity847Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_848<'a>(value: &'a crate::CustomEntity848) -> crate::CustomEntity848Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity848(id={})", value.id()));
        crate::CustomEntity848Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_849<'a>(value: &'a crate::CustomEntity849) -> crate::CustomEntity849Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity849(id={})", value.id()));
        crate::CustomEntity849Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_850<'a>(value: &'a crate::CustomEntity850) -> crate::CustomEntity850Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity850(id={})", value.id()));
        crate::CustomEntity850Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_851<'a>(value: &'a crate::CustomEntity851) -> crate::CustomEntity851Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity851(id={})", value.id()));
        crate::CustomEntity851Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_852<'a>(value: &'a crate::CustomEntity852) -> crate::CustomEntity852Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity852(id={})", value.id()));
        crate::CustomEntity852Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_853<'a>(value: &'a crate::CustomEntity853) -> crate::CustomEntity853Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity853(id={})", value.id()));
        crate::CustomEntity853Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_854<'a>(value: &'a crate::CustomEntity854) -> crate::CustomEntity854Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity854(id={})", value.id()));
        crate::CustomEntity854Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_855<'a>(value: &'a crate::CustomEntity855) -> crate::CustomEntity855Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity855(id={})", value.id()));
        crate::CustomEntity855Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_856<'a>(value: &'a crate::CustomEntity856) -> crate::CustomEntity856Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity856(id={})", value.id()));
        crate::CustomEntity856Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_857<'a>(value: &'a crate::CustomEntity857) -> crate::CustomEntity857Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity857(id={})", value.id()));
        crate::CustomEntity857Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_858<'a>(value: &'a crate::CustomEntity858) -> crate::CustomEntity858Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity858(id={})", value.id()));
        crate::CustomEntity858Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_859<'a>(value: &'a crate::CustomEntity859) -> crate::CustomEntity859Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity859(id={})", value.id()));
        crate::CustomEntity859Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_860<'a>(value: &'a crate::CustomEntity860) -> crate::CustomEntity860Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity860(id={})", value.id()));
        crate::CustomEntity860Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_861<'a>(value: &'a crate::CustomEntity861) -> crate::CustomEntity861Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity861(id={})", value.id()));
        crate::CustomEntity861Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_862<'a>(value: &'a crate::CustomEntity862) -> crate::CustomEntity862Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity862(id={})", value.id()));
        crate::CustomEntity862Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_863<'a>(value: &'a crate::CustomEntity863) -> crate::CustomEntity863Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity863(id={})", value.id()));
        crate::CustomEntity863Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_864<'a>(value: &'a crate::CustomEntity864) -> crate::CustomEntity864Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity864(id={})", value.id()));
        crate::CustomEntity864Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_865<'a>(value: &'a crate::CustomEntity865) -> crate::CustomEntity865Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity865(id={})", value.id()));
        crate::CustomEntity865Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_866<'a>(value: &'a crate::CustomEntity866) -> crate::CustomEntity866Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity866(id={})", value.id()));
        crate::CustomEntity866Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_867<'a>(value: &'a crate::CustomEntity867) -> crate::CustomEntity867Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity867(id={})", value.id()));
        crate::CustomEntity867Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_868<'a>(value: &'a crate::CustomEntity868) -> crate::CustomEntity868Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity868(id={})", value.id()));
        crate::CustomEntity868Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_869<'a>(value: &'a crate::CustomEntity869) -> crate::CustomEntity869Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity869(id={})", value.id()));
        crate::CustomEntity869Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_870<'a>(value: &'a crate::CustomEntity870) -> crate::CustomEntity870Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity870(id={})", value.id()));
        crate::CustomEntity870Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_871<'a>(value: &'a crate::CustomEntity871) -> crate::CustomEntity871Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity871(id={})", value.id()));
        crate::CustomEntity871Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_872<'a>(value: &'a crate::CustomEntity872) -> crate::CustomEntity872Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity872(id={})", value.id()));
        crate::CustomEntity872Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_873<'a>(value: &'a crate::CustomEntity873) -> crate::CustomEntity873Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity873(id={})", value.id()));
        crate::CustomEntity873Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_874<'a>(value: &'a crate::CustomEntity874) -> crate::CustomEntity874Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity874(id={})", value.id()));
        crate::CustomEntity874Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_875<'a>(value: &'a crate::CustomEntity875) -> crate::CustomEntity875Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity875(id={})", value.id()));
        crate::CustomEntity875Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_876<'a>(value: &'a crate::CustomEntity876) -> crate::CustomEntity876Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity876(id={})", value.id()));
        crate::CustomEntity876Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_877<'a>(value: &'a crate::CustomEntity877) -> crate::CustomEntity877Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity877(id={})", value.id()));
        crate::CustomEntity877Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_878<'a>(value: &'a crate::CustomEntity878) -> crate::CustomEntity878Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity878(id={})", value.id()));
        crate::CustomEntity878Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_879<'a>(value: &'a crate::CustomEntity879) -> crate::CustomEntity879Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity879(id={})", value.id()));
        crate::CustomEntity879Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_880<'a>(value: &'a crate::CustomEntity880) -> crate::CustomEntity880Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity880(id={})", value.id()));
        crate::CustomEntity880Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_881<'a>(value: &'a crate::CustomEntity881) -> crate::CustomEntity881Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity881(id={})", value.id()));
        crate::CustomEntity881Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_882<'a>(value: &'a crate::CustomEntity882) -> crate::CustomEntity882Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity882(id={})", value.id()));
        crate::CustomEntity882Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_883<'a>(value: &'a crate::CustomEntity883) -> crate::CustomEntity883Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity883(id={})", value.id()));
        crate::CustomEntity883Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_884<'a>(value: &'a crate::CustomEntity884) -> crate::CustomEntity884Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity884(id={})", value.id()));
        crate::CustomEntity884Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_885<'a>(value: &'a crate::CustomEntity885) -> crate::CustomEntity885Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity885(id={})", value.id()));
        crate::CustomEntity885Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_886<'a>(value: &'a crate::CustomEntity886) -> crate::CustomEntity886Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity886(id={})", value.id()));
        crate::CustomEntity886Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_887<'a>(value: &'a crate::CustomEntity887) -> crate::CustomEntity887Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity887(id={})", value.id()));
        crate::CustomEntity887Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_888<'a>(value: &'a crate::CustomEntity888) -> crate::CustomEntity888Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity888(id={})", value.id()));
        crate::CustomEntity888Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_889<'a>(value: &'a crate::CustomEntity889) -> crate::CustomEntity889Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity889(id={})", value.id()));
        crate::CustomEntity889Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_890<'a>(value: &'a crate::CustomEntity890) -> crate::CustomEntity890Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity890(id={})", value.id()));
        crate::CustomEntity890Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_891<'a>(value: &'a crate::CustomEntity891) -> crate::CustomEntity891Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity891(id={})", value.id()));
        crate::CustomEntity891Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_892<'a>(value: &'a crate::CustomEntity892) -> crate::CustomEntity892Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity892(id={})", value.id()));
        crate::CustomEntity892Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_893<'a>(value: &'a crate::CustomEntity893) -> crate::CustomEntity893Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity893(id={})", value.id()));
        crate::CustomEntity893Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_894<'a>(value: &'a crate::CustomEntity894) -> crate::CustomEntity894Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity894(id={})", value.id()));
        crate::CustomEntity894Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_895<'a>(value: &'a crate::CustomEntity895) -> crate::CustomEntity895Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity895(id={})", value.id()));
        crate::CustomEntity895Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_896<'a>(value: &'a crate::CustomEntity896) -> crate::CustomEntity896Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity896(id={})", value.id()));
        crate::CustomEntity896Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_897<'a>(value: &'a crate::CustomEntity897) -> crate::CustomEntity897Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity897(id={})", value.id()));
        crate::CustomEntity897Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_898<'a>(value: &'a crate::CustomEntity898) -> crate::CustomEntity898Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity898(id={})", value.id()));
        crate::CustomEntity898Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_899<'a>(value: &'a crate::CustomEntity899) -> crate::CustomEntity899Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity899(id={})", value.id()));
        crate::CustomEntity899Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_900<'a>(value: &'a crate::CustomEntity900) -> crate::CustomEntity900Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity900(id={})", value.id()));
        crate::CustomEntity900Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_901<'a>(value: &'a crate::CustomEntity901) -> crate::CustomEntity901Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity901(id={})", value.id()));
        crate::CustomEntity901Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_902<'a>(value: &'a crate::CustomEntity902) -> crate::CustomEntity902Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity902(id={})", value.id()));
        crate::CustomEntity902Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_903<'a>(value: &'a crate::CustomEntity903) -> crate::CustomEntity903Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity903(id={})", value.id()));
        crate::CustomEntity903Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_904<'a>(value: &'a crate::CustomEntity904) -> crate::CustomEntity904Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity904(id={})", value.id()));
        crate::CustomEntity904Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_905<'a>(value: &'a crate::CustomEntity905) -> crate::CustomEntity905Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity905(id={})", value.id()));
        crate::CustomEntity905Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_906<'a>(value: &'a crate::CustomEntity906) -> crate::CustomEntity906Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity906(id={})", value.id()));
        crate::CustomEntity906Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_907<'a>(value: &'a crate::CustomEntity907) -> crate::CustomEntity907Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity907(id={})", value.id()));
        crate::CustomEntity907Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_908<'a>(value: &'a crate::CustomEntity908) -> crate::CustomEntity908Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity908(id={})", value.id()));
        crate::CustomEntity908Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_909<'a>(value: &'a crate::CustomEntity909) -> crate::CustomEntity909Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity909(id={})", value.id()));
        crate::CustomEntity909Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_910<'a>(value: &'a crate::CustomEntity910) -> crate::CustomEntity910Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity910(id={})", value.id()));
        crate::CustomEntity910Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_911<'a>(value: &'a crate::CustomEntity911) -> crate::CustomEntity911Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity911(id={})", value.id()));
        crate::CustomEntity911Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_912<'a>(value: &'a crate::CustomEntity912) -> crate::CustomEntity912Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity912(id={})", value.id()));
        crate::CustomEntity912Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_913<'a>(value: &'a crate::CustomEntity913) -> crate::CustomEntity913Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity913(id={})", value.id()));
        crate::CustomEntity913Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_914<'a>(value: &'a crate::CustomEntity914) -> crate::CustomEntity914Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity914(id={})", value.id()));
        crate::CustomEntity914Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_915<'a>(value: &'a crate::CustomEntity915) -> crate::CustomEntity915Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity915(id={})", value.id()));
        crate::CustomEntity915Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_916<'a>(value: &'a crate::CustomEntity916) -> crate::CustomEntity916Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity916(id={})", value.id()));
        crate::CustomEntity916Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_917<'a>(value: &'a crate::CustomEntity917) -> crate::CustomEntity917Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity917(id={})", value.id()));
        crate::CustomEntity917Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_918<'a>(value: &'a crate::CustomEntity918) -> crate::CustomEntity918Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity918(id={})", value.id()));
        crate::CustomEntity918Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_919<'a>(value: &'a crate::CustomEntity919) -> crate::CustomEntity919Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity919(id={})", value.id()));
        crate::CustomEntity919Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_920<'a>(value: &'a crate::CustomEntity920) -> crate::CustomEntity920Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity920(id={})", value.id()));
        crate::CustomEntity920Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_921<'a>(value: &'a crate::CustomEntity921) -> crate::CustomEntity921Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity921(id={})", value.id()));
        crate::CustomEntity921Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_922<'a>(value: &'a crate::CustomEntity922) -> crate::CustomEntity922Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity922(id={})", value.id()));
        crate::CustomEntity922Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_923<'a>(value: &'a crate::CustomEntity923) -> crate::CustomEntity923Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity923(id={})", value.id()));
        crate::CustomEntity923Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_924<'a>(value: &'a crate::CustomEntity924) -> crate::CustomEntity924Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity924(id={})", value.id()));
        crate::CustomEntity924Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_925<'a>(value: &'a crate::CustomEntity925) -> crate::CustomEntity925Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity925(id={})", value.id()));
        crate::CustomEntity925Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_926<'a>(value: &'a crate::CustomEntity926) -> crate::CustomEntity926Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity926(id={})", value.id()));
        crate::CustomEntity926Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_927<'a>(value: &'a crate::CustomEntity927) -> crate::CustomEntity927Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity927(id={})", value.id()));
        crate::CustomEntity927Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_928<'a>(value: &'a crate::CustomEntity928) -> crate::CustomEntity928Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity928(id={})", value.id()));
        crate::CustomEntity928Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_929<'a>(value: &'a crate::CustomEntity929) -> crate::CustomEntity929Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity929(id={})", value.id()));
        crate::CustomEntity929Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_930<'a>(value: &'a crate::CustomEntity930) -> crate::CustomEntity930Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity930(id={})", value.id()));
        crate::CustomEntity930Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_931<'a>(value: &'a crate::CustomEntity931) -> crate::CustomEntity931Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity931(id={})", value.id()));
        crate::CustomEntity931Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_932<'a>(value: &'a crate::CustomEntity932) -> crate::CustomEntity932Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity932(id={})", value.id()));
        crate::CustomEntity932Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_933<'a>(value: &'a crate::CustomEntity933) -> crate::CustomEntity933Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity933(id={})", value.id()));
        crate::CustomEntity933Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_934<'a>(value: &'a crate::CustomEntity934) -> crate::CustomEntity934Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity934(id={})", value.id()));
        crate::CustomEntity934Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_935<'a>(value: &'a crate::CustomEntity935) -> crate::CustomEntity935Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity935(id={})", value.id()));
        crate::CustomEntity935Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_936<'a>(value: &'a crate::CustomEntity936) -> crate::CustomEntity936Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity936(id={})", value.id()));
        crate::CustomEntity936Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_937<'a>(value: &'a crate::CustomEntity937) -> crate::CustomEntity937Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity937(id={})", value.id()));
        crate::CustomEntity937Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_938<'a>(value: &'a crate::CustomEntity938) -> crate::CustomEntity938Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity938(id={})", value.id()));
        crate::CustomEntity938Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_939<'a>(value: &'a crate::CustomEntity939) -> crate::CustomEntity939Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity939(id={})", value.id()));
        crate::CustomEntity939Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_940<'a>(value: &'a crate::CustomEntity940) -> crate::CustomEntity940Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity940(id={})", value.id()));
        crate::CustomEntity940Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_941<'a>(value: &'a crate::CustomEntity941) -> crate::CustomEntity941Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity941(id={})", value.id()));
        crate::CustomEntity941Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_942<'a>(value: &'a crate::CustomEntity942) -> crate::CustomEntity942Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity942(id={})", value.id()));
        crate::CustomEntity942Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_943<'a>(value: &'a crate::CustomEntity943) -> crate::CustomEntity943Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity943(id={})", value.id()));
        crate::CustomEntity943Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_944<'a>(value: &'a crate::CustomEntity944) -> crate::CustomEntity944Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity944(id={})", value.id()));
        crate::CustomEntity944Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_945<'a>(value: &'a crate::CustomEntity945) -> crate::CustomEntity945Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity945(id={})", value.id()));
        crate::CustomEntity945Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_946<'a>(value: &'a crate::CustomEntity946) -> crate::CustomEntity946Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity946(id={})", value.id()));
        crate::CustomEntity946Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_947<'a>(value: &'a crate::CustomEntity947) -> crate::CustomEntity947Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity947(id={})", value.id()));
        crate::CustomEntity947Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_948<'a>(value: &'a crate::CustomEntity948) -> crate::CustomEntity948Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity948(id={})", value.id()));
        crate::CustomEntity948Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_949<'a>(value: &'a crate::CustomEntity949) -> crate::CustomEntity949Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity949(id={})", value.id()));
        crate::CustomEntity949Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_950<'a>(value: &'a crate::CustomEntity950) -> crate::CustomEntity950Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity950(id={})", value.id()));
        crate::CustomEntity950Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_951<'a>(value: &'a crate::CustomEntity951) -> crate::CustomEntity951Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity951(id={})", value.id()));
        crate::CustomEntity951Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_952<'a>(value: &'a crate::CustomEntity952) -> crate::CustomEntity952Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity952(id={})", value.id()));
        crate::CustomEntity952Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_953<'a>(value: &'a crate::CustomEntity953) -> crate::CustomEntity953Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity953(id={})", value.id()));
        crate::CustomEntity953Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_954<'a>(value: &'a crate::CustomEntity954) -> crate::CustomEntity954Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity954(id={})", value.id()));
        crate::CustomEntity954Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_955<'a>(value: &'a crate::CustomEntity955) -> crate::CustomEntity955Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity955(id={})", value.id()));
        crate::CustomEntity955Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_956<'a>(value: &'a crate::CustomEntity956) -> crate::CustomEntity956Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity956(id={})", value.id()));
        crate::CustomEntity956Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_957<'a>(value: &'a crate::CustomEntity957) -> crate::CustomEntity957Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity957(id={})", value.id()));
        crate::CustomEntity957Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_958<'a>(value: &'a crate::CustomEntity958) -> crate::CustomEntity958Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity958(id={})", value.id()));
        crate::CustomEntity958Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_959<'a>(value: &'a crate::CustomEntity959) -> crate::CustomEntity959Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity959(id={})", value.id()));
        crate::CustomEntity959Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_960<'a>(value: &'a crate::CustomEntity960) -> crate::CustomEntity960Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity960(id={})", value.id()));
        crate::CustomEntity960Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_961<'a>(value: &'a crate::CustomEntity961) -> crate::CustomEntity961Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity961(id={})", value.id()));
        crate::CustomEntity961Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_962<'a>(value: &'a crate::CustomEntity962) -> crate::CustomEntity962Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity962(id={})", value.id()));
        crate::CustomEntity962Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_963<'a>(value: &'a crate::CustomEntity963) -> crate::CustomEntity963Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity963(id={})", value.id()));
        crate::CustomEntity963Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_964<'a>(value: &'a crate::CustomEntity964) -> crate::CustomEntity964Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity964(id={})", value.id()));
        crate::CustomEntity964Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_965<'a>(value: &'a crate::CustomEntity965) -> crate::CustomEntity965Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity965(id={})", value.id()));
        crate::CustomEntity965Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_966<'a>(value: &'a crate::CustomEntity966) -> crate::CustomEntity966Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity966(id={})", value.id()));
        crate::CustomEntity966Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_967<'a>(value: &'a crate::CustomEntity967) -> crate::CustomEntity967Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity967(id={})", value.id()));
        crate::CustomEntity967Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_968<'a>(value: &'a crate::CustomEntity968) -> crate::CustomEntity968Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity968(id={})", value.id()));
        crate::CustomEntity968Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_969<'a>(value: &'a crate::CustomEntity969) -> crate::CustomEntity969Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity969(id={})", value.id()));
        crate::CustomEntity969Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_970<'a>(value: &'a crate::CustomEntity970) -> crate::CustomEntity970Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity970(id={})", value.id()));
        crate::CustomEntity970Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_971<'a>(value: &'a crate::CustomEntity971) -> crate::CustomEntity971Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity971(id={})", value.id()));
        crate::CustomEntity971Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_972<'a>(value: &'a crate::CustomEntity972) -> crate::CustomEntity972Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity972(id={})", value.id()));
        crate::CustomEntity972Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_973<'a>(value: &'a crate::CustomEntity973) -> crate::CustomEntity973Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity973(id={})", value.id()));
        crate::CustomEntity973Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_974<'a>(value: &'a crate::CustomEntity974) -> crate::CustomEntity974Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity974(id={})", value.id()));
        crate::CustomEntity974Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_975<'a>(value: &'a crate::CustomEntity975) -> crate::CustomEntity975Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity975(id={})", value.id()));
        crate::CustomEntity975Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_976<'a>(value: &'a crate::CustomEntity976) -> crate::CustomEntity976Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity976(id={})", value.id()));
        crate::CustomEntity976Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_977<'a>(value: &'a crate::CustomEntity977) -> crate::CustomEntity977Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity977(id={})", value.id()));
        crate::CustomEntity977Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_978<'a>(value: &'a crate::CustomEntity978) -> crate::CustomEntity978Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity978(id={})", value.id()));
        crate::CustomEntity978Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_979<'a>(value: &'a crate::CustomEntity979) -> crate::CustomEntity979Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity979(id={})", value.id()));
        crate::CustomEntity979Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_980<'a>(value: &'a crate::CustomEntity980) -> crate::CustomEntity980Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity980(id={})", value.id()));
        crate::CustomEntity980Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_981<'a>(value: &'a crate::CustomEntity981) -> crate::CustomEntity981Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity981(id={})", value.id()));
        crate::CustomEntity981Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_982<'a>(value: &'a crate::CustomEntity982) -> crate::CustomEntity982Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity982(id={})", value.id()));
        crate::CustomEntity982Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_983<'a>(value: &'a crate::CustomEntity983) -> crate::CustomEntity983Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity983(id={})", value.id()));
        crate::CustomEntity983Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_984<'a>(value: &'a crate::CustomEntity984) -> crate::CustomEntity984Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity984(id={})", value.id()));
        crate::CustomEntity984Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_985<'a>(value: &'a crate::CustomEntity985) -> crate::CustomEntity985Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity985(id={})", value.id()));
        crate::CustomEntity985Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_986<'a>(value: &'a crate::CustomEntity986) -> crate::CustomEntity986Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity986(id={})", value.id()));
        crate::CustomEntity986Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_987<'a>(value: &'a crate::CustomEntity987) -> crate::CustomEntity987Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity987(id={})", value.id()));
        crate::CustomEntity987Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_988<'a>(value: &'a crate::CustomEntity988) -> crate::CustomEntity988Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity988(id={})", value.id()));
        crate::CustomEntity988Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_989<'a>(value: &'a crate::CustomEntity989) -> crate::CustomEntity989Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity989(id={})", value.id()));
        crate::CustomEntity989Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_990<'a>(value: &'a crate::CustomEntity990) -> crate::CustomEntity990Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity990(id={})", value.id()));
        crate::CustomEntity990Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_991<'a>(value: &'a crate::CustomEntity991) -> crate::CustomEntity991Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity991(id={})", value.id()));
        crate::CustomEntity991Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_992<'a>(value: &'a crate::CustomEntity992) -> crate::CustomEntity992Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity992(id={})", value.id()));
        crate::CustomEntity992Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_993<'a>(value: &'a crate::CustomEntity993) -> crate::CustomEntity993Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity993(id={})", value.id()));
        crate::CustomEntity993Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_994<'a>(value: &'a crate::CustomEntity994) -> crate::CustomEntity994Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity994(id={})", value.id()));
        crate::CustomEntity994Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_995<'a>(value: &'a crate::CustomEntity995) -> crate::CustomEntity995Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity995(id={})", value.id()));
        crate::CustomEntity995Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_996<'a>(value: &'a crate::CustomEntity996) -> crate::CustomEntity996Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity996(id={})", value.id()));
        crate::CustomEntity996Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_997<'a>(value: &'a crate::CustomEntity997) -> crate::CustomEntity997Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity997(id={})", value.id()));
        crate::CustomEntity997Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_998<'a>(value: &'a crate::CustomEntity998) -> crate::CustomEntity998Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity998(id={})", value.id()));
        crate::CustomEntity998Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn custom_entity_999<'a>(value: &'a crate::CustomEntity999) -> crate::CustomEntity999Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomEntity999(id={})", value.id()));
        crate::CustomEntity999Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

