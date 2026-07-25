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

    pub fn nda_agreements() -> NdaAgreementRequest {
        NdaAgreementRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn nda_agreements_minimal() -> NdaAgreementRequest {
        NdaAgreementRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn nda_agreements_with_children() -> NdaAgreementRequest {
        NdaAgreementRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn terms_of_services() -> TermsOfServiceRequest {
        TermsOfServiceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn terms_of_services_minimal() -> TermsOfServiceRequest {
        TermsOfServiceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn terms_of_services_with_children() -> TermsOfServiceRequest {
        TermsOfServiceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn privacy_policies() -> PrivacyPolicyRequest {
        PrivacyPolicyRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn privacy_policies_minimal() -> PrivacyPolicyRequest {
        PrivacyPolicyRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn privacy_policies_with_children() -> PrivacyPolicyRequest {
        PrivacyPolicyRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn cookie_consents() -> CookieConsentRequest {
        CookieConsentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cookie_consents_minimal() -> CookieConsentRequest {
        CookieConsentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cookie_consents_with_children() -> CookieConsentRequest {
        CookieConsentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn gdpr_requests() -> GdprRequestRequest {
        GdprRequestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn gdpr_requests_minimal() -> GdprRequestRequest {
        GdprRequestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn gdpr_requests_with_children() -> GdprRequestRequest {
        GdprRequestRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn osha_incidents() -> OshaIncidentRequest {
        OshaIncidentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn osha_incidents_minimal() -> OshaIncidentRequest {
        OshaIncidentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn osha_incidents_with_children() -> OshaIncidentRequest {
        OshaIncidentRequest::new()
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

    pub fn password_resets() -> PasswordResetRequest {
        PasswordResetRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn password_resets_minimal() -> PasswordResetRequest {
        PasswordResetRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn password_resets_with_children() -> PasswordResetRequest {
        PasswordResetRequest::new()
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

    pub fn failed_auth_logs() -> FailedAuthLogRequest {
        FailedAuthLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn failed_auth_logs_minimal() -> FailedAuthLogRequest {
        FailedAuthLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn failed_auth_logs_with_children() -> FailedAuthLogRequest {
        FailedAuthLogRequest::new()
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

    pub fn sms_delivery_receipts() -> SmsDeliveryReceiptRequest {
        SmsDeliveryReceiptRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sms_delivery_receipts_minimal() -> SmsDeliveryReceiptRequest {
        SmsDeliveryReceiptRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sms_delivery_receipts_with_children() -> SmsDeliveryReceiptRequest {
        SmsDeliveryReceiptRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn email_bounce_logs() -> EmailBounceLogRequest {
        EmailBounceLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn email_bounce_logs_minimal() -> EmailBounceLogRequest {
        EmailBounceLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn email_bounce_logs_with_children() -> EmailBounceLogRequest {
        EmailBounceLogRequest::new()
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

    pub fn sync_jobs() -> SyncJobRequest {
        SyncJobRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sync_jobs_minimal() -> SyncJobRequest {
        SyncJobRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sync_jobs_with_children() -> SyncJobRequest {
        SyncJobRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn api_rate_limits() -> ApiRateLimitRequest {
        ApiRateLimitRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn api_rate_limits_minimal() -> ApiRateLimitRequest {
        ApiRateLimitRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn api_rate_limits_with_children() -> ApiRateLimitRequest {
        ApiRateLimitRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_180s() -> CustomEntity180Request {
        CustomEntity180Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_180s_minimal() -> CustomEntity180Request {
        CustomEntity180Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_180s_with_children() -> CustomEntity180Request {
        CustomEntity180Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_181s() -> CustomEntity181Request {
        CustomEntity181Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_181s_minimal() -> CustomEntity181Request {
        CustomEntity181Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_181s_with_children() -> CustomEntity181Request {
        CustomEntity181Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_182s() -> CustomEntity182Request {
        CustomEntity182Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_182s_minimal() -> CustomEntity182Request {
        CustomEntity182Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_182s_with_children() -> CustomEntity182Request {
        CustomEntity182Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_183s() -> CustomEntity183Request {
        CustomEntity183Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_183s_minimal() -> CustomEntity183Request {
        CustomEntity183Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_183s_with_children() -> CustomEntity183Request {
        CustomEntity183Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_184s() -> CustomEntity184Request {
        CustomEntity184Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_184s_minimal() -> CustomEntity184Request {
        CustomEntity184Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_184s_with_children() -> CustomEntity184Request {
        CustomEntity184Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_185s() -> CustomEntity185Request {
        CustomEntity185Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_185s_minimal() -> CustomEntity185Request {
        CustomEntity185Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_185s_with_children() -> CustomEntity185Request {
        CustomEntity185Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_186s() -> CustomEntity186Request {
        CustomEntity186Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_186s_minimal() -> CustomEntity186Request {
        CustomEntity186Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_186s_with_children() -> CustomEntity186Request {
        CustomEntity186Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_187s() -> CustomEntity187Request {
        CustomEntity187Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_187s_minimal() -> CustomEntity187Request {
        CustomEntity187Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_187s_with_children() -> CustomEntity187Request {
        CustomEntity187Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_188s() -> CustomEntity188Request {
        CustomEntity188Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_188s_minimal() -> CustomEntity188Request {
        CustomEntity188Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_188s_with_children() -> CustomEntity188Request {
        CustomEntity188Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_189s() -> CustomEntity189Request {
        CustomEntity189Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_189s_minimal() -> CustomEntity189Request {
        CustomEntity189Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_189s_with_children() -> CustomEntity189Request {
        CustomEntity189Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_190s() -> CustomEntity190Request {
        CustomEntity190Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_190s_minimal() -> CustomEntity190Request {
        CustomEntity190Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_190s_with_children() -> CustomEntity190Request {
        CustomEntity190Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_191s() -> CustomEntity191Request {
        CustomEntity191Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_191s_minimal() -> CustomEntity191Request {
        CustomEntity191Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_191s_with_children() -> CustomEntity191Request {
        CustomEntity191Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_192s() -> CustomEntity192Request {
        CustomEntity192Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_192s_minimal() -> CustomEntity192Request {
        CustomEntity192Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_192s_with_children() -> CustomEntity192Request {
        CustomEntity192Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_193s() -> CustomEntity193Request {
        CustomEntity193Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_193s_minimal() -> CustomEntity193Request {
        CustomEntity193Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_193s_with_children() -> CustomEntity193Request {
        CustomEntity193Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_194s() -> CustomEntity194Request {
        CustomEntity194Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_194s_minimal() -> CustomEntity194Request {
        CustomEntity194Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_194s_with_children() -> CustomEntity194Request {
        CustomEntity194Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_195s() -> CustomEntity195Request {
        CustomEntity195Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_195s_minimal() -> CustomEntity195Request {
        CustomEntity195Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_195s_with_children() -> CustomEntity195Request {
        CustomEntity195Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_196s() -> CustomEntity196Request {
        CustomEntity196Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_196s_minimal() -> CustomEntity196Request {
        CustomEntity196Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_196s_with_children() -> CustomEntity196Request {
        CustomEntity196Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_197s() -> CustomEntity197Request {
        CustomEntity197Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_197s_minimal() -> CustomEntity197Request {
        CustomEntity197Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_197s_with_children() -> CustomEntity197Request {
        CustomEntity197Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_198s() -> CustomEntity198Request {
        CustomEntity198Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_198s_minimal() -> CustomEntity198Request {
        CustomEntity198Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_198s_with_children() -> CustomEntity198Request {
        CustomEntity198Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_199s() -> CustomEntity199Request {
        CustomEntity199Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_199s_minimal() -> CustomEntity199Request {
        CustomEntity199Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_199s_with_children() -> CustomEntity199Request {
        CustomEntity199Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_200s() -> CustomEntity200Request {
        CustomEntity200Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_200s_minimal() -> CustomEntity200Request {
        CustomEntity200Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_200s_with_children() -> CustomEntity200Request {
        CustomEntity200Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_201s() -> CustomEntity201Request {
        CustomEntity201Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_201s_minimal() -> CustomEntity201Request {
        CustomEntity201Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_201s_with_children() -> CustomEntity201Request {
        CustomEntity201Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_202s() -> CustomEntity202Request {
        CustomEntity202Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_202s_minimal() -> CustomEntity202Request {
        CustomEntity202Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_202s_with_children() -> CustomEntity202Request {
        CustomEntity202Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_203s() -> CustomEntity203Request {
        CustomEntity203Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_203s_minimal() -> CustomEntity203Request {
        CustomEntity203Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_203s_with_children() -> CustomEntity203Request {
        CustomEntity203Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_204s() -> CustomEntity204Request {
        CustomEntity204Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_204s_minimal() -> CustomEntity204Request {
        CustomEntity204Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_204s_with_children() -> CustomEntity204Request {
        CustomEntity204Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_205s() -> CustomEntity205Request {
        CustomEntity205Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_205s_minimal() -> CustomEntity205Request {
        CustomEntity205Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_205s_with_children() -> CustomEntity205Request {
        CustomEntity205Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_206s() -> CustomEntity206Request {
        CustomEntity206Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_206s_minimal() -> CustomEntity206Request {
        CustomEntity206Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_206s_with_children() -> CustomEntity206Request {
        CustomEntity206Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_207s() -> CustomEntity207Request {
        CustomEntity207Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_207s_minimal() -> CustomEntity207Request {
        CustomEntity207Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_207s_with_children() -> CustomEntity207Request {
        CustomEntity207Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_208s() -> CustomEntity208Request {
        CustomEntity208Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_208s_minimal() -> CustomEntity208Request {
        CustomEntity208Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_208s_with_children() -> CustomEntity208Request {
        CustomEntity208Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_209s() -> CustomEntity209Request {
        CustomEntity209Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_209s_minimal() -> CustomEntity209Request {
        CustomEntity209Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_209s_with_children() -> CustomEntity209Request {
        CustomEntity209Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_210s() -> CustomEntity210Request {
        CustomEntity210Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_210s_minimal() -> CustomEntity210Request {
        CustomEntity210Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_210s_with_children() -> CustomEntity210Request {
        CustomEntity210Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_211s() -> CustomEntity211Request {
        CustomEntity211Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_211s_minimal() -> CustomEntity211Request {
        CustomEntity211Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_211s_with_children() -> CustomEntity211Request {
        CustomEntity211Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_212s() -> CustomEntity212Request {
        CustomEntity212Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_212s_minimal() -> CustomEntity212Request {
        CustomEntity212Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_212s_with_children() -> CustomEntity212Request {
        CustomEntity212Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_213s() -> CustomEntity213Request {
        CustomEntity213Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_213s_minimal() -> CustomEntity213Request {
        CustomEntity213Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_213s_with_children() -> CustomEntity213Request {
        CustomEntity213Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_214s() -> CustomEntity214Request {
        CustomEntity214Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_214s_minimal() -> CustomEntity214Request {
        CustomEntity214Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_214s_with_children() -> CustomEntity214Request {
        CustomEntity214Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_215s() -> CustomEntity215Request {
        CustomEntity215Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_215s_minimal() -> CustomEntity215Request {
        CustomEntity215Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_215s_with_children() -> CustomEntity215Request {
        CustomEntity215Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_216s() -> CustomEntity216Request {
        CustomEntity216Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_216s_minimal() -> CustomEntity216Request {
        CustomEntity216Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_216s_with_children() -> CustomEntity216Request {
        CustomEntity216Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_217s() -> CustomEntity217Request {
        CustomEntity217Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_217s_minimal() -> CustomEntity217Request {
        CustomEntity217Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_217s_with_children() -> CustomEntity217Request {
        CustomEntity217Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_218s() -> CustomEntity218Request {
        CustomEntity218Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_218s_minimal() -> CustomEntity218Request {
        CustomEntity218Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_218s_with_children() -> CustomEntity218Request {
        CustomEntity218Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_219s() -> CustomEntity219Request {
        CustomEntity219Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_219s_minimal() -> CustomEntity219Request {
        CustomEntity219Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_219s_with_children() -> CustomEntity219Request {
        CustomEntity219Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_220s() -> CustomEntity220Request {
        CustomEntity220Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_220s_minimal() -> CustomEntity220Request {
        CustomEntity220Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_220s_with_children() -> CustomEntity220Request {
        CustomEntity220Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_221s() -> CustomEntity221Request {
        CustomEntity221Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_221s_minimal() -> CustomEntity221Request {
        CustomEntity221Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_221s_with_children() -> CustomEntity221Request {
        CustomEntity221Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_222s() -> CustomEntity222Request {
        CustomEntity222Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_222s_minimal() -> CustomEntity222Request {
        CustomEntity222Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_222s_with_children() -> CustomEntity222Request {
        CustomEntity222Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_223s() -> CustomEntity223Request {
        CustomEntity223Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_223s_minimal() -> CustomEntity223Request {
        CustomEntity223Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_223s_with_children() -> CustomEntity223Request {
        CustomEntity223Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_224s() -> CustomEntity224Request {
        CustomEntity224Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_224s_minimal() -> CustomEntity224Request {
        CustomEntity224Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_224s_with_children() -> CustomEntity224Request {
        CustomEntity224Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_225s() -> CustomEntity225Request {
        CustomEntity225Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_225s_minimal() -> CustomEntity225Request {
        CustomEntity225Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_225s_with_children() -> CustomEntity225Request {
        CustomEntity225Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_226s() -> CustomEntity226Request {
        CustomEntity226Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_226s_minimal() -> CustomEntity226Request {
        CustomEntity226Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_226s_with_children() -> CustomEntity226Request {
        CustomEntity226Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_227s() -> CustomEntity227Request {
        CustomEntity227Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_227s_minimal() -> CustomEntity227Request {
        CustomEntity227Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_227s_with_children() -> CustomEntity227Request {
        CustomEntity227Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_228s() -> CustomEntity228Request {
        CustomEntity228Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_228s_minimal() -> CustomEntity228Request {
        CustomEntity228Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_228s_with_children() -> CustomEntity228Request {
        CustomEntity228Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_229s() -> CustomEntity229Request {
        CustomEntity229Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_229s_minimal() -> CustomEntity229Request {
        CustomEntity229Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_229s_with_children() -> CustomEntity229Request {
        CustomEntity229Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_230s() -> CustomEntity230Request {
        CustomEntity230Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_230s_minimal() -> CustomEntity230Request {
        CustomEntity230Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_230s_with_children() -> CustomEntity230Request {
        CustomEntity230Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_231s() -> CustomEntity231Request {
        CustomEntity231Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_231s_minimal() -> CustomEntity231Request {
        CustomEntity231Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_231s_with_children() -> CustomEntity231Request {
        CustomEntity231Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_232s() -> CustomEntity232Request {
        CustomEntity232Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_232s_minimal() -> CustomEntity232Request {
        CustomEntity232Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_232s_with_children() -> CustomEntity232Request {
        CustomEntity232Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_233s() -> CustomEntity233Request {
        CustomEntity233Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_233s_minimal() -> CustomEntity233Request {
        CustomEntity233Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_233s_with_children() -> CustomEntity233Request {
        CustomEntity233Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_234s() -> CustomEntity234Request {
        CustomEntity234Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_234s_minimal() -> CustomEntity234Request {
        CustomEntity234Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_234s_with_children() -> CustomEntity234Request {
        CustomEntity234Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_235s() -> CustomEntity235Request {
        CustomEntity235Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_235s_minimal() -> CustomEntity235Request {
        CustomEntity235Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_235s_with_children() -> CustomEntity235Request {
        CustomEntity235Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_236s() -> CustomEntity236Request {
        CustomEntity236Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_236s_minimal() -> CustomEntity236Request {
        CustomEntity236Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_236s_with_children() -> CustomEntity236Request {
        CustomEntity236Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_237s() -> CustomEntity237Request {
        CustomEntity237Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_237s_minimal() -> CustomEntity237Request {
        CustomEntity237Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_237s_with_children() -> CustomEntity237Request {
        CustomEntity237Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_238s() -> CustomEntity238Request {
        CustomEntity238Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_238s_minimal() -> CustomEntity238Request {
        CustomEntity238Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_238s_with_children() -> CustomEntity238Request {
        CustomEntity238Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_239s() -> CustomEntity239Request {
        CustomEntity239Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_239s_minimal() -> CustomEntity239Request {
        CustomEntity239Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_239s_with_children() -> CustomEntity239Request {
        CustomEntity239Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_240s() -> CustomEntity240Request {
        CustomEntity240Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_240s_minimal() -> CustomEntity240Request {
        CustomEntity240Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_240s_with_children() -> CustomEntity240Request {
        CustomEntity240Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_241s() -> CustomEntity241Request {
        CustomEntity241Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_241s_minimal() -> CustomEntity241Request {
        CustomEntity241Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_241s_with_children() -> CustomEntity241Request {
        CustomEntity241Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_242s() -> CustomEntity242Request {
        CustomEntity242Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_242s_minimal() -> CustomEntity242Request {
        CustomEntity242Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_242s_with_children() -> CustomEntity242Request {
        CustomEntity242Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_243s() -> CustomEntity243Request {
        CustomEntity243Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_243s_minimal() -> CustomEntity243Request {
        CustomEntity243Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_243s_with_children() -> CustomEntity243Request {
        CustomEntity243Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_244s() -> CustomEntity244Request {
        CustomEntity244Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_244s_minimal() -> CustomEntity244Request {
        CustomEntity244Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_244s_with_children() -> CustomEntity244Request {
        CustomEntity244Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_245s() -> CustomEntity245Request {
        CustomEntity245Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_245s_minimal() -> CustomEntity245Request {
        CustomEntity245Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_245s_with_children() -> CustomEntity245Request {
        CustomEntity245Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_246s() -> CustomEntity246Request {
        CustomEntity246Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_246s_minimal() -> CustomEntity246Request {
        CustomEntity246Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_246s_with_children() -> CustomEntity246Request {
        CustomEntity246Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_247s() -> CustomEntity247Request {
        CustomEntity247Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_247s_minimal() -> CustomEntity247Request {
        CustomEntity247Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_247s_with_children() -> CustomEntity247Request {
        CustomEntity247Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_248s() -> CustomEntity248Request {
        CustomEntity248Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_248s_minimal() -> CustomEntity248Request {
        CustomEntity248Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_248s_with_children() -> CustomEntity248Request {
        CustomEntity248Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_249s() -> CustomEntity249Request {
        CustomEntity249Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_249s_minimal() -> CustomEntity249Request {
        CustomEntity249Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_249s_with_children() -> CustomEntity249Request {
        CustomEntity249Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_250s() -> CustomEntity250Request {
        CustomEntity250Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_250s_minimal() -> CustomEntity250Request {
        CustomEntity250Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_250s_with_children() -> CustomEntity250Request {
        CustomEntity250Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_251s() -> CustomEntity251Request {
        CustomEntity251Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_251s_minimal() -> CustomEntity251Request {
        CustomEntity251Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_251s_with_children() -> CustomEntity251Request {
        CustomEntity251Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_252s() -> CustomEntity252Request {
        CustomEntity252Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_252s_minimal() -> CustomEntity252Request {
        CustomEntity252Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_252s_with_children() -> CustomEntity252Request {
        CustomEntity252Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_253s() -> CustomEntity253Request {
        CustomEntity253Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_253s_minimal() -> CustomEntity253Request {
        CustomEntity253Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_253s_with_children() -> CustomEntity253Request {
        CustomEntity253Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_254s() -> CustomEntity254Request {
        CustomEntity254Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_254s_minimal() -> CustomEntity254Request {
        CustomEntity254Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_254s_with_children() -> CustomEntity254Request {
        CustomEntity254Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_255s() -> CustomEntity255Request {
        CustomEntity255Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_255s_minimal() -> CustomEntity255Request {
        CustomEntity255Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_255s_with_children() -> CustomEntity255Request {
        CustomEntity255Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_256s() -> CustomEntity256Request {
        CustomEntity256Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_256s_minimal() -> CustomEntity256Request {
        CustomEntity256Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_256s_with_children() -> CustomEntity256Request {
        CustomEntity256Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_257s() -> CustomEntity257Request {
        CustomEntity257Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_257s_minimal() -> CustomEntity257Request {
        CustomEntity257Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_257s_with_children() -> CustomEntity257Request {
        CustomEntity257Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_258s() -> CustomEntity258Request {
        CustomEntity258Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_258s_minimal() -> CustomEntity258Request {
        CustomEntity258Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_258s_with_children() -> CustomEntity258Request {
        CustomEntity258Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_259s() -> CustomEntity259Request {
        CustomEntity259Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_259s_minimal() -> CustomEntity259Request {
        CustomEntity259Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_259s_with_children() -> CustomEntity259Request {
        CustomEntity259Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_260s() -> CustomEntity260Request {
        CustomEntity260Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_260s_minimal() -> CustomEntity260Request {
        CustomEntity260Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_260s_with_children() -> CustomEntity260Request {
        CustomEntity260Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_261s() -> CustomEntity261Request {
        CustomEntity261Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_261s_minimal() -> CustomEntity261Request {
        CustomEntity261Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_261s_with_children() -> CustomEntity261Request {
        CustomEntity261Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_262s() -> CustomEntity262Request {
        CustomEntity262Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_262s_minimal() -> CustomEntity262Request {
        CustomEntity262Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_262s_with_children() -> CustomEntity262Request {
        CustomEntity262Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_263s() -> CustomEntity263Request {
        CustomEntity263Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_263s_minimal() -> CustomEntity263Request {
        CustomEntity263Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_263s_with_children() -> CustomEntity263Request {
        CustomEntity263Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_264s() -> CustomEntity264Request {
        CustomEntity264Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_264s_minimal() -> CustomEntity264Request {
        CustomEntity264Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_264s_with_children() -> CustomEntity264Request {
        CustomEntity264Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_265s() -> CustomEntity265Request {
        CustomEntity265Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_265s_minimal() -> CustomEntity265Request {
        CustomEntity265Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_265s_with_children() -> CustomEntity265Request {
        CustomEntity265Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_266s() -> CustomEntity266Request {
        CustomEntity266Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_266s_minimal() -> CustomEntity266Request {
        CustomEntity266Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_266s_with_children() -> CustomEntity266Request {
        CustomEntity266Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_267s() -> CustomEntity267Request {
        CustomEntity267Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_267s_minimal() -> CustomEntity267Request {
        CustomEntity267Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_267s_with_children() -> CustomEntity267Request {
        CustomEntity267Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_268s() -> CustomEntity268Request {
        CustomEntity268Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_268s_minimal() -> CustomEntity268Request {
        CustomEntity268Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_268s_with_children() -> CustomEntity268Request {
        CustomEntity268Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_269s() -> CustomEntity269Request {
        CustomEntity269Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_269s_minimal() -> CustomEntity269Request {
        CustomEntity269Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_269s_with_children() -> CustomEntity269Request {
        CustomEntity269Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_270s() -> CustomEntity270Request {
        CustomEntity270Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_270s_minimal() -> CustomEntity270Request {
        CustomEntity270Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_270s_with_children() -> CustomEntity270Request {
        CustomEntity270Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_271s() -> CustomEntity271Request {
        CustomEntity271Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_271s_minimal() -> CustomEntity271Request {
        CustomEntity271Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_271s_with_children() -> CustomEntity271Request {
        CustomEntity271Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_272s() -> CustomEntity272Request {
        CustomEntity272Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_272s_minimal() -> CustomEntity272Request {
        CustomEntity272Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_272s_with_children() -> CustomEntity272Request {
        CustomEntity272Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_273s() -> CustomEntity273Request {
        CustomEntity273Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_273s_minimal() -> CustomEntity273Request {
        CustomEntity273Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_273s_with_children() -> CustomEntity273Request {
        CustomEntity273Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_274s() -> CustomEntity274Request {
        CustomEntity274Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_274s_minimal() -> CustomEntity274Request {
        CustomEntity274Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_274s_with_children() -> CustomEntity274Request {
        CustomEntity274Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_275s() -> CustomEntity275Request {
        CustomEntity275Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_275s_minimal() -> CustomEntity275Request {
        CustomEntity275Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_275s_with_children() -> CustomEntity275Request {
        CustomEntity275Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_276s() -> CustomEntity276Request {
        CustomEntity276Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_276s_minimal() -> CustomEntity276Request {
        CustomEntity276Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_276s_with_children() -> CustomEntity276Request {
        CustomEntity276Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_277s() -> CustomEntity277Request {
        CustomEntity277Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_277s_minimal() -> CustomEntity277Request {
        CustomEntity277Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_277s_with_children() -> CustomEntity277Request {
        CustomEntity277Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_278s() -> CustomEntity278Request {
        CustomEntity278Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_278s_minimal() -> CustomEntity278Request {
        CustomEntity278Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_278s_with_children() -> CustomEntity278Request {
        CustomEntity278Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_279s() -> CustomEntity279Request {
        CustomEntity279Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_279s_minimal() -> CustomEntity279Request {
        CustomEntity279Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_279s_with_children() -> CustomEntity279Request {
        CustomEntity279Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_280s() -> CustomEntity280Request {
        CustomEntity280Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_280s_minimal() -> CustomEntity280Request {
        CustomEntity280Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_280s_with_children() -> CustomEntity280Request {
        CustomEntity280Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_281s() -> CustomEntity281Request {
        CustomEntity281Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_281s_minimal() -> CustomEntity281Request {
        CustomEntity281Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_281s_with_children() -> CustomEntity281Request {
        CustomEntity281Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_282s() -> CustomEntity282Request {
        CustomEntity282Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_282s_minimal() -> CustomEntity282Request {
        CustomEntity282Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_282s_with_children() -> CustomEntity282Request {
        CustomEntity282Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_283s() -> CustomEntity283Request {
        CustomEntity283Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_283s_minimal() -> CustomEntity283Request {
        CustomEntity283Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_283s_with_children() -> CustomEntity283Request {
        CustomEntity283Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_284s() -> CustomEntity284Request {
        CustomEntity284Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_284s_minimal() -> CustomEntity284Request {
        CustomEntity284Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_284s_with_children() -> CustomEntity284Request {
        CustomEntity284Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_285s() -> CustomEntity285Request {
        CustomEntity285Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_285s_minimal() -> CustomEntity285Request {
        CustomEntity285Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_285s_with_children() -> CustomEntity285Request {
        CustomEntity285Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_286s() -> CustomEntity286Request {
        CustomEntity286Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_286s_minimal() -> CustomEntity286Request {
        CustomEntity286Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_286s_with_children() -> CustomEntity286Request {
        CustomEntity286Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_287s() -> CustomEntity287Request {
        CustomEntity287Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_287s_minimal() -> CustomEntity287Request {
        CustomEntity287Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_287s_with_children() -> CustomEntity287Request {
        CustomEntity287Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_288s() -> CustomEntity288Request {
        CustomEntity288Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_288s_minimal() -> CustomEntity288Request {
        CustomEntity288Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_288s_with_children() -> CustomEntity288Request {
        CustomEntity288Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_289s() -> CustomEntity289Request {
        CustomEntity289Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_289s_minimal() -> CustomEntity289Request {
        CustomEntity289Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_289s_with_children() -> CustomEntity289Request {
        CustomEntity289Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_290s() -> CustomEntity290Request {
        CustomEntity290Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_290s_minimal() -> CustomEntity290Request {
        CustomEntity290Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_290s_with_children() -> CustomEntity290Request {
        CustomEntity290Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_291s() -> CustomEntity291Request {
        CustomEntity291Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_291s_minimal() -> CustomEntity291Request {
        CustomEntity291Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_291s_with_children() -> CustomEntity291Request {
        CustomEntity291Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_292s() -> CustomEntity292Request {
        CustomEntity292Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_292s_minimal() -> CustomEntity292Request {
        CustomEntity292Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_292s_with_children() -> CustomEntity292Request {
        CustomEntity292Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_293s() -> CustomEntity293Request {
        CustomEntity293Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_293s_minimal() -> CustomEntity293Request {
        CustomEntity293Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_293s_with_children() -> CustomEntity293Request {
        CustomEntity293Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_294s() -> CustomEntity294Request {
        CustomEntity294Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_294s_minimal() -> CustomEntity294Request {
        CustomEntity294Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_294s_with_children() -> CustomEntity294Request {
        CustomEntity294Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_295s() -> CustomEntity295Request {
        CustomEntity295Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_295s_minimal() -> CustomEntity295Request {
        CustomEntity295Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_295s_with_children() -> CustomEntity295Request {
        CustomEntity295Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_296s() -> CustomEntity296Request {
        CustomEntity296Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_296s_minimal() -> CustomEntity296Request {
        CustomEntity296Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_296s_with_children() -> CustomEntity296Request {
        CustomEntity296Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_297s() -> CustomEntity297Request {
        CustomEntity297Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_297s_minimal() -> CustomEntity297Request {
        CustomEntity297Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_297s_with_children() -> CustomEntity297Request {
        CustomEntity297Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_298s() -> CustomEntity298Request {
        CustomEntity298Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_298s_minimal() -> CustomEntity298Request {
        CustomEntity298Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_298s_with_children() -> CustomEntity298Request {
        CustomEntity298Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_299s() -> CustomEntity299Request {
        CustomEntity299Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_299s_minimal() -> CustomEntity299Request {
        CustomEntity299Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_299s_with_children() -> CustomEntity299Request {
        CustomEntity299Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_300s() -> CustomEntity300Request {
        CustomEntity300Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_300s_minimal() -> CustomEntity300Request {
        CustomEntity300Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_300s_with_children() -> CustomEntity300Request {
        CustomEntity300Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_301s() -> CustomEntity301Request {
        CustomEntity301Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_301s_minimal() -> CustomEntity301Request {
        CustomEntity301Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_301s_with_children() -> CustomEntity301Request {
        CustomEntity301Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_302s() -> CustomEntity302Request {
        CustomEntity302Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_302s_minimal() -> CustomEntity302Request {
        CustomEntity302Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_302s_with_children() -> CustomEntity302Request {
        CustomEntity302Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_303s() -> CustomEntity303Request {
        CustomEntity303Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_303s_minimal() -> CustomEntity303Request {
        CustomEntity303Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_303s_with_children() -> CustomEntity303Request {
        CustomEntity303Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_304s() -> CustomEntity304Request {
        CustomEntity304Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_304s_minimal() -> CustomEntity304Request {
        CustomEntity304Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_304s_with_children() -> CustomEntity304Request {
        CustomEntity304Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_305s() -> CustomEntity305Request {
        CustomEntity305Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_305s_minimal() -> CustomEntity305Request {
        CustomEntity305Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_305s_with_children() -> CustomEntity305Request {
        CustomEntity305Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_306s() -> CustomEntity306Request {
        CustomEntity306Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_306s_minimal() -> CustomEntity306Request {
        CustomEntity306Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_306s_with_children() -> CustomEntity306Request {
        CustomEntity306Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_307s() -> CustomEntity307Request {
        CustomEntity307Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_307s_minimal() -> CustomEntity307Request {
        CustomEntity307Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_307s_with_children() -> CustomEntity307Request {
        CustomEntity307Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_308s() -> CustomEntity308Request {
        CustomEntity308Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_308s_minimal() -> CustomEntity308Request {
        CustomEntity308Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_308s_with_children() -> CustomEntity308Request {
        CustomEntity308Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_309s() -> CustomEntity309Request {
        CustomEntity309Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_309s_minimal() -> CustomEntity309Request {
        CustomEntity309Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_309s_with_children() -> CustomEntity309Request {
        CustomEntity309Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_310s() -> CustomEntity310Request {
        CustomEntity310Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_310s_minimal() -> CustomEntity310Request {
        CustomEntity310Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_310s_with_children() -> CustomEntity310Request {
        CustomEntity310Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_311s() -> CustomEntity311Request {
        CustomEntity311Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_311s_minimal() -> CustomEntity311Request {
        CustomEntity311Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_311s_with_children() -> CustomEntity311Request {
        CustomEntity311Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_312s() -> CustomEntity312Request {
        CustomEntity312Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_312s_minimal() -> CustomEntity312Request {
        CustomEntity312Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_312s_with_children() -> CustomEntity312Request {
        CustomEntity312Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_313s() -> CustomEntity313Request {
        CustomEntity313Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_313s_minimal() -> CustomEntity313Request {
        CustomEntity313Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_313s_with_children() -> CustomEntity313Request {
        CustomEntity313Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_314s() -> CustomEntity314Request {
        CustomEntity314Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_314s_minimal() -> CustomEntity314Request {
        CustomEntity314Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_314s_with_children() -> CustomEntity314Request {
        CustomEntity314Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_315s() -> CustomEntity315Request {
        CustomEntity315Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_315s_minimal() -> CustomEntity315Request {
        CustomEntity315Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_315s_with_children() -> CustomEntity315Request {
        CustomEntity315Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_316s() -> CustomEntity316Request {
        CustomEntity316Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_316s_minimal() -> CustomEntity316Request {
        CustomEntity316Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_316s_with_children() -> CustomEntity316Request {
        CustomEntity316Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_317s() -> CustomEntity317Request {
        CustomEntity317Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_317s_minimal() -> CustomEntity317Request {
        CustomEntity317Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_317s_with_children() -> CustomEntity317Request {
        CustomEntity317Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_318s() -> CustomEntity318Request {
        CustomEntity318Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_318s_minimal() -> CustomEntity318Request {
        CustomEntity318Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_318s_with_children() -> CustomEntity318Request {
        CustomEntity318Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_319s() -> CustomEntity319Request {
        CustomEntity319Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_319s_minimal() -> CustomEntity319Request {
        CustomEntity319Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_319s_with_children() -> CustomEntity319Request {
        CustomEntity319Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_320s() -> CustomEntity320Request {
        CustomEntity320Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_320s_minimal() -> CustomEntity320Request {
        CustomEntity320Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_320s_with_children() -> CustomEntity320Request {
        CustomEntity320Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_321s() -> CustomEntity321Request {
        CustomEntity321Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_321s_minimal() -> CustomEntity321Request {
        CustomEntity321Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_321s_with_children() -> CustomEntity321Request {
        CustomEntity321Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_322s() -> CustomEntity322Request {
        CustomEntity322Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_322s_minimal() -> CustomEntity322Request {
        CustomEntity322Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_322s_with_children() -> CustomEntity322Request {
        CustomEntity322Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_323s() -> CustomEntity323Request {
        CustomEntity323Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_323s_minimal() -> CustomEntity323Request {
        CustomEntity323Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_323s_with_children() -> CustomEntity323Request {
        CustomEntity323Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_324s() -> CustomEntity324Request {
        CustomEntity324Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_324s_minimal() -> CustomEntity324Request {
        CustomEntity324Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_324s_with_children() -> CustomEntity324Request {
        CustomEntity324Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_325s() -> CustomEntity325Request {
        CustomEntity325Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_325s_minimal() -> CustomEntity325Request {
        CustomEntity325Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_325s_with_children() -> CustomEntity325Request {
        CustomEntity325Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_326s() -> CustomEntity326Request {
        CustomEntity326Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_326s_minimal() -> CustomEntity326Request {
        CustomEntity326Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_326s_with_children() -> CustomEntity326Request {
        CustomEntity326Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_327s() -> CustomEntity327Request {
        CustomEntity327Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_327s_minimal() -> CustomEntity327Request {
        CustomEntity327Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_327s_with_children() -> CustomEntity327Request {
        CustomEntity327Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_328s() -> CustomEntity328Request {
        CustomEntity328Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_328s_minimal() -> CustomEntity328Request {
        CustomEntity328Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_328s_with_children() -> CustomEntity328Request {
        CustomEntity328Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_329s() -> CustomEntity329Request {
        CustomEntity329Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_329s_minimal() -> CustomEntity329Request {
        CustomEntity329Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_329s_with_children() -> CustomEntity329Request {
        CustomEntity329Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_330s() -> CustomEntity330Request {
        CustomEntity330Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_330s_minimal() -> CustomEntity330Request {
        CustomEntity330Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_330s_with_children() -> CustomEntity330Request {
        CustomEntity330Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_331s() -> CustomEntity331Request {
        CustomEntity331Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_331s_minimal() -> CustomEntity331Request {
        CustomEntity331Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_331s_with_children() -> CustomEntity331Request {
        CustomEntity331Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_332s() -> CustomEntity332Request {
        CustomEntity332Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_332s_minimal() -> CustomEntity332Request {
        CustomEntity332Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_332s_with_children() -> CustomEntity332Request {
        CustomEntity332Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_333s() -> CustomEntity333Request {
        CustomEntity333Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_333s_minimal() -> CustomEntity333Request {
        CustomEntity333Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_333s_with_children() -> CustomEntity333Request {
        CustomEntity333Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_334s() -> CustomEntity334Request {
        CustomEntity334Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_334s_minimal() -> CustomEntity334Request {
        CustomEntity334Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_334s_with_children() -> CustomEntity334Request {
        CustomEntity334Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_335s() -> CustomEntity335Request {
        CustomEntity335Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_335s_minimal() -> CustomEntity335Request {
        CustomEntity335Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_335s_with_children() -> CustomEntity335Request {
        CustomEntity335Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_336s() -> CustomEntity336Request {
        CustomEntity336Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_336s_minimal() -> CustomEntity336Request {
        CustomEntity336Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_336s_with_children() -> CustomEntity336Request {
        CustomEntity336Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_337s() -> CustomEntity337Request {
        CustomEntity337Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_337s_minimal() -> CustomEntity337Request {
        CustomEntity337Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_337s_with_children() -> CustomEntity337Request {
        CustomEntity337Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_338s() -> CustomEntity338Request {
        CustomEntity338Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_338s_minimal() -> CustomEntity338Request {
        CustomEntity338Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_338s_with_children() -> CustomEntity338Request {
        CustomEntity338Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_339s() -> CustomEntity339Request {
        CustomEntity339Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_339s_minimal() -> CustomEntity339Request {
        CustomEntity339Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_339s_with_children() -> CustomEntity339Request {
        CustomEntity339Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_340s() -> CustomEntity340Request {
        CustomEntity340Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_340s_minimal() -> CustomEntity340Request {
        CustomEntity340Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_340s_with_children() -> CustomEntity340Request {
        CustomEntity340Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_341s() -> CustomEntity341Request {
        CustomEntity341Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_341s_minimal() -> CustomEntity341Request {
        CustomEntity341Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_341s_with_children() -> CustomEntity341Request {
        CustomEntity341Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_342s() -> CustomEntity342Request {
        CustomEntity342Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_342s_minimal() -> CustomEntity342Request {
        CustomEntity342Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_342s_with_children() -> CustomEntity342Request {
        CustomEntity342Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_343s() -> CustomEntity343Request {
        CustomEntity343Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_343s_minimal() -> CustomEntity343Request {
        CustomEntity343Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_343s_with_children() -> CustomEntity343Request {
        CustomEntity343Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_344s() -> CustomEntity344Request {
        CustomEntity344Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_344s_minimal() -> CustomEntity344Request {
        CustomEntity344Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_344s_with_children() -> CustomEntity344Request {
        CustomEntity344Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_345s() -> CustomEntity345Request {
        CustomEntity345Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_345s_minimal() -> CustomEntity345Request {
        CustomEntity345Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_345s_with_children() -> CustomEntity345Request {
        CustomEntity345Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_346s() -> CustomEntity346Request {
        CustomEntity346Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_346s_minimal() -> CustomEntity346Request {
        CustomEntity346Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_346s_with_children() -> CustomEntity346Request {
        CustomEntity346Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_347s() -> CustomEntity347Request {
        CustomEntity347Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_347s_minimal() -> CustomEntity347Request {
        CustomEntity347Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_347s_with_children() -> CustomEntity347Request {
        CustomEntity347Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_348s() -> CustomEntity348Request {
        CustomEntity348Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_348s_minimal() -> CustomEntity348Request {
        CustomEntity348Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_348s_with_children() -> CustomEntity348Request {
        CustomEntity348Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_349s() -> CustomEntity349Request {
        CustomEntity349Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_349s_minimal() -> CustomEntity349Request {
        CustomEntity349Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_349s_with_children() -> CustomEntity349Request {
        CustomEntity349Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_350s() -> CustomEntity350Request {
        CustomEntity350Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_350s_minimal() -> CustomEntity350Request {
        CustomEntity350Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_350s_with_children() -> CustomEntity350Request {
        CustomEntity350Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_351s() -> CustomEntity351Request {
        CustomEntity351Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_351s_minimal() -> CustomEntity351Request {
        CustomEntity351Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_351s_with_children() -> CustomEntity351Request {
        CustomEntity351Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_352s() -> CustomEntity352Request {
        CustomEntity352Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_352s_minimal() -> CustomEntity352Request {
        CustomEntity352Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_352s_with_children() -> CustomEntity352Request {
        CustomEntity352Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_353s() -> CustomEntity353Request {
        CustomEntity353Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_353s_minimal() -> CustomEntity353Request {
        CustomEntity353Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_353s_with_children() -> CustomEntity353Request {
        CustomEntity353Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_354s() -> CustomEntity354Request {
        CustomEntity354Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_354s_minimal() -> CustomEntity354Request {
        CustomEntity354Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_354s_with_children() -> CustomEntity354Request {
        CustomEntity354Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_355s() -> CustomEntity355Request {
        CustomEntity355Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_355s_minimal() -> CustomEntity355Request {
        CustomEntity355Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_355s_with_children() -> CustomEntity355Request {
        CustomEntity355Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_356s() -> CustomEntity356Request {
        CustomEntity356Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_356s_minimal() -> CustomEntity356Request {
        CustomEntity356Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_356s_with_children() -> CustomEntity356Request {
        CustomEntity356Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_357s() -> CustomEntity357Request {
        CustomEntity357Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_357s_minimal() -> CustomEntity357Request {
        CustomEntity357Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_357s_with_children() -> CustomEntity357Request {
        CustomEntity357Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_358s() -> CustomEntity358Request {
        CustomEntity358Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_358s_minimal() -> CustomEntity358Request {
        CustomEntity358Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_358s_with_children() -> CustomEntity358Request {
        CustomEntity358Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_359s() -> CustomEntity359Request {
        CustomEntity359Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_359s_minimal() -> CustomEntity359Request {
        CustomEntity359Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_359s_with_children() -> CustomEntity359Request {
        CustomEntity359Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_360s() -> CustomEntity360Request {
        CustomEntity360Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_360s_minimal() -> CustomEntity360Request {
        CustomEntity360Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_360s_with_children() -> CustomEntity360Request {
        CustomEntity360Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_361s() -> CustomEntity361Request {
        CustomEntity361Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_361s_minimal() -> CustomEntity361Request {
        CustomEntity361Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_361s_with_children() -> CustomEntity361Request {
        CustomEntity361Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_362s() -> CustomEntity362Request {
        CustomEntity362Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_362s_minimal() -> CustomEntity362Request {
        CustomEntity362Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_362s_with_children() -> CustomEntity362Request {
        CustomEntity362Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_363s() -> CustomEntity363Request {
        CustomEntity363Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_363s_minimal() -> CustomEntity363Request {
        CustomEntity363Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_363s_with_children() -> CustomEntity363Request {
        CustomEntity363Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_364s() -> CustomEntity364Request {
        CustomEntity364Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_364s_minimal() -> CustomEntity364Request {
        CustomEntity364Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_364s_with_children() -> CustomEntity364Request {
        CustomEntity364Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_365s() -> CustomEntity365Request {
        CustomEntity365Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_365s_minimal() -> CustomEntity365Request {
        CustomEntity365Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_365s_with_children() -> CustomEntity365Request {
        CustomEntity365Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_366s() -> CustomEntity366Request {
        CustomEntity366Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_366s_minimal() -> CustomEntity366Request {
        CustomEntity366Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_366s_with_children() -> CustomEntity366Request {
        CustomEntity366Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_367s() -> CustomEntity367Request {
        CustomEntity367Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_367s_minimal() -> CustomEntity367Request {
        CustomEntity367Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_367s_with_children() -> CustomEntity367Request {
        CustomEntity367Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_368s() -> CustomEntity368Request {
        CustomEntity368Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_368s_minimal() -> CustomEntity368Request {
        CustomEntity368Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_368s_with_children() -> CustomEntity368Request {
        CustomEntity368Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_369s() -> CustomEntity369Request {
        CustomEntity369Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_369s_minimal() -> CustomEntity369Request {
        CustomEntity369Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_369s_with_children() -> CustomEntity369Request {
        CustomEntity369Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_370s() -> CustomEntity370Request {
        CustomEntity370Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_370s_minimal() -> CustomEntity370Request {
        CustomEntity370Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_370s_with_children() -> CustomEntity370Request {
        CustomEntity370Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_371s() -> CustomEntity371Request {
        CustomEntity371Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_371s_minimal() -> CustomEntity371Request {
        CustomEntity371Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_371s_with_children() -> CustomEntity371Request {
        CustomEntity371Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_372s() -> CustomEntity372Request {
        CustomEntity372Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_372s_minimal() -> CustomEntity372Request {
        CustomEntity372Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_372s_with_children() -> CustomEntity372Request {
        CustomEntity372Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_373s() -> CustomEntity373Request {
        CustomEntity373Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_373s_minimal() -> CustomEntity373Request {
        CustomEntity373Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_373s_with_children() -> CustomEntity373Request {
        CustomEntity373Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_374s() -> CustomEntity374Request {
        CustomEntity374Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_374s_minimal() -> CustomEntity374Request {
        CustomEntity374Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_374s_with_children() -> CustomEntity374Request {
        CustomEntity374Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_375s() -> CustomEntity375Request {
        CustomEntity375Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_375s_minimal() -> CustomEntity375Request {
        CustomEntity375Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_375s_with_children() -> CustomEntity375Request {
        CustomEntity375Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_376s() -> CustomEntity376Request {
        CustomEntity376Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_376s_minimal() -> CustomEntity376Request {
        CustomEntity376Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_376s_with_children() -> CustomEntity376Request {
        CustomEntity376Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_377s() -> CustomEntity377Request {
        CustomEntity377Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_377s_minimal() -> CustomEntity377Request {
        CustomEntity377Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_377s_with_children() -> CustomEntity377Request {
        CustomEntity377Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_378s() -> CustomEntity378Request {
        CustomEntity378Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_378s_minimal() -> CustomEntity378Request {
        CustomEntity378Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_378s_with_children() -> CustomEntity378Request {
        CustomEntity378Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_379s() -> CustomEntity379Request {
        CustomEntity379Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_379s_minimal() -> CustomEntity379Request {
        CustomEntity379Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_379s_with_children() -> CustomEntity379Request {
        CustomEntity379Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_380s() -> CustomEntity380Request {
        CustomEntity380Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_380s_minimal() -> CustomEntity380Request {
        CustomEntity380Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_380s_with_children() -> CustomEntity380Request {
        CustomEntity380Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_381s() -> CustomEntity381Request {
        CustomEntity381Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_381s_minimal() -> CustomEntity381Request {
        CustomEntity381Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_381s_with_children() -> CustomEntity381Request {
        CustomEntity381Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_382s() -> CustomEntity382Request {
        CustomEntity382Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_382s_minimal() -> CustomEntity382Request {
        CustomEntity382Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_382s_with_children() -> CustomEntity382Request {
        CustomEntity382Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_383s() -> CustomEntity383Request {
        CustomEntity383Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_383s_minimal() -> CustomEntity383Request {
        CustomEntity383Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_383s_with_children() -> CustomEntity383Request {
        CustomEntity383Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_384s() -> CustomEntity384Request {
        CustomEntity384Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_384s_minimal() -> CustomEntity384Request {
        CustomEntity384Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_384s_with_children() -> CustomEntity384Request {
        CustomEntity384Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_385s() -> CustomEntity385Request {
        CustomEntity385Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_385s_minimal() -> CustomEntity385Request {
        CustomEntity385Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_385s_with_children() -> CustomEntity385Request {
        CustomEntity385Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_386s() -> CustomEntity386Request {
        CustomEntity386Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_386s_minimal() -> CustomEntity386Request {
        CustomEntity386Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_386s_with_children() -> CustomEntity386Request {
        CustomEntity386Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_387s() -> CustomEntity387Request {
        CustomEntity387Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_387s_minimal() -> CustomEntity387Request {
        CustomEntity387Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_387s_with_children() -> CustomEntity387Request {
        CustomEntity387Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_388s() -> CustomEntity388Request {
        CustomEntity388Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_388s_minimal() -> CustomEntity388Request {
        CustomEntity388Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_388s_with_children() -> CustomEntity388Request {
        CustomEntity388Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_389s() -> CustomEntity389Request {
        CustomEntity389Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_389s_minimal() -> CustomEntity389Request {
        CustomEntity389Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_389s_with_children() -> CustomEntity389Request {
        CustomEntity389Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_390s() -> CustomEntity390Request {
        CustomEntity390Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_390s_minimal() -> CustomEntity390Request {
        CustomEntity390Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_390s_with_children() -> CustomEntity390Request {
        CustomEntity390Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_391s() -> CustomEntity391Request {
        CustomEntity391Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_391s_minimal() -> CustomEntity391Request {
        CustomEntity391Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_391s_with_children() -> CustomEntity391Request {
        CustomEntity391Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_392s() -> CustomEntity392Request {
        CustomEntity392Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_392s_minimal() -> CustomEntity392Request {
        CustomEntity392Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_392s_with_children() -> CustomEntity392Request {
        CustomEntity392Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_393s() -> CustomEntity393Request {
        CustomEntity393Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_393s_minimal() -> CustomEntity393Request {
        CustomEntity393Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_393s_with_children() -> CustomEntity393Request {
        CustomEntity393Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_394s() -> CustomEntity394Request {
        CustomEntity394Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_394s_minimal() -> CustomEntity394Request {
        CustomEntity394Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_394s_with_children() -> CustomEntity394Request {
        CustomEntity394Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_395s() -> CustomEntity395Request {
        CustomEntity395Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_395s_minimal() -> CustomEntity395Request {
        CustomEntity395Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_395s_with_children() -> CustomEntity395Request {
        CustomEntity395Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_396s() -> CustomEntity396Request {
        CustomEntity396Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_396s_minimal() -> CustomEntity396Request {
        CustomEntity396Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_396s_with_children() -> CustomEntity396Request {
        CustomEntity396Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_397s() -> CustomEntity397Request {
        CustomEntity397Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_397s_minimal() -> CustomEntity397Request {
        CustomEntity397Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_397s_with_children() -> CustomEntity397Request {
        CustomEntity397Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_398s() -> CustomEntity398Request {
        CustomEntity398Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_398s_minimal() -> CustomEntity398Request {
        CustomEntity398Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_398s_with_children() -> CustomEntity398Request {
        CustomEntity398Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_399s() -> CustomEntity399Request {
        CustomEntity399Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_399s_minimal() -> CustomEntity399Request {
        CustomEntity399Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_399s_with_children() -> CustomEntity399Request {
        CustomEntity399Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_400s() -> CustomEntity400Request {
        CustomEntity400Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_400s_minimal() -> CustomEntity400Request {
        CustomEntity400Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_400s_with_children() -> CustomEntity400Request {
        CustomEntity400Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_401s() -> CustomEntity401Request {
        CustomEntity401Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_401s_minimal() -> CustomEntity401Request {
        CustomEntity401Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_401s_with_children() -> CustomEntity401Request {
        CustomEntity401Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_402s() -> CustomEntity402Request {
        CustomEntity402Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_402s_minimal() -> CustomEntity402Request {
        CustomEntity402Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_402s_with_children() -> CustomEntity402Request {
        CustomEntity402Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_403s() -> CustomEntity403Request {
        CustomEntity403Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_403s_minimal() -> CustomEntity403Request {
        CustomEntity403Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_403s_with_children() -> CustomEntity403Request {
        CustomEntity403Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_404s() -> CustomEntity404Request {
        CustomEntity404Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_404s_minimal() -> CustomEntity404Request {
        CustomEntity404Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_404s_with_children() -> CustomEntity404Request {
        CustomEntity404Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_405s() -> CustomEntity405Request {
        CustomEntity405Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_405s_minimal() -> CustomEntity405Request {
        CustomEntity405Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_405s_with_children() -> CustomEntity405Request {
        CustomEntity405Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_406s() -> CustomEntity406Request {
        CustomEntity406Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_406s_minimal() -> CustomEntity406Request {
        CustomEntity406Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_406s_with_children() -> CustomEntity406Request {
        CustomEntity406Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_407s() -> CustomEntity407Request {
        CustomEntity407Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_407s_minimal() -> CustomEntity407Request {
        CustomEntity407Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_407s_with_children() -> CustomEntity407Request {
        CustomEntity407Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_408s() -> CustomEntity408Request {
        CustomEntity408Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_408s_minimal() -> CustomEntity408Request {
        CustomEntity408Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_408s_with_children() -> CustomEntity408Request {
        CustomEntity408Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_409s() -> CustomEntity409Request {
        CustomEntity409Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_409s_minimal() -> CustomEntity409Request {
        CustomEntity409Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_409s_with_children() -> CustomEntity409Request {
        CustomEntity409Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_410s() -> CustomEntity410Request {
        CustomEntity410Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_410s_minimal() -> CustomEntity410Request {
        CustomEntity410Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_410s_with_children() -> CustomEntity410Request {
        CustomEntity410Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_411s() -> CustomEntity411Request {
        CustomEntity411Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_411s_minimal() -> CustomEntity411Request {
        CustomEntity411Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_411s_with_children() -> CustomEntity411Request {
        CustomEntity411Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_412s() -> CustomEntity412Request {
        CustomEntity412Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_412s_minimal() -> CustomEntity412Request {
        CustomEntity412Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_412s_with_children() -> CustomEntity412Request {
        CustomEntity412Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_413s() -> CustomEntity413Request {
        CustomEntity413Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_413s_minimal() -> CustomEntity413Request {
        CustomEntity413Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_413s_with_children() -> CustomEntity413Request {
        CustomEntity413Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_414s() -> CustomEntity414Request {
        CustomEntity414Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_414s_minimal() -> CustomEntity414Request {
        CustomEntity414Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_414s_with_children() -> CustomEntity414Request {
        CustomEntity414Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_415s() -> CustomEntity415Request {
        CustomEntity415Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_415s_minimal() -> CustomEntity415Request {
        CustomEntity415Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_415s_with_children() -> CustomEntity415Request {
        CustomEntity415Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_416s() -> CustomEntity416Request {
        CustomEntity416Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_416s_minimal() -> CustomEntity416Request {
        CustomEntity416Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_416s_with_children() -> CustomEntity416Request {
        CustomEntity416Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_417s() -> CustomEntity417Request {
        CustomEntity417Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_417s_minimal() -> CustomEntity417Request {
        CustomEntity417Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_417s_with_children() -> CustomEntity417Request {
        CustomEntity417Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_418s() -> CustomEntity418Request {
        CustomEntity418Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_418s_minimal() -> CustomEntity418Request {
        CustomEntity418Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_418s_with_children() -> CustomEntity418Request {
        CustomEntity418Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_419s() -> CustomEntity419Request {
        CustomEntity419Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_419s_minimal() -> CustomEntity419Request {
        CustomEntity419Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_419s_with_children() -> CustomEntity419Request {
        CustomEntity419Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_420s() -> CustomEntity420Request {
        CustomEntity420Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_420s_minimal() -> CustomEntity420Request {
        CustomEntity420Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_420s_with_children() -> CustomEntity420Request {
        CustomEntity420Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_421s() -> CustomEntity421Request {
        CustomEntity421Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_421s_minimal() -> CustomEntity421Request {
        CustomEntity421Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_421s_with_children() -> CustomEntity421Request {
        CustomEntity421Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_422s() -> CustomEntity422Request {
        CustomEntity422Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_422s_minimal() -> CustomEntity422Request {
        CustomEntity422Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_422s_with_children() -> CustomEntity422Request {
        CustomEntity422Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_423s() -> CustomEntity423Request {
        CustomEntity423Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_423s_minimal() -> CustomEntity423Request {
        CustomEntity423Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_423s_with_children() -> CustomEntity423Request {
        CustomEntity423Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_424s() -> CustomEntity424Request {
        CustomEntity424Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_424s_minimal() -> CustomEntity424Request {
        CustomEntity424Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_424s_with_children() -> CustomEntity424Request {
        CustomEntity424Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_425s() -> CustomEntity425Request {
        CustomEntity425Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_425s_minimal() -> CustomEntity425Request {
        CustomEntity425Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_425s_with_children() -> CustomEntity425Request {
        CustomEntity425Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_426s() -> CustomEntity426Request {
        CustomEntity426Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_426s_minimal() -> CustomEntity426Request {
        CustomEntity426Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_426s_with_children() -> CustomEntity426Request {
        CustomEntity426Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_427s() -> CustomEntity427Request {
        CustomEntity427Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_427s_minimal() -> CustomEntity427Request {
        CustomEntity427Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_427s_with_children() -> CustomEntity427Request {
        CustomEntity427Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_428s() -> CustomEntity428Request {
        CustomEntity428Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_428s_minimal() -> CustomEntity428Request {
        CustomEntity428Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_428s_with_children() -> CustomEntity428Request {
        CustomEntity428Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_429s() -> CustomEntity429Request {
        CustomEntity429Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_429s_minimal() -> CustomEntity429Request {
        CustomEntity429Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_429s_with_children() -> CustomEntity429Request {
        CustomEntity429Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_430s() -> CustomEntity430Request {
        CustomEntity430Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_430s_minimal() -> CustomEntity430Request {
        CustomEntity430Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_430s_with_children() -> CustomEntity430Request {
        CustomEntity430Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_431s() -> CustomEntity431Request {
        CustomEntity431Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_431s_minimal() -> CustomEntity431Request {
        CustomEntity431Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_431s_with_children() -> CustomEntity431Request {
        CustomEntity431Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_432s() -> CustomEntity432Request {
        CustomEntity432Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_432s_minimal() -> CustomEntity432Request {
        CustomEntity432Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_432s_with_children() -> CustomEntity432Request {
        CustomEntity432Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_433s() -> CustomEntity433Request {
        CustomEntity433Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_433s_minimal() -> CustomEntity433Request {
        CustomEntity433Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_433s_with_children() -> CustomEntity433Request {
        CustomEntity433Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_434s() -> CustomEntity434Request {
        CustomEntity434Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_434s_minimal() -> CustomEntity434Request {
        CustomEntity434Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_434s_with_children() -> CustomEntity434Request {
        CustomEntity434Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_435s() -> CustomEntity435Request {
        CustomEntity435Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_435s_minimal() -> CustomEntity435Request {
        CustomEntity435Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_435s_with_children() -> CustomEntity435Request {
        CustomEntity435Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_436s() -> CustomEntity436Request {
        CustomEntity436Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_436s_minimal() -> CustomEntity436Request {
        CustomEntity436Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_436s_with_children() -> CustomEntity436Request {
        CustomEntity436Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_437s() -> CustomEntity437Request {
        CustomEntity437Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_437s_minimal() -> CustomEntity437Request {
        CustomEntity437Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_437s_with_children() -> CustomEntity437Request {
        CustomEntity437Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_438s() -> CustomEntity438Request {
        CustomEntity438Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_438s_minimal() -> CustomEntity438Request {
        CustomEntity438Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_438s_with_children() -> CustomEntity438Request {
        CustomEntity438Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_439s() -> CustomEntity439Request {
        CustomEntity439Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_439s_minimal() -> CustomEntity439Request {
        CustomEntity439Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_439s_with_children() -> CustomEntity439Request {
        CustomEntity439Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_440s() -> CustomEntity440Request {
        CustomEntity440Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_440s_minimal() -> CustomEntity440Request {
        CustomEntity440Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_440s_with_children() -> CustomEntity440Request {
        CustomEntity440Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_441s() -> CustomEntity441Request {
        CustomEntity441Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_441s_minimal() -> CustomEntity441Request {
        CustomEntity441Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_441s_with_children() -> CustomEntity441Request {
        CustomEntity441Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_442s() -> CustomEntity442Request {
        CustomEntity442Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_442s_minimal() -> CustomEntity442Request {
        CustomEntity442Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_442s_with_children() -> CustomEntity442Request {
        CustomEntity442Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_443s() -> CustomEntity443Request {
        CustomEntity443Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_443s_minimal() -> CustomEntity443Request {
        CustomEntity443Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_443s_with_children() -> CustomEntity443Request {
        CustomEntity443Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_444s() -> CustomEntity444Request {
        CustomEntity444Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_444s_minimal() -> CustomEntity444Request {
        CustomEntity444Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_444s_with_children() -> CustomEntity444Request {
        CustomEntity444Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_445s() -> CustomEntity445Request {
        CustomEntity445Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_445s_minimal() -> CustomEntity445Request {
        CustomEntity445Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_445s_with_children() -> CustomEntity445Request {
        CustomEntity445Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_446s() -> CustomEntity446Request {
        CustomEntity446Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_446s_minimal() -> CustomEntity446Request {
        CustomEntity446Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_446s_with_children() -> CustomEntity446Request {
        CustomEntity446Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_447s() -> CustomEntity447Request {
        CustomEntity447Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_447s_minimal() -> CustomEntity447Request {
        CustomEntity447Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_447s_with_children() -> CustomEntity447Request {
        CustomEntity447Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_448s() -> CustomEntity448Request {
        CustomEntity448Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_448s_minimal() -> CustomEntity448Request {
        CustomEntity448Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_448s_with_children() -> CustomEntity448Request {
        CustomEntity448Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_449s() -> CustomEntity449Request {
        CustomEntity449Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_449s_minimal() -> CustomEntity449Request {
        CustomEntity449Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_449s_with_children() -> CustomEntity449Request {
        CustomEntity449Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_450s() -> CustomEntity450Request {
        CustomEntity450Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_450s_minimal() -> CustomEntity450Request {
        CustomEntity450Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_450s_with_children() -> CustomEntity450Request {
        CustomEntity450Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_451s() -> CustomEntity451Request {
        CustomEntity451Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_451s_minimal() -> CustomEntity451Request {
        CustomEntity451Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_451s_with_children() -> CustomEntity451Request {
        CustomEntity451Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_452s() -> CustomEntity452Request {
        CustomEntity452Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_452s_minimal() -> CustomEntity452Request {
        CustomEntity452Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_452s_with_children() -> CustomEntity452Request {
        CustomEntity452Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_453s() -> CustomEntity453Request {
        CustomEntity453Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_453s_minimal() -> CustomEntity453Request {
        CustomEntity453Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_453s_with_children() -> CustomEntity453Request {
        CustomEntity453Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_454s() -> CustomEntity454Request {
        CustomEntity454Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_454s_minimal() -> CustomEntity454Request {
        CustomEntity454Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_454s_with_children() -> CustomEntity454Request {
        CustomEntity454Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_455s() -> CustomEntity455Request {
        CustomEntity455Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_455s_minimal() -> CustomEntity455Request {
        CustomEntity455Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_455s_with_children() -> CustomEntity455Request {
        CustomEntity455Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_456s() -> CustomEntity456Request {
        CustomEntity456Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_456s_minimal() -> CustomEntity456Request {
        CustomEntity456Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_456s_with_children() -> CustomEntity456Request {
        CustomEntity456Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_457s() -> CustomEntity457Request {
        CustomEntity457Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_457s_minimal() -> CustomEntity457Request {
        CustomEntity457Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_457s_with_children() -> CustomEntity457Request {
        CustomEntity457Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_458s() -> CustomEntity458Request {
        CustomEntity458Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_458s_minimal() -> CustomEntity458Request {
        CustomEntity458Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_458s_with_children() -> CustomEntity458Request {
        CustomEntity458Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_459s() -> CustomEntity459Request {
        CustomEntity459Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_459s_minimal() -> CustomEntity459Request {
        CustomEntity459Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_459s_with_children() -> CustomEntity459Request {
        CustomEntity459Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_460s() -> CustomEntity460Request {
        CustomEntity460Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_460s_minimal() -> CustomEntity460Request {
        CustomEntity460Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_460s_with_children() -> CustomEntity460Request {
        CustomEntity460Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_461s() -> CustomEntity461Request {
        CustomEntity461Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_461s_minimal() -> CustomEntity461Request {
        CustomEntity461Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_461s_with_children() -> CustomEntity461Request {
        CustomEntity461Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_462s() -> CustomEntity462Request {
        CustomEntity462Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_462s_minimal() -> CustomEntity462Request {
        CustomEntity462Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_462s_with_children() -> CustomEntity462Request {
        CustomEntity462Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_463s() -> CustomEntity463Request {
        CustomEntity463Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_463s_minimal() -> CustomEntity463Request {
        CustomEntity463Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_463s_with_children() -> CustomEntity463Request {
        CustomEntity463Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_464s() -> CustomEntity464Request {
        CustomEntity464Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_464s_minimal() -> CustomEntity464Request {
        CustomEntity464Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_464s_with_children() -> CustomEntity464Request {
        CustomEntity464Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_465s() -> CustomEntity465Request {
        CustomEntity465Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_465s_minimal() -> CustomEntity465Request {
        CustomEntity465Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_465s_with_children() -> CustomEntity465Request {
        CustomEntity465Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_466s() -> CustomEntity466Request {
        CustomEntity466Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_466s_minimal() -> CustomEntity466Request {
        CustomEntity466Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_466s_with_children() -> CustomEntity466Request {
        CustomEntity466Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_467s() -> CustomEntity467Request {
        CustomEntity467Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_467s_minimal() -> CustomEntity467Request {
        CustomEntity467Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_467s_with_children() -> CustomEntity467Request {
        CustomEntity467Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_468s() -> CustomEntity468Request {
        CustomEntity468Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_468s_minimal() -> CustomEntity468Request {
        CustomEntity468Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_468s_with_children() -> CustomEntity468Request {
        CustomEntity468Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_469s() -> CustomEntity469Request {
        CustomEntity469Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_469s_minimal() -> CustomEntity469Request {
        CustomEntity469Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_469s_with_children() -> CustomEntity469Request {
        CustomEntity469Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_470s() -> CustomEntity470Request {
        CustomEntity470Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_470s_minimal() -> CustomEntity470Request {
        CustomEntity470Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_470s_with_children() -> CustomEntity470Request {
        CustomEntity470Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_471s() -> CustomEntity471Request {
        CustomEntity471Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_471s_minimal() -> CustomEntity471Request {
        CustomEntity471Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_471s_with_children() -> CustomEntity471Request {
        CustomEntity471Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_472s() -> CustomEntity472Request {
        CustomEntity472Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_472s_minimal() -> CustomEntity472Request {
        CustomEntity472Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_472s_with_children() -> CustomEntity472Request {
        CustomEntity472Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_473s() -> CustomEntity473Request {
        CustomEntity473Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_473s_minimal() -> CustomEntity473Request {
        CustomEntity473Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_473s_with_children() -> CustomEntity473Request {
        CustomEntity473Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_474s() -> CustomEntity474Request {
        CustomEntity474Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_474s_minimal() -> CustomEntity474Request {
        CustomEntity474Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_474s_with_children() -> CustomEntity474Request {
        CustomEntity474Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_475s() -> CustomEntity475Request {
        CustomEntity475Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_475s_minimal() -> CustomEntity475Request {
        CustomEntity475Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_475s_with_children() -> CustomEntity475Request {
        CustomEntity475Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_476s() -> CustomEntity476Request {
        CustomEntity476Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_476s_minimal() -> CustomEntity476Request {
        CustomEntity476Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_476s_with_children() -> CustomEntity476Request {
        CustomEntity476Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_477s() -> CustomEntity477Request {
        CustomEntity477Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_477s_minimal() -> CustomEntity477Request {
        CustomEntity477Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_477s_with_children() -> CustomEntity477Request {
        CustomEntity477Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_478s() -> CustomEntity478Request {
        CustomEntity478Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_478s_minimal() -> CustomEntity478Request {
        CustomEntity478Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_478s_with_children() -> CustomEntity478Request {
        CustomEntity478Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_479s() -> CustomEntity479Request {
        CustomEntity479Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_479s_minimal() -> CustomEntity479Request {
        CustomEntity479Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_479s_with_children() -> CustomEntity479Request {
        CustomEntity479Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_480s() -> CustomEntity480Request {
        CustomEntity480Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_480s_minimal() -> CustomEntity480Request {
        CustomEntity480Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_480s_with_children() -> CustomEntity480Request {
        CustomEntity480Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_481s() -> CustomEntity481Request {
        CustomEntity481Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_481s_minimal() -> CustomEntity481Request {
        CustomEntity481Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_481s_with_children() -> CustomEntity481Request {
        CustomEntity481Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_482s() -> CustomEntity482Request {
        CustomEntity482Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_482s_minimal() -> CustomEntity482Request {
        CustomEntity482Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_482s_with_children() -> CustomEntity482Request {
        CustomEntity482Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_483s() -> CustomEntity483Request {
        CustomEntity483Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_483s_minimal() -> CustomEntity483Request {
        CustomEntity483Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_483s_with_children() -> CustomEntity483Request {
        CustomEntity483Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_484s() -> CustomEntity484Request {
        CustomEntity484Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_484s_minimal() -> CustomEntity484Request {
        CustomEntity484Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_484s_with_children() -> CustomEntity484Request {
        CustomEntity484Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_485s() -> CustomEntity485Request {
        CustomEntity485Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_485s_minimal() -> CustomEntity485Request {
        CustomEntity485Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_485s_with_children() -> CustomEntity485Request {
        CustomEntity485Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_486s() -> CustomEntity486Request {
        CustomEntity486Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_486s_minimal() -> CustomEntity486Request {
        CustomEntity486Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_486s_with_children() -> CustomEntity486Request {
        CustomEntity486Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_487s() -> CustomEntity487Request {
        CustomEntity487Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_487s_minimal() -> CustomEntity487Request {
        CustomEntity487Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_487s_with_children() -> CustomEntity487Request {
        CustomEntity487Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_488s() -> CustomEntity488Request {
        CustomEntity488Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_488s_minimal() -> CustomEntity488Request {
        CustomEntity488Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_488s_with_children() -> CustomEntity488Request {
        CustomEntity488Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_489s() -> CustomEntity489Request {
        CustomEntity489Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_489s_minimal() -> CustomEntity489Request {
        CustomEntity489Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_489s_with_children() -> CustomEntity489Request {
        CustomEntity489Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_490s() -> CustomEntity490Request {
        CustomEntity490Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_490s_minimal() -> CustomEntity490Request {
        CustomEntity490Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_490s_with_children() -> CustomEntity490Request {
        CustomEntity490Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_491s() -> CustomEntity491Request {
        CustomEntity491Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_491s_minimal() -> CustomEntity491Request {
        CustomEntity491Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_491s_with_children() -> CustomEntity491Request {
        CustomEntity491Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_492s() -> CustomEntity492Request {
        CustomEntity492Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_492s_minimal() -> CustomEntity492Request {
        CustomEntity492Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_492s_with_children() -> CustomEntity492Request {
        CustomEntity492Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_493s() -> CustomEntity493Request {
        CustomEntity493Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_493s_minimal() -> CustomEntity493Request {
        CustomEntity493Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_493s_with_children() -> CustomEntity493Request {
        CustomEntity493Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_494s() -> CustomEntity494Request {
        CustomEntity494Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_494s_minimal() -> CustomEntity494Request {
        CustomEntity494Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_494s_with_children() -> CustomEntity494Request {
        CustomEntity494Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_495s() -> CustomEntity495Request {
        CustomEntity495Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_495s_minimal() -> CustomEntity495Request {
        CustomEntity495Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_495s_with_children() -> CustomEntity495Request {
        CustomEntity495Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_496s() -> CustomEntity496Request {
        CustomEntity496Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_496s_minimal() -> CustomEntity496Request {
        CustomEntity496Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_496s_with_children() -> CustomEntity496Request {
        CustomEntity496Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_497s() -> CustomEntity497Request {
        CustomEntity497Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_497s_minimal() -> CustomEntity497Request {
        CustomEntity497Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_497s_with_children() -> CustomEntity497Request {
        CustomEntity497Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_498s() -> CustomEntity498Request {
        CustomEntity498Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_498s_minimal() -> CustomEntity498Request {
        CustomEntity498Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_498s_with_children() -> CustomEntity498Request {
        CustomEntity498Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_499s() -> CustomEntity499Request {
        CustomEntity499Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_499s_minimal() -> CustomEntity499Request {
        CustomEntity499Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_499s_with_children() -> CustomEntity499Request {
        CustomEntity499Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_500s() -> CustomEntity500Request {
        CustomEntity500Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_500s_minimal() -> CustomEntity500Request {
        CustomEntity500Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_500s_with_children() -> CustomEntity500Request {
        CustomEntity500Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_501s() -> CustomEntity501Request {
        CustomEntity501Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_501s_minimal() -> CustomEntity501Request {
        CustomEntity501Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_501s_with_children() -> CustomEntity501Request {
        CustomEntity501Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_502s() -> CustomEntity502Request {
        CustomEntity502Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_502s_minimal() -> CustomEntity502Request {
        CustomEntity502Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_502s_with_children() -> CustomEntity502Request {
        CustomEntity502Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_503s() -> CustomEntity503Request {
        CustomEntity503Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_503s_minimal() -> CustomEntity503Request {
        CustomEntity503Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_503s_with_children() -> CustomEntity503Request {
        CustomEntity503Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_504s() -> CustomEntity504Request {
        CustomEntity504Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_504s_minimal() -> CustomEntity504Request {
        CustomEntity504Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_504s_with_children() -> CustomEntity504Request {
        CustomEntity504Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_505s() -> CustomEntity505Request {
        CustomEntity505Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_505s_minimal() -> CustomEntity505Request {
        CustomEntity505Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_505s_with_children() -> CustomEntity505Request {
        CustomEntity505Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_506s() -> CustomEntity506Request {
        CustomEntity506Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_506s_minimal() -> CustomEntity506Request {
        CustomEntity506Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_506s_with_children() -> CustomEntity506Request {
        CustomEntity506Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_507s() -> CustomEntity507Request {
        CustomEntity507Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_507s_minimal() -> CustomEntity507Request {
        CustomEntity507Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_507s_with_children() -> CustomEntity507Request {
        CustomEntity507Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_508s() -> CustomEntity508Request {
        CustomEntity508Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_508s_minimal() -> CustomEntity508Request {
        CustomEntity508Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_508s_with_children() -> CustomEntity508Request {
        CustomEntity508Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_509s() -> CustomEntity509Request {
        CustomEntity509Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_509s_minimal() -> CustomEntity509Request {
        CustomEntity509Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_509s_with_children() -> CustomEntity509Request {
        CustomEntity509Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_510s() -> CustomEntity510Request {
        CustomEntity510Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_510s_minimal() -> CustomEntity510Request {
        CustomEntity510Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_510s_with_children() -> CustomEntity510Request {
        CustomEntity510Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_511s() -> CustomEntity511Request {
        CustomEntity511Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_511s_minimal() -> CustomEntity511Request {
        CustomEntity511Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_511s_with_children() -> CustomEntity511Request {
        CustomEntity511Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_512s() -> CustomEntity512Request {
        CustomEntity512Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_512s_minimal() -> CustomEntity512Request {
        CustomEntity512Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_512s_with_children() -> CustomEntity512Request {
        CustomEntity512Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_513s() -> CustomEntity513Request {
        CustomEntity513Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_513s_minimal() -> CustomEntity513Request {
        CustomEntity513Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_513s_with_children() -> CustomEntity513Request {
        CustomEntity513Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_514s() -> CustomEntity514Request {
        CustomEntity514Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_514s_minimal() -> CustomEntity514Request {
        CustomEntity514Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_514s_with_children() -> CustomEntity514Request {
        CustomEntity514Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_515s() -> CustomEntity515Request {
        CustomEntity515Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_515s_minimal() -> CustomEntity515Request {
        CustomEntity515Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_515s_with_children() -> CustomEntity515Request {
        CustomEntity515Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_516s() -> CustomEntity516Request {
        CustomEntity516Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_516s_minimal() -> CustomEntity516Request {
        CustomEntity516Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_516s_with_children() -> CustomEntity516Request {
        CustomEntity516Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_517s() -> CustomEntity517Request {
        CustomEntity517Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_517s_minimal() -> CustomEntity517Request {
        CustomEntity517Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_517s_with_children() -> CustomEntity517Request {
        CustomEntity517Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_518s() -> CustomEntity518Request {
        CustomEntity518Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_518s_minimal() -> CustomEntity518Request {
        CustomEntity518Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_518s_with_children() -> CustomEntity518Request {
        CustomEntity518Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_519s() -> CustomEntity519Request {
        CustomEntity519Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_519s_minimal() -> CustomEntity519Request {
        CustomEntity519Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_519s_with_children() -> CustomEntity519Request {
        CustomEntity519Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_520s() -> CustomEntity520Request {
        CustomEntity520Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_520s_minimal() -> CustomEntity520Request {
        CustomEntity520Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_520s_with_children() -> CustomEntity520Request {
        CustomEntity520Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_521s() -> CustomEntity521Request {
        CustomEntity521Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_521s_minimal() -> CustomEntity521Request {
        CustomEntity521Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_521s_with_children() -> CustomEntity521Request {
        CustomEntity521Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_522s() -> CustomEntity522Request {
        CustomEntity522Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_522s_minimal() -> CustomEntity522Request {
        CustomEntity522Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_522s_with_children() -> CustomEntity522Request {
        CustomEntity522Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_523s() -> CustomEntity523Request {
        CustomEntity523Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_523s_minimal() -> CustomEntity523Request {
        CustomEntity523Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_523s_with_children() -> CustomEntity523Request {
        CustomEntity523Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_524s() -> CustomEntity524Request {
        CustomEntity524Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_524s_minimal() -> CustomEntity524Request {
        CustomEntity524Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_524s_with_children() -> CustomEntity524Request {
        CustomEntity524Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_525s() -> CustomEntity525Request {
        CustomEntity525Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_525s_minimal() -> CustomEntity525Request {
        CustomEntity525Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_525s_with_children() -> CustomEntity525Request {
        CustomEntity525Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_526s() -> CustomEntity526Request {
        CustomEntity526Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_526s_minimal() -> CustomEntity526Request {
        CustomEntity526Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_526s_with_children() -> CustomEntity526Request {
        CustomEntity526Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_527s() -> CustomEntity527Request {
        CustomEntity527Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_527s_minimal() -> CustomEntity527Request {
        CustomEntity527Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_527s_with_children() -> CustomEntity527Request {
        CustomEntity527Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_528s() -> CustomEntity528Request {
        CustomEntity528Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_528s_minimal() -> CustomEntity528Request {
        CustomEntity528Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_528s_with_children() -> CustomEntity528Request {
        CustomEntity528Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_529s() -> CustomEntity529Request {
        CustomEntity529Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_529s_minimal() -> CustomEntity529Request {
        CustomEntity529Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_529s_with_children() -> CustomEntity529Request {
        CustomEntity529Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_530s() -> CustomEntity530Request {
        CustomEntity530Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_530s_minimal() -> CustomEntity530Request {
        CustomEntity530Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_530s_with_children() -> CustomEntity530Request {
        CustomEntity530Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_531s() -> CustomEntity531Request {
        CustomEntity531Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_531s_minimal() -> CustomEntity531Request {
        CustomEntity531Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_531s_with_children() -> CustomEntity531Request {
        CustomEntity531Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_532s() -> CustomEntity532Request {
        CustomEntity532Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_532s_minimal() -> CustomEntity532Request {
        CustomEntity532Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_532s_with_children() -> CustomEntity532Request {
        CustomEntity532Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_533s() -> CustomEntity533Request {
        CustomEntity533Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_533s_minimal() -> CustomEntity533Request {
        CustomEntity533Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_533s_with_children() -> CustomEntity533Request {
        CustomEntity533Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_534s() -> CustomEntity534Request {
        CustomEntity534Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_534s_minimal() -> CustomEntity534Request {
        CustomEntity534Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_534s_with_children() -> CustomEntity534Request {
        CustomEntity534Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_535s() -> CustomEntity535Request {
        CustomEntity535Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_535s_minimal() -> CustomEntity535Request {
        CustomEntity535Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_535s_with_children() -> CustomEntity535Request {
        CustomEntity535Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_536s() -> CustomEntity536Request {
        CustomEntity536Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_536s_minimal() -> CustomEntity536Request {
        CustomEntity536Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_536s_with_children() -> CustomEntity536Request {
        CustomEntity536Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_537s() -> CustomEntity537Request {
        CustomEntity537Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_537s_minimal() -> CustomEntity537Request {
        CustomEntity537Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_537s_with_children() -> CustomEntity537Request {
        CustomEntity537Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_538s() -> CustomEntity538Request {
        CustomEntity538Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_538s_minimal() -> CustomEntity538Request {
        CustomEntity538Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_538s_with_children() -> CustomEntity538Request {
        CustomEntity538Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_539s() -> CustomEntity539Request {
        CustomEntity539Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_539s_minimal() -> CustomEntity539Request {
        CustomEntity539Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_539s_with_children() -> CustomEntity539Request {
        CustomEntity539Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_540s() -> CustomEntity540Request {
        CustomEntity540Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_540s_minimal() -> CustomEntity540Request {
        CustomEntity540Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_540s_with_children() -> CustomEntity540Request {
        CustomEntity540Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_541s() -> CustomEntity541Request {
        CustomEntity541Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_541s_minimal() -> CustomEntity541Request {
        CustomEntity541Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_541s_with_children() -> CustomEntity541Request {
        CustomEntity541Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_542s() -> CustomEntity542Request {
        CustomEntity542Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_542s_minimal() -> CustomEntity542Request {
        CustomEntity542Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_542s_with_children() -> CustomEntity542Request {
        CustomEntity542Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_543s() -> CustomEntity543Request {
        CustomEntity543Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_543s_minimal() -> CustomEntity543Request {
        CustomEntity543Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_543s_with_children() -> CustomEntity543Request {
        CustomEntity543Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_544s() -> CustomEntity544Request {
        CustomEntity544Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_544s_minimal() -> CustomEntity544Request {
        CustomEntity544Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_544s_with_children() -> CustomEntity544Request {
        CustomEntity544Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_545s() -> CustomEntity545Request {
        CustomEntity545Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_545s_minimal() -> CustomEntity545Request {
        CustomEntity545Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_545s_with_children() -> CustomEntity545Request {
        CustomEntity545Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_546s() -> CustomEntity546Request {
        CustomEntity546Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_546s_minimal() -> CustomEntity546Request {
        CustomEntity546Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_546s_with_children() -> CustomEntity546Request {
        CustomEntity546Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_547s() -> CustomEntity547Request {
        CustomEntity547Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_547s_minimal() -> CustomEntity547Request {
        CustomEntity547Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_547s_with_children() -> CustomEntity547Request {
        CustomEntity547Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_548s() -> CustomEntity548Request {
        CustomEntity548Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_548s_minimal() -> CustomEntity548Request {
        CustomEntity548Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_548s_with_children() -> CustomEntity548Request {
        CustomEntity548Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_549s() -> CustomEntity549Request {
        CustomEntity549Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_549s_minimal() -> CustomEntity549Request {
        CustomEntity549Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_549s_with_children() -> CustomEntity549Request {
        CustomEntity549Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_550s() -> CustomEntity550Request {
        CustomEntity550Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_550s_minimal() -> CustomEntity550Request {
        CustomEntity550Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_550s_with_children() -> CustomEntity550Request {
        CustomEntity550Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_551s() -> CustomEntity551Request {
        CustomEntity551Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_551s_minimal() -> CustomEntity551Request {
        CustomEntity551Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_551s_with_children() -> CustomEntity551Request {
        CustomEntity551Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_552s() -> CustomEntity552Request {
        CustomEntity552Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_552s_minimal() -> CustomEntity552Request {
        CustomEntity552Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_552s_with_children() -> CustomEntity552Request {
        CustomEntity552Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_553s() -> CustomEntity553Request {
        CustomEntity553Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_553s_minimal() -> CustomEntity553Request {
        CustomEntity553Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_553s_with_children() -> CustomEntity553Request {
        CustomEntity553Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_554s() -> CustomEntity554Request {
        CustomEntity554Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_554s_minimal() -> CustomEntity554Request {
        CustomEntity554Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_554s_with_children() -> CustomEntity554Request {
        CustomEntity554Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_555s() -> CustomEntity555Request {
        CustomEntity555Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_555s_minimal() -> CustomEntity555Request {
        CustomEntity555Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_555s_with_children() -> CustomEntity555Request {
        CustomEntity555Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_556s() -> CustomEntity556Request {
        CustomEntity556Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_556s_minimal() -> CustomEntity556Request {
        CustomEntity556Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_556s_with_children() -> CustomEntity556Request {
        CustomEntity556Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_557s() -> CustomEntity557Request {
        CustomEntity557Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_557s_minimal() -> CustomEntity557Request {
        CustomEntity557Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_557s_with_children() -> CustomEntity557Request {
        CustomEntity557Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_558s() -> CustomEntity558Request {
        CustomEntity558Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_558s_minimal() -> CustomEntity558Request {
        CustomEntity558Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_558s_with_children() -> CustomEntity558Request {
        CustomEntity558Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_559s() -> CustomEntity559Request {
        CustomEntity559Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_559s_minimal() -> CustomEntity559Request {
        CustomEntity559Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_559s_with_children() -> CustomEntity559Request {
        CustomEntity559Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_560s() -> CustomEntity560Request {
        CustomEntity560Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_560s_minimal() -> CustomEntity560Request {
        CustomEntity560Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_560s_with_children() -> CustomEntity560Request {
        CustomEntity560Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_561s() -> CustomEntity561Request {
        CustomEntity561Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_561s_minimal() -> CustomEntity561Request {
        CustomEntity561Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_561s_with_children() -> CustomEntity561Request {
        CustomEntity561Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_562s() -> CustomEntity562Request {
        CustomEntity562Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_562s_minimal() -> CustomEntity562Request {
        CustomEntity562Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_562s_with_children() -> CustomEntity562Request {
        CustomEntity562Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_563s() -> CustomEntity563Request {
        CustomEntity563Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_563s_minimal() -> CustomEntity563Request {
        CustomEntity563Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_563s_with_children() -> CustomEntity563Request {
        CustomEntity563Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_564s() -> CustomEntity564Request {
        CustomEntity564Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_564s_minimal() -> CustomEntity564Request {
        CustomEntity564Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_564s_with_children() -> CustomEntity564Request {
        CustomEntity564Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_565s() -> CustomEntity565Request {
        CustomEntity565Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_565s_minimal() -> CustomEntity565Request {
        CustomEntity565Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_565s_with_children() -> CustomEntity565Request {
        CustomEntity565Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_566s() -> CustomEntity566Request {
        CustomEntity566Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_566s_minimal() -> CustomEntity566Request {
        CustomEntity566Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_566s_with_children() -> CustomEntity566Request {
        CustomEntity566Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_567s() -> CustomEntity567Request {
        CustomEntity567Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_567s_minimal() -> CustomEntity567Request {
        CustomEntity567Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_567s_with_children() -> CustomEntity567Request {
        CustomEntity567Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_568s() -> CustomEntity568Request {
        CustomEntity568Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_568s_minimal() -> CustomEntity568Request {
        CustomEntity568Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_568s_with_children() -> CustomEntity568Request {
        CustomEntity568Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_569s() -> CustomEntity569Request {
        CustomEntity569Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_569s_minimal() -> CustomEntity569Request {
        CustomEntity569Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_569s_with_children() -> CustomEntity569Request {
        CustomEntity569Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_570s() -> CustomEntity570Request {
        CustomEntity570Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_570s_minimal() -> CustomEntity570Request {
        CustomEntity570Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_570s_with_children() -> CustomEntity570Request {
        CustomEntity570Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_571s() -> CustomEntity571Request {
        CustomEntity571Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_571s_minimal() -> CustomEntity571Request {
        CustomEntity571Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_571s_with_children() -> CustomEntity571Request {
        CustomEntity571Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_572s() -> CustomEntity572Request {
        CustomEntity572Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_572s_minimal() -> CustomEntity572Request {
        CustomEntity572Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_572s_with_children() -> CustomEntity572Request {
        CustomEntity572Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_573s() -> CustomEntity573Request {
        CustomEntity573Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_573s_minimal() -> CustomEntity573Request {
        CustomEntity573Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_573s_with_children() -> CustomEntity573Request {
        CustomEntity573Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_574s() -> CustomEntity574Request {
        CustomEntity574Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_574s_minimal() -> CustomEntity574Request {
        CustomEntity574Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_574s_with_children() -> CustomEntity574Request {
        CustomEntity574Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_575s() -> CustomEntity575Request {
        CustomEntity575Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_575s_minimal() -> CustomEntity575Request {
        CustomEntity575Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_575s_with_children() -> CustomEntity575Request {
        CustomEntity575Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_576s() -> CustomEntity576Request {
        CustomEntity576Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_576s_minimal() -> CustomEntity576Request {
        CustomEntity576Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_576s_with_children() -> CustomEntity576Request {
        CustomEntity576Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_577s() -> CustomEntity577Request {
        CustomEntity577Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_577s_minimal() -> CustomEntity577Request {
        CustomEntity577Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_577s_with_children() -> CustomEntity577Request {
        CustomEntity577Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_578s() -> CustomEntity578Request {
        CustomEntity578Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_578s_minimal() -> CustomEntity578Request {
        CustomEntity578Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_578s_with_children() -> CustomEntity578Request {
        CustomEntity578Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_579s() -> CustomEntity579Request {
        CustomEntity579Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_579s_minimal() -> CustomEntity579Request {
        CustomEntity579Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_579s_with_children() -> CustomEntity579Request {
        CustomEntity579Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_580s() -> CustomEntity580Request {
        CustomEntity580Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_580s_minimal() -> CustomEntity580Request {
        CustomEntity580Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_580s_with_children() -> CustomEntity580Request {
        CustomEntity580Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_581s() -> CustomEntity581Request {
        CustomEntity581Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_581s_minimal() -> CustomEntity581Request {
        CustomEntity581Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_581s_with_children() -> CustomEntity581Request {
        CustomEntity581Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_582s() -> CustomEntity582Request {
        CustomEntity582Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_582s_minimal() -> CustomEntity582Request {
        CustomEntity582Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_582s_with_children() -> CustomEntity582Request {
        CustomEntity582Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_583s() -> CustomEntity583Request {
        CustomEntity583Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_583s_minimal() -> CustomEntity583Request {
        CustomEntity583Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_583s_with_children() -> CustomEntity583Request {
        CustomEntity583Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_584s() -> CustomEntity584Request {
        CustomEntity584Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_584s_minimal() -> CustomEntity584Request {
        CustomEntity584Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_584s_with_children() -> CustomEntity584Request {
        CustomEntity584Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_585s() -> CustomEntity585Request {
        CustomEntity585Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_585s_minimal() -> CustomEntity585Request {
        CustomEntity585Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_585s_with_children() -> CustomEntity585Request {
        CustomEntity585Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_586s() -> CustomEntity586Request {
        CustomEntity586Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_586s_minimal() -> CustomEntity586Request {
        CustomEntity586Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_586s_with_children() -> CustomEntity586Request {
        CustomEntity586Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_587s() -> CustomEntity587Request {
        CustomEntity587Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_587s_minimal() -> CustomEntity587Request {
        CustomEntity587Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_587s_with_children() -> CustomEntity587Request {
        CustomEntity587Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_588s() -> CustomEntity588Request {
        CustomEntity588Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_588s_minimal() -> CustomEntity588Request {
        CustomEntity588Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_588s_with_children() -> CustomEntity588Request {
        CustomEntity588Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_589s() -> CustomEntity589Request {
        CustomEntity589Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_589s_minimal() -> CustomEntity589Request {
        CustomEntity589Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_589s_with_children() -> CustomEntity589Request {
        CustomEntity589Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_590s() -> CustomEntity590Request {
        CustomEntity590Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_590s_minimal() -> CustomEntity590Request {
        CustomEntity590Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_590s_with_children() -> CustomEntity590Request {
        CustomEntity590Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_591s() -> CustomEntity591Request {
        CustomEntity591Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_591s_minimal() -> CustomEntity591Request {
        CustomEntity591Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_591s_with_children() -> CustomEntity591Request {
        CustomEntity591Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_592s() -> CustomEntity592Request {
        CustomEntity592Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_592s_minimal() -> CustomEntity592Request {
        CustomEntity592Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_592s_with_children() -> CustomEntity592Request {
        CustomEntity592Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_593s() -> CustomEntity593Request {
        CustomEntity593Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_593s_minimal() -> CustomEntity593Request {
        CustomEntity593Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_593s_with_children() -> CustomEntity593Request {
        CustomEntity593Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_594s() -> CustomEntity594Request {
        CustomEntity594Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_594s_minimal() -> CustomEntity594Request {
        CustomEntity594Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_594s_with_children() -> CustomEntity594Request {
        CustomEntity594Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_595s() -> CustomEntity595Request {
        CustomEntity595Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_595s_minimal() -> CustomEntity595Request {
        CustomEntity595Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_595s_with_children() -> CustomEntity595Request {
        CustomEntity595Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_596s() -> CustomEntity596Request {
        CustomEntity596Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_596s_minimal() -> CustomEntity596Request {
        CustomEntity596Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_596s_with_children() -> CustomEntity596Request {
        CustomEntity596Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_597s() -> CustomEntity597Request {
        CustomEntity597Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_597s_minimal() -> CustomEntity597Request {
        CustomEntity597Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_597s_with_children() -> CustomEntity597Request {
        CustomEntity597Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_598s() -> CustomEntity598Request {
        CustomEntity598Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_598s_minimal() -> CustomEntity598Request {
        CustomEntity598Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_598s_with_children() -> CustomEntity598Request {
        CustomEntity598Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_599s() -> CustomEntity599Request {
        CustomEntity599Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_599s_minimal() -> CustomEntity599Request {
        CustomEntity599Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_599s_with_children() -> CustomEntity599Request {
        CustomEntity599Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_600s() -> CustomEntity600Request {
        CustomEntity600Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_600s_minimal() -> CustomEntity600Request {
        CustomEntity600Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_600s_with_children() -> CustomEntity600Request {
        CustomEntity600Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_601s() -> CustomEntity601Request {
        CustomEntity601Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_601s_minimal() -> CustomEntity601Request {
        CustomEntity601Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_601s_with_children() -> CustomEntity601Request {
        CustomEntity601Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_602s() -> CustomEntity602Request {
        CustomEntity602Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_602s_minimal() -> CustomEntity602Request {
        CustomEntity602Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_602s_with_children() -> CustomEntity602Request {
        CustomEntity602Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_603s() -> CustomEntity603Request {
        CustomEntity603Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_603s_minimal() -> CustomEntity603Request {
        CustomEntity603Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_603s_with_children() -> CustomEntity603Request {
        CustomEntity603Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_604s() -> CustomEntity604Request {
        CustomEntity604Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_604s_minimal() -> CustomEntity604Request {
        CustomEntity604Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_604s_with_children() -> CustomEntity604Request {
        CustomEntity604Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_605s() -> CustomEntity605Request {
        CustomEntity605Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_605s_minimal() -> CustomEntity605Request {
        CustomEntity605Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_605s_with_children() -> CustomEntity605Request {
        CustomEntity605Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_606s() -> CustomEntity606Request {
        CustomEntity606Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_606s_minimal() -> CustomEntity606Request {
        CustomEntity606Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_606s_with_children() -> CustomEntity606Request {
        CustomEntity606Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_607s() -> CustomEntity607Request {
        CustomEntity607Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_607s_minimal() -> CustomEntity607Request {
        CustomEntity607Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_607s_with_children() -> CustomEntity607Request {
        CustomEntity607Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_608s() -> CustomEntity608Request {
        CustomEntity608Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_608s_minimal() -> CustomEntity608Request {
        CustomEntity608Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_608s_with_children() -> CustomEntity608Request {
        CustomEntity608Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_609s() -> CustomEntity609Request {
        CustomEntity609Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_609s_minimal() -> CustomEntity609Request {
        CustomEntity609Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_609s_with_children() -> CustomEntity609Request {
        CustomEntity609Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_610s() -> CustomEntity610Request {
        CustomEntity610Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_610s_minimal() -> CustomEntity610Request {
        CustomEntity610Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_610s_with_children() -> CustomEntity610Request {
        CustomEntity610Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_611s() -> CustomEntity611Request {
        CustomEntity611Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_611s_minimal() -> CustomEntity611Request {
        CustomEntity611Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_611s_with_children() -> CustomEntity611Request {
        CustomEntity611Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_612s() -> CustomEntity612Request {
        CustomEntity612Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_612s_minimal() -> CustomEntity612Request {
        CustomEntity612Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_612s_with_children() -> CustomEntity612Request {
        CustomEntity612Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_613s() -> CustomEntity613Request {
        CustomEntity613Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_613s_minimal() -> CustomEntity613Request {
        CustomEntity613Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_613s_with_children() -> CustomEntity613Request {
        CustomEntity613Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_614s() -> CustomEntity614Request {
        CustomEntity614Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_614s_minimal() -> CustomEntity614Request {
        CustomEntity614Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_614s_with_children() -> CustomEntity614Request {
        CustomEntity614Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_615s() -> CustomEntity615Request {
        CustomEntity615Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_615s_minimal() -> CustomEntity615Request {
        CustomEntity615Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_615s_with_children() -> CustomEntity615Request {
        CustomEntity615Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_616s() -> CustomEntity616Request {
        CustomEntity616Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_616s_minimal() -> CustomEntity616Request {
        CustomEntity616Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_616s_with_children() -> CustomEntity616Request {
        CustomEntity616Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_617s() -> CustomEntity617Request {
        CustomEntity617Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_617s_minimal() -> CustomEntity617Request {
        CustomEntity617Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_617s_with_children() -> CustomEntity617Request {
        CustomEntity617Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_618s() -> CustomEntity618Request {
        CustomEntity618Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_618s_minimal() -> CustomEntity618Request {
        CustomEntity618Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_618s_with_children() -> CustomEntity618Request {
        CustomEntity618Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_619s() -> CustomEntity619Request {
        CustomEntity619Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_619s_minimal() -> CustomEntity619Request {
        CustomEntity619Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_619s_with_children() -> CustomEntity619Request {
        CustomEntity619Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_620s() -> CustomEntity620Request {
        CustomEntity620Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_620s_minimal() -> CustomEntity620Request {
        CustomEntity620Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_620s_with_children() -> CustomEntity620Request {
        CustomEntity620Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_621s() -> CustomEntity621Request {
        CustomEntity621Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_621s_minimal() -> CustomEntity621Request {
        CustomEntity621Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_621s_with_children() -> CustomEntity621Request {
        CustomEntity621Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_622s() -> CustomEntity622Request {
        CustomEntity622Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_622s_minimal() -> CustomEntity622Request {
        CustomEntity622Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_622s_with_children() -> CustomEntity622Request {
        CustomEntity622Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_623s() -> CustomEntity623Request {
        CustomEntity623Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_623s_minimal() -> CustomEntity623Request {
        CustomEntity623Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_623s_with_children() -> CustomEntity623Request {
        CustomEntity623Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_624s() -> CustomEntity624Request {
        CustomEntity624Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_624s_minimal() -> CustomEntity624Request {
        CustomEntity624Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_624s_with_children() -> CustomEntity624Request {
        CustomEntity624Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_625s() -> CustomEntity625Request {
        CustomEntity625Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_625s_minimal() -> CustomEntity625Request {
        CustomEntity625Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_625s_with_children() -> CustomEntity625Request {
        CustomEntity625Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_626s() -> CustomEntity626Request {
        CustomEntity626Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_626s_minimal() -> CustomEntity626Request {
        CustomEntity626Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_626s_with_children() -> CustomEntity626Request {
        CustomEntity626Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_627s() -> CustomEntity627Request {
        CustomEntity627Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_627s_minimal() -> CustomEntity627Request {
        CustomEntity627Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_627s_with_children() -> CustomEntity627Request {
        CustomEntity627Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_628s() -> CustomEntity628Request {
        CustomEntity628Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_628s_minimal() -> CustomEntity628Request {
        CustomEntity628Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_628s_with_children() -> CustomEntity628Request {
        CustomEntity628Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_629s() -> CustomEntity629Request {
        CustomEntity629Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_629s_minimal() -> CustomEntity629Request {
        CustomEntity629Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_629s_with_children() -> CustomEntity629Request {
        CustomEntity629Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_630s() -> CustomEntity630Request {
        CustomEntity630Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_630s_minimal() -> CustomEntity630Request {
        CustomEntity630Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_630s_with_children() -> CustomEntity630Request {
        CustomEntity630Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_631s() -> CustomEntity631Request {
        CustomEntity631Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_631s_minimal() -> CustomEntity631Request {
        CustomEntity631Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_631s_with_children() -> CustomEntity631Request {
        CustomEntity631Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_632s() -> CustomEntity632Request {
        CustomEntity632Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_632s_minimal() -> CustomEntity632Request {
        CustomEntity632Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_632s_with_children() -> CustomEntity632Request {
        CustomEntity632Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_633s() -> CustomEntity633Request {
        CustomEntity633Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_633s_minimal() -> CustomEntity633Request {
        CustomEntity633Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_633s_with_children() -> CustomEntity633Request {
        CustomEntity633Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_634s() -> CustomEntity634Request {
        CustomEntity634Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_634s_minimal() -> CustomEntity634Request {
        CustomEntity634Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_634s_with_children() -> CustomEntity634Request {
        CustomEntity634Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_635s() -> CustomEntity635Request {
        CustomEntity635Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_635s_minimal() -> CustomEntity635Request {
        CustomEntity635Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_635s_with_children() -> CustomEntity635Request {
        CustomEntity635Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_636s() -> CustomEntity636Request {
        CustomEntity636Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_636s_minimal() -> CustomEntity636Request {
        CustomEntity636Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_636s_with_children() -> CustomEntity636Request {
        CustomEntity636Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_637s() -> CustomEntity637Request {
        CustomEntity637Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_637s_minimal() -> CustomEntity637Request {
        CustomEntity637Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_637s_with_children() -> CustomEntity637Request {
        CustomEntity637Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_638s() -> CustomEntity638Request {
        CustomEntity638Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_638s_minimal() -> CustomEntity638Request {
        CustomEntity638Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_638s_with_children() -> CustomEntity638Request {
        CustomEntity638Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_639s() -> CustomEntity639Request {
        CustomEntity639Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_639s_minimal() -> CustomEntity639Request {
        CustomEntity639Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_639s_with_children() -> CustomEntity639Request {
        CustomEntity639Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_640s() -> CustomEntity640Request {
        CustomEntity640Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_640s_minimal() -> CustomEntity640Request {
        CustomEntity640Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_640s_with_children() -> CustomEntity640Request {
        CustomEntity640Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_641s() -> CustomEntity641Request {
        CustomEntity641Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_641s_minimal() -> CustomEntity641Request {
        CustomEntity641Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_641s_with_children() -> CustomEntity641Request {
        CustomEntity641Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_642s() -> CustomEntity642Request {
        CustomEntity642Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_642s_minimal() -> CustomEntity642Request {
        CustomEntity642Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_642s_with_children() -> CustomEntity642Request {
        CustomEntity642Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_643s() -> CustomEntity643Request {
        CustomEntity643Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_643s_minimal() -> CustomEntity643Request {
        CustomEntity643Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_643s_with_children() -> CustomEntity643Request {
        CustomEntity643Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_644s() -> CustomEntity644Request {
        CustomEntity644Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_644s_minimal() -> CustomEntity644Request {
        CustomEntity644Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_644s_with_children() -> CustomEntity644Request {
        CustomEntity644Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_645s() -> CustomEntity645Request {
        CustomEntity645Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_645s_minimal() -> CustomEntity645Request {
        CustomEntity645Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_645s_with_children() -> CustomEntity645Request {
        CustomEntity645Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_646s() -> CustomEntity646Request {
        CustomEntity646Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_646s_minimal() -> CustomEntity646Request {
        CustomEntity646Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_646s_with_children() -> CustomEntity646Request {
        CustomEntity646Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_647s() -> CustomEntity647Request {
        CustomEntity647Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_647s_minimal() -> CustomEntity647Request {
        CustomEntity647Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_647s_with_children() -> CustomEntity647Request {
        CustomEntity647Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_648s() -> CustomEntity648Request {
        CustomEntity648Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_648s_minimal() -> CustomEntity648Request {
        CustomEntity648Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_648s_with_children() -> CustomEntity648Request {
        CustomEntity648Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_649s() -> CustomEntity649Request {
        CustomEntity649Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_649s_minimal() -> CustomEntity649Request {
        CustomEntity649Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_649s_with_children() -> CustomEntity649Request {
        CustomEntity649Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_650s() -> CustomEntity650Request {
        CustomEntity650Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_650s_minimal() -> CustomEntity650Request {
        CustomEntity650Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_650s_with_children() -> CustomEntity650Request {
        CustomEntity650Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_651s() -> CustomEntity651Request {
        CustomEntity651Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_651s_minimal() -> CustomEntity651Request {
        CustomEntity651Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_651s_with_children() -> CustomEntity651Request {
        CustomEntity651Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_652s() -> CustomEntity652Request {
        CustomEntity652Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_652s_minimal() -> CustomEntity652Request {
        CustomEntity652Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_652s_with_children() -> CustomEntity652Request {
        CustomEntity652Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_653s() -> CustomEntity653Request {
        CustomEntity653Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_653s_minimal() -> CustomEntity653Request {
        CustomEntity653Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_653s_with_children() -> CustomEntity653Request {
        CustomEntity653Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_654s() -> CustomEntity654Request {
        CustomEntity654Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_654s_minimal() -> CustomEntity654Request {
        CustomEntity654Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_654s_with_children() -> CustomEntity654Request {
        CustomEntity654Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_655s() -> CustomEntity655Request {
        CustomEntity655Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_655s_minimal() -> CustomEntity655Request {
        CustomEntity655Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_655s_with_children() -> CustomEntity655Request {
        CustomEntity655Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_656s() -> CustomEntity656Request {
        CustomEntity656Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_656s_minimal() -> CustomEntity656Request {
        CustomEntity656Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_656s_with_children() -> CustomEntity656Request {
        CustomEntity656Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_657s() -> CustomEntity657Request {
        CustomEntity657Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_657s_minimal() -> CustomEntity657Request {
        CustomEntity657Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_657s_with_children() -> CustomEntity657Request {
        CustomEntity657Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_658s() -> CustomEntity658Request {
        CustomEntity658Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_658s_minimal() -> CustomEntity658Request {
        CustomEntity658Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_658s_with_children() -> CustomEntity658Request {
        CustomEntity658Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_659s() -> CustomEntity659Request {
        CustomEntity659Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_659s_minimal() -> CustomEntity659Request {
        CustomEntity659Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_659s_with_children() -> CustomEntity659Request {
        CustomEntity659Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_660s() -> CustomEntity660Request {
        CustomEntity660Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_660s_minimal() -> CustomEntity660Request {
        CustomEntity660Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_660s_with_children() -> CustomEntity660Request {
        CustomEntity660Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_661s() -> CustomEntity661Request {
        CustomEntity661Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_661s_minimal() -> CustomEntity661Request {
        CustomEntity661Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_661s_with_children() -> CustomEntity661Request {
        CustomEntity661Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_662s() -> CustomEntity662Request {
        CustomEntity662Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_662s_minimal() -> CustomEntity662Request {
        CustomEntity662Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_662s_with_children() -> CustomEntity662Request {
        CustomEntity662Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_663s() -> CustomEntity663Request {
        CustomEntity663Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_663s_minimal() -> CustomEntity663Request {
        CustomEntity663Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_663s_with_children() -> CustomEntity663Request {
        CustomEntity663Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_664s() -> CustomEntity664Request {
        CustomEntity664Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_664s_minimal() -> CustomEntity664Request {
        CustomEntity664Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_664s_with_children() -> CustomEntity664Request {
        CustomEntity664Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_665s() -> CustomEntity665Request {
        CustomEntity665Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_665s_minimal() -> CustomEntity665Request {
        CustomEntity665Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_665s_with_children() -> CustomEntity665Request {
        CustomEntity665Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_666s() -> CustomEntity666Request {
        CustomEntity666Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_666s_minimal() -> CustomEntity666Request {
        CustomEntity666Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_666s_with_children() -> CustomEntity666Request {
        CustomEntity666Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_667s() -> CustomEntity667Request {
        CustomEntity667Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_667s_minimal() -> CustomEntity667Request {
        CustomEntity667Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_667s_with_children() -> CustomEntity667Request {
        CustomEntity667Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_668s() -> CustomEntity668Request {
        CustomEntity668Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_668s_minimal() -> CustomEntity668Request {
        CustomEntity668Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_668s_with_children() -> CustomEntity668Request {
        CustomEntity668Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_669s() -> CustomEntity669Request {
        CustomEntity669Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_669s_minimal() -> CustomEntity669Request {
        CustomEntity669Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_669s_with_children() -> CustomEntity669Request {
        CustomEntity669Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_670s() -> CustomEntity670Request {
        CustomEntity670Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_670s_minimal() -> CustomEntity670Request {
        CustomEntity670Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_670s_with_children() -> CustomEntity670Request {
        CustomEntity670Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_671s() -> CustomEntity671Request {
        CustomEntity671Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_671s_minimal() -> CustomEntity671Request {
        CustomEntity671Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_671s_with_children() -> CustomEntity671Request {
        CustomEntity671Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_672s() -> CustomEntity672Request {
        CustomEntity672Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_672s_minimal() -> CustomEntity672Request {
        CustomEntity672Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_672s_with_children() -> CustomEntity672Request {
        CustomEntity672Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_673s() -> CustomEntity673Request {
        CustomEntity673Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_673s_minimal() -> CustomEntity673Request {
        CustomEntity673Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_673s_with_children() -> CustomEntity673Request {
        CustomEntity673Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_674s() -> CustomEntity674Request {
        CustomEntity674Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_674s_minimal() -> CustomEntity674Request {
        CustomEntity674Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_674s_with_children() -> CustomEntity674Request {
        CustomEntity674Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_675s() -> CustomEntity675Request {
        CustomEntity675Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_675s_minimal() -> CustomEntity675Request {
        CustomEntity675Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_675s_with_children() -> CustomEntity675Request {
        CustomEntity675Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_676s() -> CustomEntity676Request {
        CustomEntity676Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_676s_minimal() -> CustomEntity676Request {
        CustomEntity676Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_676s_with_children() -> CustomEntity676Request {
        CustomEntity676Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_677s() -> CustomEntity677Request {
        CustomEntity677Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_677s_minimal() -> CustomEntity677Request {
        CustomEntity677Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_677s_with_children() -> CustomEntity677Request {
        CustomEntity677Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_678s() -> CustomEntity678Request {
        CustomEntity678Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_678s_minimal() -> CustomEntity678Request {
        CustomEntity678Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_678s_with_children() -> CustomEntity678Request {
        CustomEntity678Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_679s() -> CustomEntity679Request {
        CustomEntity679Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_679s_minimal() -> CustomEntity679Request {
        CustomEntity679Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_679s_with_children() -> CustomEntity679Request {
        CustomEntity679Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_680s() -> CustomEntity680Request {
        CustomEntity680Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_680s_minimal() -> CustomEntity680Request {
        CustomEntity680Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_680s_with_children() -> CustomEntity680Request {
        CustomEntity680Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_681s() -> CustomEntity681Request {
        CustomEntity681Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_681s_minimal() -> CustomEntity681Request {
        CustomEntity681Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_681s_with_children() -> CustomEntity681Request {
        CustomEntity681Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_682s() -> CustomEntity682Request {
        CustomEntity682Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_682s_minimal() -> CustomEntity682Request {
        CustomEntity682Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_682s_with_children() -> CustomEntity682Request {
        CustomEntity682Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_683s() -> CustomEntity683Request {
        CustomEntity683Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_683s_minimal() -> CustomEntity683Request {
        CustomEntity683Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_683s_with_children() -> CustomEntity683Request {
        CustomEntity683Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_684s() -> CustomEntity684Request {
        CustomEntity684Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_684s_minimal() -> CustomEntity684Request {
        CustomEntity684Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_684s_with_children() -> CustomEntity684Request {
        CustomEntity684Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_685s() -> CustomEntity685Request {
        CustomEntity685Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_685s_minimal() -> CustomEntity685Request {
        CustomEntity685Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_685s_with_children() -> CustomEntity685Request {
        CustomEntity685Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_686s() -> CustomEntity686Request {
        CustomEntity686Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_686s_minimal() -> CustomEntity686Request {
        CustomEntity686Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_686s_with_children() -> CustomEntity686Request {
        CustomEntity686Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_687s() -> CustomEntity687Request {
        CustomEntity687Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_687s_minimal() -> CustomEntity687Request {
        CustomEntity687Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_687s_with_children() -> CustomEntity687Request {
        CustomEntity687Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_688s() -> CustomEntity688Request {
        CustomEntity688Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_688s_minimal() -> CustomEntity688Request {
        CustomEntity688Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_688s_with_children() -> CustomEntity688Request {
        CustomEntity688Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_689s() -> CustomEntity689Request {
        CustomEntity689Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_689s_minimal() -> CustomEntity689Request {
        CustomEntity689Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_689s_with_children() -> CustomEntity689Request {
        CustomEntity689Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_690s() -> CustomEntity690Request {
        CustomEntity690Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_690s_minimal() -> CustomEntity690Request {
        CustomEntity690Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_690s_with_children() -> CustomEntity690Request {
        CustomEntity690Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_691s() -> CustomEntity691Request {
        CustomEntity691Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_691s_minimal() -> CustomEntity691Request {
        CustomEntity691Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_691s_with_children() -> CustomEntity691Request {
        CustomEntity691Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_692s() -> CustomEntity692Request {
        CustomEntity692Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_692s_minimal() -> CustomEntity692Request {
        CustomEntity692Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_692s_with_children() -> CustomEntity692Request {
        CustomEntity692Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_693s() -> CustomEntity693Request {
        CustomEntity693Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_693s_minimal() -> CustomEntity693Request {
        CustomEntity693Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_693s_with_children() -> CustomEntity693Request {
        CustomEntity693Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_694s() -> CustomEntity694Request {
        CustomEntity694Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_694s_minimal() -> CustomEntity694Request {
        CustomEntity694Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_694s_with_children() -> CustomEntity694Request {
        CustomEntity694Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_695s() -> CustomEntity695Request {
        CustomEntity695Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_695s_minimal() -> CustomEntity695Request {
        CustomEntity695Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_695s_with_children() -> CustomEntity695Request {
        CustomEntity695Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_696s() -> CustomEntity696Request {
        CustomEntity696Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_696s_minimal() -> CustomEntity696Request {
        CustomEntity696Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_696s_with_children() -> CustomEntity696Request {
        CustomEntity696Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_697s() -> CustomEntity697Request {
        CustomEntity697Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_697s_minimal() -> CustomEntity697Request {
        CustomEntity697Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_697s_with_children() -> CustomEntity697Request {
        CustomEntity697Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_698s() -> CustomEntity698Request {
        CustomEntity698Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_698s_minimal() -> CustomEntity698Request {
        CustomEntity698Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_698s_with_children() -> CustomEntity698Request {
        CustomEntity698Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_699s() -> CustomEntity699Request {
        CustomEntity699Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_699s_minimal() -> CustomEntity699Request {
        CustomEntity699Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_699s_with_children() -> CustomEntity699Request {
        CustomEntity699Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_700s() -> CustomEntity700Request {
        CustomEntity700Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_700s_minimal() -> CustomEntity700Request {
        CustomEntity700Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_700s_with_children() -> CustomEntity700Request {
        CustomEntity700Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_701s() -> CustomEntity701Request {
        CustomEntity701Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_701s_minimal() -> CustomEntity701Request {
        CustomEntity701Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_701s_with_children() -> CustomEntity701Request {
        CustomEntity701Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_702s() -> CustomEntity702Request {
        CustomEntity702Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_702s_minimal() -> CustomEntity702Request {
        CustomEntity702Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_702s_with_children() -> CustomEntity702Request {
        CustomEntity702Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_703s() -> CustomEntity703Request {
        CustomEntity703Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_703s_minimal() -> CustomEntity703Request {
        CustomEntity703Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_703s_with_children() -> CustomEntity703Request {
        CustomEntity703Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_704s() -> CustomEntity704Request {
        CustomEntity704Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_704s_minimal() -> CustomEntity704Request {
        CustomEntity704Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_704s_with_children() -> CustomEntity704Request {
        CustomEntity704Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_705s() -> CustomEntity705Request {
        CustomEntity705Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_705s_minimal() -> CustomEntity705Request {
        CustomEntity705Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_705s_with_children() -> CustomEntity705Request {
        CustomEntity705Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_706s() -> CustomEntity706Request {
        CustomEntity706Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_706s_minimal() -> CustomEntity706Request {
        CustomEntity706Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_706s_with_children() -> CustomEntity706Request {
        CustomEntity706Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_707s() -> CustomEntity707Request {
        CustomEntity707Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_707s_minimal() -> CustomEntity707Request {
        CustomEntity707Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_707s_with_children() -> CustomEntity707Request {
        CustomEntity707Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_708s() -> CustomEntity708Request {
        CustomEntity708Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_708s_minimal() -> CustomEntity708Request {
        CustomEntity708Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_708s_with_children() -> CustomEntity708Request {
        CustomEntity708Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_709s() -> CustomEntity709Request {
        CustomEntity709Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_709s_minimal() -> CustomEntity709Request {
        CustomEntity709Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_709s_with_children() -> CustomEntity709Request {
        CustomEntity709Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_710s() -> CustomEntity710Request {
        CustomEntity710Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_710s_minimal() -> CustomEntity710Request {
        CustomEntity710Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_710s_with_children() -> CustomEntity710Request {
        CustomEntity710Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_711s() -> CustomEntity711Request {
        CustomEntity711Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_711s_minimal() -> CustomEntity711Request {
        CustomEntity711Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_711s_with_children() -> CustomEntity711Request {
        CustomEntity711Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_712s() -> CustomEntity712Request {
        CustomEntity712Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_712s_minimal() -> CustomEntity712Request {
        CustomEntity712Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_712s_with_children() -> CustomEntity712Request {
        CustomEntity712Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_713s() -> CustomEntity713Request {
        CustomEntity713Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_713s_minimal() -> CustomEntity713Request {
        CustomEntity713Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_713s_with_children() -> CustomEntity713Request {
        CustomEntity713Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_714s() -> CustomEntity714Request {
        CustomEntity714Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_714s_minimal() -> CustomEntity714Request {
        CustomEntity714Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_714s_with_children() -> CustomEntity714Request {
        CustomEntity714Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_715s() -> CustomEntity715Request {
        CustomEntity715Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_715s_minimal() -> CustomEntity715Request {
        CustomEntity715Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_715s_with_children() -> CustomEntity715Request {
        CustomEntity715Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_716s() -> CustomEntity716Request {
        CustomEntity716Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_716s_minimal() -> CustomEntity716Request {
        CustomEntity716Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_716s_with_children() -> CustomEntity716Request {
        CustomEntity716Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_717s() -> CustomEntity717Request {
        CustomEntity717Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_717s_minimal() -> CustomEntity717Request {
        CustomEntity717Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_717s_with_children() -> CustomEntity717Request {
        CustomEntity717Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_718s() -> CustomEntity718Request {
        CustomEntity718Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_718s_minimal() -> CustomEntity718Request {
        CustomEntity718Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_718s_with_children() -> CustomEntity718Request {
        CustomEntity718Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_719s() -> CustomEntity719Request {
        CustomEntity719Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_719s_minimal() -> CustomEntity719Request {
        CustomEntity719Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_719s_with_children() -> CustomEntity719Request {
        CustomEntity719Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_720s() -> CustomEntity720Request {
        CustomEntity720Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_720s_minimal() -> CustomEntity720Request {
        CustomEntity720Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_720s_with_children() -> CustomEntity720Request {
        CustomEntity720Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_721s() -> CustomEntity721Request {
        CustomEntity721Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_721s_minimal() -> CustomEntity721Request {
        CustomEntity721Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_721s_with_children() -> CustomEntity721Request {
        CustomEntity721Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_722s() -> CustomEntity722Request {
        CustomEntity722Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_722s_minimal() -> CustomEntity722Request {
        CustomEntity722Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_722s_with_children() -> CustomEntity722Request {
        CustomEntity722Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_723s() -> CustomEntity723Request {
        CustomEntity723Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_723s_minimal() -> CustomEntity723Request {
        CustomEntity723Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_723s_with_children() -> CustomEntity723Request {
        CustomEntity723Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_724s() -> CustomEntity724Request {
        CustomEntity724Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_724s_minimal() -> CustomEntity724Request {
        CustomEntity724Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_724s_with_children() -> CustomEntity724Request {
        CustomEntity724Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_725s() -> CustomEntity725Request {
        CustomEntity725Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_725s_minimal() -> CustomEntity725Request {
        CustomEntity725Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_725s_with_children() -> CustomEntity725Request {
        CustomEntity725Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_726s() -> CustomEntity726Request {
        CustomEntity726Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_726s_minimal() -> CustomEntity726Request {
        CustomEntity726Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_726s_with_children() -> CustomEntity726Request {
        CustomEntity726Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_727s() -> CustomEntity727Request {
        CustomEntity727Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_727s_minimal() -> CustomEntity727Request {
        CustomEntity727Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_727s_with_children() -> CustomEntity727Request {
        CustomEntity727Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_728s() -> CustomEntity728Request {
        CustomEntity728Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_728s_minimal() -> CustomEntity728Request {
        CustomEntity728Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_728s_with_children() -> CustomEntity728Request {
        CustomEntity728Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_729s() -> CustomEntity729Request {
        CustomEntity729Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_729s_minimal() -> CustomEntity729Request {
        CustomEntity729Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_729s_with_children() -> CustomEntity729Request {
        CustomEntity729Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_730s() -> CustomEntity730Request {
        CustomEntity730Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_730s_minimal() -> CustomEntity730Request {
        CustomEntity730Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_730s_with_children() -> CustomEntity730Request {
        CustomEntity730Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_731s() -> CustomEntity731Request {
        CustomEntity731Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_731s_minimal() -> CustomEntity731Request {
        CustomEntity731Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_731s_with_children() -> CustomEntity731Request {
        CustomEntity731Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_732s() -> CustomEntity732Request {
        CustomEntity732Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_732s_minimal() -> CustomEntity732Request {
        CustomEntity732Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_732s_with_children() -> CustomEntity732Request {
        CustomEntity732Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_733s() -> CustomEntity733Request {
        CustomEntity733Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_733s_minimal() -> CustomEntity733Request {
        CustomEntity733Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_733s_with_children() -> CustomEntity733Request {
        CustomEntity733Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_734s() -> CustomEntity734Request {
        CustomEntity734Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_734s_minimal() -> CustomEntity734Request {
        CustomEntity734Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_734s_with_children() -> CustomEntity734Request {
        CustomEntity734Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_735s() -> CustomEntity735Request {
        CustomEntity735Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_735s_minimal() -> CustomEntity735Request {
        CustomEntity735Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_735s_with_children() -> CustomEntity735Request {
        CustomEntity735Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_736s() -> CustomEntity736Request {
        CustomEntity736Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_736s_minimal() -> CustomEntity736Request {
        CustomEntity736Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_736s_with_children() -> CustomEntity736Request {
        CustomEntity736Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_737s() -> CustomEntity737Request {
        CustomEntity737Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_737s_minimal() -> CustomEntity737Request {
        CustomEntity737Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_737s_with_children() -> CustomEntity737Request {
        CustomEntity737Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_738s() -> CustomEntity738Request {
        CustomEntity738Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_738s_minimal() -> CustomEntity738Request {
        CustomEntity738Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_738s_with_children() -> CustomEntity738Request {
        CustomEntity738Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_739s() -> CustomEntity739Request {
        CustomEntity739Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_739s_minimal() -> CustomEntity739Request {
        CustomEntity739Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_739s_with_children() -> CustomEntity739Request {
        CustomEntity739Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_740s() -> CustomEntity740Request {
        CustomEntity740Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_740s_minimal() -> CustomEntity740Request {
        CustomEntity740Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_740s_with_children() -> CustomEntity740Request {
        CustomEntity740Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_741s() -> CustomEntity741Request {
        CustomEntity741Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_741s_minimal() -> CustomEntity741Request {
        CustomEntity741Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_741s_with_children() -> CustomEntity741Request {
        CustomEntity741Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_742s() -> CustomEntity742Request {
        CustomEntity742Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_742s_minimal() -> CustomEntity742Request {
        CustomEntity742Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_742s_with_children() -> CustomEntity742Request {
        CustomEntity742Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_743s() -> CustomEntity743Request {
        CustomEntity743Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_743s_minimal() -> CustomEntity743Request {
        CustomEntity743Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_743s_with_children() -> CustomEntity743Request {
        CustomEntity743Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_744s() -> CustomEntity744Request {
        CustomEntity744Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_744s_minimal() -> CustomEntity744Request {
        CustomEntity744Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_744s_with_children() -> CustomEntity744Request {
        CustomEntity744Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_745s() -> CustomEntity745Request {
        CustomEntity745Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_745s_minimal() -> CustomEntity745Request {
        CustomEntity745Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_745s_with_children() -> CustomEntity745Request {
        CustomEntity745Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_746s() -> CustomEntity746Request {
        CustomEntity746Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_746s_minimal() -> CustomEntity746Request {
        CustomEntity746Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_746s_with_children() -> CustomEntity746Request {
        CustomEntity746Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_747s() -> CustomEntity747Request {
        CustomEntity747Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_747s_minimal() -> CustomEntity747Request {
        CustomEntity747Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_747s_with_children() -> CustomEntity747Request {
        CustomEntity747Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_748s() -> CustomEntity748Request {
        CustomEntity748Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_748s_minimal() -> CustomEntity748Request {
        CustomEntity748Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_748s_with_children() -> CustomEntity748Request {
        CustomEntity748Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_749s() -> CustomEntity749Request {
        CustomEntity749Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_749s_minimal() -> CustomEntity749Request {
        CustomEntity749Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_749s_with_children() -> CustomEntity749Request {
        CustomEntity749Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_750s() -> CustomEntity750Request {
        CustomEntity750Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_750s_minimal() -> CustomEntity750Request {
        CustomEntity750Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_750s_with_children() -> CustomEntity750Request {
        CustomEntity750Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_751s() -> CustomEntity751Request {
        CustomEntity751Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_751s_minimal() -> CustomEntity751Request {
        CustomEntity751Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_751s_with_children() -> CustomEntity751Request {
        CustomEntity751Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_752s() -> CustomEntity752Request {
        CustomEntity752Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_752s_minimal() -> CustomEntity752Request {
        CustomEntity752Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_752s_with_children() -> CustomEntity752Request {
        CustomEntity752Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_753s() -> CustomEntity753Request {
        CustomEntity753Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_753s_minimal() -> CustomEntity753Request {
        CustomEntity753Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_753s_with_children() -> CustomEntity753Request {
        CustomEntity753Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_754s() -> CustomEntity754Request {
        CustomEntity754Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_754s_minimal() -> CustomEntity754Request {
        CustomEntity754Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_754s_with_children() -> CustomEntity754Request {
        CustomEntity754Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_755s() -> CustomEntity755Request {
        CustomEntity755Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_755s_minimal() -> CustomEntity755Request {
        CustomEntity755Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_755s_with_children() -> CustomEntity755Request {
        CustomEntity755Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_756s() -> CustomEntity756Request {
        CustomEntity756Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_756s_minimal() -> CustomEntity756Request {
        CustomEntity756Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_756s_with_children() -> CustomEntity756Request {
        CustomEntity756Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_757s() -> CustomEntity757Request {
        CustomEntity757Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_757s_minimal() -> CustomEntity757Request {
        CustomEntity757Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_757s_with_children() -> CustomEntity757Request {
        CustomEntity757Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_758s() -> CustomEntity758Request {
        CustomEntity758Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_758s_minimal() -> CustomEntity758Request {
        CustomEntity758Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_758s_with_children() -> CustomEntity758Request {
        CustomEntity758Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_759s() -> CustomEntity759Request {
        CustomEntity759Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_759s_minimal() -> CustomEntity759Request {
        CustomEntity759Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_759s_with_children() -> CustomEntity759Request {
        CustomEntity759Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_760s() -> CustomEntity760Request {
        CustomEntity760Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_760s_minimal() -> CustomEntity760Request {
        CustomEntity760Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_760s_with_children() -> CustomEntity760Request {
        CustomEntity760Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_761s() -> CustomEntity761Request {
        CustomEntity761Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_761s_minimal() -> CustomEntity761Request {
        CustomEntity761Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_761s_with_children() -> CustomEntity761Request {
        CustomEntity761Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_762s() -> CustomEntity762Request {
        CustomEntity762Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_762s_minimal() -> CustomEntity762Request {
        CustomEntity762Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_762s_with_children() -> CustomEntity762Request {
        CustomEntity762Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_763s() -> CustomEntity763Request {
        CustomEntity763Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_763s_minimal() -> CustomEntity763Request {
        CustomEntity763Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_763s_with_children() -> CustomEntity763Request {
        CustomEntity763Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_764s() -> CustomEntity764Request {
        CustomEntity764Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_764s_minimal() -> CustomEntity764Request {
        CustomEntity764Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_764s_with_children() -> CustomEntity764Request {
        CustomEntity764Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_765s() -> CustomEntity765Request {
        CustomEntity765Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_765s_minimal() -> CustomEntity765Request {
        CustomEntity765Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_765s_with_children() -> CustomEntity765Request {
        CustomEntity765Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_766s() -> CustomEntity766Request {
        CustomEntity766Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_766s_minimal() -> CustomEntity766Request {
        CustomEntity766Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_766s_with_children() -> CustomEntity766Request {
        CustomEntity766Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_767s() -> CustomEntity767Request {
        CustomEntity767Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_767s_minimal() -> CustomEntity767Request {
        CustomEntity767Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_767s_with_children() -> CustomEntity767Request {
        CustomEntity767Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_768s() -> CustomEntity768Request {
        CustomEntity768Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_768s_minimal() -> CustomEntity768Request {
        CustomEntity768Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_768s_with_children() -> CustomEntity768Request {
        CustomEntity768Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_769s() -> CustomEntity769Request {
        CustomEntity769Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_769s_minimal() -> CustomEntity769Request {
        CustomEntity769Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_769s_with_children() -> CustomEntity769Request {
        CustomEntity769Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_770s() -> CustomEntity770Request {
        CustomEntity770Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_770s_minimal() -> CustomEntity770Request {
        CustomEntity770Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_770s_with_children() -> CustomEntity770Request {
        CustomEntity770Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_771s() -> CustomEntity771Request {
        CustomEntity771Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_771s_minimal() -> CustomEntity771Request {
        CustomEntity771Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_771s_with_children() -> CustomEntity771Request {
        CustomEntity771Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_772s() -> CustomEntity772Request {
        CustomEntity772Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_772s_minimal() -> CustomEntity772Request {
        CustomEntity772Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_772s_with_children() -> CustomEntity772Request {
        CustomEntity772Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_773s() -> CustomEntity773Request {
        CustomEntity773Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_773s_minimal() -> CustomEntity773Request {
        CustomEntity773Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_773s_with_children() -> CustomEntity773Request {
        CustomEntity773Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_774s() -> CustomEntity774Request {
        CustomEntity774Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_774s_minimal() -> CustomEntity774Request {
        CustomEntity774Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_774s_with_children() -> CustomEntity774Request {
        CustomEntity774Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_775s() -> CustomEntity775Request {
        CustomEntity775Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_775s_minimal() -> CustomEntity775Request {
        CustomEntity775Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_775s_with_children() -> CustomEntity775Request {
        CustomEntity775Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_776s() -> CustomEntity776Request {
        CustomEntity776Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_776s_minimal() -> CustomEntity776Request {
        CustomEntity776Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_776s_with_children() -> CustomEntity776Request {
        CustomEntity776Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_777s() -> CustomEntity777Request {
        CustomEntity777Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_777s_minimal() -> CustomEntity777Request {
        CustomEntity777Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_777s_with_children() -> CustomEntity777Request {
        CustomEntity777Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_778s() -> CustomEntity778Request {
        CustomEntity778Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_778s_minimal() -> CustomEntity778Request {
        CustomEntity778Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_778s_with_children() -> CustomEntity778Request {
        CustomEntity778Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_779s() -> CustomEntity779Request {
        CustomEntity779Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_779s_minimal() -> CustomEntity779Request {
        CustomEntity779Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_779s_with_children() -> CustomEntity779Request {
        CustomEntity779Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_780s() -> CustomEntity780Request {
        CustomEntity780Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_780s_minimal() -> CustomEntity780Request {
        CustomEntity780Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_780s_with_children() -> CustomEntity780Request {
        CustomEntity780Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_781s() -> CustomEntity781Request {
        CustomEntity781Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_781s_minimal() -> CustomEntity781Request {
        CustomEntity781Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_781s_with_children() -> CustomEntity781Request {
        CustomEntity781Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_782s() -> CustomEntity782Request {
        CustomEntity782Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_782s_minimal() -> CustomEntity782Request {
        CustomEntity782Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_782s_with_children() -> CustomEntity782Request {
        CustomEntity782Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_783s() -> CustomEntity783Request {
        CustomEntity783Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_783s_minimal() -> CustomEntity783Request {
        CustomEntity783Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_783s_with_children() -> CustomEntity783Request {
        CustomEntity783Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_784s() -> CustomEntity784Request {
        CustomEntity784Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_784s_minimal() -> CustomEntity784Request {
        CustomEntity784Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_784s_with_children() -> CustomEntity784Request {
        CustomEntity784Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_785s() -> CustomEntity785Request {
        CustomEntity785Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_785s_minimal() -> CustomEntity785Request {
        CustomEntity785Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_785s_with_children() -> CustomEntity785Request {
        CustomEntity785Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_786s() -> CustomEntity786Request {
        CustomEntity786Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_786s_minimal() -> CustomEntity786Request {
        CustomEntity786Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_786s_with_children() -> CustomEntity786Request {
        CustomEntity786Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_787s() -> CustomEntity787Request {
        CustomEntity787Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_787s_minimal() -> CustomEntity787Request {
        CustomEntity787Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_787s_with_children() -> CustomEntity787Request {
        CustomEntity787Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_788s() -> CustomEntity788Request {
        CustomEntity788Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_788s_minimal() -> CustomEntity788Request {
        CustomEntity788Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_788s_with_children() -> CustomEntity788Request {
        CustomEntity788Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_789s() -> CustomEntity789Request {
        CustomEntity789Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_789s_minimal() -> CustomEntity789Request {
        CustomEntity789Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_789s_with_children() -> CustomEntity789Request {
        CustomEntity789Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_790s() -> CustomEntity790Request {
        CustomEntity790Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_790s_minimal() -> CustomEntity790Request {
        CustomEntity790Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_790s_with_children() -> CustomEntity790Request {
        CustomEntity790Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_791s() -> CustomEntity791Request {
        CustomEntity791Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_791s_minimal() -> CustomEntity791Request {
        CustomEntity791Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_791s_with_children() -> CustomEntity791Request {
        CustomEntity791Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_792s() -> CustomEntity792Request {
        CustomEntity792Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_792s_minimal() -> CustomEntity792Request {
        CustomEntity792Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_792s_with_children() -> CustomEntity792Request {
        CustomEntity792Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_793s() -> CustomEntity793Request {
        CustomEntity793Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_793s_minimal() -> CustomEntity793Request {
        CustomEntity793Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_793s_with_children() -> CustomEntity793Request {
        CustomEntity793Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_794s() -> CustomEntity794Request {
        CustomEntity794Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_794s_minimal() -> CustomEntity794Request {
        CustomEntity794Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_794s_with_children() -> CustomEntity794Request {
        CustomEntity794Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_795s() -> CustomEntity795Request {
        CustomEntity795Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_795s_minimal() -> CustomEntity795Request {
        CustomEntity795Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_795s_with_children() -> CustomEntity795Request {
        CustomEntity795Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_796s() -> CustomEntity796Request {
        CustomEntity796Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_796s_minimal() -> CustomEntity796Request {
        CustomEntity796Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_796s_with_children() -> CustomEntity796Request {
        CustomEntity796Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_797s() -> CustomEntity797Request {
        CustomEntity797Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_797s_minimal() -> CustomEntity797Request {
        CustomEntity797Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_797s_with_children() -> CustomEntity797Request {
        CustomEntity797Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_798s() -> CustomEntity798Request {
        CustomEntity798Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_798s_minimal() -> CustomEntity798Request {
        CustomEntity798Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_798s_with_children() -> CustomEntity798Request {
        CustomEntity798Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_799s() -> CustomEntity799Request {
        CustomEntity799Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_799s_minimal() -> CustomEntity799Request {
        CustomEntity799Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_799s_with_children() -> CustomEntity799Request {
        CustomEntity799Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_800s() -> CustomEntity800Request {
        CustomEntity800Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_800s_minimal() -> CustomEntity800Request {
        CustomEntity800Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_800s_with_children() -> CustomEntity800Request {
        CustomEntity800Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_801s() -> CustomEntity801Request {
        CustomEntity801Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_801s_minimal() -> CustomEntity801Request {
        CustomEntity801Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_801s_with_children() -> CustomEntity801Request {
        CustomEntity801Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_802s() -> CustomEntity802Request {
        CustomEntity802Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_802s_minimal() -> CustomEntity802Request {
        CustomEntity802Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_802s_with_children() -> CustomEntity802Request {
        CustomEntity802Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_803s() -> CustomEntity803Request {
        CustomEntity803Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_803s_minimal() -> CustomEntity803Request {
        CustomEntity803Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_803s_with_children() -> CustomEntity803Request {
        CustomEntity803Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_804s() -> CustomEntity804Request {
        CustomEntity804Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_804s_minimal() -> CustomEntity804Request {
        CustomEntity804Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_804s_with_children() -> CustomEntity804Request {
        CustomEntity804Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_805s() -> CustomEntity805Request {
        CustomEntity805Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_805s_minimal() -> CustomEntity805Request {
        CustomEntity805Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_805s_with_children() -> CustomEntity805Request {
        CustomEntity805Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_806s() -> CustomEntity806Request {
        CustomEntity806Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_806s_minimal() -> CustomEntity806Request {
        CustomEntity806Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_806s_with_children() -> CustomEntity806Request {
        CustomEntity806Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_807s() -> CustomEntity807Request {
        CustomEntity807Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_807s_minimal() -> CustomEntity807Request {
        CustomEntity807Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_807s_with_children() -> CustomEntity807Request {
        CustomEntity807Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_808s() -> CustomEntity808Request {
        CustomEntity808Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_808s_minimal() -> CustomEntity808Request {
        CustomEntity808Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_808s_with_children() -> CustomEntity808Request {
        CustomEntity808Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_809s() -> CustomEntity809Request {
        CustomEntity809Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_809s_minimal() -> CustomEntity809Request {
        CustomEntity809Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_809s_with_children() -> CustomEntity809Request {
        CustomEntity809Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_810s() -> CustomEntity810Request {
        CustomEntity810Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_810s_minimal() -> CustomEntity810Request {
        CustomEntity810Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_810s_with_children() -> CustomEntity810Request {
        CustomEntity810Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_811s() -> CustomEntity811Request {
        CustomEntity811Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_811s_minimal() -> CustomEntity811Request {
        CustomEntity811Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_811s_with_children() -> CustomEntity811Request {
        CustomEntity811Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_812s() -> CustomEntity812Request {
        CustomEntity812Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_812s_minimal() -> CustomEntity812Request {
        CustomEntity812Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_812s_with_children() -> CustomEntity812Request {
        CustomEntity812Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_813s() -> CustomEntity813Request {
        CustomEntity813Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_813s_minimal() -> CustomEntity813Request {
        CustomEntity813Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_813s_with_children() -> CustomEntity813Request {
        CustomEntity813Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_814s() -> CustomEntity814Request {
        CustomEntity814Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_814s_minimal() -> CustomEntity814Request {
        CustomEntity814Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_814s_with_children() -> CustomEntity814Request {
        CustomEntity814Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_815s() -> CustomEntity815Request {
        CustomEntity815Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_815s_minimal() -> CustomEntity815Request {
        CustomEntity815Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_815s_with_children() -> CustomEntity815Request {
        CustomEntity815Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_816s() -> CustomEntity816Request {
        CustomEntity816Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_816s_minimal() -> CustomEntity816Request {
        CustomEntity816Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_816s_with_children() -> CustomEntity816Request {
        CustomEntity816Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_817s() -> CustomEntity817Request {
        CustomEntity817Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_817s_minimal() -> CustomEntity817Request {
        CustomEntity817Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_817s_with_children() -> CustomEntity817Request {
        CustomEntity817Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_818s() -> CustomEntity818Request {
        CustomEntity818Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_818s_minimal() -> CustomEntity818Request {
        CustomEntity818Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_818s_with_children() -> CustomEntity818Request {
        CustomEntity818Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_819s() -> CustomEntity819Request {
        CustomEntity819Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_819s_minimal() -> CustomEntity819Request {
        CustomEntity819Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_819s_with_children() -> CustomEntity819Request {
        CustomEntity819Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_820s() -> CustomEntity820Request {
        CustomEntity820Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_820s_minimal() -> CustomEntity820Request {
        CustomEntity820Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_820s_with_children() -> CustomEntity820Request {
        CustomEntity820Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_821s() -> CustomEntity821Request {
        CustomEntity821Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_821s_minimal() -> CustomEntity821Request {
        CustomEntity821Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_821s_with_children() -> CustomEntity821Request {
        CustomEntity821Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_822s() -> CustomEntity822Request {
        CustomEntity822Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_822s_minimal() -> CustomEntity822Request {
        CustomEntity822Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_822s_with_children() -> CustomEntity822Request {
        CustomEntity822Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_823s() -> CustomEntity823Request {
        CustomEntity823Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_823s_minimal() -> CustomEntity823Request {
        CustomEntity823Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_823s_with_children() -> CustomEntity823Request {
        CustomEntity823Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_824s() -> CustomEntity824Request {
        CustomEntity824Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_824s_minimal() -> CustomEntity824Request {
        CustomEntity824Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_824s_with_children() -> CustomEntity824Request {
        CustomEntity824Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_825s() -> CustomEntity825Request {
        CustomEntity825Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_825s_minimal() -> CustomEntity825Request {
        CustomEntity825Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_825s_with_children() -> CustomEntity825Request {
        CustomEntity825Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_826s() -> CustomEntity826Request {
        CustomEntity826Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_826s_minimal() -> CustomEntity826Request {
        CustomEntity826Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_826s_with_children() -> CustomEntity826Request {
        CustomEntity826Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_827s() -> CustomEntity827Request {
        CustomEntity827Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_827s_minimal() -> CustomEntity827Request {
        CustomEntity827Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_827s_with_children() -> CustomEntity827Request {
        CustomEntity827Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_828s() -> CustomEntity828Request {
        CustomEntity828Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_828s_minimal() -> CustomEntity828Request {
        CustomEntity828Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_828s_with_children() -> CustomEntity828Request {
        CustomEntity828Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_829s() -> CustomEntity829Request {
        CustomEntity829Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_829s_minimal() -> CustomEntity829Request {
        CustomEntity829Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_829s_with_children() -> CustomEntity829Request {
        CustomEntity829Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_830s() -> CustomEntity830Request {
        CustomEntity830Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_830s_minimal() -> CustomEntity830Request {
        CustomEntity830Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_830s_with_children() -> CustomEntity830Request {
        CustomEntity830Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_831s() -> CustomEntity831Request {
        CustomEntity831Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_831s_minimal() -> CustomEntity831Request {
        CustomEntity831Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_831s_with_children() -> CustomEntity831Request {
        CustomEntity831Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_832s() -> CustomEntity832Request {
        CustomEntity832Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_832s_minimal() -> CustomEntity832Request {
        CustomEntity832Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_832s_with_children() -> CustomEntity832Request {
        CustomEntity832Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_833s() -> CustomEntity833Request {
        CustomEntity833Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_833s_minimal() -> CustomEntity833Request {
        CustomEntity833Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_833s_with_children() -> CustomEntity833Request {
        CustomEntity833Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_834s() -> CustomEntity834Request {
        CustomEntity834Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_834s_minimal() -> CustomEntity834Request {
        CustomEntity834Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_834s_with_children() -> CustomEntity834Request {
        CustomEntity834Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_835s() -> CustomEntity835Request {
        CustomEntity835Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_835s_minimal() -> CustomEntity835Request {
        CustomEntity835Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_835s_with_children() -> CustomEntity835Request {
        CustomEntity835Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_836s() -> CustomEntity836Request {
        CustomEntity836Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_836s_minimal() -> CustomEntity836Request {
        CustomEntity836Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_836s_with_children() -> CustomEntity836Request {
        CustomEntity836Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_837s() -> CustomEntity837Request {
        CustomEntity837Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_837s_minimal() -> CustomEntity837Request {
        CustomEntity837Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_837s_with_children() -> CustomEntity837Request {
        CustomEntity837Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_838s() -> CustomEntity838Request {
        CustomEntity838Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_838s_minimal() -> CustomEntity838Request {
        CustomEntity838Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_838s_with_children() -> CustomEntity838Request {
        CustomEntity838Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_839s() -> CustomEntity839Request {
        CustomEntity839Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_839s_minimal() -> CustomEntity839Request {
        CustomEntity839Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_839s_with_children() -> CustomEntity839Request {
        CustomEntity839Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_840s() -> CustomEntity840Request {
        CustomEntity840Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_840s_minimal() -> CustomEntity840Request {
        CustomEntity840Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_840s_with_children() -> CustomEntity840Request {
        CustomEntity840Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_841s() -> CustomEntity841Request {
        CustomEntity841Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_841s_minimal() -> CustomEntity841Request {
        CustomEntity841Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_841s_with_children() -> CustomEntity841Request {
        CustomEntity841Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_842s() -> CustomEntity842Request {
        CustomEntity842Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_842s_minimal() -> CustomEntity842Request {
        CustomEntity842Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_842s_with_children() -> CustomEntity842Request {
        CustomEntity842Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_843s() -> CustomEntity843Request {
        CustomEntity843Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_843s_minimal() -> CustomEntity843Request {
        CustomEntity843Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_843s_with_children() -> CustomEntity843Request {
        CustomEntity843Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_844s() -> CustomEntity844Request {
        CustomEntity844Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_844s_minimal() -> CustomEntity844Request {
        CustomEntity844Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_844s_with_children() -> CustomEntity844Request {
        CustomEntity844Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_845s() -> CustomEntity845Request {
        CustomEntity845Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_845s_minimal() -> CustomEntity845Request {
        CustomEntity845Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_845s_with_children() -> CustomEntity845Request {
        CustomEntity845Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_846s() -> CustomEntity846Request {
        CustomEntity846Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_846s_minimal() -> CustomEntity846Request {
        CustomEntity846Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_846s_with_children() -> CustomEntity846Request {
        CustomEntity846Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_847s() -> CustomEntity847Request {
        CustomEntity847Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_847s_minimal() -> CustomEntity847Request {
        CustomEntity847Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_847s_with_children() -> CustomEntity847Request {
        CustomEntity847Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_848s() -> CustomEntity848Request {
        CustomEntity848Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_848s_minimal() -> CustomEntity848Request {
        CustomEntity848Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_848s_with_children() -> CustomEntity848Request {
        CustomEntity848Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_849s() -> CustomEntity849Request {
        CustomEntity849Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_849s_minimal() -> CustomEntity849Request {
        CustomEntity849Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_849s_with_children() -> CustomEntity849Request {
        CustomEntity849Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_850s() -> CustomEntity850Request {
        CustomEntity850Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_850s_minimal() -> CustomEntity850Request {
        CustomEntity850Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_850s_with_children() -> CustomEntity850Request {
        CustomEntity850Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_851s() -> CustomEntity851Request {
        CustomEntity851Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_851s_minimal() -> CustomEntity851Request {
        CustomEntity851Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_851s_with_children() -> CustomEntity851Request {
        CustomEntity851Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_852s() -> CustomEntity852Request {
        CustomEntity852Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_852s_minimal() -> CustomEntity852Request {
        CustomEntity852Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_852s_with_children() -> CustomEntity852Request {
        CustomEntity852Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_853s() -> CustomEntity853Request {
        CustomEntity853Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_853s_minimal() -> CustomEntity853Request {
        CustomEntity853Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_853s_with_children() -> CustomEntity853Request {
        CustomEntity853Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_854s() -> CustomEntity854Request {
        CustomEntity854Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_854s_minimal() -> CustomEntity854Request {
        CustomEntity854Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_854s_with_children() -> CustomEntity854Request {
        CustomEntity854Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_855s() -> CustomEntity855Request {
        CustomEntity855Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_855s_minimal() -> CustomEntity855Request {
        CustomEntity855Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_855s_with_children() -> CustomEntity855Request {
        CustomEntity855Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_856s() -> CustomEntity856Request {
        CustomEntity856Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_856s_minimal() -> CustomEntity856Request {
        CustomEntity856Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_856s_with_children() -> CustomEntity856Request {
        CustomEntity856Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_857s() -> CustomEntity857Request {
        CustomEntity857Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_857s_minimal() -> CustomEntity857Request {
        CustomEntity857Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_857s_with_children() -> CustomEntity857Request {
        CustomEntity857Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_858s() -> CustomEntity858Request {
        CustomEntity858Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_858s_minimal() -> CustomEntity858Request {
        CustomEntity858Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_858s_with_children() -> CustomEntity858Request {
        CustomEntity858Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_859s() -> CustomEntity859Request {
        CustomEntity859Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_859s_minimal() -> CustomEntity859Request {
        CustomEntity859Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_859s_with_children() -> CustomEntity859Request {
        CustomEntity859Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_860s() -> CustomEntity860Request {
        CustomEntity860Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_860s_minimal() -> CustomEntity860Request {
        CustomEntity860Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_860s_with_children() -> CustomEntity860Request {
        CustomEntity860Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_861s() -> CustomEntity861Request {
        CustomEntity861Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_861s_minimal() -> CustomEntity861Request {
        CustomEntity861Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_861s_with_children() -> CustomEntity861Request {
        CustomEntity861Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_862s() -> CustomEntity862Request {
        CustomEntity862Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_862s_minimal() -> CustomEntity862Request {
        CustomEntity862Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_862s_with_children() -> CustomEntity862Request {
        CustomEntity862Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_863s() -> CustomEntity863Request {
        CustomEntity863Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_863s_minimal() -> CustomEntity863Request {
        CustomEntity863Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_863s_with_children() -> CustomEntity863Request {
        CustomEntity863Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_864s() -> CustomEntity864Request {
        CustomEntity864Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_864s_minimal() -> CustomEntity864Request {
        CustomEntity864Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_864s_with_children() -> CustomEntity864Request {
        CustomEntity864Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_865s() -> CustomEntity865Request {
        CustomEntity865Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_865s_minimal() -> CustomEntity865Request {
        CustomEntity865Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_865s_with_children() -> CustomEntity865Request {
        CustomEntity865Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_866s() -> CustomEntity866Request {
        CustomEntity866Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_866s_minimal() -> CustomEntity866Request {
        CustomEntity866Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_866s_with_children() -> CustomEntity866Request {
        CustomEntity866Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_867s() -> CustomEntity867Request {
        CustomEntity867Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_867s_minimal() -> CustomEntity867Request {
        CustomEntity867Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_867s_with_children() -> CustomEntity867Request {
        CustomEntity867Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_868s() -> CustomEntity868Request {
        CustomEntity868Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_868s_minimal() -> CustomEntity868Request {
        CustomEntity868Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_868s_with_children() -> CustomEntity868Request {
        CustomEntity868Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_869s() -> CustomEntity869Request {
        CustomEntity869Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_869s_minimal() -> CustomEntity869Request {
        CustomEntity869Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_869s_with_children() -> CustomEntity869Request {
        CustomEntity869Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_870s() -> CustomEntity870Request {
        CustomEntity870Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_870s_minimal() -> CustomEntity870Request {
        CustomEntity870Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_870s_with_children() -> CustomEntity870Request {
        CustomEntity870Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_871s() -> CustomEntity871Request {
        CustomEntity871Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_871s_minimal() -> CustomEntity871Request {
        CustomEntity871Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_871s_with_children() -> CustomEntity871Request {
        CustomEntity871Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_872s() -> CustomEntity872Request {
        CustomEntity872Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_872s_minimal() -> CustomEntity872Request {
        CustomEntity872Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_872s_with_children() -> CustomEntity872Request {
        CustomEntity872Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_873s() -> CustomEntity873Request {
        CustomEntity873Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_873s_minimal() -> CustomEntity873Request {
        CustomEntity873Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_873s_with_children() -> CustomEntity873Request {
        CustomEntity873Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_874s() -> CustomEntity874Request {
        CustomEntity874Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_874s_minimal() -> CustomEntity874Request {
        CustomEntity874Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_874s_with_children() -> CustomEntity874Request {
        CustomEntity874Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_875s() -> CustomEntity875Request {
        CustomEntity875Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_875s_minimal() -> CustomEntity875Request {
        CustomEntity875Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_875s_with_children() -> CustomEntity875Request {
        CustomEntity875Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_876s() -> CustomEntity876Request {
        CustomEntity876Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_876s_minimal() -> CustomEntity876Request {
        CustomEntity876Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_876s_with_children() -> CustomEntity876Request {
        CustomEntity876Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_877s() -> CustomEntity877Request {
        CustomEntity877Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_877s_minimal() -> CustomEntity877Request {
        CustomEntity877Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_877s_with_children() -> CustomEntity877Request {
        CustomEntity877Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_878s() -> CustomEntity878Request {
        CustomEntity878Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_878s_minimal() -> CustomEntity878Request {
        CustomEntity878Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_878s_with_children() -> CustomEntity878Request {
        CustomEntity878Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_879s() -> CustomEntity879Request {
        CustomEntity879Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_879s_minimal() -> CustomEntity879Request {
        CustomEntity879Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_879s_with_children() -> CustomEntity879Request {
        CustomEntity879Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_880s() -> CustomEntity880Request {
        CustomEntity880Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_880s_minimal() -> CustomEntity880Request {
        CustomEntity880Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_880s_with_children() -> CustomEntity880Request {
        CustomEntity880Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_881s() -> CustomEntity881Request {
        CustomEntity881Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_881s_minimal() -> CustomEntity881Request {
        CustomEntity881Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_881s_with_children() -> CustomEntity881Request {
        CustomEntity881Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_882s() -> CustomEntity882Request {
        CustomEntity882Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_882s_minimal() -> CustomEntity882Request {
        CustomEntity882Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_882s_with_children() -> CustomEntity882Request {
        CustomEntity882Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_883s() -> CustomEntity883Request {
        CustomEntity883Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_883s_minimal() -> CustomEntity883Request {
        CustomEntity883Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_883s_with_children() -> CustomEntity883Request {
        CustomEntity883Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_884s() -> CustomEntity884Request {
        CustomEntity884Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_884s_minimal() -> CustomEntity884Request {
        CustomEntity884Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_884s_with_children() -> CustomEntity884Request {
        CustomEntity884Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_885s() -> CustomEntity885Request {
        CustomEntity885Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_885s_minimal() -> CustomEntity885Request {
        CustomEntity885Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_885s_with_children() -> CustomEntity885Request {
        CustomEntity885Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_886s() -> CustomEntity886Request {
        CustomEntity886Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_886s_minimal() -> CustomEntity886Request {
        CustomEntity886Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_886s_with_children() -> CustomEntity886Request {
        CustomEntity886Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_887s() -> CustomEntity887Request {
        CustomEntity887Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_887s_minimal() -> CustomEntity887Request {
        CustomEntity887Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_887s_with_children() -> CustomEntity887Request {
        CustomEntity887Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_888s() -> CustomEntity888Request {
        CustomEntity888Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_888s_minimal() -> CustomEntity888Request {
        CustomEntity888Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_888s_with_children() -> CustomEntity888Request {
        CustomEntity888Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_889s() -> CustomEntity889Request {
        CustomEntity889Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_889s_minimal() -> CustomEntity889Request {
        CustomEntity889Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_889s_with_children() -> CustomEntity889Request {
        CustomEntity889Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_890s() -> CustomEntity890Request {
        CustomEntity890Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_890s_minimal() -> CustomEntity890Request {
        CustomEntity890Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_890s_with_children() -> CustomEntity890Request {
        CustomEntity890Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_891s() -> CustomEntity891Request {
        CustomEntity891Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_891s_minimal() -> CustomEntity891Request {
        CustomEntity891Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_891s_with_children() -> CustomEntity891Request {
        CustomEntity891Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_892s() -> CustomEntity892Request {
        CustomEntity892Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_892s_minimal() -> CustomEntity892Request {
        CustomEntity892Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_892s_with_children() -> CustomEntity892Request {
        CustomEntity892Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_893s() -> CustomEntity893Request {
        CustomEntity893Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_893s_minimal() -> CustomEntity893Request {
        CustomEntity893Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_893s_with_children() -> CustomEntity893Request {
        CustomEntity893Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_894s() -> CustomEntity894Request {
        CustomEntity894Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_894s_minimal() -> CustomEntity894Request {
        CustomEntity894Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_894s_with_children() -> CustomEntity894Request {
        CustomEntity894Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_895s() -> CustomEntity895Request {
        CustomEntity895Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_895s_minimal() -> CustomEntity895Request {
        CustomEntity895Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_895s_with_children() -> CustomEntity895Request {
        CustomEntity895Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_896s() -> CustomEntity896Request {
        CustomEntity896Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_896s_minimal() -> CustomEntity896Request {
        CustomEntity896Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_896s_with_children() -> CustomEntity896Request {
        CustomEntity896Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_897s() -> CustomEntity897Request {
        CustomEntity897Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_897s_minimal() -> CustomEntity897Request {
        CustomEntity897Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_897s_with_children() -> CustomEntity897Request {
        CustomEntity897Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_898s() -> CustomEntity898Request {
        CustomEntity898Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_898s_minimal() -> CustomEntity898Request {
        CustomEntity898Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_898s_with_children() -> CustomEntity898Request {
        CustomEntity898Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_899s() -> CustomEntity899Request {
        CustomEntity899Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_899s_minimal() -> CustomEntity899Request {
        CustomEntity899Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_899s_with_children() -> CustomEntity899Request {
        CustomEntity899Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_900s() -> CustomEntity900Request {
        CustomEntity900Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_900s_minimal() -> CustomEntity900Request {
        CustomEntity900Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_900s_with_children() -> CustomEntity900Request {
        CustomEntity900Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_901s() -> CustomEntity901Request {
        CustomEntity901Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_901s_minimal() -> CustomEntity901Request {
        CustomEntity901Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_901s_with_children() -> CustomEntity901Request {
        CustomEntity901Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_902s() -> CustomEntity902Request {
        CustomEntity902Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_902s_minimal() -> CustomEntity902Request {
        CustomEntity902Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_902s_with_children() -> CustomEntity902Request {
        CustomEntity902Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_903s() -> CustomEntity903Request {
        CustomEntity903Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_903s_minimal() -> CustomEntity903Request {
        CustomEntity903Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_903s_with_children() -> CustomEntity903Request {
        CustomEntity903Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_904s() -> CustomEntity904Request {
        CustomEntity904Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_904s_minimal() -> CustomEntity904Request {
        CustomEntity904Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_904s_with_children() -> CustomEntity904Request {
        CustomEntity904Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_905s() -> CustomEntity905Request {
        CustomEntity905Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_905s_minimal() -> CustomEntity905Request {
        CustomEntity905Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_905s_with_children() -> CustomEntity905Request {
        CustomEntity905Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_906s() -> CustomEntity906Request {
        CustomEntity906Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_906s_minimal() -> CustomEntity906Request {
        CustomEntity906Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_906s_with_children() -> CustomEntity906Request {
        CustomEntity906Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_907s() -> CustomEntity907Request {
        CustomEntity907Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_907s_minimal() -> CustomEntity907Request {
        CustomEntity907Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_907s_with_children() -> CustomEntity907Request {
        CustomEntity907Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_908s() -> CustomEntity908Request {
        CustomEntity908Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_908s_minimal() -> CustomEntity908Request {
        CustomEntity908Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_908s_with_children() -> CustomEntity908Request {
        CustomEntity908Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_909s() -> CustomEntity909Request {
        CustomEntity909Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_909s_minimal() -> CustomEntity909Request {
        CustomEntity909Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_909s_with_children() -> CustomEntity909Request {
        CustomEntity909Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_910s() -> CustomEntity910Request {
        CustomEntity910Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_910s_minimal() -> CustomEntity910Request {
        CustomEntity910Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_910s_with_children() -> CustomEntity910Request {
        CustomEntity910Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_911s() -> CustomEntity911Request {
        CustomEntity911Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_911s_minimal() -> CustomEntity911Request {
        CustomEntity911Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_911s_with_children() -> CustomEntity911Request {
        CustomEntity911Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_912s() -> CustomEntity912Request {
        CustomEntity912Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_912s_minimal() -> CustomEntity912Request {
        CustomEntity912Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_912s_with_children() -> CustomEntity912Request {
        CustomEntity912Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_913s() -> CustomEntity913Request {
        CustomEntity913Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_913s_minimal() -> CustomEntity913Request {
        CustomEntity913Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_913s_with_children() -> CustomEntity913Request {
        CustomEntity913Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_914s() -> CustomEntity914Request {
        CustomEntity914Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_914s_minimal() -> CustomEntity914Request {
        CustomEntity914Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_914s_with_children() -> CustomEntity914Request {
        CustomEntity914Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_915s() -> CustomEntity915Request {
        CustomEntity915Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_915s_minimal() -> CustomEntity915Request {
        CustomEntity915Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_915s_with_children() -> CustomEntity915Request {
        CustomEntity915Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_916s() -> CustomEntity916Request {
        CustomEntity916Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_916s_minimal() -> CustomEntity916Request {
        CustomEntity916Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_916s_with_children() -> CustomEntity916Request {
        CustomEntity916Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_917s() -> CustomEntity917Request {
        CustomEntity917Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_917s_minimal() -> CustomEntity917Request {
        CustomEntity917Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_917s_with_children() -> CustomEntity917Request {
        CustomEntity917Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_918s() -> CustomEntity918Request {
        CustomEntity918Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_918s_minimal() -> CustomEntity918Request {
        CustomEntity918Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_918s_with_children() -> CustomEntity918Request {
        CustomEntity918Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_919s() -> CustomEntity919Request {
        CustomEntity919Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_919s_minimal() -> CustomEntity919Request {
        CustomEntity919Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_919s_with_children() -> CustomEntity919Request {
        CustomEntity919Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_920s() -> CustomEntity920Request {
        CustomEntity920Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_920s_minimal() -> CustomEntity920Request {
        CustomEntity920Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_920s_with_children() -> CustomEntity920Request {
        CustomEntity920Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_921s() -> CustomEntity921Request {
        CustomEntity921Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_921s_minimal() -> CustomEntity921Request {
        CustomEntity921Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_921s_with_children() -> CustomEntity921Request {
        CustomEntity921Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_922s() -> CustomEntity922Request {
        CustomEntity922Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_922s_minimal() -> CustomEntity922Request {
        CustomEntity922Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_922s_with_children() -> CustomEntity922Request {
        CustomEntity922Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_923s() -> CustomEntity923Request {
        CustomEntity923Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_923s_minimal() -> CustomEntity923Request {
        CustomEntity923Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_923s_with_children() -> CustomEntity923Request {
        CustomEntity923Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_924s() -> CustomEntity924Request {
        CustomEntity924Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_924s_minimal() -> CustomEntity924Request {
        CustomEntity924Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_924s_with_children() -> CustomEntity924Request {
        CustomEntity924Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_925s() -> CustomEntity925Request {
        CustomEntity925Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_925s_minimal() -> CustomEntity925Request {
        CustomEntity925Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_925s_with_children() -> CustomEntity925Request {
        CustomEntity925Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_926s() -> CustomEntity926Request {
        CustomEntity926Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_926s_minimal() -> CustomEntity926Request {
        CustomEntity926Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_926s_with_children() -> CustomEntity926Request {
        CustomEntity926Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_927s() -> CustomEntity927Request {
        CustomEntity927Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_927s_minimal() -> CustomEntity927Request {
        CustomEntity927Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_927s_with_children() -> CustomEntity927Request {
        CustomEntity927Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_928s() -> CustomEntity928Request {
        CustomEntity928Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_928s_minimal() -> CustomEntity928Request {
        CustomEntity928Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_928s_with_children() -> CustomEntity928Request {
        CustomEntity928Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_929s() -> CustomEntity929Request {
        CustomEntity929Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_929s_minimal() -> CustomEntity929Request {
        CustomEntity929Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_929s_with_children() -> CustomEntity929Request {
        CustomEntity929Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_930s() -> CustomEntity930Request {
        CustomEntity930Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_930s_minimal() -> CustomEntity930Request {
        CustomEntity930Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_930s_with_children() -> CustomEntity930Request {
        CustomEntity930Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_931s() -> CustomEntity931Request {
        CustomEntity931Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_931s_minimal() -> CustomEntity931Request {
        CustomEntity931Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_931s_with_children() -> CustomEntity931Request {
        CustomEntity931Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_932s() -> CustomEntity932Request {
        CustomEntity932Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_932s_minimal() -> CustomEntity932Request {
        CustomEntity932Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_932s_with_children() -> CustomEntity932Request {
        CustomEntity932Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_933s() -> CustomEntity933Request {
        CustomEntity933Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_933s_minimal() -> CustomEntity933Request {
        CustomEntity933Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_933s_with_children() -> CustomEntity933Request {
        CustomEntity933Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_934s() -> CustomEntity934Request {
        CustomEntity934Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_934s_minimal() -> CustomEntity934Request {
        CustomEntity934Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_934s_with_children() -> CustomEntity934Request {
        CustomEntity934Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_935s() -> CustomEntity935Request {
        CustomEntity935Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_935s_minimal() -> CustomEntity935Request {
        CustomEntity935Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_935s_with_children() -> CustomEntity935Request {
        CustomEntity935Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_936s() -> CustomEntity936Request {
        CustomEntity936Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_936s_minimal() -> CustomEntity936Request {
        CustomEntity936Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_936s_with_children() -> CustomEntity936Request {
        CustomEntity936Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_937s() -> CustomEntity937Request {
        CustomEntity937Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_937s_minimal() -> CustomEntity937Request {
        CustomEntity937Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_937s_with_children() -> CustomEntity937Request {
        CustomEntity937Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_938s() -> CustomEntity938Request {
        CustomEntity938Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_938s_minimal() -> CustomEntity938Request {
        CustomEntity938Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_938s_with_children() -> CustomEntity938Request {
        CustomEntity938Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_939s() -> CustomEntity939Request {
        CustomEntity939Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_939s_minimal() -> CustomEntity939Request {
        CustomEntity939Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_939s_with_children() -> CustomEntity939Request {
        CustomEntity939Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_940s() -> CustomEntity940Request {
        CustomEntity940Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_940s_minimal() -> CustomEntity940Request {
        CustomEntity940Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_940s_with_children() -> CustomEntity940Request {
        CustomEntity940Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_941s() -> CustomEntity941Request {
        CustomEntity941Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_941s_minimal() -> CustomEntity941Request {
        CustomEntity941Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_941s_with_children() -> CustomEntity941Request {
        CustomEntity941Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_942s() -> CustomEntity942Request {
        CustomEntity942Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_942s_minimal() -> CustomEntity942Request {
        CustomEntity942Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_942s_with_children() -> CustomEntity942Request {
        CustomEntity942Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_943s() -> CustomEntity943Request {
        CustomEntity943Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_943s_minimal() -> CustomEntity943Request {
        CustomEntity943Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_943s_with_children() -> CustomEntity943Request {
        CustomEntity943Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_944s() -> CustomEntity944Request {
        CustomEntity944Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_944s_minimal() -> CustomEntity944Request {
        CustomEntity944Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_944s_with_children() -> CustomEntity944Request {
        CustomEntity944Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_945s() -> CustomEntity945Request {
        CustomEntity945Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_945s_minimal() -> CustomEntity945Request {
        CustomEntity945Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_945s_with_children() -> CustomEntity945Request {
        CustomEntity945Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_946s() -> CustomEntity946Request {
        CustomEntity946Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_946s_minimal() -> CustomEntity946Request {
        CustomEntity946Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_946s_with_children() -> CustomEntity946Request {
        CustomEntity946Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_947s() -> CustomEntity947Request {
        CustomEntity947Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_947s_minimal() -> CustomEntity947Request {
        CustomEntity947Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_947s_with_children() -> CustomEntity947Request {
        CustomEntity947Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_948s() -> CustomEntity948Request {
        CustomEntity948Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_948s_minimal() -> CustomEntity948Request {
        CustomEntity948Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_948s_with_children() -> CustomEntity948Request {
        CustomEntity948Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_949s() -> CustomEntity949Request {
        CustomEntity949Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_949s_minimal() -> CustomEntity949Request {
        CustomEntity949Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_949s_with_children() -> CustomEntity949Request {
        CustomEntity949Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_950s() -> CustomEntity950Request {
        CustomEntity950Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_950s_minimal() -> CustomEntity950Request {
        CustomEntity950Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_950s_with_children() -> CustomEntity950Request {
        CustomEntity950Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_951s() -> CustomEntity951Request {
        CustomEntity951Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_951s_minimal() -> CustomEntity951Request {
        CustomEntity951Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_951s_with_children() -> CustomEntity951Request {
        CustomEntity951Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_952s() -> CustomEntity952Request {
        CustomEntity952Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_952s_minimal() -> CustomEntity952Request {
        CustomEntity952Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_952s_with_children() -> CustomEntity952Request {
        CustomEntity952Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_953s() -> CustomEntity953Request {
        CustomEntity953Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_953s_minimal() -> CustomEntity953Request {
        CustomEntity953Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_953s_with_children() -> CustomEntity953Request {
        CustomEntity953Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_954s() -> CustomEntity954Request {
        CustomEntity954Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_954s_minimal() -> CustomEntity954Request {
        CustomEntity954Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_954s_with_children() -> CustomEntity954Request {
        CustomEntity954Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_955s() -> CustomEntity955Request {
        CustomEntity955Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_955s_minimal() -> CustomEntity955Request {
        CustomEntity955Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_955s_with_children() -> CustomEntity955Request {
        CustomEntity955Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_956s() -> CustomEntity956Request {
        CustomEntity956Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_956s_minimal() -> CustomEntity956Request {
        CustomEntity956Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_956s_with_children() -> CustomEntity956Request {
        CustomEntity956Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_957s() -> CustomEntity957Request {
        CustomEntity957Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_957s_minimal() -> CustomEntity957Request {
        CustomEntity957Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_957s_with_children() -> CustomEntity957Request {
        CustomEntity957Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_958s() -> CustomEntity958Request {
        CustomEntity958Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_958s_minimal() -> CustomEntity958Request {
        CustomEntity958Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_958s_with_children() -> CustomEntity958Request {
        CustomEntity958Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_959s() -> CustomEntity959Request {
        CustomEntity959Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_959s_minimal() -> CustomEntity959Request {
        CustomEntity959Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_959s_with_children() -> CustomEntity959Request {
        CustomEntity959Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_960s() -> CustomEntity960Request {
        CustomEntity960Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_960s_minimal() -> CustomEntity960Request {
        CustomEntity960Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_960s_with_children() -> CustomEntity960Request {
        CustomEntity960Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_961s() -> CustomEntity961Request {
        CustomEntity961Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_961s_minimal() -> CustomEntity961Request {
        CustomEntity961Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_961s_with_children() -> CustomEntity961Request {
        CustomEntity961Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_962s() -> CustomEntity962Request {
        CustomEntity962Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_962s_minimal() -> CustomEntity962Request {
        CustomEntity962Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_962s_with_children() -> CustomEntity962Request {
        CustomEntity962Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_963s() -> CustomEntity963Request {
        CustomEntity963Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_963s_minimal() -> CustomEntity963Request {
        CustomEntity963Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_963s_with_children() -> CustomEntity963Request {
        CustomEntity963Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_964s() -> CustomEntity964Request {
        CustomEntity964Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_964s_minimal() -> CustomEntity964Request {
        CustomEntity964Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_964s_with_children() -> CustomEntity964Request {
        CustomEntity964Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_965s() -> CustomEntity965Request {
        CustomEntity965Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_965s_minimal() -> CustomEntity965Request {
        CustomEntity965Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_965s_with_children() -> CustomEntity965Request {
        CustomEntity965Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_966s() -> CustomEntity966Request {
        CustomEntity966Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_966s_minimal() -> CustomEntity966Request {
        CustomEntity966Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_966s_with_children() -> CustomEntity966Request {
        CustomEntity966Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_967s() -> CustomEntity967Request {
        CustomEntity967Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_967s_minimal() -> CustomEntity967Request {
        CustomEntity967Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_967s_with_children() -> CustomEntity967Request {
        CustomEntity967Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_968s() -> CustomEntity968Request {
        CustomEntity968Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_968s_minimal() -> CustomEntity968Request {
        CustomEntity968Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_968s_with_children() -> CustomEntity968Request {
        CustomEntity968Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_969s() -> CustomEntity969Request {
        CustomEntity969Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_969s_minimal() -> CustomEntity969Request {
        CustomEntity969Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_969s_with_children() -> CustomEntity969Request {
        CustomEntity969Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_970s() -> CustomEntity970Request {
        CustomEntity970Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_970s_minimal() -> CustomEntity970Request {
        CustomEntity970Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_970s_with_children() -> CustomEntity970Request {
        CustomEntity970Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_971s() -> CustomEntity971Request {
        CustomEntity971Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_971s_minimal() -> CustomEntity971Request {
        CustomEntity971Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_971s_with_children() -> CustomEntity971Request {
        CustomEntity971Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_972s() -> CustomEntity972Request {
        CustomEntity972Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_972s_minimal() -> CustomEntity972Request {
        CustomEntity972Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_972s_with_children() -> CustomEntity972Request {
        CustomEntity972Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_973s() -> CustomEntity973Request {
        CustomEntity973Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_973s_minimal() -> CustomEntity973Request {
        CustomEntity973Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_973s_with_children() -> CustomEntity973Request {
        CustomEntity973Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_974s() -> CustomEntity974Request {
        CustomEntity974Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_974s_minimal() -> CustomEntity974Request {
        CustomEntity974Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_974s_with_children() -> CustomEntity974Request {
        CustomEntity974Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_975s() -> CustomEntity975Request {
        CustomEntity975Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_975s_minimal() -> CustomEntity975Request {
        CustomEntity975Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_975s_with_children() -> CustomEntity975Request {
        CustomEntity975Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_976s() -> CustomEntity976Request {
        CustomEntity976Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_976s_minimal() -> CustomEntity976Request {
        CustomEntity976Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_976s_with_children() -> CustomEntity976Request {
        CustomEntity976Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_977s() -> CustomEntity977Request {
        CustomEntity977Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_977s_minimal() -> CustomEntity977Request {
        CustomEntity977Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_977s_with_children() -> CustomEntity977Request {
        CustomEntity977Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_978s() -> CustomEntity978Request {
        CustomEntity978Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_978s_minimal() -> CustomEntity978Request {
        CustomEntity978Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_978s_with_children() -> CustomEntity978Request {
        CustomEntity978Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_979s() -> CustomEntity979Request {
        CustomEntity979Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_979s_minimal() -> CustomEntity979Request {
        CustomEntity979Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_979s_with_children() -> CustomEntity979Request {
        CustomEntity979Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_980s() -> CustomEntity980Request {
        CustomEntity980Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_980s_minimal() -> CustomEntity980Request {
        CustomEntity980Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_980s_with_children() -> CustomEntity980Request {
        CustomEntity980Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_981s() -> CustomEntity981Request {
        CustomEntity981Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_981s_minimal() -> CustomEntity981Request {
        CustomEntity981Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_981s_with_children() -> CustomEntity981Request {
        CustomEntity981Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_982s() -> CustomEntity982Request {
        CustomEntity982Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_982s_minimal() -> CustomEntity982Request {
        CustomEntity982Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_982s_with_children() -> CustomEntity982Request {
        CustomEntity982Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_983s() -> CustomEntity983Request {
        CustomEntity983Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_983s_minimal() -> CustomEntity983Request {
        CustomEntity983Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_983s_with_children() -> CustomEntity983Request {
        CustomEntity983Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_984s() -> CustomEntity984Request {
        CustomEntity984Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_984s_minimal() -> CustomEntity984Request {
        CustomEntity984Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_984s_with_children() -> CustomEntity984Request {
        CustomEntity984Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_985s() -> CustomEntity985Request {
        CustomEntity985Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_985s_minimal() -> CustomEntity985Request {
        CustomEntity985Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_985s_with_children() -> CustomEntity985Request {
        CustomEntity985Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_986s() -> CustomEntity986Request {
        CustomEntity986Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_986s_minimal() -> CustomEntity986Request {
        CustomEntity986Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_986s_with_children() -> CustomEntity986Request {
        CustomEntity986Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_987s() -> CustomEntity987Request {
        CustomEntity987Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_987s_minimal() -> CustomEntity987Request {
        CustomEntity987Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_987s_with_children() -> CustomEntity987Request {
        CustomEntity987Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_988s() -> CustomEntity988Request {
        CustomEntity988Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_988s_minimal() -> CustomEntity988Request {
        CustomEntity988Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_988s_with_children() -> CustomEntity988Request {
        CustomEntity988Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_989s() -> CustomEntity989Request {
        CustomEntity989Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_989s_minimal() -> CustomEntity989Request {
        CustomEntity989Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_989s_with_children() -> CustomEntity989Request {
        CustomEntity989Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_990s() -> CustomEntity990Request {
        CustomEntity990Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_990s_minimal() -> CustomEntity990Request {
        CustomEntity990Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_990s_with_children() -> CustomEntity990Request {
        CustomEntity990Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_991s() -> CustomEntity991Request {
        CustomEntity991Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_991s_minimal() -> CustomEntity991Request {
        CustomEntity991Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_991s_with_children() -> CustomEntity991Request {
        CustomEntity991Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_992s() -> CustomEntity992Request {
        CustomEntity992Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_992s_minimal() -> CustomEntity992Request {
        CustomEntity992Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_992s_with_children() -> CustomEntity992Request {
        CustomEntity992Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_993s() -> CustomEntity993Request {
        CustomEntity993Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_993s_minimal() -> CustomEntity993Request {
        CustomEntity993Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_993s_with_children() -> CustomEntity993Request {
        CustomEntity993Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_994s() -> CustomEntity994Request {
        CustomEntity994Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_994s_minimal() -> CustomEntity994Request {
        CustomEntity994Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_994s_with_children() -> CustomEntity994Request {
        CustomEntity994Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_995s() -> CustomEntity995Request {
        CustomEntity995Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_995s_minimal() -> CustomEntity995Request {
        CustomEntity995Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_995s_with_children() -> CustomEntity995Request {
        CustomEntity995Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_996s() -> CustomEntity996Request {
        CustomEntity996Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_996s_minimal() -> CustomEntity996Request {
        CustomEntity996Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_996s_with_children() -> CustomEntity996Request {
        CustomEntity996Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_997s() -> CustomEntity997Request {
        CustomEntity997Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_997s_minimal() -> CustomEntity997Request {
        CustomEntity997Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_997s_with_children() -> CustomEntity997Request {
        CustomEntity997Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_998s() -> CustomEntity998Request {
        CustomEntity998Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_998s_minimal() -> CustomEntity998Request {
        CustomEntity998Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_998s_with_children() -> CustomEntity998Request {
        CustomEntity998Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn custom_entity_999s() -> CustomEntity999Request {
        CustomEntity999Request::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_999s_minimal() -> CustomEntity999Request {
        CustomEntity999Request::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn custom_entity_999s_with_children() -> CustomEntity999Request {
        CustomEntity999Request::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}