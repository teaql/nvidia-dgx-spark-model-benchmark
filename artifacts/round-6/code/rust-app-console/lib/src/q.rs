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
    pub fn route_status_types() -> RouteStatusTypeRequest {
        RouteStatusTypeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn route_status_types_minimal() -> RouteStatusTypeRequest {
        RouteStatusTypeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn route_status_types_with_children() -> RouteStatusTypeRequest {
        RouteStatusTypeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn inventory_condition_types() -> InventoryConditionTypeRequest {
        InventoryConditionTypeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_condition_types_minimal() -> InventoryConditionTypeRequest {
        InventoryConditionTypeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_condition_types_with_children() -> InventoryConditionTypeRequest {
        InventoryConditionTypeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn exception_severities() -> ExceptionSeverityRequest {
        ExceptionSeverityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn exception_severities_minimal() -> ExceptionSeverityRequest {
        ExceptionSeverityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn exception_severities_with_children() -> ExceptionSeverityRequest {
        ExceptionSeverityRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn order_statuses() -> OrderStatusRequest {
        OrderStatusRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn order_statuses_minimal() -> OrderStatusRequest {
        OrderStatusRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn order_statuses_with_children() -> OrderStatusRequest {
        OrderStatusRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn crew_roles() -> CrewRoleRequest {
        CrewRoleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn crew_roles_minimal() -> CrewRoleRequest {
        CrewRoleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn crew_roles_with_children() -> CrewRoleRequest {
        CrewRoleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

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

    pub fn packaging_items() -> PackagingItemRequest {
        PackagingItemRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packaging_items_minimal() -> PackagingItemRequest {
        PackagingItemRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn packaging_items_with_children() -> PackagingItemRequest {
        PackagingItemRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn logistics_providers() -> LogisticsProviderRequest {
        LogisticsProviderRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn logistics_providers_minimal() -> LogisticsProviderRequest {
        LogisticsProviderRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn logistics_providers_with_children() -> LogisticsProviderRequest {
        LogisticsProviderRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn third_party_dispatches() -> ThirdPartyDispatchRequest {
        ThirdPartyDispatchRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn third_party_dispatches_minimal() -> ThirdPartyDispatchRequest {
        ThirdPartyDispatchRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn third_party_dispatches_with_children() -> ThirdPartyDispatchRequest {
        ThirdPartyDispatchRequest::new()
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

    pub fn maintenance_records() -> MaintenanceRecordRequest {
        MaintenanceRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenance_records_minimal() -> MaintenanceRecordRequest {
        MaintenanceRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenance_records_with_children() -> MaintenanceRecordRequest {
        MaintenanceRecordRequest::new()
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

    pub fn shift_logs() -> ShiftLogRequest {
        ShiftLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shift_logs_minimal() -> ShiftLogRequest {
        ShiftLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shift_logs_with_children() -> ShiftLogRequest {
        ShiftLogRequest::new()
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
}