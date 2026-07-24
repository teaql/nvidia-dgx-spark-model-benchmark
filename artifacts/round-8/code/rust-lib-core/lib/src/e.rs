// The `E` expression wrapper provides zero-cost AST traversal
// and will automatically panic if it encounters a NotLoaded error.
pub struct E;

impl E {
    pub fn platform<'a>(value: &'a crate::Platform) -> crate::PlatformExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Platform(id={})", value.id()));
        crate::PlatformExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn merchant<'a>(value: &'a crate::Merchant) -> crate::MerchantExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Merchant(id={})", value.id()));
        crate::MerchantExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn employee<'a>(value: &'a crate::Employee) -> crate::EmployeeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Employee(id={})", value.id()));
        crate::EmployeeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn platform_setting<'a>(value: &'a crate::PlatformSetting) -> crate::PlatformSettingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PlatformSetting(id={})", value.id()));
        crate::PlatformSettingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tenant_configuration<'a>(value: &'a crate::TenantConfiguration) -> crate::TenantConfigurationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TenantConfiguration(id={})", value.id()));
        crate::TenantConfigurationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn organization_unit<'a>(value: &'a crate::OrganizationUnit) -> crate::OrganizationUnitExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OrganizationUnit(id={})", value.id()));
        crate::OrganizationUnitExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn department_hierarchy<'a>(value: &'a crate::DepartmentHierarchy) -> crate::DepartmentHierarchyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DepartmentHierarchy(id={})", value.id()));
        crate::DepartmentHierarchyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn branch_office<'a>(value: &'a crate::BranchOffice) -> crate::BranchOfficeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BranchOffice(id={})", value.id()));
        crate::BranchOfficeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn move_item<'a>(value: &'a crate::MoveItem) -> crate::MoveItemExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveItem(id={})", value.id()));
        crate::MoveItemExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn inventory_list<'a>(value: &'a crate::InventoryList) -> crate::InventoryListExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InventoryList(id={})", value.id()));
        crate::InventoryListExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn packing_material<'a>(value: &'a crate::PackingMaterial) -> crate::PackingMaterialExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PackingMaterial(id={})", value.id()));
        crate::PackingMaterialExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn loading_zone<'a>(value: &'a crate::LoadingZone) -> crate::LoadingZoneExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoadingZone(id={})", value.id()));
        crate::LoadingZoneExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn unloading_zone<'a>(value: &'a crate::UnloadingZone) -> crate::UnloadingZoneExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UnloadingZone(id={})", value.id()));
        crate::UnloadingZoneExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn transit_log<'a>(value: &'a crate::TransitLog) -> crate::TransitLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TransitLog(id={})", value.id()));
        crate::TransitLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn delay_record<'a>(value: &'a crate::DelayRecord) -> crate::DelayRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DelayRecord(id={})", value.id()));
        crate::DelayRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn route_optimization_rule<'a>(value: &'a crate::RouteOptimizationRule) -> crate::RouteOptimizationRuleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RouteOptimizationRule(id={})", value.id()));
        crate::RouteOptimizationRuleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_assignment<'a>(value: &'a crate::VehicleAssignment) -> crate::VehicleAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleAssignment(id={})", value.id()));
        crate::VehicleAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn cargo_weight_record<'a>(value: &'a crate::CargoWeightRecord) -> crate::CargoWeightRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CargoWeightRecord(id={})", value.id()));
        crate::CargoWeightRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn special_handling_instruction<'a>(value: &'a crate::SpecialHandlingInstruction) -> crate::SpecialHandlingInstructionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SpecialHandlingInstruction(id={})", value.id()));
        crate::SpecialHandlingInstructionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_status<'a>(value: &'a crate::MoveStatus) -> crate::MoveStatusExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveStatus(id={})", value.id()));
        crate::MoveStatusExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn delivery_window<'a>(value: &'a crate::DeliveryWindow) -> crate::DeliveryWindowExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DeliveryWindow(id={})", value.id()));
        crate::DeliveryWindowExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn deduction<'a>(value: &'a crate::Deduction) -> crate::DeductionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Deduction(id={})", value.id()));
        crate::DeductionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn leave_request<'a>(value: &'a crate::LeaveRequest) -> crate::LeaveRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LeaveRequest(id={})", value.id()));
        crate::LeaveRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn employee_certification<'a>(value: &'a crate::EmployeeCertification) -> crate::EmployeeCertificationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EmployeeCertification(id={})", value.id()));
        crate::EmployeeCertificationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn training_module<'a>(value: &'a crate::TrainingModule) -> crate::TrainingModuleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TrainingModule(id={})", value.id()));
        crate::TrainingModuleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn availability_schedule<'a>(value: &'a crate::AvailabilitySchedule) -> crate::AvailabilityScheduleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AvailabilitySchedule(id={})", value.id()));
        crate::AvailabilityScheduleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn skill_profile<'a>(value: &'a crate::SkillProfile) -> crate::SkillProfileExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SkillProfile(id={})", value.id()));
        crate::SkillProfileExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn performance_review<'a>(value: &'a crate::PerformanceReview) -> crate::PerformanceReviewExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PerformanceReview(id={})", value.id()));
        crate::PerformanceReviewExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn overtime_record<'a>(value: &'a crate::OvertimeRecord) -> crate::OvertimeRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OvertimeRecord(id={})", value.id()));
        crate::OvertimeRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tax_withholding<'a>(value: &'a crate::TaxWithholding) -> crate::TaxWithholdingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TaxWithholding(id={})", value.id()));
        crate::TaxWithholdingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn benefit_enrollment<'a>(value: &'a crate::BenefitEnrollment) -> crate::BenefitEnrollmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BenefitEnrollment(id={})", value.id()));
        crate::BenefitEnrollmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn shift_swap_request<'a>(value: &'a crate::ShiftSwapRequest) -> crate::ShiftSwapRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ShiftSwapRequest(id={})", value.id()));
        crate::ShiftSwapRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn attendance_record<'a>(value: &'a crate::AttendanceRecord) -> crate::AttendanceRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AttendanceRecord(id={})", value.id()));
        crate::AttendanceRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payroll_adjustment<'a>(value: &'a crate::PayrollAdjustment) -> crate::PayrollAdjustmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PayrollAdjustment(id={})", value.id()));
        crate::PayrollAdjustmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn commission_record<'a>(value: &'a crate::CommissionRecord) -> crate::CommissionRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CommissionRecord(id={})", value.id()));
        crate::CommissionRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn customer_feedback<'a>(value: &'a crate::CustomerFeedback) -> crate::CustomerFeedbackExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerFeedback(id={})", value.id()));
        crate::CustomerFeedbackExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn loyalty_tier<'a>(value: &'a crate::LoyaltyTier) -> crate::LoyaltyTierExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoyaltyTier(id={})", value.id()));
        crate::LoyaltyTierExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn referral_code<'a>(value: &'a crate::ReferralCode) -> crate::ReferralCodeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ReferralCode(id={})", value.id()));
        crate::ReferralCodeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn communication_log<'a>(value: &'a crate::CommunicationLog) -> crate::CommunicationLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CommunicationLog(id={})", value.id()));
        crate::CommunicationLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_rating<'a>(value: &'a crate::ServiceRating) -> crate::ServiceRatingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceRating(id={})", value.id()));
        crate::ServiceRatingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn account_status<'a>(value: &'a crate::AccountStatus) -> crate::AccountStatusExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AccountStatus(id={})", value.id()));
        crate::AccountStatusExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn contact_method<'a>(value: &'a crate::ContactMethod) -> crate::ContactMethodExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ContactMethod(id={})", value.id()));
        crate::ContactMethodExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_segment<'a>(value: &'a crate::CustomerSegment) -> crate::CustomerSegmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerSegment(id={})", value.id()));
        crate::CustomerSegmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn packing_kit<'a>(value: &'a crate::PackingKit) -> crate::PackingKitExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PackingKit(id={})", value.id()));
        crate::PackingKitExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn disposal_service<'a>(value: &'a crate::DisposalService) -> crate::DisposalServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DisposalService(id={})", value.id()));
        crate::DisposalServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_area<'a>(value: &'a crate::ServiceArea) -> crate::ServiceAreaExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceArea(id={})", value.id()));
        crate::ServiceAreaExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn availability_calendar<'a>(value: &'a crate::AvailabilityCalendar) -> crate::AvailabilityCalendarExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AvailabilityCalendar(id={})", value.id()));
        crate::AvailabilityCalendarExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_level_agreement<'a>(value: &'a crate::ServiceLevelAgreement) -> crate::ServiceLevelAgreementExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceLevelAgreement(id={})", value.id()));
        crate::ServiceLevelAgreementExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn add_on_service<'a>(value: &'a crate::AddOnService) -> crate::AddOnServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AddOnService(id={})", value.id()));
        crate::AddOnServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn inventory_item<'a>(value: &'a crate::InventoryItem) -> crate::InventoryItemExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InventoryItem(id={})", value.id()));
        crate::InventoryItemExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_category<'a>(value: &'a crate::ServiceCategory) -> crate::ServiceCategoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceCategory(id={})", value.id()));
        crate::ServiceCategoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn marketing_channel<'a>(value: &'a crate::MarketingChannel) -> crate::MarketingChannelExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MarketingChannel(id={})", value.id()));
        crate::MarketingChannelExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn audience_segment<'a>(value: &'a crate::AudienceSegment) -> crate::AudienceSegmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AudienceSegment(id={})", value.id()));
        crate::AudienceSegmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn promotional_offer<'a>(value: &'a crate::PromotionalOffer) -> crate::PromotionalOfferExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PromotionalOffer(id={})", value.id()));
        crate::PromotionalOfferExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn sales_funnel<'a>(value: &'a crate::SalesFunnel) -> crate::SalesFunnelExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SalesFunnel(id={})", value.id()));
        crate::SalesFunnelExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn attribution_model<'a>(value: &'a crate::AttributionModel) -> crate::AttributionModelExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AttributionModel(id={})", value.id()));
        crate::AttributionModelExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn lead_score<'a>(value: &'a crate::LeadScore) -> crate::LeadScoreExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LeadScore(id={})", value.id()));
        crate::LeadScoreExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn campaign_budget<'a>(value: &'a crate::CampaignBudget) -> crate::CampaignBudgetExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CampaignBudget(id={})", value.id()));
        crate::CampaignBudgetExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn conversion_report<'a>(value: &'a crate::ConversionReport) -> crate::ConversionReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ConversionReport(id={})", value.id()));
        crate::ConversionReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn budget<'a>(value: &'a crate::Budget) -> crate::BudgetExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Budget(id={})", value.id()));
        crate::BudgetExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn settlement<'a>(value: &'a crate::Settlement) -> crate::SettlementExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Settlement(id={})", value.id()));
        crate::SettlementExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn receivable<'a>(value: &'a crate::Receivable) -> crate::ReceivableExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Receivable(id={})", value.id()));
        crate::ReceivableExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payable<'a>(value: &'a crate::Payable) -> crate::PayableExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Payable(id={})", value.id()));
        crate::PayableExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tax_record<'a>(value: &'a crate::TaxRecord) -> crate::TaxRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TaxRecord(id={})", value.id()));
        crate::TaxRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn currency_rate<'a>(value: &'a crate::CurrencyRate) -> crate::CurrencyRateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CurrencyRate(id={})", value.id()));
        crate::CurrencyRateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payment_method<'a>(value: &'a crate::PaymentMethod) -> crate::PaymentMethodExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PaymentMethod(id={})", value.id()));
        crate::PaymentMethodExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn financial_period<'a>(value: &'a crate::FinancialPeriod) -> crate::FinancialPeriodExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FinancialPeriod(id={})", value.id()));
        crate::FinancialPeriodExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn inventory_stock<'a>(value: &'a crate::InventoryStock) -> crate::InventoryStockExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InventoryStock(id={})", value.id()));
        crate::InventoryStockExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn maintenance_cost<'a>(value: &'a crate::MaintenanceCost) -> crate::MaintenanceCostExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MaintenanceCost(id={})", value.id()));
        crate::MaintenanceCostExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_registration<'a>(value: &'a crate::VehicleRegistration) -> crate::VehicleRegistrationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleRegistration(id={})", value.id()));
        crate::VehicleRegistrationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn equipment_serial<'a>(value: &'a crate::EquipmentSerial) -> crate::EquipmentSerialExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EquipmentSerial(id={})", value.id()));
        crate::EquipmentSerialExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn supplier_contract<'a>(value: &'a crate::SupplierContract) -> crate::SupplierContractExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SupplierContract(id={})", value.id()));
        crate::SupplierContractExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn asset_condition<'a>(value: &'a crate::AssetCondition) -> crate::AssetConditionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AssetCondition(id={})", value.id()));
        crate::AssetConditionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn depreciation_record<'a>(value: &'a crate::DepreciationRecord) -> crate::DepreciationRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DepreciationRecord(id={})", value.id()));
        crate::DepreciationRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn warranty_claim<'a>(value: &'a crate::WarrantyClaim) -> crate::WarrantyClaimExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("WarrantyClaim(id={})", value.id()));
        crate::WarrantyClaimExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn storage_location<'a>(value: &'a crate::StorageLocation) -> crate::StorageLocationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("StorageLocation(id={})", value.id()));
        crate::StorageLocationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn policy_document<'a>(value: &'a crate::PolicyDocument) -> crate::PolicyDocumentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PolicyDocument(id={})", value.id()));
        crate::PolicyDocumentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn incident_report<'a>(value: &'a crate::IncidentReport) -> crate::IncidentReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("IncidentReport(id={})", value.id()));
        crate::IncidentReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn audit_trail<'a>(value: &'a crate::AuditTrail) -> crate::AuditTrailExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AuditTrail(id={})", value.id()));
        crate::AuditTrailExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn legal_entity<'a>(value: &'a crate::LegalEntity) -> crate::LegalEntityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LegalEntity(id={})", value.id()));
        crate::LegalEntityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn regulatory_requirement<'a>(value: &'a crate::RegulatoryRequirement) -> crate::RegulatoryRequirementExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RegulatoryRequirement(id={})", value.id()));
        crate::RegulatoryRequirementExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn compliance_certificate<'a>(value: &'a crate::ComplianceCertificate) -> crate::ComplianceCertificateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ComplianceCertificate(id={})", value.id()));
        crate::ComplianceCertificateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn access_token<'a>(value: &'a crate::AccessToken) -> crate::AccessTokenExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AccessToken(id={})", value.id()));
        crate::AccessTokenExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn two_factor_auth<'a>(value: &'a crate::TwoFactorAuth) -> crate::TwoFactorAuthExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TwoFactorAuth(id={})", value.id()));
        crate::TwoFactorAuthExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn login_attempt<'a>(value: &'a crate::LoginAttempt) -> crate::LoginAttemptExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoginAttempt(id={})", value.id()));
        crate::LoginAttemptExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn system_event<'a>(value: &'a crate::SystemEvent) -> crate::SystemEventExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SystemEvent(id={})", value.id()));
        crate::SystemEventExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn data_export<'a>(value: &'a crate::DataExport) -> crate::DataExportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DataExport(id={})", value.id()));
        crate::DataExportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn operational_hook<'a>(value: &'a crate::OperationalHook) -> crate::OperationalHookExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OperationalHook(id={})", value.id()));
        crate::OperationalHookExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn financial_hook<'a>(value: &'a crate::FinancialHook) -> crate::FinancialHookExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FinancialHook(id={})", value.id()));
        crate::FinancialHookExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn synchronization_run<'a>(value: &'a crate::SynchronizationRun) -> crate::SynchronizationRunExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SynchronizationRun(id={})", value.id()));
        crate::SynchronizationRunExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn api_key<'a>(value: &'a crate::ApiKey) -> crate::ApiKeyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ApiKey(id={})", value.id()));
        crate::ApiKeyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

