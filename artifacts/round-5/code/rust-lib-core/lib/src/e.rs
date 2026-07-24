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

    pub fn platform_user<'a>(value: &'a crate::PlatformUser) -> crate::PlatformUserExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PlatformUser(id={})", value.id()));
        crate::PlatformUserExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn platform_audit_log<'a>(value: &'a crate::PlatformAuditLog) -> crate::PlatformAuditLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PlatformAuditLog(id={})", value.id()));
        crate::PlatformAuditLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn organization<'a>(value: &'a crate::Organization) -> crate::OrganizationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Organization(id={})", value.id()));
        crate::OrganizationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn organization_setting<'a>(value: &'a crate::OrganizationSetting) -> crate::OrganizationSettingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OrganizationSetting(id={})", value.id()));
        crate::OrganizationSettingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn organization_member<'a>(value: &'a crate::OrganizationMember) -> crate::OrganizationMemberExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OrganizationMember(id={})", value.id()));
        crate::OrganizationMemberExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn inventory_item<'a>(value: &'a crate::InventoryItem) -> crate::InventoryItemExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InventoryItem(id={})", value.id()));
        crate::InventoryItemExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn packing_list<'a>(value: &'a crate::PackingList) -> crate::PackingListExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PackingList(id={})", value.id()));
        crate::PackingListExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn packing_item<'a>(value: &'a crate::PackingItem) -> crate::PackingItemExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PackingItem(id={})", value.id()));
        crate::PackingItemExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn loading_plan<'a>(value: &'a crate::LoadingPlan) -> crate::LoadingPlanExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoadingPlan(id={})", value.id()));
        crate::LoadingPlanExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn unloading_plan<'a>(value: &'a crate::UnloadingPlan) -> crate::UnloadingPlanExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UnloadingPlan(id={})", value.id()));
        crate::UnloadingPlanExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn storage_facility<'a>(value: &'a crate::StorageFacility) -> crate::StorageFacilityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("StorageFacility(id={})", value.id()));
        crate::StorageFacilityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn storage_unit<'a>(value: &'a crate::StorageUnit) -> crate::StorageUnitExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("StorageUnit(id={})", value.id()));
        crate::StorageUnitExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn storage_inventory<'a>(value: &'a crate::StorageInventory) -> crate::StorageInventoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("StorageInventory(id={})", value.id()));
        crate::StorageInventoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn transport_manifest<'a>(value: &'a crate::TransportManifest) -> crate::TransportManifestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TransportManifest(id={})", value.id()));
        crate::TransportManifestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customs_declaration<'a>(value: &'a crate::CustomsDeclaration) -> crate::CustomsDeclarationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomsDeclaration(id={})", value.id()));
        crate::CustomsDeclarationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn equipment_checklist<'a>(value: &'a crate::EquipmentChecklist) -> crate::EquipmentChecklistExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EquipmentChecklist(id={})", value.id()));
        crate::EquipmentChecklistExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fuel_log<'a>(value: &'a crate::FuelLog) -> crate::FuelLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FuelLog(id={})", value.id()));
        crate::FuelLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn maintenance_request<'a>(value: &'a crate::MaintenanceRequest) -> crate::MaintenanceRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MaintenanceRequest(id={})", value.id()));
        crate::MaintenanceRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn employee_certification<'a>(value: &'a crate::EmployeeCertification) -> crate::EmployeeCertificationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EmployeeCertification(id={})", value.id()));
        crate::EmployeeCertificationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn leave_request<'a>(value: &'a crate::LeaveRequest) -> crate::LeaveRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LeaveRequest(id={})", value.id()));
        crate::LeaveRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn billing_profile<'a>(value: &'a crate::BillingProfile) -> crate::BillingProfileExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BillingProfile(id={})", value.id()));
        crate::BillingProfileExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn corporate_customer_profile<'a>(value: &'a crate::CorporateCustomerProfile) -> crate::CorporateCustomerProfileExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CorporateCustomerProfile(id={})", value.id()));
        crate::CorporateCustomerProfileExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer<'a>(value: &'a crate::Customer) -> crate::CustomerExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Customer(id={})", value.id()));
        crate::CustomerExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_consent<'a>(value: &'a crate::CustomerConsent) -> crate::CustomerConsentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerConsent(id={})", value.id()));
        crate::CustomerConsentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_contact<'a>(value: &'a crate::CustomerContact) -> crate::CustomerContactExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerContact(id={})", value.id()));
        crate::CustomerContactExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_history<'a>(value: &'a crate::CustomerHistory) -> crate::CustomerHistoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerHistory(id={})", value.id()));
        crate::CustomerHistoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_preference<'a>(value: &'a crate::CustomerPreference) -> crate::CustomerPreferenceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerPreference(id={})", value.id()));
        crate::CustomerPreferenceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn private_customer_profile<'a>(value: &'a crate::PrivateCustomerProfile) -> crate::PrivateCustomerProfileExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PrivateCustomerProfile(id={})", value.id()));
        crate::PrivateCustomerProfileExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn box_rental<'a>(value: &'a crate::BoxRental) -> crate::BoxRentalExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BoxRental(id={})", value.id()));
        crate::BoxRentalExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn cleaning_service<'a>(value: &'a crate::CleaningService) -> crate::CleaningServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CleaningService(id={})", value.id()));
        crate::CleaningServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn moving_service<'a>(value: &'a crate::MovingService) -> crate::MovingServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MovingService(id={})", value.id()));
        crate::MovingServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn price_list<'a>(value: &'a crate::PriceList) -> crate::PriceListExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PriceList(id={})", value.id()));
        crate::PriceListExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn product<'a>(value: &'a crate::Product) -> crate::ProductExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Product(id={})", value.id()));
        crate::ProductExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service<'a>(value: &'a crate::Service) -> crate::ServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Service(id={})", value.id()));
        crate::ServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_bundle<'a>(value: &'a crate::ServiceBundle) -> crate::ServiceBundleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceBundle(id={})", value.id()));
        crate::ServiceBundleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_configuration<'a>(value: &'a crate::ServiceConfiguration) -> crate::ServiceConfigurationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceConfiguration(id={})", value.id()));
        crate::ServiceConfigurationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_price<'a>(value: &'a crate::ServicePrice) -> crate::ServicePriceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServicePrice(id={})", value.id()));
        crate::ServicePriceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn campaign<'a>(value: &'a crate::Campaign) -> crate::CampaignExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Campaign(id={})", value.id()));
        crate::CampaignExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn conversion_event<'a>(value: &'a crate::ConversionEvent) -> crate::ConversionEventExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ConversionEvent(id={})", value.id()));
        crate::ConversionEventExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn conversion_metric<'a>(value: &'a crate::ConversionMetric) -> crate::ConversionMetricExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ConversionMetric(id={})", value.id()));
        crate::ConversionMetricExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn discount_code<'a>(value: &'a crate::DiscountCode) -> crate::DiscountCodeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DiscountCode(id={})", value.id()));
        crate::DiscountCodeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn lead<'a>(value: &'a crate::Lead) -> crate::LeadExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Lead(id={})", value.id()));
        crate::LeadExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn lead_activity<'a>(value: &'a crate::LeadActivity) -> crate::LeadActivityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LeadActivity(id={})", value.id()));
        crate::LeadActivityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn sales_opportunity<'a>(value: &'a crate::SalesOpportunity) -> crate::SalesOpportunityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SalesOpportunity(id={})", value.id()));
        crate::SalesOpportunityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn account<'a>(value: &'a crate::Account) -> crate::AccountExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Account(id={})", value.id()));
        crate::AccountExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn expense<'a>(value: &'a crate::Expense) -> crate::ExpenseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Expense(id={})", value.id()));
        crate::ExpenseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn financial_summary<'a>(value: &'a crate::FinancialSummary) -> crate::FinancialSummaryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FinancialSummary(id={})", value.id()));
        crate::FinancialSummaryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn invoice<'a>(value: &'a crate::Invoice) -> crate::InvoiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Invoice(id={})", value.id()));
        crate::InvoiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn invoice_line<'a>(value: &'a crate::InvoiceLine) -> crate::InvoiceLineExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InvoiceLine(id={})", value.id()));
        crate::InvoiceLineExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn journal_entry<'a>(value: &'a crate::JournalEntry) -> crate::JournalEntryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("JournalEntry(id={})", value.id()));
        crate::JournalEntryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payment<'a>(value: &'a crate::Payment) -> crate::PaymentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Payment(id={})", value.id()));
        crate::PaymentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn refund<'a>(value: &'a crate::Refund) -> crate::RefundExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Refund(id={})", value.id()));
        crate::RefundExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vat_rate<'a>(value: &'a crate::VatRate) -> crate::VatRateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VatRate(id={})", value.id()));
        crate::VatRateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn asset_assignment<'a>(value: &'a crate::AssetAssignment) -> crate::AssetAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AssetAssignment(id={})", value.id()));
        crate::AssetAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn asset_inspection<'a>(value: &'a crate::AssetInspection) -> crate::AssetInspectionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AssetInspection(id={})", value.id()));
        crate::AssetInspectionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn consumable<'a>(value: &'a crate::Consumable) -> crate::ConsumableExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Consumable(id={})", value.id()));
        crate::ConsumableExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn equipment<'a>(value: &'a crate::Equipment) -> crate::EquipmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Equipment(id={})", value.id()));
        crate::EquipmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fuel_record<'a>(value: &'a crate::FuelRecord) -> crate::FuelRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FuelRecord(id={})", value.id()));
        crate::FuelRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn maintenance_event<'a>(value: &'a crate::MaintenanceEvent) -> crate::MaintenanceEventExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MaintenanceEvent(id={})", value.id()));
        crate::MaintenanceEventExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn maintenance_schedule<'a>(value: &'a crate::MaintenanceSchedule) -> crate::MaintenanceScheduleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MaintenanceSchedule(id={})", value.id()));
        crate::MaintenanceScheduleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn supplier<'a>(value: &'a crate::Supplier) -> crate::SupplierExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Supplier(id={})", value.id()));
        crate::SupplierExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle<'a>(value: &'a crate::Vehicle) -> crate::VehicleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Vehicle(id={})", value.id()));
        crate::VehicleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn compliance_check<'a>(value: &'a crate::ComplianceCheck) -> crate::ComplianceCheckExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ComplianceCheck(id={})", value.id()));
        crate::ComplianceCheckExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn contract<'a>(value: &'a crate::Contract) -> crate::ContractExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Contract(id={})", value.id()));
        crate::ContractExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn data_retention_policy<'a>(value: &'a crate::DataRetentionPolicy) -> crate::DataRetentionPolicyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DataRetentionPolicy(id={})", value.id()));
        crate::DataRetentionPolicyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn document<'a>(value: &'a crate::Document) -> crate::DocumentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Document(id={})", value.id()));
        crate::DocumentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn document_version<'a>(value: &'a crate::DocumentVersion) -> crate::DocumentVersionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DocumentVersion(id={})", value.id()));
        crate::DocumentVersionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn insurance_claim<'a>(value: &'a crate::InsuranceClaim) -> crate::InsuranceClaimExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InsuranceClaim(id={})", value.id()));
        crate::InsuranceClaimExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn insurance_policy<'a>(value: &'a crate::InsurancePolicy) -> crate::InsurancePolicyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InsurancePolicy(id={})", value.id()));
        crate::InsurancePolicyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn recovery_request<'a>(value: &'a crate::RecoveryRequest) -> crate::RecoveryRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RecoveryRequest(id={})", value.id()));
        crate::RecoveryRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn magic_link<'a>(value: &'a crate::MagicLink) -> crate::MagicLinkExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MagicLink(id={})", value.id()));
        crate::MagicLinkExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn permission<'a>(value: &'a crate::Permission) -> crate::PermissionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Permission(id={})", value.id()));
        crate::PermissionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn role<'a>(value: &'a crate::Role) -> crate::RoleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Role(id={})", value.id()));
        crate::RoleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn role_permission<'a>(value: &'a crate::RolePermission) -> crate::RolePermissionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RolePermission(id={})", value.id()));
        crate::RolePermissionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn user_account<'a>(value: &'a crate::UserAccount) -> crate::UserAccountExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UserAccount(id={})", value.id()));
        crate::UserAccountExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn user_role_assignment<'a>(value: &'a crate::UserRoleAssignment) -> crate::UserRoleAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UserRoleAssignment(id={})", value.id()));
        crate::UserRoleAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn user_session<'a>(value: &'a crate::UserSession) -> crate::UserSessionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("UserSession(id={})", value.id()));
        crate::UserSessionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn activity_log<'a>(value: &'a crate::ActivityLog) -> crate::ActivityLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ActivityLog(id={})", value.id()));
        crate::ActivityLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn audit_log<'a>(value: &'a crate::AuditLog) -> crate::AuditLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AuditLog(id={})", value.id()));
        crate::AuditLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn change_set<'a>(value: &'a crate::ChangeSet) -> crate::ChangeSetExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ChangeSet(id={})", value.id()));
        crate::ChangeSetExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn entity_change<'a>(value: &'a crate::EntityChange) -> crate::EntityChangeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EntityChange(id={})", value.id()));
        crate::EntityChangeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn automation_action<'a>(value: &'a crate::AutomationAction) -> crate::AutomationActionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AutomationAction(id={})", value.id()));
        crate::AutomationActionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn automation_rule<'a>(value: &'a crate::AutomationRule) -> crate::AutomationRuleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AutomationRule(id={})", value.id()));
        crate::AutomationRuleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn automation_trigger<'a>(value: &'a crate::AutomationTrigger) -> crate::AutomationTriggerExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AutomationTrigger(id={})", value.id()));
        crate::AutomationTriggerExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn notification<'a>(value: &'a crate::Notification) -> crate::NotificationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Notification(id={})", value.id()));
        crate::NotificationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn notification_template<'a>(value: &'a crate::NotificationTemplate) -> crate::NotificationTemplateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("NotificationTemplate(id={})", value.id()));
        crate::NotificationTemplateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn api_client<'a>(value: &'a crate::ApiClient) -> crate::ApiClientExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ApiClient(id={})", value.id()));
        crate::ApiClientExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn api_endpoint<'a>(value: &'a crate::ApiEndpoint) -> crate::ApiEndpointExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ApiEndpoint(id={})", value.id()));
        crate::ApiEndpointExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn integration_mapping<'a>(value: &'a crate::IntegrationMapping) -> crate::IntegrationMappingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("IntegrationMapping(id={})", value.id()));
        crate::IntegrationMappingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn webhook<'a>(value: &'a crate::Webhook) -> crate::WebhookExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Webhook(id={})", value.id()));
        crate::WebhookExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn webhook_delivery<'a>(value: &'a crate::WebhookDelivery) -> crate::WebhookDeliveryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("WebhookDelivery(id={})", value.id()));
        crate::WebhookDeliveryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn platform_configuration<'a>(value: &'a crate::PlatformConfiguration) -> crate::PlatformConfigurationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PlatformConfiguration(id={})", value.id()));
        crate::PlatformConfigurationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn platform_locale<'a>(value: &'a crate::PlatformLocale) -> crate::PlatformLocaleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PlatformLocale(id={})", value.id()));
        crate::PlatformLocaleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn merchant_branch<'a>(value: &'a crate::MerchantBranch) -> crate::MerchantBranchExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MerchantBranch(id={})", value.id()));
        crate::MerchantBranchExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn merchant_setting<'a>(value: &'a crate::MerchantSetting) -> crate::MerchantSettingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MerchantSetting(id={})", value.id()));
        crate::MerchantSettingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn operational_exception<'a>(value: &'a crate::OperationalException) -> crate::OperationalExceptionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OperationalException(id={})", value.id()));
        crate::OperationalExceptionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn crew_member_assignment<'a>(value: &'a crate::CrewMemberAssignment) -> crate::CrewMemberAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CrewMemberAssignment(id={})", value.id()));
        crate::CrewMemberAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn pickup_instruction<'a>(value: &'a crate::PickupInstruction) -> crate::PickupInstructionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PickupInstruction(id={})", value.id()));
        crate::PickupInstructionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn delivery_instruction<'a>(value: &'a crate::DeliveryInstruction) -> crate::DeliveryInstructionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DeliveryInstruction(id={})", value.id()));
        crate::DeliveryInstructionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_inventory<'a>(value: &'a crate::MoveInventory) -> crate::MoveInventoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveInventory(id={})", value.id()));
        crate::MoveInventoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_operations_logistics_1<'a>(value: &'a crate::ExtraOperationsLogistics1) -> crate::ExtraOperationsLogistics1Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraOperationsLogistics1(id={})", value.id()));
        crate::ExtraOperationsLogistics1Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_operations_logistics_2<'a>(value: &'a crate::ExtraOperationsLogistics2) -> crate::ExtraOperationsLogistics2Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraOperationsLogistics2(id={})", value.id()));
        crate::ExtraOperationsLogistics2Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_operations_logistics_3<'a>(value: &'a crate::ExtraOperationsLogistics3) -> crate::ExtraOperationsLogistics3Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraOperationsLogistics3(id={})", value.id()));
        crate::ExtraOperationsLogistics3Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_operations_logistics_4<'a>(value: &'a crate::ExtraOperationsLogistics4) -> crate::ExtraOperationsLogistics4Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraOperationsLogistics4(id={})", value.id()));
        crate::ExtraOperationsLogistics4Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_operations_logistics_5<'a>(value: &'a crate::ExtraOperationsLogistics5) -> crate::ExtraOperationsLogistics5Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraOperationsLogistics5(id={})", value.id()));
        crate::ExtraOperationsLogistics5Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_operations_logistics_6<'a>(value: &'a crate::ExtraOperationsLogistics6) -> crate::ExtraOperationsLogistics6Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraOperationsLogistics6(id={})", value.id()));
        crate::ExtraOperationsLogistics6Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_operations_logistics_7<'a>(value: &'a crate::ExtraOperationsLogistics7) -> crate::ExtraOperationsLogistics7Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraOperationsLogistics7(id={})", value.id()));
        crate::ExtraOperationsLogistics7Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_operations_logistics_8<'a>(value: &'a crate::ExtraOperationsLogistics8) -> crate::ExtraOperationsLogistics8Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraOperationsLogistics8(id={})", value.id()));
        crate::ExtraOperationsLogistics8Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_operations_logistics_9<'a>(value: &'a crate::ExtraOperationsLogistics9) -> crate::ExtraOperationsLogistics9Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraOperationsLogistics9(id={})", value.id()));
        crate::ExtraOperationsLogistics9Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn employee_availability<'a>(value: &'a crate::EmployeeAvailability) -> crate::EmployeeAvailabilityExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EmployeeAvailability(id={})", value.id()));
        crate::EmployeeAvailabilityExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payroll_deduction<'a>(value: &'a crate::PayrollDeduction) -> crate::PayrollDeductionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PayrollDeduction(id={})", value.id()));
        crate::PayrollDeductionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn training_session<'a>(value: &'a crate::TrainingSession) -> crate::TrainingSessionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TrainingSession(id={})", value.id()));
        crate::TrainingSessionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn shift_assignment<'a>(value: &'a crate::ShiftAssignment) -> crate::ShiftAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ShiftAssignment(id={})", value.id()));
        crate::ShiftAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_employees_payroll_1<'a>(value: &'a crate::ExtraEmployeesPayroll1) -> crate::ExtraEmployeesPayroll1Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraEmployeesPayroll1(id={})", value.id()));
        crate::ExtraEmployeesPayroll1Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_employees_payroll_2<'a>(value: &'a crate::ExtraEmployeesPayroll2) -> crate::ExtraEmployeesPayroll2Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraEmployeesPayroll2(id={})", value.id()));
        crate::ExtraEmployeesPayroll2Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_employees_payroll_3<'a>(value: &'a crate::ExtraEmployeesPayroll3) -> crate::ExtraEmployeesPayroll3Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraEmployeesPayroll3(id={})", value.id()));
        crate::ExtraEmployeesPayroll3Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_employees_payroll_4<'a>(value: &'a crate::ExtraEmployeesPayroll4) -> crate::ExtraEmployeesPayroll4Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraEmployeesPayroll4(id={})", value.id()));
        crate::ExtraEmployeesPayroll4Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_employees_payroll_5<'a>(value: &'a crate::ExtraEmployeesPayroll5) -> crate::ExtraEmployeesPayroll5Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraEmployeesPayroll5(id={})", value.id()));
        crate::ExtraEmployeesPayroll5Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_employees_payroll_6<'a>(value: &'a crate::ExtraEmployeesPayroll6) -> crate::ExtraEmployeesPayroll6Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraEmployeesPayroll6(id={})", value.id()));
        crate::ExtraEmployeesPayroll6Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_employees_payroll_7<'a>(value: &'a crate::ExtraEmployeesPayroll7) -> crate::ExtraEmployeesPayroll7Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraEmployeesPayroll7(id={})", value.id()));
        crate::ExtraEmployeesPayroll7Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_complaint<'a>(value: &'a crate::CustomerComplaint) -> crate::CustomerComplaintExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerComplaint(id={})", value.id()));
        crate::CustomerComplaintExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn customer_note<'a>(value: &'a crate::CustomerNote) -> crate::CustomerNoteExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerNote(id={})", value.id()));
        crate::CustomerNoteExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_customer_management_1<'a>(value: &'a crate::ExtraCustomerManagement1) -> crate::ExtraCustomerManagement1Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraCustomerManagement1(id={})", value.id()));
        crate::ExtraCustomerManagement1Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_customer_management_2<'a>(value: &'a crate::ExtraCustomerManagement2) -> crate::ExtraCustomerManagement2Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraCustomerManagement2(id={})", value.id()));
        crate::ExtraCustomerManagement2Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_customer_management_3<'a>(value: &'a crate::ExtraCustomerManagement3) -> crate::ExtraCustomerManagement3Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraCustomerManagement3(id={})", value.id()));
        crate::ExtraCustomerManagement3Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_customer_management_4<'a>(value: &'a crate::ExtraCustomerManagement4) -> crate::ExtraCustomerManagement4Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraCustomerManagement4(id={})", value.id()));
        crate::ExtraCustomerManagement4Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_customer_management_5<'a>(value: &'a crate::ExtraCustomerManagement5) -> crate::ExtraCustomerManagement5Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraCustomerManagement5(id={})", value.id()));
        crate::ExtraCustomerManagement5Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_customer_management_6<'a>(value: &'a crate::ExtraCustomerManagement6) -> crate::ExtraCustomerManagement6Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraCustomerManagement6(id={})", value.id()));
        crate::ExtraCustomerManagement6Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn storage_service<'a>(value: &'a crate::StorageService) -> crate::StorageServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("StorageService(id={})", value.id()));
        crate::StorageServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn packing_service<'a>(value: &'a crate::PackingService) -> crate::PackingServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PackingService(id={})", value.id()));
        crate::PackingServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn disposal_service<'a>(value: &'a crate::DisposalService) -> crate::DisposalServiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DisposalService(id={})", value.id()));
        crate::DisposalServiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn rental_period<'a>(value: &'a crate::RentalPeriod) -> crate::RentalPeriodExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RentalPeriod(id={})", value.id()));
        crate::RentalPeriodExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_area<'a>(value: &'a crate::ServiceArea) -> crate::ServiceAreaExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceArea(id={})", value.id()));
        crate::ServiceAreaExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_products_services_1<'a>(value: &'a crate::ExtraProductsServices1) -> crate::ExtraProductsServices1Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraProductsServices1(id={})", value.id()));
        crate::ExtraProductsServices1Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_products_services_2<'a>(value: &'a crate::ExtraProductsServices2) -> crate::ExtraProductsServices2Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraProductsServices2(id={})", value.id()));
        crate::ExtraProductsServices2Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_products_services_3<'a>(value: &'a crate::ExtraProductsServices3) -> crate::ExtraProductsServices3Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraProductsServices3(id={})", value.id()));
        crate::ExtraProductsServices3Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_products_services_4<'a>(value: &'a crate::ExtraProductsServices4) -> crate::ExtraProductsServices4Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraProductsServices4(id={})", value.id()));
        crate::ExtraProductsServices4Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn campaign_audience<'a>(value: &'a crate::CampaignAudience) -> crate::CampaignAudienceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CampaignAudience(id={})", value.id()));
        crate::CampaignAudienceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn campaign_channel<'a>(value: &'a crate::CampaignChannel) -> crate::CampaignChannelExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CampaignChannel(id={})", value.id()));
        crate::CampaignChannelExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn lead_attribution<'a>(value: &'a crate::LeadAttribution) -> crate::LeadAttributionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LeadAttribution(id={})", value.id()));
        crate::LeadAttributionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn sales_funnel<'a>(value: &'a crate::SalesFunnel) -> crate::SalesFunnelExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SalesFunnel(id={})", value.id()));
        crate::SalesFunnelExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_marketing_sales_1<'a>(value: &'a crate::ExtraMarketingSales1) -> crate::ExtraMarketingSales1Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraMarketingSales1(id={})", value.id()));
        crate::ExtraMarketingSales1Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_marketing_sales_2<'a>(value: &'a crate::ExtraMarketingSales2) -> crate::ExtraMarketingSales2Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraMarketingSales2(id={})", value.id()));
        crate::ExtraMarketingSales2Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_marketing_sales_3<'a>(value: &'a crate::ExtraMarketingSales3) -> crate::ExtraMarketingSales3Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraMarketingSales3(id={})", value.id()));
        crate::ExtraMarketingSales3Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_marketing_sales_4<'a>(value: &'a crate::ExtraMarketingSales4) -> crate::ExtraMarketingSales4Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraMarketingSales4(id={})", value.id()));
        crate::ExtraMarketingSales4Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn expense_claim<'a>(value: &'a crate::ExpenseClaim) -> crate::ExpenseClaimExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExpenseClaim(id={})", value.id()));
        crate::ExpenseClaimExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

    pub fn extra_finance_accounting_1<'a>(value: &'a crate::ExtraFinanceAccounting1) -> crate::ExtraFinanceAccounting1Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraFinanceAccounting1(id={})", value.id()));
        crate::ExtraFinanceAccounting1Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_finance_accounting_2<'a>(value: &'a crate::ExtraFinanceAccounting2) -> crate::ExtraFinanceAccounting2Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraFinanceAccounting2(id={})", value.id()));
        crate::ExtraFinanceAccounting2Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_finance_accounting_3<'a>(value: &'a crate::ExtraFinanceAccounting3) -> crate::ExtraFinanceAccounting3Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraFinanceAccounting3(id={})", value.id()));
        crate::ExtraFinanceAccounting3Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_finance_accounting_4<'a>(value: &'a crate::ExtraFinanceAccounting4) -> crate::ExtraFinanceAccounting4Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraFinanceAccounting4(id={})", value.id()));
        crate::ExtraFinanceAccounting4Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_inspection<'a>(value: &'a crate::VehicleInspection) -> crate::VehicleInspectionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleInspection(id={})", value.id()));
        crate::VehicleInspectionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn equipment_checkout<'a>(value: &'a crate::EquipmentCheckout) -> crate::EquipmentCheckoutExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EquipmentCheckout(id={})", value.id()));
        crate::EquipmentCheckoutExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn consumable_reorder<'a>(value: &'a crate::ConsumableReorder) -> crate::ConsumableReorderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ConsumableReorder(id={})", value.id()));
        crate::ConsumableReorderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_asset_management_1<'a>(value: &'a crate::ExtraAssetManagement1) -> crate::ExtraAssetManagement1Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraAssetManagement1(id={})", value.id()));
        crate::ExtraAssetManagement1Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_asset_management_2<'a>(value: &'a crate::ExtraAssetManagement2) -> crate::ExtraAssetManagement2Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraAssetManagement2(id={})", value.id()));
        crate::ExtraAssetManagement2Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_asset_management_3<'a>(value: &'a crate::ExtraAssetManagement3) -> crate::ExtraAssetManagement3Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraAssetManagement3(id={})", value.id()));
        crate::ExtraAssetManagement3Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_asset_management_4<'a>(value: &'a crate::ExtraAssetManagement4) -> crate::ExtraAssetManagement4Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraAssetManagement4(id={})", value.id()));
        crate::ExtraAssetManagement4Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_asset_management_5<'a>(value: &'a crate::ExtraAssetManagement5) -> crate::ExtraAssetManagement5Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraAssetManagement5(id={})", value.id()));
        crate::ExtraAssetManagement5Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn authentication_attempt<'a>(value: &'a crate::AuthenticationAttempt) -> crate::AuthenticationAttemptExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AuthenticationAttempt(id={})", value.id()));
        crate::AuthenticationAttemptExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn access_policy<'a>(value: &'a crate::AccessPolicy) -> crate::AccessPolicyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AccessPolicy(id={})", value.id()));
        crate::AccessPolicyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_identity_access_1<'a>(value: &'a crate::ExtraIdentityAccess1) -> crate::ExtraIdentityAccess1Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraIdentityAccess1(id={})", value.id()));
        crate::ExtraIdentityAccess1Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn audit_export<'a>(value: &'a crate::AuditExport) -> crate::AuditExportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AuditExport(id={})", value.id()));
        crate::AuditExportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_activity_audit_1<'a>(value: &'a crate::ExtraActivityAudit1) -> crate::ExtraActivityAudit1Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraActivityAudit1(id={})", value.id()));
        crate::ExtraActivityAudit1Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn notification_preference<'a>(value: &'a crate::NotificationPreference) -> crate::NotificationPreferenceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("NotificationPreference(id={})", value.id()));
        crate::NotificationPreferenceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn notification_delivery<'a>(value: &'a crate::NotificationDelivery) -> crate::NotificationDeliveryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("NotificationDelivery(id={})", value.id()));
        crate::NotificationDeliveryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn synchronization_run<'a>(value: &'a crate::SynchronizationRun) -> crate::SynchronizationRunExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SynchronizationRun(id={})", value.id()));
        crate::SynchronizationRunExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn extra_api_integrations_1<'a>(value: &'a crate::ExtraApiIntegrations1) -> crate::ExtraApiIntegrations1Expression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExtraApiIntegrations1(id={})", value.id()));
        crate::ExtraApiIntegrations1Expression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn gender_type<'a>(value: &'a crate::GenderType) -> crate::GenderTypeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("GenderType(id={})", value.id()));
        crate::GenderTypeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

