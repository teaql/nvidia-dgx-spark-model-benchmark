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
    pub fn platforms() -> PlatformRequest {
        PlatformRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platforms_minimal() -> PlatformRequest {
        PlatformRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platforms_with_children() -> PlatformRequest {
        PlatformRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn merchants() -> MerchantRequest {
        MerchantRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn merchants_minimal() -> MerchantRequest {
        MerchantRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn merchants_with_children() -> MerchantRequest {
        MerchantRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn employees() -> EmployeeRequest {
        EmployeeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employees_minimal() -> EmployeeRequest {
        EmployeeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employees_with_children() -> EmployeeRequest {
        EmployeeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn platform_settings() -> PlatformSettingRequest {
        PlatformSettingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platform_settings_minimal() -> PlatformSettingRequest {
        PlatformSettingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platform_settings_with_children() -> PlatformSettingRequest {
        PlatformSettingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn platform_users() -> PlatformUserRequest {
        PlatformUserRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platform_users_minimal() -> PlatformUserRequest {
        PlatformUserRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platform_users_with_children() -> PlatformUserRequest {
        PlatformUserRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn platform_audit_logs() -> PlatformAuditLogRequest {
        PlatformAuditLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platform_audit_logs_minimal() -> PlatformAuditLogRequest {
        PlatformAuditLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platform_audit_logs_with_children() -> PlatformAuditLogRequest {
        PlatformAuditLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn organizations() -> OrganizationRequest {
        OrganizationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn organizations_minimal() -> OrganizationRequest {
        OrganizationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn organizations_with_children() -> OrganizationRequest {
        OrganizationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn organization_settings() -> OrganizationSettingRequest {
        OrganizationSettingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn organization_settings_minimal() -> OrganizationSettingRequest {
        OrganizationSettingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn organization_settings_with_children() -> OrganizationSettingRequest {
        OrganizationSettingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn organization_members() -> OrganizationMemberRequest {
        OrganizationMemberRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn organization_members_minimal() -> OrganizationMemberRequest {
        OrganizationMemberRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn organization_members_with_children() -> OrganizationMemberRequest {
        OrganizationMemberRequest::new()
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

    pub fn move_quotes() -> MoveQuoteRequest {
        MoveQuoteRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_quotes_minimal() -> MoveQuoteRequest {
        MoveQuoteRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_quotes_with_children() -> MoveQuoteRequest {
        MoveQuoteRequest::new()
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

    pub fn route_stops() -> RouteStopRequest {
        RouteStopRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn route_stops_minimal() -> RouteStopRequest {
        RouteStopRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn route_stops_with_children() -> RouteStopRequest {
        RouteStopRequest::new()
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

    pub fn fulfillment_events() -> FulfillmentEventRequest {
        FulfillmentEventRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fulfillment_events_minimal() -> FulfillmentEventRequest {
        FulfillmentEventRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fulfillment_events_with_children() -> FulfillmentEventRequest {
        FulfillmentEventRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn addresses() -> AddressRequest {
        AddressRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn addresses_minimal() -> AddressRequest {
        AddressRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn addresses_with_children() -> AddressRequest {
        AddressRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn crews() -> CrewRequest {
        CrewRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn crews_minimal() -> CrewRequest {
        CrewRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn crews_with_children() -> CrewRequest {
        CrewRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn dispatch_assignments() -> DispatchAssignmentRequest {
        DispatchAssignmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dispatch_assignments_minimal() -> DispatchAssignmentRequest {
        DispatchAssignmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dispatch_assignments_with_children() -> DispatchAssignmentRequest {
        DispatchAssignmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn damage_reports() -> DamageReportRequest {
        DamageReportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn damage_reports_minimal() -> DamageReportRequest {
        DamageReportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn damage_reports_with_children() -> DamageReportRequest {
        DamageReportRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn proof_of_deliveries() -> ProofOfDeliveryRequest {
        ProofOfDeliveryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn proof_of_deliveries_minimal() -> ProofOfDeliveryRequest {
        ProofOfDeliveryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn proof_of_deliveries_with_children() -> ProofOfDeliveryRequest {
        ProofOfDeliveryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn inventory_items() -> InventoryItemRequest {
        InventoryItemRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_items_minimal() -> InventoryItemRequest {
        InventoryItemRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_items_with_children() -> InventoryItemRequest {
        InventoryItemRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn packing_lists() -> PackingListRequest {
        PackingListRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packing_lists_minimal() -> PackingListRequest {
        PackingListRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packing_lists_with_children() -> PackingListRequest {
        PackingListRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn packing_items() -> PackingItemRequest {
        PackingItemRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packing_items_minimal() -> PackingItemRequest {
        PackingItemRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packing_items_with_children() -> PackingItemRequest {
        PackingItemRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn loading_plans() -> LoadingPlanRequest {
        LoadingPlanRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loading_plans_minimal() -> LoadingPlanRequest {
        LoadingPlanRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loading_plans_with_children() -> LoadingPlanRequest {
        LoadingPlanRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn unloading_plans() -> UnloadingPlanRequest {
        UnloadingPlanRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn unloading_plans_minimal() -> UnloadingPlanRequest {
        UnloadingPlanRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn unloading_plans_with_children() -> UnloadingPlanRequest {
        UnloadingPlanRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn storage_facilities() -> StorageFacilityRequest {
        StorageFacilityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn storage_facilities_minimal() -> StorageFacilityRequest {
        StorageFacilityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn storage_facilities_with_children() -> StorageFacilityRequest {
        StorageFacilityRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn storage_units() -> StorageUnitRequest {
        StorageUnitRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn storage_units_minimal() -> StorageUnitRequest {
        StorageUnitRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn storage_units_with_children() -> StorageUnitRequest {
        StorageUnitRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn storage_inventory() -> StorageInventoryRequest {
        StorageInventoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn storage_inventory_minimal() -> StorageInventoryRequest {
        StorageInventoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn storage_inventory_with_children() -> StorageInventoryRequest {
        StorageInventoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn transport_manifests() -> TransportManifestRequest {
        TransportManifestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn transport_manifests_minimal() -> TransportManifestRequest {
        TransportManifestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn transport_manifests_with_children() -> TransportManifestRequest {
        TransportManifestRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customs_declarations() -> CustomsDeclarationRequest {
        CustomsDeclarationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customs_declarations_minimal() -> CustomsDeclarationRequest {
        CustomsDeclarationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customs_declarations_with_children() -> CustomsDeclarationRequest {
        CustomsDeclarationRequest::new()
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

    pub fn fuel_logs() -> FuelLogRequest {
        FuelLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fuel_logs_minimal() -> FuelLogRequest {
        FuelLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fuel_logs_with_children() -> FuelLogRequest {
        FuelLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn maintenance_requests() -> MaintenanceRequestRequest {
        MaintenanceRequestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenance_requests_minimal() -> MaintenanceRequestRequest {
        MaintenanceRequestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenance_requests_with_children() -> MaintenanceRequestRequest {
        MaintenanceRequestRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn departments() -> DepartmentRequest {
        DepartmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn departments_minimal() -> DepartmentRequest {
        DepartmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn departments_with_children() -> DepartmentRequest {
        DepartmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn job_assignments() -> JobAssignmentRequest {
        JobAssignmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn job_assignments_minimal() -> JobAssignmentRequest {
        JobAssignmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn job_assignments_with_children() -> JobAssignmentRequest {
        JobAssignmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn work_shifts() -> WorkShiftRequest {
        WorkShiftRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn work_shifts_minimal() -> WorkShiftRequest {
        WorkShiftRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn work_shifts_with_children() -> WorkShiftRequest {
        WorkShiftRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn worked_hourses() -> WorkedHoursRequest {
        WorkedHoursRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn worked_hourses_minimal() -> WorkedHoursRequest {
        WorkedHoursRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn worked_hourses_with_children() -> WorkedHoursRequest {
        WorkedHoursRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payroll_periods() -> PayrollPeriodRequest {
        PayrollPeriodRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_periods_minimal() -> PayrollPeriodRequest {
        PayrollPeriodRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_periods_with_children() -> PayrollPeriodRequest {
        PayrollPeriodRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payroll_calculations() -> PayrollCalculationRequest {
        PayrollCalculationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_calculations_minimal() -> PayrollCalculationRequest {
        PayrollCalculationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_calculations_with_children() -> PayrollCalculationRequest {
        PayrollCalculationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payslips() -> PayslipRequest {
        PayslipRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payslips_minimal() -> PayslipRequest {
        PayslipRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payslips_with_children() -> PayslipRequest {
        PayslipRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn bonuses() -> BonusRequest {
        BonusRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn bonuses_minimal() -> BonusRequest {
        BonusRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn bonuses_with_children() -> BonusRequest {
        BonusRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn employee_certifications() -> EmployeeCertificationRequest {
        EmployeeCertificationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employee_certifications_minimal() -> EmployeeCertificationRequest {
        EmployeeCertificationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employee_certifications_with_children() -> EmployeeCertificationRequest {
        EmployeeCertificationRequest::new()
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

    pub fn billing_profiles() -> BillingProfileRequest {
        BillingProfileRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_profiles_minimal() -> BillingProfileRequest {
        BillingProfileRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn billing_profiles_with_children() -> BillingProfileRequest {
        BillingProfileRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn corporate_customer_profiles() -> CorporateCustomerProfileRequest {
        CorporateCustomerProfileRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn corporate_customer_profiles_minimal() -> CorporateCustomerProfileRequest {
        CorporateCustomerProfileRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn corporate_customer_profiles_with_children() -> CorporateCustomerProfileRequest {
        CorporateCustomerProfileRequest::new()
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

    pub fn customer_consents() -> CustomerConsentRequest {
        CustomerConsentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_consents_minimal() -> CustomerConsentRequest {
        CustomerConsentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_consents_with_children() -> CustomerConsentRequest {
        CustomerConsentRequest::new()
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

    pub fn customer_histories() -> CustomerHistoryRequest {
        CustomerHistoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_histories_minimal() -> CustomerHistoryRequest {
        CustomerHistoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_histories_with_children() -> CustomerHistoryRequest {
        CustomerHistoryRequest::new()
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

    pub fn private_customer_profiles() -> PrivateCustomerProfileRequest {
        PrivateCustomerProfileRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn private_customer_profiles_minimal() -> PrivateCustomerProfileRequest {
        PrivateCustomerProfileRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn private_customer_profiles_with_children() -> PrivateCustomerProfileRequest {
        PrivateCustomerProfileRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn box_rentals() -> BoxRentalRequest {
        BoxRentalRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn box_rentals_minimal() -> BoxRentalRequest {
        BoxRentalRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn box_rentals_with_children() -> BoxRentalRequest {
        BoxRentalRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn cleaning_services() -> CleaningServiceRequest {
        CleaningServiceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cleaning_services_minimal() -> CleaningServiceRequest {
        CleaningServiceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cleaning_services_with_children() -> CleaningServiceRequest {
        CleaningServiceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn moving_services() -> MovingServiceRequest {
        MovingServiceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn moving_services_minimal() -> MovingServiceRequest {
        MovingServiceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn moving_services_with_children() -> MovingServiceRequest {
        MovingServiceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn price_lists() -> PriceListRequest {
        PriceListRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn price_lists_minimal() -> PriceListRequest {
        PriceListRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn price_lists_with_children() -> PriceListRequest {
        PriceListRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn products() -> ProductRequest {
        ProductRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn products_minimal() -> ProductRequest {
        ProductRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn products_with_children() -> ProductRequest {
        ProductRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn services() -> ServiceRequest {
        ServiceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn services_minimal() -> ServiceRequest {
        ServiceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn services_with_children() -> ServiceRequest {
        ServiceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn service_bundles() -> ServiceBundleRequest {
        ServiceBundleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_bundles_minimal() -> ServiceBundleRequest {
        ServiceBundleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_bundles_with_children() -> ServiceBundleRequest {
        ServiceBundleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn service_configurations() -> ServiceConfigurationRequest {
        ServiceConfigurationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_configurations_minimal() -> ServiceConfigurationRequest {
        ServiceConfigurationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_configurations_with_children() -> ServiceConfigurationRequest {
        ServiceConfigurationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn service_prices() -> ServicePriceRequest {
        ServicePriceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_prices_minimal() -> ServicePriceRequest {
        ServicePriceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_prices_with_children() -> ServicePriceRequest {
        ServicePriceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn campaigns() -> CampaignRequest {
        CampaignRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn campaigns_minimal() -> CampaignRequest {
        CampaignRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn campaigns_with_children() -> CampaignRequest {
        CampaignRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn conversion_events() -> ConversionEventRequest {
        ConversionEventRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn conversion_events_minimal() -> ConversionEventRequest {
        ConversionEventRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn conversion_events_with_children() -> ConversionEventRequest {
        ConversionEventRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn conversion_metrics() -> ConversionMetricRequest {
        ConversionMetricRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn conversion_metrics_minimal() -> ConversionMetricRequest {
        ConversionMetricRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn conversion_metrics_with_children() -> ConversionMetricRequest {
        ConversionMetricRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn discount_codes() -> DiscountCodeRequest {
        DiscountCodeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn discount_codes_minimal() -> DiscountCodeRequest {
        DiscountCodeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn discount_codes_with_children() -> DiscountCodeRequest {
        DiscountCodeRequest::new()
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

    pub fn lead_activities() -> LeadActivityRequest {
        LeadActivityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn lead_activities_minimal() -> LeadActivityRequest {
        LeadActivityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn lead_activities_with_children() -> LeadActivityRequest {
        LeadActivityRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn sales_opportunities() -> SalesOpportunityRequest {
        SalesOpportunityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sales_opportunities_minimal() -> SalesOpportunityRequest {
        SalesOpportunityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sales_opportunities_with_children() -> SalesOpportunityRequest {
        SalesOpportunityRequest::new()
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

    pub fn financial_summaries() -> FinancialSummaryRequest {
        FinancialSummaryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn financial_summaries_minimal() -> FinancialSummaryRequest {
        FinancialSummaryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn financial_summaries_with_children() -> FinancialSummaryRequest {
        FinancialSummaryRequest::new()
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

    pub fn invoice_lines() -> InvoiceLineRequest {
        InvoiceLineRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoice_lines_minimal() -> InvoiceLineRequest {
        InvoiceLineRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoice_lines_with_children() -> InvoiceLineRequest {
        InvoiceLineRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn journal_entries() -> JournalEntryRequest {
        JournalEntryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn journal_entries_minimal() -> JournalEntryRequest {
        JournalEntryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn journal_entries_with_children() -> JournalEntryRequest {
        JournalEntryRequest::new()
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

    pub fn refunds() -> RefundRequest {
        RefundRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn refunds_minimal() -> RefundRequest {
        RefundRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn refunds_with_children() -> RefundRequest {
        RefundRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vat_rates() -> VatRateRequest {
        VatRateRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vat_rates_minimal() -> VatRateRequest {
        VatRateRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vat_rates_with_children() -> VatRateRequest {
        VatRateRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn asset_assignments() -> AssetAssignmentRequest {
        AssetAssignmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn asset_assignments_minimal() -> AssetAssignmentRequest {
        AssetAssignmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn asset_assignments_with_children() -> AssetAssignmentRequest {
        AssetAssignmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn asset_inspections() -> AssetInspectionRequest {
        AssetInspectionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn asset_inspections_minimal() -> AssetInspectionRequest {
        AssetInspectionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn asset_inspections_with_children() -> AssetInspectionRequest {
        AssetInspectionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn consumables() -> ConsumableRequest {
        ConsumableRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn consumables_minimal() -> ConsumableRequest {
        ConsumableRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn consumables_with_children() -> ConsumableRequest {
        ConsumableRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn equipment() -> EquipmentRequest {
        EquipmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_minimal() -> EquipmentRequest {
        EquipmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_with_children() -> EquipmentRequest {
        EquipmentRequest::new()
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

    pub fn maintenance_events() -> MaintenanceEventRequest {
        MaintenanceEventRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenance_events_minimal() -> MaintenanceEventRequest {
        MaintenanceEventRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenance_events_with_children() -> MaintenanceEventRequest {
        MaintenanceEventRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn maintenance_schedules() -> MaintenanceScheduleRequest {
        MaintenanceScheduleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenance_schedules_minimal() -> MaintenanceScheduleRequest {
        MaintenanceScheduleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenance_schedules_with_children() -> MaintenanceScheduleRequest {
        MaintenanceScheduleRequest::new()
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

    pub fn data_retention_policies() -> DataRetentionPolicyRequest {
        DataRetentionPolicyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn data_retention_policies_minimal() -> DataRetentionPolicyRequest {
        DataRetentionPolicyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn data_retention_policies_with_children() -> DataRetentionPolicyRequest {
        DataRetentionPolicyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn documents() -> DocumentRequest {
        DocumentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn documents_minimal() -> DocumentRequest {
        DocumentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn documents_with_children() -> DocumentRequest {
        DocumentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn document_versions() -> DocumentVersionRequest {
        DocumentVersionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn document_versions_minimal() -> DocumentVersionRequest {
        DocumentVersionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn document_versions_with_children() -> DocumentVersionRequest {
        DocumentVersionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn insurance_claims() -> InsuranceClaimRequest {
        InsuranceClaimRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn insurance_claims_minimal() -> InsuranceClaimRequest {
        InsuranceClaimRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn insurance_claims_with_children() -> InsuranceClaimRequest {
        InsuranceClaimRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn insurance_policies() -> InsurancePolicyRequest {
        InsurancePolicyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn insurance_policies_minimal() -> InsurancePolicyRequest {
        InsurancePolicyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn insurance_policies_with_children() -> InsurancePolicyRequest {
        InsurancePolicyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn recovery_requests() -> RecoveryRequestRequest {
        RecoveryRequestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn recovery_requests_minimal() -> RecoveryRequestRequest {
        RecoveryRequestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn recovery_requests_with_children() -> RecoveryRequestRequest {
        RecoveryRequestRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn magic_links() -> MagicLinkRequest {
        MagicLinkRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn magic_links_minimal() -> MagicLinkRequest {
        MagicLinkRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn magic_links_with_children() -> MagicLinkRequest {
        MagicLinkRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn permissions() -> PermissionRequest {
        PermissionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn permissions_minimal() -> PermissionRequest {
        PermissionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn permissions_with_children() -> PermissionRequest {
        PermissionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn roles() -> RoleRequest {
        RoleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn roles_minimal() -> RoleRequest {
        RoleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn roles_with_children() -> RoleRequest {
        RoleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn role_permissions() -> RolePermissionRequest {
        RolePermissionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn role_permissions_minimal() -> RolePermissionRequest {
        RolePermissionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn role_permissions_with_children() -> RolePermissionRequest {
        RolePermissionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn user_accounts() -> UserAccountRequest {
        UserAccountRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn user_accounts_minimal() -> UserAccountRequest {
        UserAccountRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn user_accounts_with_children() -> UserAccountRequest {
        UserAccountRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn user_role_assignments() -> UserRoleAssignmentRequest {
        UserRoleAssignmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn user_role_assignments_minimal() -> UserRoleAssignmentRequest {
        UserRoleAssignmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn user_role_assignments_with_children() -> UserRoleAssignmentRequest {
        UserRoleAssignmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn user_sessions() -> UserSessionRequest {
        UserSessionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn user_sessions_minimal() -> UserSessionRequest {
        UserSessionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn user_sessions_with_children() -> UserSessionRequest {
        UserSessionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn activity_logs() -> ActivityLogRequest {
        ActivityLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn activity_logs_minimal() -> ActivityLogRequest {
        ActivityLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn activity_logs_with_children() -> ActivityLogRequest {
        ActivityLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn audit_logs() -> AuditLogRequest {
        AuditLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audit_logs_minimal() -> AuditLogRequest {
        AuditLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audit_logs_with_children() -> AuditLogRequest {
        AuditLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn change_sets() -> ChangeSetRequest {
        ChangeSetRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn change_sets_minimal() -> ChangeSetRequest {
        ChangeSetRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn change_sets_with_children() -> ChangeSetRequest {
        ChangeSetRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn entity_changes() -> EntityChangeRequest {
        EntityChangeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn entity_changes_minimal() -> EntityChangeRequest {
        EntityChangeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn entity_changes_with_children() -> EntityChangeRequest {
        EntityChangeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn automation_actions() -> AutomationActionRequest {
        AutomationActionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn automation_actions_minimal() -> AutomationActionRequest {
        AutomationActionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn automation_actions_with_children() -> AutomationActionRequest {
        AutomationActionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn automation_rules() -> AutomationRuleRequest {
        AutomationRuleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn automation_rules_minimal() -> AutomationRuleRequest {
        AutomationRuleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn automation_rules_with_children() -> AutomationRuleRequest {
        AutomationRuleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn automation_triggers() -> AutomationTriggerRequest {
        AutomationTriggerRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn automation_triggers_minimal() -> AutomationTriggerRequest {
        AutomationTriggerRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn automation_triggers_with_children() -> AutomationTriggerRequest {
        AutomationTriggerRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn notifications() -> NotificationRequest {
        NotificationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notifications_minimal() -> NotificationRequest {
        NotificationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notifications_with_children() -> NotificationRequest {
        NotificationRequest::new()
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

    pub fn api_clients() -> ApiClientRequest {
        ApiClientRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn api_clients_minimal() -> ApiClientRequest {
        ApiClientRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn api_clients_with_children() -> ApiClientRequest {
        ApiClientRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn api_endpoints() -> ApiEndpointRequest {
        ApiEndpointRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn api_endpoints_minimal() -> ApiEndpointRequest {
        ApiEndpointRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn api_endpoints_with_children() -> ApiEndpointRequest {
        ApiEndpointRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn integration_mappings() -> IntegrationMappingRequest {
        IntegrationMappingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn integration_mappings_minimal() -> IntegrationMappingRequest {
        IntegrationMappingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn integration_mappings_with_children() -> IntegrationMappingRequest {
        IntegrationMappingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn webhooks() -> WebhookRequest {
        WebhookRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn webhooks_minimal() -> WebhookRequest {
        WebhookRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn webhooks_with_children() -> WebhookRequest {
        WebhookRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn webhook_deliveries() -> WebhookDeliveryRequest {
        WebhookDeliveryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn webhook_deliveries_minimal() -> WebhookDeliveryRequest {
        WebhookDeliveryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn webhook_deliveries_with_children() -> WebhookDeliveryRequest {
        WebhookDeliveryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn platform_configurations() -> PlatformConfigurationRequest {
        PlatformConfigurationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platform_configurations_minimal() -> PlatformConfigurationRequest {
        PlatformConfigurationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platform_configurations_with_children() -> PlatformConfigurationRequest {
        PlatformConfigurationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn platform_locales() -> PlatformLocaleRequest {
        PlatformLocaleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platform_locales_minimal() -> PlatformLocaleRequest {
        PlatformLocaleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platform_locales_with_children() -> PlatformLocaleRequest {
        PlatformLocaleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn merchant_branches() -> MerchantBranchRequest {
        MerchantBranchRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn merchant_branches_minimal() -> MerchantBranchRequest {
        MerchantBranchRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn merchant_branches_with_children() -> MerchantBranchRequest {
        MerchantBranchRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn merchant_settings() -> MerchantSettingRequest {
        MerchantSettingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn merchant_settings_minimal() -> MerchantSettingRequest {
        MerchantSettingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn merchant_settings_with_children() -> MerchantSettingRequest {
        MerchantSettingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn operational_exceptions() -> OperationalExceptionRequest {
        OperationalExceptionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn operational_exceptions_minimal() -> OperationalExceptionRequest {
        OperationalExceptionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn operational_exceptions_with_children() -> OperationalExceptionRequest {
        OperationalExceptionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn crew_member_assignments() -> CrewMemberAssignmentRequest {
        CrewMemberAssignmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn crew_member_assignments_minimal() -> CrewMemberAssignmentRequest {
        CrewMemberAssignmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn crew_member_assignments_with_children() -> CrewMemberAssignmentRequest {
        CrewMemberAssignmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn pickup_instructions() -> PickupInstructionRequest {
        PickupInstructionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pickup_instructions_minimal() -> PickupInstructionRequest {
        PickupInstructionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pickup_instructions_with_children() -> PickupInstructionRequest {
        PickupInstructionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn delivery_instructions() -> DeliveryInstructionRequest {
        DeliveryInstructionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn delivery_instructions_minimal() -> DeliveryInstructionRequest {
        DeliveryInstructionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn delivery_instructions_with_children() -> DeliveryInstructionRequest {
        DeliveryInstructionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn move_inventory() -> MoveInventoryRequest {
        MoveInventoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_inventory_minimal() -> MoveInventoryRequest {
        MoveInventoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_inventory_with_children() -> MoveInventoryRequest {
        MoveInventoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_operations_logistics_1s() -> ExtraOperationsLogistics1Request {
        ExtraOperationsLogistics1Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_1s_minimal() -> ExtraOperationsLogistics1Request {
        ExtraOperationsLogistics1Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_1s_with_children() -> ExtraOperationsLogistics1Request {
        ExtraOperationsLogistics1Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_operations_logistics_2s() -> ExtraOperationsLogistics2Request {
        ExtraOperationsLogistics2Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_2s_minimal() -> ExtraOperationsLogistics2Request {
        ExtraOperationsLogistics2Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_2s_with_children() -> ExtraOperationsLogistics2Request {
        ExtraOperationsLogistics2Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_operations_logistics_3s() -> ExtraOperationsLogistics3Request {
        ExtraOperationsLogistics3Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_3s_minimal() -> ExtraOperationsLogistics3Request {
        ExtraOperationsLogistics3Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_3s_with_children() -> ExtraOperationsLogistics3Request {
        ExtraOperationsLogistics3Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_operations_logistics_4s() -> ExtraOperationsLogistics4Request {
        ExtraOperationsLogistics4Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_4s_minimal() -> ExtraOperationsLogistics4Request {
        ExtraOperationsLogistics4Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_4s_with_children() -> ExtraOperationsLogistics4Request {
        ExtraOperationsLogistics4Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_operations_logistics_5s() -> ExtraOperationsLogistics5Request {
        ExtraOperationsLogistics5Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_5s_minimal() -> ExtraOperationsLogistics5Request {
        ExtraOperationsLogistics5Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_5s_with_children() -> ExtraOperationsLogistics5Request {
        ExtraOperationsLogistics5Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_operations_logistics_6s() -> ExtraOperationsLogistics6Request {
        ExtraOperationsLogistics6Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_6s_minimal() -> ExtraOperationsLogistics6Request {
        ExtraOperationsLogistics6Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_6s_with_children() -> ExtraOperationsLogistics6Request {
        ExtraOperationsLogistics6Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_operations_logistics_7s() -> ExtraOperationsLogistics7Request {
        ExtraOperationsLogistics7Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_7s_minimal() -> ExtraOperationsLogistics7Request {
        ExtraOperationsLogistics7Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_7s_with_children() -> ExtraOperationsLogistics7Request {
        ExtraOperationsLogistics7Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_operations_logistics_8s() -> ExtraOperationsLogistics8Request {
        ExtraOperationsLogistics8Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_8s_minimal() -> ExtraOperationsLogistics8Request {
        ExtraOperationsLogistics8Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_8s_with_children() -> ExtraOperationsLogistics8Request {
        ExtraOperationsLogistics8Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_operations_logistics_9s() -> ExtraOperationsLogistics9Request {
        ExtraOperationsLogistics9Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_9s_minimal() -> ExtraOperationsLogistics9Request {
        ExtraOperationsLogistics9Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_operations_logistics_9s_with_children() -> ExtraOperationsLogistics9Request {
        ExtraOperationsLogistics9Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn employee_availabilities() -> EmployeeAvailabilityRequest {
        EmployeeAvailabilityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employee_availabilities_minimal() -> EmployeeAvailabilityRequest {
        EmployeeAvailabilityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employee_availabilities_with_children() -> EmployeeAvailabilityRequest {
        EmployeeAvailabilityRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payroll_deductions() -> PayrollDeductionRequest {
        PayrollDeductionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_deductions_minimal() -> PayrollDeductionRequest {
        PayrollDeductionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_deductions_with_children() -> PayrollDeductionRequest {
        PayrollDeductionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn training_sessions() -> TrainingSessionRequest {
        TrainingSessionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn training_sessions_minimal() -> TrainingSessionRequest {
        TrainingSessionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn training_sessions_with_children() -> TrainingSessionRequest {
        TrainingSessionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn shift_assignments() -> ShiftAssignmentRequest {
        ShiftAssignmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shift_assignments_minimal() -> ShiftAssignmentRequest {
        ShiftAssignmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shift_assignments_with_children() -> ShiftAssignmentRequest {
        ShiftAssignmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_employees_payroll_1s() -> ExtraEmployeesPayroll1Request {
        ExtraEmployeesPayroll1Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_1s_minimal() -> ExtraEmployeesPayroll1Request {
        ExtraEmployeesPayroll1Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_1s_with_children() -> ExtraEmployeesPayroll1Request {
        ExtraEmployeesPayroll1Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_employees_payroll_2s() -> ExtraEmployeesPayroll2Request {
        ExtraEmployeesPayroll2Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_2s_minimal() -> ExtraEmployeesPayroll2Request {
        ExtraEmployeesPayroll2Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_2s_with_children() -> ExtraEmployeesPayroll2Request {
        ExtraEmployeesPayroll2Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_employees_payroll_3s() -> ExtraEmployeesPayroll3Request {
        ExtraEmployeesPayroll3Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_3s_minimal() -> ExtraEmployeesPayroll3Request {
        ExtraEmployeesPayroll3Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_3s_with_children() -> ExtraEmployeesPayroll3Request {
        ExtraEmployeesPayroll3Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_employees_payroll_4s() -> ExtraEmployeesPayroll4Request {
        ExtraEmployeesPayroll4Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_4s_minimal() -> ExtraEmployeesPayroll4Request {
        ExtraEmployeesPayroll4Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_4s_with_children() -> ExtraEmployeesPayroll4Request {
        ExtraEmployeesPayroll4Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_employees_payroll_5s() -> ExtraEmployeesPayroll5Request {
        ExtraEmployeesPayroll5Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_5s_minimal() -> ExtraEmployeesPayroll5Request {
        ExtraEmployeesPayroll5Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_5s_with_children() -> ExtraEmployeesPayroll5Request {
        ExtraEmployeesPayroll5Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_employees_payroll_6s() -> ExtraEmployeesPayroll6Request {
        ExtraEmployeesPayroll6Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_6s_minimal() -> ExtraEmployeesPayroll6Request {
        ExtraEmployeesPayroll6Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_6s_with_children() -> ExtraEmployeesPayroll6Request {
        ExtraEmployeesPayroll6Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_employees_payroll_7s() -> ExtraEmployeesPayroll7Request {
        ExtraEmployeesPayroll7Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_7s_minimal() -> ExtraEmployeesPayroll7Request {
        ExtraEmployeesPayroll7Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_employees_payroll_7s_with_children() -> ExtraEmployeesPayroll7Request {
        ExtraEmployeesPayroll7Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_complaints() -> CustomerComplaintRequest {
        CustomerComplaintRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_complaints_minimal() -> CustomerComplaintRequest {
        CustomerComplaintRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_complaints_with_children() -> CustomerComplaintRequest {
        CustomerComplaintRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_notes() -> CustomerNoteRequest {
        CustomerNoteRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_notes_minimal() -> CustomerNoteRequest {
        CustomerNoteRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_notes_with_children() -> CustomerNoteRequest {
        CustomerNoteRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_customer_management_1s() -> ExtraCustomerManagement1Request {
        ExtraCustomerManagement1Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_customer_management_1s_minimal() -> ExtraCustomerManagement1Request {
        ExtraCustomerManagement1Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_customer_management_1s_with_children() -> ExtraCustomerManagement1Request {
        ExtraCustomerManagement1Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_customer_management_2s() -> ExtraCustomerManagement2Request {
        ExtraCustomerManagement2Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_customer_management_2s_minimal() -> ExtraCustomerManagement2Request {
        ExtraCustomerManagement2Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_customer_management_2s_with_children() -> ExtraCustomerManagement2Request {
        ExtraCustomerManagement2Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_customer_management_3s() -> ExtraCustomerManagement3Request {
        ExtraCustomerManagement3Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_customer_management_3s_minimal() -> ExtraCustomerManagement3Request {
        ExtraCustomerManagement3Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_customer_management_3s_with_children() -> ExtraCustomerManagement3Request {
        ExtraCustomerManagement3Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_customer_management_4s() -> ExtraCustomerManagement4Request {
        ExtraCustomerManagement4Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_customer_management_4s_minimal() -> ExtraCustomerManagement4Request {
        ExtraCustomerManagement4Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_customer_management_4s_with_children() -> ExtraCustomerManagement4Request {
        ExtraCustomerManagement4Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_customer_management_5s() -> ExtraCustomerManagement5Request {
        ExtraCustomerManagement5Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_customer_management_5s_minimal() -> ExtraCustomerManagement5Request {
        ExtraCustomerManagement5Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_customer_management_5s_with_children() -> ExtraCustomerManagement5Request {
        ExtraCustomerManagement5Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_customer_management_6s() -> ExtraCustomerManagement6Request {
        ExtraCustomerManagement6Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_customer_management_6s_minimal() -> ExtraCustomerManagement6Request {
        ExtraCustomerManagement6Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_customer_management_6s_with_children() -> ExtraCustomerManagement6Request {
        ExtraCustomerManagement6Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn storage_services() -> StorageServiceRequest {
        StorageServiceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn storage_services_minimal() -> StorageServiceRequest {
        StorageServiceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn storage_services_with_children() -> StorageServiceRequest {
        StorageServiceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn packing_services() -> PackingServiceRequest {
        PackingServiceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packing_services_minimal() -> PackingServiceRequest {
        PackingServiceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packing_services_with_children() -> PackingServiceRequest {
        PackingServiceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn disposal_services() -> DisposalServiceRequest {
        DisposalServiceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn disposal_services_minimal() -> DisposalServiceRequest {
        DisposalServiceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn disposal_services_with_children() -> DisposalServiceRequest {
        DisposalServiceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn rental_periods() -> RentalPeriodRequest {
        RentalPeriodRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn rental_periods_minimal() -> RentalPeriodRequest {
        RentalPeriodRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn rental_periods_with_children() -> RentalPeriodRequest {
        RentalPeriodRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn service_areas() -> ServiceAreaRequest {
        ServiceAreaRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_areas_minimal() -> ServiceAreaRequest {
        ServiceAreaRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_areas_with_children() -> ServiceAreaRequest {
        ServiceAreaRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_products_services_1s() -> ExtraProductsServices1Request {
        ExtraProductsServices1Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_products_services_1s_minimal() -> ExtraProductsServices1Request {
        ExtraProductsServices1Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_products_services_1s_with_children() -> ExtraProductsServices1Request {
        ExtraProductsServices1Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_products_services_2s() -> ExtraProductsServices2Request {
        ExtraProductsServices2Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_products_services_2s_minimal() -> ExtraProductsServices2Request {
        ExtraProductsServices2Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_products_services_2s_with_children() -> ExtraProductsServices2Request {
        ExtraProductsServices2Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_products_services_3s() -> ExtraProductsServices3Request {
        ExtraProductsServices3Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_products_services_3s_minimal() -> ExtraProductsServices3Request {
        ExtraProductsServices3Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_products_services_3s_with_children() -> ExtraProductsServices3Request {
        ExtraProductsServices3Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_products_services_4s() -> ExtraProductsServices4Request {
        ExtraProductsServices4Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_products_services_4s_minimal() -> ExtraProductsServices4Request {
        ExtraProductsServices4Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_products_services_4s_with_children() -> ExtraProductsServices4Request {
        ExtraProductsServices4Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn campaign_audiences() -> CampaignAudienceRequest {
        CampaignAudienceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn campaign_audiences_minimal() -> CampaignAudienceRequest {
        CampaignAudienceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn campaign_audiences_with_children() -> CampaignAudienceRequest {
        CampaignAudienceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn campaign_channels() -> CampaignChannelRequest {
        CampaignChannelRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn campaign_channels_minimal() -> CampaignChannelRequest {
        CampaignChannelRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn campaign_channels_with_children() -> CampaignChannelRequest {
        CampaignChannelRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn lead_attributions() -> LeadAttributionRequest {
        LeadAttributionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn lead_attributions_minimal() -> LeadAttributionRequest {
        LeadAttributionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn lead_attributions_with_children() -> LeadAttributionRequest {
        LeadAttributionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn sales_funnels() -> SalesFunnelRequest {
        SalesFunnelRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sales_funnels_minimal() -> SalesFunnelRequest {
        SalesFunnelRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sales_funnels_with_children() -> SalesFunnelRequest {
        SalesFunnelRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_marketing_sales_1s() -> ExtraMarketingSales1Request {
        ExtraMarketingSales1Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_marketing_sales_1s_minimal() -> ExtraMarketingSales1Request {
        ExtraMarketingSales1Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_marketing_sales_1s_with_children() -> ExtraMarketingSales1Request {
        ExtraMarketingSales1Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_marketing_sales_2s() -> ExtraMarketingSales2Request {
        ExtraMarketingSales2Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_marketing_sales_2s_minimal() -> ExtraMarketingSales2Request {
        ExtraMarketingSales2Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_marketing_sales_2s_with_children() -> ExtraMarketingSales2Request {
        ExtraMarketingSales2Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_marketing_sales_3s() -> ExtraMarketingSales3Request {
        ExtraMarketingSales3Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_marketing_sales_3s_minimal() -> ExtraMarketingSales3Request {
        ExtraMarketingSales3Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_marketing_sales_3s_with_children() -> ExtraMarketingSales3Request {
        ExtraMarketingSales3Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_marketing_sales_4s() -> ExtraMarketingSales4Request {
        ExtraMarketingSales4Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_marketing_sales_4s_minimal() -> ExtraMarketingSales4Request {
        ExtraMarketingSales4Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_marketing_sales_4s_with_children() -> ExtraMarketingSales4Request {
        ExtraMarketingSales4Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn expense_claims() -> ExpenseClaimRequest {
        ExpenseClaimRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expense_claims_minimal() -> ExpenseClaimRequest {
        ExpenseClaimRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expense_claims_with_children() -> ExpenseClaimRequest {
        ExpenseClaimRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn settlements() -> SettlementRequest {
        SettlementRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn settlements_minimal() -> SettlementRequest {
        SettlementRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn settlements_with_children() -> SettlementRequest {
        SettlementRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn receivables() -> ReceivableRequest {
        ReceivableRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn receivables_minimal() -> ReceivableRequest {
        ReceivableRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn receivables_with_children() -> ReceivableRequest {
        ReceivableRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payables() -> PayableRequest {
        PayableRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payables_minimal() -> PayableRequest {
        PayableRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payables_with_children() -> PayableRequest {
        PayableRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_finance_accounting_1s() -> ExtraFinanceAccounting1Request {
        ExtraFinanceAccounting1Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_finance_accounting_1s_minimal() -> ExtraFinanceAccounting1Request {
        ExtraFinanceAccounting1Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_finance_accounting_1s_with_children() -> ExtraFinanceAccounting1Request {
        ExtraFinanceAccounting1Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_finance_accounting_2s() -> ExtraFinanceAccounting2Request {
        ExtraFinanceAccounting2Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_finance_accounting_2s_minimal() -> ExtraFinanceAccounting2Request {
        ExtraFinanceAccounting2Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_finance_accounting_2s_with_children() -> ExtraFinanceAccounting2Request {
        ExtraFinanceAccounting2Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_finance_accounting_3s() -> ExtraFinanceAccounting3Request {
        ExtraFinanceAccounting3Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_finance_accounting_3s_minimal() -> ExtraFinanceAccounting3Request {
        ExtraFinanceAccounting3Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_finance_accounting_3s_with_children() -> ExtraFinanceAccounting3Request {
        ExtraFinanceAccounting3Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_finance_accounting_4s() -> ExtraFinanceAccounting4Request {
        ExtraFinanceAccounting4Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_finance_accounting_4s_minimal() -> ExtraFinanceAccounting4Request {
        ExtraFinanceAccounting4Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_finance_accounting_4s_with_children() -> ExtraFinanceAccounting4Request {
        ExtraFinanceAccounting4Request::new()
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

    pub fn equipment_checkouts() -> EquipmentCheckoutRequest {
        EquipmentCheckoutRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_checkouts_minimal() -> EquipmentCheckoutRequest {
        EquipmentCheckoutRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_checkouts_with_children() -> EquipmentCheckoutRequest {
        EquipmentCheckoutRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn consumable_reorders() -> ConsumableReorderRequest {
        ConsumableReorderRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn consumable_reorders_minimal() -> ConsumableReorderRequest {
        ConsumableReorderRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn consumable_reorders_with_children() -> ConsumableReorderRequest {
        ConsumableReorderRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_asset_management_1s() -> ExtraAssetManagement1Request {
        ExtraAssetManagement1Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_asset_management_1s_minimal() -> ExtraAssetManagement1Request {
        ExtraAssetManagement1Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_asset_management_1s_with_children() -> ExtraAssetManagement1Request {
        ExtraAssetManagement1Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_asset_management_2s() -> ExtraAssetManagement2Request {
        ExtraAssetManagement2Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_asset_management_2s_minimal() -> ExtraAssetManagement2Request {
        ExtraAssetManagement2Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_asset_management_2s_with_children() -> ExtraAssetManagement2Request {
        ExtraAssetManagement2Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_asset_management_3s() -> ExtraAssetManagement3Request {
        ExtraAssetManagement3Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_asset_management_3s_minimal() -> ExtraAssetManagement3Request {
        ExtraAssetManagement3Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_asset_management_3s_with_children() -> ExtraAssetManagement3Request {
        ExtraAssetManagement3Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_asset_management_4s() -> ExtraAssetManagement4Request {
        ExtraAssetManagement4Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_asset_management_4s_minimal() -> ExtraAssetManagement4Request {
        ExtraAssetManagement4Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_asset_management_4s_with_children() -> ExtraAssetManagement4Request {
        ExtraAssetManagement4Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_asset_management_5s() -> ExtraAssetManagement5Request {
        ExtraAssetManagement5Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_asset_management_5s_minimal() -> ExtraAssetManagement5Request {
        ExtraAssetManagement5Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_asset_management_5s_with_children() -> ExtraAssetManagement5Request {
        ExtraAssetManagement5Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn authentication_attempts() -> AuthenticationAttemptRequest {
        AuthenticationAttemptRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn authentication_attempts_minimal() -> AuthenticationAttemptRequest {
        AuthenticationAttemptRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn authentication_attempts_with_children() -> AuthenticationAttemptRequest {
        AuthenticationAttemptRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn access_policies() -> AccessPolicyRequest {
        AccessPolicyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn access_policies_minimal() -> AccessPolicyRequest {
        AccessPolicyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn access_policies_with_children() -> AccessPolicyRequest {
        AccessPolicyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_identity_access_1s() -> ExtraIdentityAccess1Request {
        ExtraIdentityAccess1Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_identity_access_1s_minimal() -> ExtraIdentityAccess1Request {
        ExtraIdentityAccess1Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_identity_access_1s_with_children() -> ExtraIdentityAccess1Request {
        ExtraIdentityAccess1Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn audit_exports() -> AuditExportRequest {
        AuditExportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audit_exports_minimal() -> AuditExportRequest {
        AuditExportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audit_exports_with_children() -> AuditExportRequest {
        AuditExportRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_activity_audit_1s() -> ExtraActivityAudit1Request {
        ExtraActivityAudit1Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_activity_audit_1s_minimal() -> ExtraActivityAudit1Request {
        ExtraActivityAudit1Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_activity_audit_1s_with_children() -> ExtraActivityAudit1Request {
        ExtraActivityAudit1Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn notification_preferences() -> NotificationPreferenceRequest {
        NotificationPreferenceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notification_preferences_minimal() -> NotificationPreferenceRequest {
        NotificationPreferenceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notification_preferences_with_children() -> NotificationPreferenceRequest {
        NotificationPreferenceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn notification_deliveries() -> NotificationDeliveryRequest {
        NotificationDeliveryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notification_deliveries_minimal() -> NotificationDeliveryRequest {
        NotificationDeliveryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notification_deliveries_with_children() -> NotificationDeliveryRequest {
        NotificationDeliveryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn synchronization_runs() -> SynchronizationRunRequest {
        SynchronizationRunRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn synchronization_runs_minimal() -> SynchronizationRunRequest {
        SynchronizationRunRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn synchronization_runs_with_children() -> SynchronizationRunRequest {
        SynchronizationRunRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn extra_api_integrations_1s() -> ExtraApiIntegrations1Request {
        ExtraApiIntegrations1Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_api_integrations_1s_minimal() -> ExtraApiIntegrations1Request {
        ExtraApiIntegrations1Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn extra_api_integrations_1s_with_children() -> ExtraApiIntegrations1Request {
        ExtraApiIntegrations1Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn gender_types() -> GenderTypeRequest {
        GenderTypeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn gender_types_minimal() -> GenderTypeRequest {
        GenderTypeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn gender_types_with_children() -> GenderTypeRequest {
        GenderTypeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}