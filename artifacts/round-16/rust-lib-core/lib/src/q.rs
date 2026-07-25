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

    pub fn trucks() -> TruckRequest {
        TruckRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn trucks_minimal() -> TruckRequest {
        TruckRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn trucks_with_children() -> TruckRequest {
        TruckRequest::new()
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

    pub fn schedules() -> ScheduleRequest {
        ScheduleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn schedules_minimal() -> ScheduleRequest {
        ScheduleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn schedules_with_children() -> ScheduleRequest {
        ScheduleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn warehouses() -> WarehouseRequest {
        WarehouseRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warehouses_minimal() -> WarehouseRequest {
        WarehouseRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warehouses_with_children() -> WarehouseRequest {
        WarehouseRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn inventory() -> InventoryRequest {
        InventoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_minimal() -> InventoryRequest {
        InventoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inventory_with_children() -> InventoryRequest {
        InventoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn quotes() -> QuoteRequest {
        QuoteRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn quotes_minimal() -> QuoteRequest {
        QuoteRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn quotes_with_children() -> QuoteRequest {
        QuoteRequest::new()
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

    pub fn feedback() -> FeedbackRequest {
        FeedbackRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn feedback_minimal() -> FeedbackRequest {
        FeedbackRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn feedback_with_children() -> FeedbackRequest {
        FeedbackRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn insurances() -> InsuranceRequest {
        InsuranceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn insurances_minimal() -> InsuranceRequest {
        InsuranceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn insurances_with_children() -> InsuranceRequest {
        InsuranceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn maintenances() -> MaintenanceRequest {
        MaintenanceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenances_minimal() -> MaintenanceRequest {
        MaintenanceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn maintenances_with_children() -> MaintenanceRequest {
        MaintenanceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn drivers() -> DriverRequest {
        DriverRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn drivers_minimal() -> DriverRequest {
        DriverRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn drivers_with_children() -> DriverRequest {
        DriverRequest::new()
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

    pub fn trackings() -> TrackingRequest {
        TrackingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn trackings_minimal() -> TrackingRequest {
        TrackingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn trackings_with_children() -> TrackingRequest {
        TrackingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn reports() -> ReportRequest {
        ReportRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn reports_minimal() -> ReportRequest {
        ReportRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn reports_with_children() -> ReportRequest {
        ReportRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}