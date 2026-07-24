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

    pub fn tenant_configurations() -> TenantConfigurationRequest {
        TenantConfigurationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tenant_configurations_minimal() -> TenantConfigurationRequest {
        TenantConfigurationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tenant_configurations_with_children() -> TenantConfigurationRequest {
        TenantConfigurationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn organization_units() -> OrganizationUnitRequest {
        OrganizationUnitRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn organization_units_minimal() -> OrganizationUnitRequest {
        OrganizationUnitRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn organization_units_with_children() -> OrganizationUnitRequest {
        OrganizationUnitRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn department_hierarchies() -> DepartmentHierarchyRequest {
        DepartmentHierarchyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn department_hierarchies_minimal() -> DepartmentHierarchyRequest {
        DepartmentHierarchyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn department_hierarchies_with_children() -> DepartmentHierarchyRequest {
        DepartmentHierarchyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn branch_offices() -> BranchOfficeRequest {
        BranchOfficeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn branch_offices_minimal() -> BranchOfficeRequest {
        BranchOfficeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn branch_offices_with_children() -> BranchOfficeRequest {
        BranchOfficeRequest::new()
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

    pub fn move_items() -> MoveItemRequest {
        MoveItemRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_items_minimal() -> MoveItemRequest {
        MoveItemRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_items_with_children() -> MoveItemRequest {
        MoveItemRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn inventory_lists() -> InventoryListRequest {
        InventoryListRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_lists_minimal() -> InventoryListRequest {
        InventoryListRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_lists_with_children() -> InventoryListRequest {
        InventoryListRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn packing_materials() -> PackingMaterialRequest {
        PackingMaterialRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packing_materials_minimal() -> PackingMaterialRequest {
        PackingMaterialRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packing_materials_with_children() -> PackingMaterialRequest {
        PackingMaterialRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn loading_zones() -> LoadingZoneRequest {
        LoadingZoneRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loading_zones_minimal() -> LoadingZoneRequest {
        LoadingZoneRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loading_zones_with_children() -> LoadingZoneRequest {
        LoadingZoneRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn unloading_zones() -> UnloadingZoneRequest {
        UnloadingZoneRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn unloading_zones_minimal() -> UnloadingZoneRequest {
        UnloadingZoneRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn unloading_zones_with_children() -> UnloadingZoneRequest {
        UnloadingZoneRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn transit_logs() -> TransitLogRequest {
        TransitLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn transit_logs_minimal() -> TransitLogRequest {
        TransitLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn transit_logs_with_children() -> TransitLogRequest {
        TransitLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn delay_records() -> DelayRecordRequest {
        DelayRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn delay_records_minimal() -> DelayRecordRequest {
        DelayRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn delay_records_with_children() -> DelayRecordRequest {
        DelayRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn route_optimization_rules() -> RouteOptimizationRuleRequest {
        RouteOptimizationRuleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn route_optimization_rules_minimal() -> RouteOptimizationRuleRequest {
        RouteOptimizationRuleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn route_optimization_rules_with_children() -> RouteOptimizationRuleRequest {
        RouteOptimizationRuleRequest::new()
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

    pub fn cargo_weight_records() -> CargoWeightRecordRequest {
        CargoWeightRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cargo_weight_records_minimal() -> CargoWeightRecordRequest {
        CargoWeightRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cargo_weight_records_with_children() -> CargoWeightRecordRequest {
        CargoWeightRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn special_handling_instructions() -> SpecialHandlingInstructionRequest {
        SpecialHandlingInstructionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn special_handling_instructions_minimal() -> SpecialHandlingInstructionRequest {
        SpecialHandlingInstructionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn special_handling_instructions_with_children() -> SpecialHandlingInstructionRequest {
        SpecialHandlingInstructionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn move_statuses() -> MoveStatusRequest {
        MoveStatusRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_statuses_minimal() -> MoveStatusRequest {
        MoveStatusRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_statuses_with_children() -> MoveStatusRequest {
        MoveStatusRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn delivery_windows() -> DeliveryWindowRequest {
        DeliveryWindowRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn delivery_windows_minimal() -> DeliveryWindowRequest {
        DeliveryWindowRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn delivery_windows_with_children() -> DeliveryWindowRequest {
        DeliveryWindowRequest::new()
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

    pub fn deductions() -> DeductionRequest {
        DeductionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn deductions_minimal() -> DeductionRequest {
        DeductionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn deductions_with_children() -> DeductionRequest {
        DeductionRequest::new()
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

    pub fn training_modules() -> TrainingModuleRequest {
        TrainingModuleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn training_modules_minimal() -> TrainingModuleRequest {
        TrainingModuleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn training_modules_with_children() -> TrainingModuleRequest {
        TrainingModuleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn availability_schedules() -> AvailabilityScheduleRequest {
        AvailabilityScheduleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn availability_schedules_minimal() -> AvailabilityScheduleRequest {
        AvailabilityScheduleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn availability_schedules_with_children() -> AvailabilityScheduleRequest {
        AvailabilityScheduleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn skill_profiles() -> SkillProfileRequest {
        SkillProfileRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn skill_profiles_minimal() -> SkillProfileRequest {
        SkillProfileRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn skill_profiles_with_children() -> SkillProfileRequest {
        SkillProfileRequest::new()
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

    pub fn overtime_records() -> OvertimeRecordRequest {
        OvertimeRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn overtime_records_minimal() -> OvertimeRecordRequest {
        OvertimeRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn overtime_records_with_children() -> OvertimeRecordRequest {
        OvertimeRecordRequest::new()
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

    pub fn benefit_enrollments() -> BenefitEnrollmentRequest {
        BenefitEnrollmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn benefit_enrollments_minimal() -> BenefitEnrollmentRequest {
        BenefitEnrollmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn benefit_enrollments_with_children() -> BenefitEnrollmentRequest {
        BenefitEnrollmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn shift_swap_requests() -> ShiftSwapRequestRequest {
        ShiftSwapRequestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shift_swap_requests_minimal() -> ShiftSwapRequestRequest {
        ShiftSwapRequestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shift_swap_requests_with_children() -> ShiftSwapRequestRequest {
        ShiftSwapRequestRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn attendance_records() -> AttendanceRecordRequest {
        AttendanceRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn attendance_records_minimal() -> AttendanceRecordRequest {
        AttendanceRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn attendance_records_with_children() -> AttendanceRecordRequest {
        AttendanceRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payroll_adjustments() -> PayrollAdjustmentRequest {
        PayrollAdjustmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_adjustments_minimal() -> PayrollAdjustmentRequest {
        PayrollAdjustmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_adjustments_with_children() -> PayrollAdjustmentRequest {
        PayrollAdjustmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn commission_records() -> CommissionRecordRequest {
        CommissionRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn commission_records_minimal() -> CommissionRecordRequest {
        CommissionRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn commission_records_with_children() -> CommissionRecordRequest {
        CommissionRecordRequest::new()
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

    pub fn loyalty_tiers() -> LoyaltyTierRequest {
        LoyaltyTierRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loyalty_tiers_minimal() -> LoyaltyTierRequest {
        LoyaltyTierRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loyalty_tiers_with_children() -> LoyaltyTierRequest {
        LoyaltyTierRequest::new()
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

    pub fn communication_logs() -> CommunicationLogRequest {
        CommunicationLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn communication_logs_minimal() -> CommunicationLogRequest {
        CommunicationLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn communication_logs_with_children() -> CommunicationLogRequest {
        CommunicationLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn service_ratings() -> ServiceRatingRequest {
        ServiceRatingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_ratings_minimal() -> ServiceRatingRequest {
        ServiceRatingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_ratings_with_children() -> ServiceRatingRequest {
        ServiceRatingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn account_statuses() -> AccountStatusRequest {
        AccountStatusRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn account_statuses_minimal() -> AccountStatusRequest {
        AccountStatusRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn account_statuses_with_children() -> AccountStatusRequest {
        AccountStatusRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn contact_methods() -> ContactMethodRequest {
        ContactMethodRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contact_methods_minimal() -> ContactMethodRequest {
        ContactMethodRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contact_methods_with_children() -> ContactMethodRequest {
        ContactMethodRequest::new()
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

    pub fn packing_kits() -> PackingKitRequest {
        PackingKitRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packing_kits_minimal() -> PackingKitRequest {
        PackingKitRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packing_kits_with_children() -> PackingKitRequest {
        PackingKitRequest::new()
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

    pub fn availability_calendars() -> AvailabilityCalendarRequest {
        AvailabilityCalendarRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn availability_calendars_minimal() -> AvailabilityCalendarRequest {
        AvailabilityCalendarRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn availability_calendars_with_children() -> AvailabilityCalendarRequest {
        AvailabilityCalendarRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn service_level_agreements() -> ServiceLevelAgreementRequest {
        ServiceLevelAgreementRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_level_agreements_minimal() -> ServiceLevelAgreementRequest {
        ServiceLevelAgreementRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_level_agreements_with_children() -> ServiceLevelAgreementRequest {
        ServiceLevelAgreementRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn add_on_services() -> AddOnServiceRequest {
        AddOnServiceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn add_on_services_minimal() -> AddOnServiceRequest {
        AddOnServiceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn add_on_services_with_children() -> AddOnServiceRequest {
        AddOnServiceRequest::new()
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

    pub fn service_categories() -> ServiceCategoryRequest {
        ServiceCategoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_categories_minimal() -> ServiceCategoryRequest {
        ServiceCategoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn service_categories_with_children() -> ServiceCategoryRequest {
        ServiceCategoryRequest::new()
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

    pub fn marketing_channels() -> MarketingChannelRequest {
        MarketingChannelRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn marketing_channels_minimal() -> MarketingChannelRequest {
        MarketingChannelRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn marketing_channels_with_children() -> MarketingChannelRequest {
        MarketingChannelRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn audience_segments() -> AudienceSegmentRequest {
        AudienceSegmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audience_segments_minimal() -> AudienceSegmentRequest {
        AudienceSegmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audience_segments_with_children() -> AudienceSegmentRequest {
        AudienceSegmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn promotional_offers() -> PromotionalOfferRequest {
        PromotionalOfferRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn promotional_offers_minimal() -> PromotionalOfferRequest {
        PromotionalOfferRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn promotional_offers_with_children() -> PromotionalOfferRequest {
        PromotionalOfferRequest::new()
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

    pub fn attribution_models() -> AttributionModelRequest {
        AttributionModelRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn attribution_models_minimal() -> AttributionModelRequest {
        AttributionModelRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn attribution_models_with_children() -> AttributionModelRequest {
        AttributionModelRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn lead_scores() -> LeadScoreRequest {
        LeadScoreRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn lead_scores_minimal() -> LeadScoreRequest {
        LeadScoreRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn lead_scores_with_children() -> LeadScoreRequest {
        LeadScoreRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn campaign_budgets() -> CampaignBudgetRequest {
        CampaignBudgetRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn campaign_budgets_minimal() -> CampaignBudgetRequest {
        CampaignBudgetRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn campaign_budgets_with_children() -> CampaignBudgetRequest {
        CampaignBudgetRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn conversion_reports() -> ConversionReportRequest {
        ConversionReportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn conversion_reports_minimal() -> ConversionReportRequest {
        ConversionReportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn conversion_reports_with_children() -> ConversionReportRequest {
        ConversionReportRequest::new()
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

    pub fn budgets() -> BudgetRequest {
        BudgetRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn budgets_minimal() -> BudgetRequest {
        BudgetRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn budgets_with_children() -> BudgetRequest {
        BudgetRequest::new()
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

    pub fn tax_records() -> TaxRecordRequest {
        TaxRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_records_minimal() -> TaxRecordRequest {
        TaxRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_records_with_children() -> TaxRecordRequest {
        TaxRecordRequest::new()
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

    pub fn inventory_stocks() -> InventoryStockRequest {
        InventoryStockRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_stocks_minimal() -> InventoryStockRequest {
        InventoryStockRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_stocks_with_children() -> InventoryStockRequest {
        InventoryStockRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn maintenance_costs() -> MaintenanceCostRequest {
        MaintenanceCostRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenance_costs_minimal() -> MaintenanceCostRequest {
        MaintenanceCostRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenance_costs_with_children() -> MaintenanceCostRequest {
        MaintenanceCostRequest::new()
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

    pub fn equipment_serials() -> EquipmentSerialRequest {
        EquipmentSerialRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_serials_minimal() -> EquipmentSerialRequest {
        EquipmentSerialRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn equipment_serials_with_children() -> EquipmentSerialRequest {
        EquipmentSerialRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn supplier_contracts() -> SupplierContractRequest {
        SupplierContractRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn supplier_contracts_minimal() -> SupplierContractRequest {
        SupplierContractRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn supplier_contracts_with_children() -> SupplierContractRequest {
        SupplierContractRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn asset_conditions() -> AssetConditionRequest {
        AssetConditionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn asset_conditions_minimal() -> AssetConditionRequest {
        AssetConditionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn asset_conditions_with_children() -> AssetConditionRequest {
        AssetConditionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn depreciation_records() -> DepreciationRecordRequest {
        DepreciationRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn depreciation_records_minimal() -> DepreciationRecordRequest {
        DepreciationRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn depreciation_records_with_children() -> DepreciationRecordRequest {
        DepreciationRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn warranty_claims() -> WarrantyClaimRequest {
        WarrantyClaimRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warranty_claims_minimal() -> WarrantyClaimRequest {
        WarrantyClaimRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warranty_claims_with_children() -> WarrantyClaimRequest {
        WarrantyClaimRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn storage_locations() -> StorageLocationRequest {
        StorageLocationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn storage_locations_minimal() -> StorageLocationRequest {
        StorageLocationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn storage_locations_with_children() -> StorageLocationRequest {
        StorageLocationRequest::new()
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

    pub fn policy_documents() -> PolicyDocumentRequest {
        PolicyDocumentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn policy_documents_minimal() -> PolicyDocumentRequest {
        PolicyDocumentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn policy_documents_with_children() -> PolicyDocumentRequest {
        PolicyDocumentRequest::new()
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

    pub fn legal_entities() -> LegalEntityRequest {
        LegalEntityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn legal_entities_minimal() -> LegalEntityRequest {
        LegalEntityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn legal_entities_with_children() -> LegalEntityRequest {
        LegalEntityRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn regulatory_requirements() -> RegulatoryRequirementRequest {
        RegulatoryRequirementRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn regulatory_requirements_minimal() -> RegulatoryRequirementRequest {
        RegulatoryRequirementRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn regulatory_requirements_with_children() -> RegulatoryRequirementRequest {
        RegulatoryRequirementRequest::new()
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

    pub fn access_tokens() -> AccessTokenRequest {
        AccessTokenRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn access_tokens_minimal() -> AccessTokenRequest {
        AccessTokenRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn access_tokens_with_children() -> AccessTokenRequest {
        AccessTokenRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn two_factor_auths() -> TwoFactorAuthRequest {
        TwoFactorAuthRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn two_factor_auths_minimal() -> TwoFactorAuthRequest {
        TwoFactorAuthRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn two_factor_auths_with_children() -> TwoFactorAuthRequest {
        TwoFactorAuthRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn login_attempts() -> LoginAttemptRequest {
        LoginAttemptRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn login_attempts_minimal() -> LoginAttemptRequest {
        LoginAttemptRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn login_attempts_with_children() -> LoginAttemptRequest {
        LoginAttemptRequest::new()
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

    pub fn system_events() -> SystemEventRequest {
        SystemEventRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn system_events_minimal() -> SystemEventRequest {
        SystemEventRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn system_events_with_children() -> SystemEventRequest {
        SystemEventRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn data_exports() -> DataExportRequest {
        DataExportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn data_exports_minimal() -> DataExportRequest {
        DataExportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn data_exports_with_children() -> DataExportRequest {
        DataExportRequest::new()
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

    pub fn operational_hooks() -> OperationalHookRequest {
        OperationalHookRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn operational_hooks_minimal() -> OperationalHookRequest {
        OperationalHookRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn operational_hooks_with_children() -> OperationalHookRequest {
        OperationalHookRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn financial_hooks() -> FinancialHookRequest {
        FinancialHookRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn financial_hooks_minimal() -> FinancialHookRequest {
        FinancialHookRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn financial_hooks_with_children() -> FinancialHookRequest {
        FinancialHookRequest::new()
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

    pub fn api_keys() -> ApiKeyRequest {
        ApiKeyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn api_keys_minimal() -> ApiKeyRequest {
        ApiKeyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn api_keys_with_children() -> ApiKeyRequest {
        ApiKeyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}