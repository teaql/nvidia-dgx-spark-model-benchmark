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

    pub fn platform_configs() -> PlatformConfigRequest {
        PlatformConfigRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platform_configs_minimal() -> PlatformConfigRequest {
        PlatformConfigRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platform_configs_with_children() -> PlatformConfigRequest {
        PlatformConfigRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tenant_registries() -> TenantRegistryRequest {
        TenantRegistryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tenant_registries_minimal() -> TenantRegistryRequest {
        TenantRegistryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tenant_registries_with_children() -> TenantRegistryRequest {
        TenantRegistryRequest::new()
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

    pub fn branches() -> BranchRequest {
        BranchRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn branches_minimal() -> BranchRequest {
        BranchRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn branches_with_children() -> BranchRequest {
        BranchRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn franchises() -> FranchiseRequest {
        FranchiseRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn franchises_minimal() -> FranchiseRequest {
        FranchiseRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn franchises_with_children() -> FranchiseRequest {
        FranchiseRequest::new()
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

    pub fn vehicle_load_plans() -> VehicleLoadPlanRequest {
        VehicleLoadPlanRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_load_plans_minimal() -> VehicleLoadPlanRequest {
        VehicleLoadPlanRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_load_plans_with_children() -> VehicleLoadPlanRequest {
        VehicleLoadPlanRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn weigh_station_tickets() -> WeighStationTicketRequest {
        WeighStationTicketRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn weigh_station_tickets_minimal() -> WeighStationTicketRequest {
        WeighStationTicketRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn weigh_station_tickets_with_children() -> WeighStationTicketRequest {
        WeighStationTicketRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn toll_receipts() -> TollReceiptRequest {
        TollReceiptRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn toll_receipts_minimal() -> TollReceiptRequest {
        TollReceiptRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn toll_receipts_with_children() -> TollReceiptRequest {
        TollReceiptRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn parking_permits() -> ParkingPermitRequest {
        ParkingPermitRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn parking_permits_minimal() -> ParkingPermitRequest {
        ParkingPermitRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn parking_permits_with_children() -> ParkingPermitRequest {
        ParkingPermitRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn traffic_violations() -> TrafficViolationRequest {
        TrafficViolationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn traffic_violations_minimal() -> TrafficViolationRequest {
        TrafficViolationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn traffic_violations_with_children() -> TrafficViolationRequest {
        TrafficViolationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn detour_logs() -> DetourLogRequest {
        DetourLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn detour_logs_minimal() -> DetourLogRequest {
        DetourLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn detour_logs_with_children() -> DetourLogRequest {
        DetourLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn fuel_stops() -> FuelStopRequest {
        FuelStopRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fuel_stops_minimal() -> FuelStopRequest {
        FuelStopRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fuel_stops_with_children() -> FuelStopRequest {
        FuelStopRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn weather_delays() -> WeatherDelayRequest {
        WeatherDelayRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn weather_delays_minimal() -> WeatherDelayRequest {
        WeatherDelayRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn weather_delays_with_children() -> WeatherDelayRequest {
        WeatherDelayRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customer_signatures() -> CustomerSignatureRequest {
        CustomerSignatureRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_signatures_minimal() -> CustomerSignatureRequest {
        CustomerSignatureRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customer_signatures_with_children() -> CustomerSignatureRequest {
        CustomerSignatureRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn walkthrough_checklists() -> WalkthroughChecklistRequest {
        WalkthroughChecklistRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn walkthrough_checklists_minimal() -> WalkthroughChecklistRequest {
        WalkthroughChecklistRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn walkthrough_checklists_with_children() -> WalkthroughChecklistRequest {
        WalkthroughChecklistRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn post_move_surveys() -> PostMoveSurveyRequest {
        PostMoveSurveyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn post_move_surveys_minimal() -> PostMoveSurveyRequest {
        PostMoveSurveyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn post_move_surveys_with_children() -> PostMoveSurveyRequest {
        PostMoveSurveyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn operations_manager_overrides() -> OperationsManagerOverrideRequest {
        OperationsManagerOverrideRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn operations_manager_overrides_minimal() -> OperationsManagerOverrideRequest {
        OperationsManagerOverrideRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn operations_manager_overrides_with_children() -> OperationsManagerOverrideRequest {
        OperationsManagerOverrideRequest::new()
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

    pub fn direct_deposit_info() -> DirectDepositInfoRequest {
        DirectDepositInfoRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn direct_deposit_info_minimal() -> DirectDepositInfoRequest {
        DirectDepositInfoRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn direct_deposit_info_with_children() -> DirectDepositInfoRequest {
        DirectDepositInfoRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn union_dueses() -> UnionDuesRequest {
        UnionDuesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn union_dueses_minimal() -> UnionDuesRequest {
        UnionDuesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn union_dueses_with_children() -> UnionDuesRequest {
        UnionDuesRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn overtime_approvals() -> OvertimeApprovalRequest {
        OvertimeApprovalRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn overtime_approvals_minimal() -> OvertimeApprovalRequest {
        OvertimeApprovalRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn overtime_approvals_with_children() -> OvertimeApprovalRequest {
        OvertimeApprovalRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn expense_reimbursements() -> ExpenseReimbursementRequest {
        ExpenseReimbursementRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expense_reimbursements_minimal() -> ExpenseReimbursementRequest {
        ExpenseReimbursementRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expense_reimbursements_with_children() -> ExpenseReimbursementRequest {
        ExpenseReimbursementRequest::new()
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

    pub fn warning_letters() -> WarningLetterRequest {
        WarningLetterRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warning_letters_minimal() -> WarningLetterRequest {
        WarningLetterRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warning_letters_with_children() -> WarningLetterRequest {
        WarningLetterRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn termination_records() -> TerminationRecordRequest {
        TerminationRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn termination_records_minimal() -> TerminationRecordRequest {
        TerminationRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn termination_records_with_children() -> TerminationRecordRequest {
        TerminationRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn emergency_contacts() -> EmergencyContactRequest {
        EmergencyContactRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn emergency_contacts_minimal() -> EmergencyContactRequest {
        EmergencyContactRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn emergency_contacts_with_children() -> EmergencyContactRequest {
        EmergencyContactRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn uniform_assignments() -> UniformAssignmentRequest {
        UniformAssignmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn uniform_assignments_minimal() -> UniformAssignmentRequest {
        UniformAssignmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn uniform_assignments_with_children() -> UniformAssignmentRequest {
        UniformAssignmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn background_checks() -> BackgroundCheckRequest {
        BackgroundCheckRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn background_checks_minimal() -> BackgroundCheckRequest {
        BackgroundCheckRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn background_checks_with_children() -> BackgroundCheckRequest {
        BackgroundCheckRequest::new()
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

    pub fn complaint_tickets() -> ComplaintTicketRequest {
        ComplaintTicketRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn complaint_tickets_minimal() -> ComplaintTicketRequest {
        ComplaintTicketRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn complaint_tickets_with_children() -> ComplaintTicketRequest {
        ComplaintTicketRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn resolution_offers() -> ResolutionOfferRequest {
        ResolutionOfferRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn resolution_offers_minimal() -> ResolutionOfferRequest {
        ResolutionOfferRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn resolution_offers_with_children() -> ResolutionOfferRequest {
        ResolutionOfferRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vip_statuses() -> VipStatusRequest {
        VipStatusRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vip_statuses_minimal() -> VipStatusRequest {
        VipStatusRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vip_statuses_with_children() -> VipStatusRequest {
        VipStatusRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn do_not_contact_lists() -> DoNotContactListRequest {
        DoNotContactListRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn do_not_contact_lists_minimal() -> DoNotContactListRequest {
        DoNotContactListRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn do_not_contact_lists_with_children() -> DoNotContactListRequest {
        DoNotContactListRequest::new()
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

    pub fn insurance_addons() -> InsuranceAddonRequest {
        InsuranceAddonRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn insurance_addons_minimal() -> InsuranceAddonRequest {
        InsuranceAddonRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn insurance_addons_with_children() -> InsuranceAddonRequest {
        InsuranceAddonRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn piano_handlings() -> PianoHandlingRequest {
        PianoHandlingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn piano_handlings_minimal() -> PianoHandlingRequest {
        PianoHandlingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn piano_handlings_with_children() -> PianoHandlingRequest {
        PianoHandlingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn stair_fees() -> StairFeeRequest {
        StairFeeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn stair_fees_minimal() -> StairFeeRequest {
        StairFeeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn stair_fees_with_children() -> StairFeeRequest {
        StairFeeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn long_carry_fees() -> LongCarryFeeRequest {
        LongCarryFeeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn long_carry_fees_minimal() -> LongCarryFeeRequest {
        LongCarryFeeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn long_carry_fees_with_children() -> LongCarryFeeRequest {
        LongCarryFeeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn hoisting_services() -> HoistingServiceRequest {
        HoistingServiceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn hoisting_services_minimal() -> HoistingServiceRequest {
        HoistingServiceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn hoisting_services_with_children() -> HoistingServiceRequest {
        HoistingServiceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicle_transports() -> VehicleTransportRequest {
        VehicleTransportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_transports_minimal() -> VehicleTransportRequest {
        VehicleTransportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_transports_with_children() -> VehicleTransportRequest {
        VehicleTransportRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn pet_relocation_services() -> PetRelocationServiceRequest {
        PetRelocationServiceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pet_relocation_services_minimal() -> PetRelocationServiceRequest {
        PetRelocationServiceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pet_relocation_services_with_children() -> PetRelocationServiceRequest {
        PetRelocationServiceRequest::new()
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

    pub fn ad_spends() -> AdSpendRequest {
        AdSpendRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn ad_spends_minimal() -> AdSpendRequest {
        AdSpendRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn ad_spends_with_children() -> AdSpendRequest {
        AdSpendRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn social_media_posts() -> SocialMediaPostRequest {
        SocialMediaPostRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn social_media_posts_minimal() -> SocialMediaPostRequest {
        SocialMediaPostRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn social_media_posts_with_children() -> SocialMediaPostRequest {
        SocialMediaPostRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn email_blasts() -> EmailBlastRequest {
        EmailBlastRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn email_blasts_minimal() -> EmailBlastRequest {
        EmailBlastRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn email_blasts_with_children() -> EmailBlastRequest {
        EmailBlastRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn sms_campaigns() -> SmsCampaignRequest {
        SmsCampaignRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sms_campaigns_minimal() -> SmsCampaignRequest {
        SmsCampaignRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sms_campaigns_with_children() -> SmsCampaignRequest {
        SmsCampaignRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn sales_scripts() -> SalesScriptRequest {
        SalesScriptRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sales_scripts_minimal() -> SalesScriptRequest {
        SalesScriptRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sales_scripts_with_children() -> SalesScriptRequest {
        SalesScriptRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn objection_handling_guides() -> ObjectionHandlingGuideRequest {
        ObjectionHandlingGuideRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn objection_handling_guides_minimal() -> ObjectionHandlingGuideRequest {
        ObjectionHandlingGuideRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn objection_handling_guides_with_children() -> ObjectionHandlingGuideRequest {
        ObjectionHandlingGuideRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn competitor_analyses() -> CompetitorAnalysisRequest {
        CompetitorAnalysisRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn competitor_analyses_minimal() -> CompetitorAnalysisRequest {
        CompetitorAnalysisRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn competitor_analyses_with_children() -> CompetitorAnalysisRequest {
        CompetitorAnalysisRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn sales_territories() -> SalesTerritoryRequest {
        SalesTerritoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sales_territories_minimal() -> SalesTerritoryRequest {
        SalesTerritoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sales_territories_with_children() -> SalesTerritoryRequest {
        SalesTerritoryRequest::new()
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

    pub fn tax_documents() -> TaxDocumentRequest {
        TaxDocumentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_documents_minimal() -> TaxDocumentRequest {
        TaxDocumentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_documents_with_children() -> TaxDocumentRequest {
        TaxDocumentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn bank_transactions() -> BankTransactionRequest {
        BankTransactionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn bank_transactions_minimal() -> BankTransactionRequest {
        BankTransactionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn bank_transactions_with_children() -> BankTransactionRequest {
        BankTransactionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn merchant_fees() -> MerchantFeeRequest {
        MerchantFeeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn merchant_fees_minimal() -> MerchantFeeRequest {
        MerchantFeeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn merchant_fees_with_children() -> MerchantFeeRequest {
        MerchantFeeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn chargeback_records() -> ChargebackRecordRequest {
        ChargebackRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn chargeback_records_minimal() -> ChargebackRecordRequest {
        ChargebackRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn chargeback_records_with_children() -> ChargebackRecordRequest {
        ChargebackRecordRequest::new()
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

    pub fn audit_adjustments() -> AuditAdjustmentRequest {
        AuditAdjustmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audit_adjustments_minimal() -> AuditAdjustmentRequest {
        AuditAdjustmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audit_adjustments_with_children() -> AuditAdjustmentRequest {
        AuditAdjustmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn fiscal_years() -> FiscalYearRequest {
        FiscalYearRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fiscal_years_minimal() -> FiscalYearRequest {
        FiscalYearRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fiscal_years_with_children() -> FiscalYearRequest {
        FiscalYearRequest::new()
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

    pub fn gps_trackers() -> GpsTrackerRequest {
        GpsTrackerRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn gps_trackers_minimal() -> GpsTrackerRequest {
        GpsTrackerRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn gps_trackers_with_children() -> GpsTrackerRequest {
        GpsTrackerRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn dashcam_footages() -> DashcamFootageRequest {
        DashcamFootageRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dashcam_footages_minimal() -> DashcamFootageRequest {
        DashcamFootageRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dashcam_footages_with_children() -> DashcamFootageRequest {
        DashcamFootageRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tire_replacements() -> TireReplacementRequest {
        TireReplacementRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tire_replacements_minimal() -> TireReplacementRequest {
        TireReplacementRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tire_replacements_with_children() -> TireReplacementRequest {
        TireReplacementRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn oil_change_logs() -> OilChangeLogRequest {
        OilChangeLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn oil_change_logs_minimal() -> OilChangeLogRequest {
        OilChangeLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn oil_change_logs_with_children() -> OilChangeLogRequest {
        OilChangeLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn registration_renewals() -> RegistrationRenewalRequest {
        RegistrationRenewalRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn registration_renewals_minimal() -> RegistrationRenewalRequest {
        RegistrationRenewalRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn registration_renewals_with_children() -> RegistrationRenewalRequest {
        RegistrationRenewalRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn insurance_cards() -> InsuranceCardRequest {
        InsuranceCardRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn insurance_cards_minimal() -> InsuranceCardRequest {
        InsuranceCardRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn insurance_cards_with_children() -> InsuranceCardRequest {
        InsuranceCardRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn depreciation_schedules() -> DepreciationScheduleRequest {
        DepreciationScheduleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn depreciation_schedules_minimal() -> DepreciationScheduleRequest {
        DepreciationScheduleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn depreciation_schedules_with_children() -> DepreciationScheduleRequest {
        DepreciationScheduleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn scrap_records() -> ScrapRecordRequest {
        ScrapRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn scrap_records_minimal() -> ScrapRecordRequest {
        ScrapRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn scrap_records_with_children() -> ScrapRecordRequest {
        ScrapRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}