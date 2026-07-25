use crate::*;
use teaql_core::TeaqlEntity;

use teaql_provider_sqlite::SqliteProviderExt as _;

pub type DataServiceDialect = teaql_provider_sqlite::SqliteDialect;
pub type DataServiceMutationExecutor = teaql_provider_sqlite::SqliteMutationExecutor;
pub type DataServiceMutationError = teaql_provider_sqlite::MutationExecutorError;
pub type DataServiceIdGenerator = teaql_provider_sqlite::SqliteIdSpaceGenerator;
pub type DataServicePool = std::sync::Arc<std::sync::Mutex<rusqlite::Connection>>;
pub type DataServiceExecutor = ServiceRuntimeExecutor;
pub type ServiceRuntime = teaql_runtime::UserContext;

pub const DATABASE_URL_ENV: &str = "FINANCE_SERVICE_CORE_DATABASE_URL";
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceRuntimeConfig {
    pub database_url: String,
}

impl ServiceRuntimeConfig {
    pub fn from_env() -> Result<Self, ServiceRuntimeError> {
        Ok(Self {
            database_url: env_value(DATABASE_URL_ENV)?,
        })
    }
}

#[derive(Debug)]
pub enum ServiceRuntimeError {
    MissingEnv {
        name: &'static str,
        source: std::env::VarError,
    },
    ConnectionError(String),
    Rusqlite(rusqlite::Error),
    Runtime(teaql_runtime::RuntimeError),
}

impl std::fmt::Display for ServiceRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceRuntimeError::MissingEnv { name, source } => {
                write!(f, "missing environment variable {name}: {source}")
            }
            ServiceRuntimeError::ConnectionError(err) => write!(f, "connection error: {err}"),
            ServiceRuntimeError::Rusqlite(err) => write!(f, "rusqlite error: {err}"),
            ServiceRuntimeError::Runtime(err) => write!(f, "runtime error: {err}"),
        }
    }
}

impl std::error::Error for ServiceRuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServiceRuntimeError::MissingEnv { source, .. } => Some(source),
            ServiceRuntimeError::ConnectionError(_) => None,
            ServiceRuntimeError::Rusqlite(err) => Some(err),
            ServiceRuntimeError::Runtime(err) => Some(err),
        }
    }
}

impl From<rusqlite::Error> for ServiceRuntimeError {
    fn from(err: rusqlite::Error) -> Self {
        ServiceRuntimeError::Rusqlite(err)
    }
}
impl From<teaql_runtime::RuntimeError> for ServiceRuntimeError {
    fn from(err: teaql_runtime::RuntimeError) -> Self {
        ServiceRuntimeError::Runtime(err)
    }
}

#[derive(Clone)]
pub struct LocalSchemaProvider;

impl teaql_data_service::SchemaProvider for LocalSchemaProvider {
    fn get_entity(&self, name: &str) -> Option<std::sync::Arc<teaql_core::EntityDescriptor>> {
        match name {
            "Customer" => Some(std::sync::Arc::new(crate::Customer::entity_descriptor())),
            "Address" => Some(std::sync::Arc::new(crate::Address::entity_descriptor())),
            "Truck" => Some(std::sync::Arc::new(crate::Truck::entity_descriptor())),
            "Driver" => Some(std::sync::Arc::new(crate::Driver::entity_descriptor())),
            "MoveOrder" => Some(std::sync::Arc::new(crate::MoveOrder::entity_descriptor())),
            "InventoryItem" => Some(std::sync::Arc::new(crate::InventoryItem::entity_descriptor())),
            "PackingMaterial" => Some(std::sync::Arc::new(crate::PackingMaterial::entity_descriptor())),
            "Route" => Some(std::sync::Arc::new(crate::Route::entity_descriptor())),
            "Schedule" => Some(std::sync::Arc::new(crate::Schedule::entity_descriptor())),
            "LoadingUnloading" => Some(std::sync::Arc::new(crate::LoadingUnloading::entity_descriptor())),
            "Equipment" => Some(std::sync::Arc::new(crate::Equipment::entity_descriptor())),
            "Tool" => Some(std::sync::Arc::new(crate::Tool::entity_descriptor())),
            "StorageFacility" => Some(std::sync::Arc::new(crate::StorageFacility::entity_descriptor())),
            "Warehouse" => Some(std::sync::Arc::new(crate::Warehouse::entity_descriptor())),
            "Container" => Some(std::sync::Arc::new(crate::Container::entity_descriptor())),
            "Pallet" => Some(std::sync::Arc::new(crate::Pallet::entity_descriptor())),
            "Label" => Some(std::sync::Arc::new(crate::Label::entity_descriptor())),
            "Barcode" => Some(std::sync::Arc::new(crate::Barcode::entity_descriptor())),
            "TrackingNumber" => Some(std::sync::Arc::new(crate::TrackingNumber::entity_descriptor())),
            "Notification" => Some(std::sync::Arc::new(crate::Notification::entity_descriptor())),
            "Payment" => Some(std::sync::Arc::new(crate::Payment::entity_descriptor())),
            "Invoice" => Some(std::sync::Arc::new(crate::Invoice::entity_descriptor())),
            "Claim" => Some(std::sync::Arc::new(crate::Claim::entity_descriptor())),
            "Feedback" => Some(std::sync::Arc::new(crate::Feedback::entity_descriptor())),
            "Employee" => Some(std::sync::Arc::new(crate::Employee::entity_descriptor())),
            "Branch" => Some(std::sync::Arc::new(crate::Branch::entity_descriptor())),
            "VehicleMaintenance" => Some(std::sync::Arc::new(crate::VehicleMaintenance::entity_descriptor())),
            "FuelLog" => Some(std::sync::Arc::new(crate::FuelLog::entity_descriptor())),
            "InsurancePolicy" => Some(std::sync::Arc::new(crate::InsurancePolicy::entity_descriptor())),
            "License" => Some(std::sync::Arc::new(crate::License::entity_descriptor())),
            "Permit" => Some(std::sync::Arc::new(crate::Permit::entity_descriptor())),
            "CustomsDocument" => Some(std::sync::Arc::new(crate::CustomsDocument::entity_descriptor())),
            "CommunicationLog" => Some(std::sync::Arc::new(crate::CommunicationLog::entity_descriptor())),
            "AuditTrail" => Some(std::sync::Arc::new(crate::AuditTrail::entity_descriptor())),
            "Report" => Some(std::sync::Arc::new(crate::Report::entity_descriptor())),
            "Dashboard" => Some(std::sync::Arc::new(crate::Dashboard::entity_descriptor())),
            "Settings" => Some(std::sync::Arc::new(crate::Settings::entity_descriptor())),
            "UserRole" => Some(std::sync::Arc::new(crate::UserRole::entity_descriptor())),
            "Permission" => Some(std::sync::Arc::new(crate::Permission::entity_descriptor())),
            "ApiKey" => Some(std::sync::Arc::new(crate::ApiKey::entity_descriptor())),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct ServiceRuntimeExecutor {
    inner: teaql_sql::SqlDataServiceExecutor<
        DataServiceDialect,
        DataServiceMutationExecutor,
        LocalSchemaProvider
    >,
}

impl ServiceRuntimeExecutor {
    pub fn new(inner: DataServiceMutationExecutor) -> Self {
        Self {
            inner: teaql_sql::SqlDataServiceExecutor::new(
                DataServiceDialect::default(),
                inner,
                LocalSchemaProvider
            ),
        }
    }

}

impl teaql_data_service::DataServiceExecutor for ServiceRuntimeExecutor {
    type Error = teaql_sql::SqlExecutorError<DataServiceMutationError>;
    fn capabilities(&self) -> teaql_data_service::DataServiceCapabilities {
        teaql_data_service::DataServiceExecutor::capabilities(&self.inner)
    }
}

impl teaql_data_service::QueryExecutor for ServiceRuntimeExecutor {
    async fn query(&self, request: teaql_data_service::QueryRequest) -> Result<teaql_data_service::QueryResult, Self::Error> {
        teaql_data_service::QueryExecutor::query(&self.inner, request).await
    }
}

impl teaql_data_service::StreamQueryExecutor for ServiceRuntimeExecutor {
    async fn query_stream(&self, request: teaql_data_service::QueryRequest, chunk_size: usize) -> Result<Vec<teaql_data_service::StreamChunk>, Self::Error> {
        teaql_data_service::StreamQueryExecutor::query_stream(&self.inner, request, chunk_size).await
    }
}

impl teaql_data_service::MutationExecutor for ServiceRuntimeExecutor {
    async fn mutate(&self, request: teaql_data_service::MutationRequest) -> Result<teaql_data_service::MutationResult, Self::Error> {
        teaql_data_service::MutationExecutor::mutate(&self.inner, request).await
    }
}

impl teaql_data_service::TransactionExecutor for ServiceRuntimeExecutor {
    type Tx<'a> = teaql_sql::SqlDataServiceTransaction<'a, DataServiceDialect, <DataServiceMutationExecutor as teaql_sql::SqlTransactionTransport>::Tx<'a>, LocalSchemaProvider> where Self: 'a;

    async fn begin(&self) -> Result<Self::Tx<'_ >, Self::Error> {
        teaql_data_service::TransactionExecutor::begin(&self.inner).await
    }
}

pub async fn service_runtime_from_env() -> Result<ServiceRuntime, ServiceRuntimeError> {
    service_runtime(ServiceRuntimeConfig::from_env()?).await
}

pub async fn service_runtime(config: ServiceRuntimeConfig) -> Result<ServiceRuntime, ServiceRuntimeError> {
    let pool = connect_data_service_pool(&config).await?;
    service_runtime_from_pool(pool).await
}

pub async fn service_runtime_from_pool(pool: DataServicePool) -> Result<ServiceRuntime, ServiceRuntimeError> {
    let mutation_executor = DataServiceMutationExecutor::new(pool);
    let id_generator = DataServiceIdGenerator::from_executor(mutation_executor.clone());let mut context = module_with_behaviors_and_checkers().into_context();
    context.set_internal_id_generator(id_generator);
    context.use_sqlite_provider(mutation_executor.clone());
    let executor = ServiceRuntimeExecutor::new(mutation_executor);
    context.register_executor(executor.clone());
    context.insert_resource(executor);

    // 自动加载 Zero-Code 审计配置与 Schema 模式
    let env_config = teaql_tool_core::audit_config_from_env(&[
        "customer_data", "address_data", "truck_data", "driver_data", "move_order_data", "inventory_item_data", "packing_material_data", "route_data", "schedule_data", "loading_unloading_data", "equipment_data", "tool_data", "storage_facility_data", "warehouse_data", "container_data", "pallet_data", "label_data", "barcode_data", "tracking_number_data", "notification_data", "payment_data", "invoice_data", "claim_data", "feedback_data", "employee_data", "branch_data", "vehicle_maintenance_data", "fuel_log_data", "insurance_policy_data", "license_data", "permit_data", "customs_document_data", "communication_log_data", "audit_trail_data", "report_data", "dashboard_data", "settings_data", "user_role_data", "permission_data", "api_key_data"
    ]);
    let schema_mode = env_config.schema_mode;
    context.insert_resource(env_config.config.clone());
    context.insert_resource(env_config);

    match schema_mode {
        teaql_tool_core::SchemaMode::Execute => {
            context.ensure_schema().await?;
        }
        teaql_tool_core::SchemaMode::DryRun => {
            // DryRun: 目前等效于验证
            context.ensure_schema().await?;
        }
        teaql_tool_core::SchemaMode::Verify => {
            context.ensure_schema().await?;
        }
    }

    Ok(context)
}



fn env_value(name: &'static str) -> Result<String, ServiceRuntimeError> {
    std::env::var(name).map_err(|source| ServiceRuntimeError::MissingEnv { name, source })
}

async fn connect_data_service_pool(config: &ServiceRuntimeConfig) -> Result<DataServicePool, ServiceRuntimeError> {
    let url = &config.database_url;
    let sanitized_url = if url.starts_with("sqlite:") { url.strip_prefix("sqlite:").unwrap().trim_start_matches("//") } else { url };
    let pure_file_path = sanitized_url.split('?').next().unwrap_or(sanitized_url);
    let path = std::path::Path::new(pure_file_path);
    if let Some(parent) = path.parent() { if !parent.as_os_str().is_empty() { std::fs::create_dir_all(parent).map_err(|e| ServiceRuntimeError::ConnectionError(e.to_string()))?; } }
    Ok(std::sync::Arc::new(std::sync::Mutex::new(rusqlite::Connection::open(pure_file_path).map_err(|e| ServiceRuntimeError::ConnectionError(e.to_string()))?)))
}

pub fn repository_registry() -> teaql_runtime::InMemoryEntityRegistry {
    teaql_runtime::InMemoryEntityRegistry::new()
        .with_entity("Customer")
        .with_entity("Address")
        .with_entity("Truck")
        .with_entity("Driver")
        .with_entity("MoveOrder")
        .with_entity("InventoryItem")
        .with_entity("PackingMaterial")
        .with_entity("Route")
        .with_entity("Schedule")
        .with_entity("LoadingUnloading")
        .with_entity("Equipment")
        .with_entity("Tool")
        .with_entity("StorageFacility")
        .with_entity("Warehouse")
        .with_entity("Container")
        .with_entity("Pallet")
        .with_entity("Label")
        .with_entity("Barcode")
        .with_entity("TrackingNumber")
        .with_entity("Notification")
        .with_entity("Payment")
        .with_entity("Invoice")
        .with_entity("Claim")
        .with_entity("Feedback")
        .with_entity("Employee")
        .with_entity("Branch")
        .with_entity("VehicleMaintenance")
        .with_entity("FuelLog")
        .with_entity("InsurancePolicy")
        .with_entity("License")
        .with_entity("Permit")
        .with_entity("CustomsDocument")
        .with_entity("CommunicationLog")
        .with_entity("AuditTrail")
        .with_entity("Report")
        .with_entity("Dashboard")
        .with_entity("Settings")
        .with_entity("UserRole")
        .with_entity("Permission")
        .with_entity("ApiKey")
}

pub fn behavior_registry() -> teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry {
    teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry::new()
        .with_behavior("Customer", CustomerBehavior::default())
        .with_behavior("Address", AddressBehavior::default())
        .with_behavior("Truck", TruckBehavior::default())
        .with_behavior("Driver", DriverBehavior::default())
        .with_behavior("MoveOrder", MoveOrderBehavior::default())
        .with_behavior("InventoryItem", InventoryItemBehavior::default())
        .with_behavior("PackingMaterial", PackingMaterialBehavior::default())
        .with_behavior("Route", RouteBehavior::default())
        .with_behavior("Schedule", ScheduleBehavior::default())
        .with_behavior("LoadingUnloading", LoadingUnloadingBehavior::default())
        .with_behavior("Equipment", EquipmentBehavior::default())
        .with_behavior("Tool", ToolBehavior::default())
        .with_behavior("StorageFacility", StorageFacilityBehavior::default())
        .with_behavior("Warehouse", WarehouseBehavior::default())
        .with_behavior("Container", ContainerBehavior::default())
        .with_behavior("Pallet", PalletBehavior::default())
        .with_behavior("Label", LabelBehavior::default())
        .with_behavior("Barcode", BarcodeBehavior::default())
        .with_behavior("TrackingNumber", TrackingNumberBehavior::default())
        .with_behavior("Notification", NotificationBehavior::default())
        .with_behavior("Payment", PaymentBehavior::default())
        .with_behavior("Invoice", InvoiceBehavior::default())
        .with_behavior("Claim", ClaimBehavior::default())
        .with_behavior("Feedback", FeedbackBehavior::default())
        .with_behavior("Employee", EmployeeBehavior::default())
        .with_behavior("Branch", BranchBehavior::default())
        .with_behavior("VehicleMaintenance", VehicleMaintenanceBehavior::default())
        .with_behavior("FuelLog", FuelLogBehavior::default())
        .with_behavior("InsurancePolicy", InsurancePolicyBehavior::default())
        .with_behavior("License", LicenseBehavior::default())
        .with_behavior("Permit", PermitBehavior::default())
        .with_behavior("CustomsDocument", CustomsDocumentBehavior::default())
        .with_behavior("CommunicationLog", CommunicationLogBehavior::default())
        .with_behavior("AuditTrail", AuditTrailBehavior::default())
        .with_behavior("Report", ReportBehavior::default())
        .with_behavior("Dashboard", DashboardBehavior::default())
        .with_behavior("Settings", SettingsBehavior::default())
        .with_behavior("UserRole", UserRoleBehavior::default())
        .with_behavior("Permission", PermissionBehavior::default())
        .with_behavior("ApiKey", ApiKeyBehavior::default())
}

pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
    teaql_runtime::InMemoryCheckerRegistry::new()
        .with_checker(teaql_runtime::TypedEntityChecker::<Customer, _>::new(CustomerChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Address, _>::new(AddressChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Truck, _>::new(TruckChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Driver, _>::new(DriverChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<MoveOrder, _>::new(MoveOrderChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<InventoryItem, _>::new(InventoryItemChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<PackingMaterial, _>::new(PackingMaterialChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Route, _>::new(RouteChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Schedule, _>::new(ScheduleChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<LoadingUnloading, _>::new(LoadingUnloadingChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Equipment, _>::new(EquipmentChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Tool, _>::new(ToolChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<StorageFacility, _>::new(StorageFacilityChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Warehouse, _>::new(WarehouseChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Container, _>::new(ContainerChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Pallet, _>::new(PalletChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Label, _>::new(LabelChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Barcode, _>::new(BarcodeChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<TrackingNumber, _>::new(TrackingNumberChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Notification, _>::new(NotificationChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Payment, _>::new(PaymentChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Invoice, _>::new(InvoiceChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Claim, _>::new(ClaimChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Feedback, _>::new(FeedbackChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Employee, _>::new(EmployeeChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Branch, _>::new(BranchChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<VehicleMaintenance, _>::new(VehicleMaintenanceChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<FuelLog, _>::new(FuelLogChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<InsurancePolicy, _>::new(InsurancePolicyChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<License, _>::new(LicenseChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Permit, _>::new(PermitChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<CustomsDocument, _>::new(CustomsDocumentChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<CommunicationLog, _>::new(CommunicationLogChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<AuditTrail, _>::new(AuditTrailChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Report, _>::new(ReportChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Dashboard, _>::new(DashboardChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Settings, _>::new(SettingsChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<UserRole, _>::new(UserRoleChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Permission, _>::new(PermissionChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ApiKey, _>::new(ApiKeyChecker::default()))
}

pub fn module() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<Customer>()
        .entity::<Address>()
        .entity::<Truck>()
        .entity::<Driver>()
        .entity::<MoveOrder>()
        .entity::<InventoryItem>()
        .entity::<PackingMaterial>()
        .entity::<Route>()
        .entity::<Schedule>()
        .entity::<LoadingUnloading>()
        .entity::<Equipment>()
        .entity::<Tool>()
        .entity::<StorageFacility>()
        .entity::<Warehouse>()
        .entity::<Container>()
        .entity::<Pallet>()
        .entity::<Label>()
        .entity::<Barcode>()
        .entity::<TrackingNumber>()
        .entity::<Notification>()
        .entity::<Payment>()
        .entity::<Invoice>()
        .entity::<Claim>()
        .entity::<Feedback>()
        .entity::<Employee>()
        .entity::<Branch>()
        .entity::<VehicleMaintenance>()
        .entity::<FuelLog>()
        .entity::<InsurancePolicy>()
        .entity::<License>()
        .entity::<Permit>()
        .entity::<CustomsDocument>()
        .entity::<CommunicationLog>()
        .entity::<AuditTrail>()
        .entity::<Report>()
        .entity::<Dashboard>()
        .entity::<Settings>()
        .entity::<UserRole>()
        .entity::<Permission>()
        .entity::<ApiKey>()
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Address")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Truck")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Driver")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("MoveOrder")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryItem")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("PackingMaterial")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Route")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Schedule")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("LoadingUnloading")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Equipment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Tool")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("StorageFacility")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Warehouse")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Container")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Pallet")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Label")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Barcode")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TrackingNumber")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Notification")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Payment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Invoice")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Claim")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Feedback")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Employee")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Branch")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("VehicleMaintenance")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("FuelLog")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("InsurancePolicy")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("License")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Permit")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("CustomsDocument")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("CommunicationLog")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("AuditTrail")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Report")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Dashboard")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Settings")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("UserRole")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Permission")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("ApiKey")
            .value("id", 1_u64)
            .value("version", 1_i64))
}

pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<Customer>()
        .checker(teaql_runtime::TypedEntityChecker::<Customer, _>::new(CustomerChecker::default()))
        .entity::<Address>()
        .checker(teaql_runtime::TypedEntityChecker::<Address, _>::new(AddressChecker::default()))
        .entity::<Truck>()
        .checker(teaql_runtime::TypedEntityChecker::<Truck, _>::new(TruckChecker::default()))
        .entity::<Driver>()
        .checker(teaql_runtime::TypedEntityChecker::<Driver, _>::new(DriverChecker::default()))
        .entity::<MoveOrder>()
        .checker(teaql_runtime::TypedEntityChecker::<MoveOrder, _>::new(MoveOrderChecker::default()))
        .entity::<InventoryItem>()
        .checker(teaql_runtime::TypedEntityChecker::<InventoryItem, _>::new(InventoryItemChecker::default()))
        .entity::<PackingMaterial>()
        .checker(teaql_runtime::TypedEntityChecker::<PackingMaterial, _>::new(PackingMaterialChecker::default()))
        .entity::<Route>()
        .checker(teaql_runtime::TypedEntityChecker::<Route, _>::new(RouteChecker::default()))
        .entity::<Schedule>()
        .checker(teaql_runtime::TypedEntityChecker::<Schedule, _>::new(ScheduleChecker::default()))
        .entity::<LoadingUnloading>()
        .checker(teaql_runtime::TypedEntityChecker::<LoadingUnloading, _>::new(LoadingUnloadingChecker::default()))
        .entity::<Equipment>()
        .checker(teaql_runtime::TypedEntityChecker::<Equipment, _>::new(EquipmentChecker::default()))
        .entity::<Tool>()
        .checker(teaql_runtime::TypedEntityChecker::<Tool, _>::new(ToolChecker::default()))
        .entity::<StorageFacility>()
        .checker(teaql_runtime::TypedEntityChecker::<StorageFacility, _>::new(StorageFacilityChecker::default()))
        .entity::<Warehouse>()
        .checker(teaql_runtime::TypedEntityChecker::<Warehouse, _>::new(WarehouseChecker::default()))
        .entity::<Container>()
        .checker(teaql_runtime::TypedEntityChecker::<Container, _>::new(ContainerChecker::default()))
        .entity::<Pallet>()
        .checker(teaql_runtime::TypedEntityChecker::<Pallet, _>::new(PalletChecker::default()))
        .entity::<Label>()
        .checker(teaql_runtime::TypedEntityChecker::<Label, _>::new(LabelChecker::default()))
        .entity::<Barcode>()
        .checker(teaql_runtime::TypedEntityChecker::<Barcode, _>::new(BarcodeChecker::default()))
        .entity::<TrackingNumber>()
        .checker(teaql_runtime::TypedEntityChecker::<TrackingNumber, _>::new(TrackingNumberChecker::default()))
        .entity::<Notification>()
        .checker(teaql_runtime::TypedEntityChecker::<Notification, _>::new(NotificationChecker::default()))
        .entity::<Payment>()
        .checker(teaql_runtime::TypedEntityChecker::<Payment, _>::new(PaymentChecker::default()))
        .entity::<Invoice>()
        .checker(teaql_runtime::TypedEntityChecker::<Invoice, _>::new(InvoiceChecker::default()))
        .entity::<Claim>()
        .checker(teaql_runtime::TypedEntityChecker::<Claim, _>::new(ClaimChecker::default()))
        .entity::<Feedback>()
        .checker(teaql_runtime::TypedEntityChecker::<Feedback, _>::new(FeedbackChecker::default()))
        .entity::<Employee>()
        .checker(teaql_runtime::TypedEntityChecker::<Employee, _>::new(EmployeeChecker::default()))
        .entity::<Branch>()
        .checker(teaql_runtime::TypedEntityChecker::<Branch, _>::new(BranchChecker::default()))
        .entity::<VehicleMaintenance>()
        .checker(teaql_runtime::TypedEntityChecker::<VehicleMaintenance, _>::new(VehicleMaintenanceChecker::default()))
        .entity::<FuelLog>()
        .checker(teaql_runtime::TypedEntityChecker::<FuelLog, _>::new(FuelLogChecker::default()))
        .entity::<InsurancePolicy>()
        .checker(teaql_runtime::TypedEntityChecker::<InsurancePolicy, _>::new(InsurancePolicyChecker::default()))
        .entity::<License>()
        .checker(teaql_runtime::TypedEntityChecker::<License, _>::new(LicenseChecker::default()))
        .entity::<Permit>()
        .checker(teaql_runtime::TypedEntityChecker::<Permit, _>::new(PermitChecker::default()))
        .entity::<CustomsDocument>()
        .checker(teaql_runtime::TypedEntityChecker::<CustomsDocument, _>::new(CustomsDocumentChecker::default()))
        .entity::<CommunicationLog>()
        .checker(teaql_runtime::TypedEntityChecker::<CommunicationLog, _>::new(CommunicationLogChecker::default()))
        .entity::<AuditTrail>()
        .checker(teaql_runtime::TypedEntityChecker::<AuditTrail, _>::new(AuditTrailChecker::default()))
        .entity::<Report>()
        .checker(teaql_runtime::TypedEntityChecker::<Report, _>::new(ReportChecker::default()))
        .entity::<Dashboard>()
        .checker(teaql_runtime::TypedEntityChecker::<Dashboard, _>::new(DashboardChecker::default()))
        .entity::<Settings>()
        .checker(teaql_runtime::TypedEntityChecker::<Settings, _>::new(SettingsChecker::default()))
        .entity::<UserRole>()
        .checker(teaql_runtime::TypedEntityChecker::<UserRole, _>::new(UserRoleChecker::default()))
        .entity::<Permission>()
        .checker(teaql_runtime::TypedEntityChecker::<Permission, _>::new(PermissionChecker::default()))
        .entity::<ApiKey>()
        .checker(teaql_runtime::TypedEntityChecker::<ApiKey, _>::new(ApiKeyChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Address")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Truck")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Driver")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("MoveOrder")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryItem")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("PackingMaterial")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Route")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Schedule")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("LoadingUnloading")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Equipment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Tool")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("StorageFacility")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Warehouse")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Container")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Pallet")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Label")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Barcode")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TrackingNumber")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Notification")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Payment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Invoice")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Claim")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Feedback")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Employee")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Branch")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("VehicleMaintenance")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("FuelLog")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("InsurancePolicy")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("License")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Permit")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("CustomsDocument")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("CommunicationLog")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("AuditTrail")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Report")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Dashboard")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Settings")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("UserRole")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Permission")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("ApiKey")
            .value("id", 1_u64)
            .value("version", 1_i64))
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<Customer, _>(CustomerBehavior::default())
        .entity_with_behavior::<Address, _>(AddressBehavior::default())
        .entity_with_behavior::<Truck, _>(TruckBehavior::default())
        .entity_with_behavior::<Driver, _>(DriverBehavior::default())
        .entity_with_behavior::<MoveOrder, _>(MoveOrderBehavior::default())
        .entity_with_behavior::<InventoryItem, _>(InventoryItemBehavior::default())
        .entity_with_behavior::<PackingMaterial, _>(PackingMaterialBehavior::default())
        .entity_with_behavior::<Route, _>(RouteBehavior::default())
        .entity_with_behavior::<Schedule, _>(ScheduleBehavior::default())
        .entity_with_behavior::<LoadingUnloading, _>(LoadingUnloadingBehavior::default())
        .entity_with_behavior::<Equipment, _>(EquipmentBehavior::default())
        .entity_with_behavior::<Tool, _>(ToolBehavior::default())
        .entity_with_behavior::<StorageFacility, _>(StorageFacilityBehavior::default())
        .entity_with_behavior::<Warehouse, _>(WarehouseBehavior::default())
        .entity_with_behavior::<Container, _>(ContainerBehavior::default())
        .entity_with_behavior::<Pallet, _>(PalletBehavior::default())
        .entity_with_behavior::<Label, _>(LabelBehavior::default())
        .entity_with_behavior::<Barcode, _>(BarcodeBehavior::default())
        .entity_with_behavior::<TrackingNumber, _>(TrackingNumberBehavior::default())
        .entity_with_behavior::<Notification, _>(NotificationBehavior::default())
        .entity_with_behavior::<Payment, _>(PaymentBehavior::default())
        .entity_with_behavior::<Invoice, _>(InvoiceBehavior::default())
        .entity_with_behavior::<Claim, _>(ClaimBehavior::default())
        .entity_with_behavior::<Feedback, _>(FeedbackBehavior::default())
        .entity_with_behavior::<Employee, _>(EmployeeBehavior::default())
        .entity_with_behavior::<Branch, _>(BranchBehavior::default())
        .entity_with_behavior::<VehicleMaintenance, _>(VehicleMaintenanceBehavior::default())
        .entity_with_behavior::<FuelLog, _>(FuelLogBehavior::default())
        .entity_with_behavior::<InsurancePolicy, _>(InsurancePolicyBehavior::default())
        .entity_with_behavior::<License, _>(LicenseBehavior::default())
        .entity_with_behavior::<Permit, _>(PermitBehavior::default())
        .entity_with_behavior::<CustomsDocument, _>(CustomsDocumentBehavior::default())
        .entity_with_behavior::<CommunicationLog, _>(CommunicationLogBehavior::default())
        .entity_with_behavior::<AuditTrail, _>(AuditTrailBehavior::default())
        .entity_with_behavior::<Report, _>(ReportBehavior::default())
        .entity_with_behavior::<Dashboard, _>(DashboardBehavior::default())
        .entity_with_behavior::<Settings, _>(SettingsBehavior::default())
        .entity_with_behavior::<UserRole, _>(UserRoleBehavior::default())
        .entity_with_behavior::<Permission, _>(PermissionBehavior::default())
        .entity_with_behavior::<ApiKey, _>(ApiKeyBehavior::default())
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Address")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Truck")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Driver")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("MoveOrder")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryItem")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("PackingMaterial")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Route")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Schedule")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("LoadingUnloading")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Equipment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Tool")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("StorageFacility")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Warehouse")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Container")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Pallet")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Label")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Barcode")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TrackingNumber")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Notification")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Payment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Invoice")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Claim")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Feedback")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Employee")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Branch")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("VehicleMaintenance")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("FuelLog")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("InsurancePolicy")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("License")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Permit")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("CustomsDocument")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("CommunicationLog")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("AuditTrail")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Report")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Dashboard")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Settings")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("UserRole")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Permission")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("ApiKey")
            .value("id", 1_u64)
            .value("version", 1_i64))
}

pub fn module_with_behaviors_and_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<Customer, _>(CustomerBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Customer, _>::new(CustomerChecker::default()))
        .entity_with_behavior::<Address, _>(AddressBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Address, _>::new(AddressChecker::default()))
        .entity_with_behavior::<Truck, _>(TruckBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Truck, _>::new(TruckChecker::default()))
        .entity_with_behavior::<Driver, _>(DriverBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Driver, _>::new(DriverChecker::default()))
        .entity_with_behavior::<MoveOrder, _>(MoveOrderBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<MoveOrder, _>::new(MoveOrderChecker::default()))
        .entity_with_behavior::<InventoryItem, _>(InventoryItemBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<InventoryItem, _>::new(InventoryItemChecker::default()))
        .entity_with_behavior::<PackingMaterial, _>(PackingMaterialBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<PackingMaterial, _>::new(PackingMaterialChecker::default()))
        .entity_with_behavior::<Route, _>(RouteBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Route, _>::new(RouteChecker::default()))
        .entity_with_behavior::<Schedule, _>(ScheduleBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Schedule, _>::new(ScheduleChecker::default()))
        .entity_with_behavior::<LoadingUnloading, _>(LoadingUnloadingBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<LoadingUnloading, _>::new(LoadingUnloadingChecker::default()))
        .entity_with_behavior::<Equipment, _>(EquipmentBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Equipment, _>::new(EquipmentChecker::default()))
        .entity_with_behavior::<Tool, _>(ToolBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Tool, _>::new(ToolChecker::default()))
        .entity_with_behavior::<StorageFacility, _>(StorageFacilityBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<StorageFacility, _>::new(StorageFacilityChecker::default()))
        .entity_with_behavior::<Warehouse, _>(WarehouseBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Warehouse, _>::new(WarehouseChecker::default()))
        .entity_with_behavior::<Container, _>(ContainerBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Container, _>::new(ContainerChecker::default()))
        .entity_with_behavior::<Pallet, _>(PalletBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Pallet, _>::new(PalletChecker::default()))
        .entity_with_behavior::<Label, _>(LabelBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Label, _>::new(LabelChecker::default()))
        .entity_with_behavior::<Barcode, _>(BarcodeBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Barcode, _>::new(BarcodeChecker::default()))
        .entity_with_behavior::<TrackingNumber, _>(TrackingNumberBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<TrackingNumber, _>::new(TrackingNumberChecker::default()))
        .entity_with_behavior::<Notification, _>(NotificationBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Notification, _>::new(NotificationChecker::default()))
        .entity_with_behavior::<Payment, _>(PaymentBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Payment, _>::new(PaymentChecker::default()))
        .entity_with_behavior::<Invoice, _>(InvoiceBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Invoice, _>::new(InvoiceChecker::default()))
        .entity_with_behavior::<Claim, _>(ClaimBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Claim, _>::new(ClaimChecker::default()))
        .entity_with_behavior::<Feedback, _>(FeedbackBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Feedback, _>::new(FeedbackChecker::default()))
        .entity_with_behavior::<Employee, _>(EmployeeBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Employee, _>::new(EmployeeChecker::default()))
        .entity_with_behavior::<Branch, _>(BranchBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Branch, _>::new(BranchChecker::default()))
        .entity_with_behavior::<VehicleMaintenance, _>(VehicleMaintenanceBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<VehicleMaintenance, _>::new(VehicleMaintenanceChecker::default()))
        .entity_with_behavior::<FuelLog, _>(FuelLogBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<FuelLog, _>::new(FuelLogChecker::default()))
        .entity_with_behavior::<InsurancePolicy, _>(InsurancePolicyBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<InsurancePolicy, _>::new(InsurancePolicyChecker::default()))
        .entity_with_behavior::<License, _>(LicenseBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<License, _>::new(LicenseChecker::default()))
        .entity_with_behavior::<Permit, _>(PermitBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Permit, _>::new(PermitChecker::default()))
        .entity_with_behavior::<CustomsDocument, _>(CustomsDocumentBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<CustomsDocument, _>::new(CustomsDocumentChecker::default()))
        .entity_with_behavior::<CommunicationLog, _>(CommunicationLogBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<CommunicationLog, _>::new(CommunicationLogChecker::default()))
        .entity_with_behavior::<AuditTrail, _>(AuditTrailBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<AuditTrail, _>::new(AuditTrailChecker::default()))
        .entity_with_behavior::<Report, _>(ReportBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Report, _>::new(ReportChecker::default()))
        .entity_with_behavior::<Dashboard, _>(DashboardBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Dashboard, _>::new(DashboardChecker::default()))
        .entity_with_behavior::<Settings, _>(SettingsBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Settings, _>::new(SettingsChecker::default()))
        .entity_with_behavior::<UserRole, _>(UserRoleBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<UserRole, _>::new(UserRoleChecker::default()))
        .entity_with_behavior::<Permission, _>(PermissionBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Permission, _>::new(PermissionChecker::default()))
        .entity_with_behavior::<ApiKey, _>(ApiKeyBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<ApiKey, _>::new(ApiKeyChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Address")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Truck")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Driver")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("MoveOrder")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryItem")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("PackingMaterial")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Route")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Schedule")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("LoadingUnloading")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Equipment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Tool")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("StorageFacility")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Warehouse")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Container")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Pallet")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Label")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Barcode")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TrackingNumber")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Notification")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Payment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Invoice")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Claim")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Feedback")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Employee")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Branch")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("VehicleMaintenance")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("FuelLog")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("InsurancePolicy")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("License")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Permit")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("CustomsDocument")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("CommunicationLog")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("AuditTrail")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Report")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Dashboard")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Settings")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("UserRole")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Permission")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("ApiKey")
            .value("id", 1_u64)
            .value("version", 1_i64))
}