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
    pub fn truckses() -> TrucksRequest {
        TrucksRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn truckses_minimal() -> TrucksRequest {
        TrucksRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn truckses_with_children() -> TrucksRequest {
        TrucksRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicleses() -> VehiclesRequest {
        VehiclesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicleses_minimal() -> VehiclesRequest {
        VehiclesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicleses_with_children() -> VehiclesRequest {
        VehiclesRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn driverses() -> DriversRequest {
        DriversRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driverses_minimal() -> DriversRequest {
        DriversRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn driverses_with_children() -> DriversRequest {
        DriversRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn routeses() -> RoutesRequest {
        RoutesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn routeses_minimal() -> RoutesRequest {
        RoutesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn routeses_with_children() -> RoutesRequest {
        RoutesRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn locationses() -> LocationsRequest {
        LocationsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn locationses_minimal() -> LocationsRequest {
        LocationsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn locationses_with_children() -> LocationsRequest {
        LocationsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn addresseses() -> AddressesRequest {
        AddressesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn addresseses_minimal() -> AddressesRequest {
        AddressesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn addresseses_with_children() -> AddressesRequest {
        AddressesRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn dispatcheses() -> DispatchesRequest {
        DispatchesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dispatcheses_minimal() -> DispatchesRequest {
        DispatchesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dispatcheses_with_children() -> DispatchesRequest {
        DispatchesRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn jobses() -> JobsRequest {
        JobsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn jobses_minimal() -> JobsRequest {
        JobsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn jobses_with_children() -> JobsRequest {
        JobsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn scheduleses() -> SchedulesRequest {
        SchedulesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn scheduleses_minimal() -> SchedulesRequest {
        SchedulesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn scheduleses_with_children() -> SchedulesRequest {
        SchedulesRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn shiftses() -> ShiftsRequest {
        ShiftsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shiftses_minimal() -> ShiftsRequest {
        ShiftsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shiftses_with_children() -> ShiftsRequest {
        ShiftsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn timesheetses() -> TimesheetsRequest {
        TimesheetsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn timesheetses_minimal() -> TimesheetsRequest {
        TimesheetsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn timesheetses_with_children() -> TimesheetsRequest {
        TimesheetsRequest::new()
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

    pub fn geofences() -> GeofenceRequest {
        GeofenceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn geofences_minimal() -> GeofenceRequest {
        GeofenceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn geofences_with_children() -> GeofenceRequest {
        GeofenceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn fuels() -> FuelRequest {
        FuelRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fuels_minimal() -> FuelRequest {
        FuelRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn fuels_with_children() -> FuelRequest {
        FuelRequest::new()
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

    pub fn repairses() -> RepairsRequest {
        RepairsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn repairses_minimal() -> RepairsRequest {
        RepairsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn repairses_with_children() -> RepairsRequest {
        RepairsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn inspectionses() -> InspectionsRequest {
        InspectionsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inspectionses_minimal() -> InspectionsRequest {
        InspectionsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn inspectionses_with_children() -> InspectionsRequest {
        InspectionsRequest::new()
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

    pub fn invoiceses() -> InvoicesRequest {
        InvoicesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoiceses_minimal() -> InvoicesRequest {
        InvoicesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn invoiceses_with_children() -> InvoicesRequest {
        InvoicesRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn paymentses() -> PaymentsRequest {
        PaymentsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn paymentses_minimal() -> PaymentsRequest {
        PaymentsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn paymentses_with_children() -> PaymentsRequest {
        PaymentsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn expenseses() -> ExpensesRequest {
        ExpensesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expenseses_minimal() -> ExpensesRequest {
        ExpensesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expenseses_with_children() -> ExpensesRequest {
        ExpensesRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn accountses() -> AccountsRequest {
        AccountsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn accountses_minimal() -> AccountsRequest {
        AccountsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn accountses_with_children() -> AccountsRequest {
        AccountsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn ledgerses() -> LedgersRequest {
        LedgersRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn ledgerses_minimal() -> LedgersRequest {
        LedgersRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn ledgerses_with_children() -> LedgersRequest {
        LedgersRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn taxeses() -> TaxesRequest {
        TaxesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn taxeses_minimal() -> TaxesRequest {
        TaxesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn taxeses_with_children() -> TaxesRequest {
        TaxesRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn quoteses() -> QuotesRequest {
        QuotesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn quoteses_minimal() -> QuotesRequest {
        QuotesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn quoteses_with_children() -> QuotesRequest {
        QuotesRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn estimateses() -> EstimatesRequest {
        EstimatesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn estimateses_minimal() -> EstimatesRequest {
        EstimatesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn estimateses_with_children() -> EstimatesRequest {
        EstimatesRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn audits() -> AuditRequest {
        AuditRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audits_minimal() -> AuditRequest {
        AuditRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn audits_with_children() -> AuditRequest {
        AuditRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn securities() -> SecurityRequest {
        SecurityRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn securities_minimal() -> SecurityRequest {
        SecurityRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn securities_with_children() -> SecurityRequest {
        SecurityRequest::new()
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

    pub fn payrolls() -> PayrollRequest {
        PayrollRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payrolls_minimal() -> PayrollRequest {
        PayrollRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payrolls_with_children() -> PayrollRequest {
        PayrollRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn reimbursementses() -> ReimbursementsRequest {
        ReimbursementsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn reimbursementses_minimal() -> ReimbursementsRequest {
        ReimbursementsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn reimbursementses_with_children() -> ReimbursementsRequest {
        ReimbursementsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn financial_reportses() -> FinancialReportsRequest {
        FinancialReportsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn financial_reportses_minimal() -> FinancialReportsRequest {
        FinancialReportsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn financial_reportses_with_children() -> FinancialReportsRequest {
        FinancialReportsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn cash_flows() -> CashFlowRequest {
        CashFlowRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cash_flows_minimal() -> CashFlowRequest {
        CashFlowRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn cash_flows_with_children() -> CashFlowRequest {
        CashFlowRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customerses() -> CustomersRequest {
        CustomersRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customerses_minimal() -> CustomersRequest {
        CustomersRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customerses_with_children() -> CustomersRequest {
        CustomersRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn employeeses() -> EmployeesRequest {
        EmployeesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employeeses_minimal() -> EmployeesRequest {
        EmployeesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employeeses_with_children() -> EmployeesRequest {
        EmployeesRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn contactses() -> ContactsRequest {
        ContactsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contactses_minimal() -> ContactsRequest {
        ContactsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contactses_with_children() -> ContactsRequest {
        ContactsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn documentses() -> DocumentsRequest {
        DocumentsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn documentses_minimal() -> DocumentsRequest {
        DocumentsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn documentses_with_children() -> DocumentsRequest {
        DocumentsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn contractses() -> ContractsRequest {
        ContractsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contractses_minimal() -> ContractsRequest {
        ContractsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contractses_with_children() -> ContractsRequest {
        ContractsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn signatureses() -> SignaturesRequest {
        SignaturesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn signatureses_minimal() -> SignaturesRequest {
        SignaturesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn signatureses_with_children() -> SignaturesRequest {
        SignaturesRequest::new()
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

    pub fn reviewses() -> ReviewsRequest {
        ReviewsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn reviewses_minimal() -> ReviewsRequest {
        ReviewsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn reviewses_with_children() -> ReviewsRequest {
        ReviewsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn ratingses() -> RatingsRequest {
        RatingsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn ratingses_minimal() -> RatingsRequest {
        RatingsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn ratingses_with_children() -> RatingsRequest {
        RatingsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn notificationses() -> NotificationsRequest {
        NotificationsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notificationses_minimal() -> NotificationsRequest {
        NotificationsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notificationses_with_children() -> NotificationsRequest {
        NotificationsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn alertses() -> AlertsRequest {
        AlertsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn alertses_minimal() -> AlertsRequest {
        AlertsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn alertses_with_children() -> AlertsRequest {
        AlertsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn calendarses() -> CalendarsRequest {
        CalendarsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn calendarses_minimal() -> CalendarsRequest {
        CalendarsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn calendarses_with_children() -> CalendarsRequest {
        CalendarsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn userses() -> UsersRequest {
        UsersRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn userses_minimal() -> UsersRequest {
        UsersRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn userses_with_children() -> UsersRequest {
        UsersRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn roleses() -> RolesRequest {
        RolesRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn roleses_minimal() -> RolesRequest {
        RolesRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn roleses_with_children() -> RolesRequest {
        RolesRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn permissionses() -> PermissionsRequest {
        PermissionsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn permissionses_minimal() -> PermissionsRequest {
        PermissionsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn permissionses_with_children() -> PermissionsRequest {
        PermissionsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}