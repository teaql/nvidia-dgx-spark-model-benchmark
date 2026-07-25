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

    pub fn loading_unloadings() -> LoadingUnloadingRequest {
        LoadingUnloadingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loading_unloadings_minimal() -> LoadingUnloadingRequest {
        LoadingUnloadingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn loading_unloadings_with_children() -> LoadingUnloadingRequest {
        LoadingUnloadingRequest::new()
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

    pub fn tools() -> ToolRequest {
        ToolRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tools_minimal() -> ToolRequest {
        ToolRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tools_with_children() -> ToolRequest {
        ToolRequest::new()
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

    pub fn containers() -> ContainerRequest {
        ContainerRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn containers_minimal() -> ContainerRequest {
        ContainerRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn containers_with_children() -> ContainerRequest {
        ContainerRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn pallets() -> PalletRequest {
        PalletRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pallets_minimal() -> PalletRequest {
        PalletRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn pallets_with_children() -> PalletRequest {
        PalletRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn labels() -> LabelRequest {
        LabelRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn labels_minimal() -> LabelRequest {
        LabelRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn labels_with_children() -> LabelRequest {
        LabelRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn barcodes() -> BarcodeRequest {
        BarcodeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn barcodes_minimal() -> BarcodeRequest {
        BarcodeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn barcodes_with_children() -> BarcodeRequest {
        BarcodeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tracking_numbers() -> TrackingNumberRequest {
        TrackingNumberRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tracking_numbers_minimal() -> TrackingNumberRequest {
        TrackingNumberRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tracking_numbers_with_children() -> TrackingNumberRequest {
        TrackingNumberRequest::new()
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

    pub fn claims() -> ClaimRequest {
        ClaimRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn claims_minimal() -> ClaimRequest {
        ClaimRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn claims_with_children() -> ClaimRequest {
        ClaimRequest::new()
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

    pub fn vehicle_maintenances() -> VehicleMaintenanceRequest {
        VehicleMaintenanceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_maintenances_minimal() -> VehicleMaintenanceRequest {
        VehicleMaintenanceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicle_maintenances_with_children() -> VehicleMaintenanceRequest {
        VehicleMaintenanceRequest::new()
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

    pub fn licenses() -> LicenseRequest {
        LicenseRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn licenses_minimal() -> LicenseRequest {
        LicenseRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn licenses_with_children() -> LicenseRequest {
        LicenseRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn permits() -> PermitRequest {
        PermitRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn permits_minimal() -> PermitRequest {
        PermitRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn permits_with_children() -> PermitRequest {
        PermitRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customs_documents() -> CustomsDocumentRequest {
        CustomsDocumentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customs_documents_minimal() -> CustomsDocumentRequest {
        CustomsDocumentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customs_documents_with_children() -> CustomsDocumentRequest {
        CustomsDocumentRequest::new()
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

    pub fn dashboards() -> DashboardRequest {
        DashboardRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dashboards_minimal() -> DashboardRequest {
        DashboardRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn dashboards_with_children() -> DashboardRequest {
        DashboardRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn settingses() -> SettingsRequest {
        SettingsRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn settingses_minimal() -> SettingsRequest {
        SettingsRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn settingses_with_children() -> SettingsRequest {
        SettingsRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn user_roles() -> UserRoleRequest {
        UserRoleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn user_roles_minimal() -> UserRoleRequest {
        UserRoleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn user_roles_with_children() -> UserRoleRequest {
        UserRoleRequest::new()
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