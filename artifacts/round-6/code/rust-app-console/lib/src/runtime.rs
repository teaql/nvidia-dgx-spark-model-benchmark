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

pub const DATABASE_URL_ENV: &str = "OPERATIONS_MICROSERVICE_CORE_DATABASE_URL";
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
            "RouteStatusType" => Some(std::sync::Arc::new(crate::RouteStatusType::entity_descriptor())),
            "InventoryConditionType" => Some(std::sync::Arc::new(crate::InventoryConditionType::entity_descriptor())),
            "ExceptionSeverity" => Some(std::sync::Arc::new(crate::ExceptionSeverity::entity_descriptor())),
            "OrderStatus" => Some(std::sync::Arc::new(crate::OrderStatus::entity_descriptor())),
            "CrewRole" => Some(std::sync::Arc::new(crate::CrewRole::entity_descriptor())),
            "Platform" => Some(std::sync::Arc::new(crate::Platform::entity_descriptor())),
            "Merchant" => Some(std::sync::Arc::new(crate::Merchant::entity_descriptor())),
            "MoveQuote" => Some(std::sync::Arc::new(crate::MoveQuote::entity_descriptor())),
            "MoveOrder" => Some(std::sync::Arc::new(crate::MoveOrder::entity_descriptor())),
            "RouteStop" => Some(std::sync::Arc::new(crate::RouteStop::entity_descriptor())),
            "Crew" => Some(std::sync::Arc::new(crate::Crew::entity_descriptor())),
            "CrewMemberAssignment" => Some(std::sync::Arc::new(crate::CrewMemberAssignment::entity_descriptor())),
            "Vehicle" => Some(std::sync::Arc::new(crate::Vehicle::entity_descriptor())),
            "VehicleAssignment" => Some(std::sync::Arc::new(crate::VehicleAssignment::entity_descriptor())),
            "DispatchAssignment" => Some(std::sync::Arc::new(crate::DispatchAssignment::entity_descriptor())),
            "DamageReport" => Some(std::sync::Arc::new(crate::DamageReport::entity_descriptor())),
            "ProofOfDelivery" => Some(std::sync::Arc::new(crate::ProofOfDelivery::entity_descriptor())),
            "OperationalException" => Some(std::sync::Arc::new(crate::OperationalException::entity_descriptor())),
            "PickupInstruction" => Some(std::sync::Arc::new(crate::PickupInstruction::entity_descriptor())),
            "DeliveryInstruction" => Some(std::sync::Arc::new(crate::DeliveryInstruction::entity_descriptor())),
            "MoveInventory" => Some(std::sync::Arc::new(crate::MoveInventory::entity_descriptor())),
            "PackagingItem" => Some(std::sync::Arc::new(crate::PackagingItem::entity_descriptor())),
            "LogisticsProvider" => Some(std::sync::Arc::new(crate::LogisticsProvider::entity_descriptor())),
            "ThirdPartyDispatch" => Some(std::sync::Arc::new(crate::ThirdPartyDispatch::entity_descriptor())),
            "FuelLog" => Some(std::sync::Arc::new(crate::FuelLog::entity_descriptor())),
            "MaintenanceRecord" => Some(std::sync::Arc::new(crate::MaintenanceRecord::entity_descriptor())),
            "TollReceipt" => Some(std::sync::Arc::new(crate::TollReceipt::entity_descriptor())),
            "ShiftLog" => Some(std::sync::Arc::new(crate::ShiftLog::entity_descriptor())),
            "CustomerFeedback" => Some(std::sync::Arc::new(crate::CustomerFeedback::entity_descriptor())),
            "IncidentReport" => Some(std::sync::Arc::new(crate::IncidentReport::entity_descriptor())),
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
        "route_status_type_data", "inventory_condition_type_data", "exception_severity_data", "order_status_data", "crew_role_data", "platform_data", "merchant_data", "move_quote_data", "move_order_data", "route_stop_data", "crew_data", "crew_member_assignment_data", "vehicle_data", "vehicle_assignment_data", "dispatch_assignment_data", "damage_report_data", "proof_of_delivery_data", "operational_exception_data", "pickup_instruction_data", "delivery_instruction_data", "move_inventory_data", "packaging_item_data", "logistics_provider_data", "third_party_dispatch_data", "fuel_log_data", "maintenance_record_data", "toll_receipt_data", "shift_log_data", "customer_feedback_data", "incident_report_data"
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
        .with_entity("RouteStatusType")
        .with_entity("InventoryConditionType")
        .with_entity("ExceptionSeverity")
        .with_entity("OrderStatus")
        .with_entity("CrewRole")
        .with_entity("Platform")
        .with_entity("Merchant")
        .with_entity("MoveQuote")
        .with_entity("MoveOrder")
        .with_entity("RouteStop")
        .with_entity("Crew")
        .with_entity("CrewMemberAssignment")
        .with_entity("Vehicle")
        .with_entity("VehicleAssignment")
        .with_entity("DispatchAssignment")
        .with_entity("DamageReport")
        .with_entity("ProofOfDelivery")
        .with_entity("OperationalException")
        .with_entity("PickupInstruction")
        .with_entity("DeliveryInstruction")
        .with_entity("MoveInventory")
        .with_entity("PackagingItem")
        .with_entity("LogisticsProvider")
        .with_entity("ThirdPartyDispatch")
        .with_entity("FuelLog")
        .with_entity("MaintenanceRecord")
        .with_entity("TollReceipt")
        .with_entity("ShiftLog")
        .with_entity("CustomerFeedback")
        .with_entity("IncidentReport")
}

pub fn behavior_registry() -> teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry {
    teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry::new()
        .with_behavior("RouteStatusType", RouteStatusTypeBehavior::default())
        .with_behavior("InventoryConditionType", InventoryConditionTypeBehavior::default())
        .with_behavior("ExceptionSeverity", ExceptionSeverityBehavior::default())
        .with_behavior("OrderStatus", OrderStatusBehavior::default())
        .with_behavior("CrewRole", CrewRoleBehavior::default())
        .with_behavior("Platform", PlatformBehavior::default())
        .with_behavior("Merchant", MerchantBehavior::default())
        .with_behavior("MoveQuote", MoveQuoteBehavior::default())
        .with_behavior("MoveOrder", MoveOrderBehavior::default())
        .with_behavior("RouteStop", RouteStopBehavior::default())
        .with_behavior("Crew", CrewBehavior::default())
        .with_behavior("CrewMemberAssignment", CrewMemberAssignmentBehavior::default())
        .with_behavior("Vehicle", VehicleBehavior::default())
        .with_behavior("VehicleAssignment", VehicleAssignmentBehavior::default())
        .with_behavior("DispatchAssignment", DispatchAssignmentBehavior::default())
        .with_behavior("DamageReport", DamageReportBehavior::default())
        .with_behavior("ProofOfDelivery", ProofOfDeliveryBehavior::default())
        .with_behavior("OperationalException", OperationalExceptionBehavior::default())
        .with_behavior("PickupInstruction", PickupInstructionBehavior::default())
        .with_behavior("DeliveryInstruction", DeliveryInstructionBehavior::default())
        .with_behavior("MoveInventory", MoveInventoryBehavior::default())
        .with_behavior("PackagingItem", PackagingItemBehavior::default())
        .with_behavior("LogisticsProvider", LogisticsProviderBehavior::default())
        .with_behavior("ThirdPartyDispatch", ThirdPartyDispatchBehavior::default())
        .with_behavior("FuelLog", FuelLogBehavior::default())
        .with_behavior("MaintenanceRecord", MaintenanceRecordBehavior::default())
        .with_behavior("TollReceipt", TollReceiptBehavior::default())
        .with_behavior("ShiftLog", ShiftLogBehavior::default())
        .with_behavior("CustomerFeedback", CustomerFeedbackBehavior::default())
        .with_behavior("IncidentReport", IncidentReportBehavior::default())
}

pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
    teaql_runtime::InMemoryCheckerRegistry::new()
        .with_checker(teaql_runtime::TypedEntityChecker::<RouteStatusType, _>::new(RouteStatusTypeChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<InventoryConditionType, _>::new(InventoryConditionTypeChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ExceptionSeverity, _>::new(ExceptionSeverityChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<OrderStatus, _>::new(OrderStatusChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<CrewRole, _>::new(CrewRoleChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Merchant, _>::new(MerchantChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<MoveQuote, _>::new(MoveQuoteChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<MoveOrder, _>::new(MoveOrderChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<RouteStop, _>::new(RouteStopChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Crew, _>::new(CrewChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<CrewMemberAssignment, _>::new(CrewMemberAssignmentChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Vehicle, _>::new(VehicleChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<VehicleAssignment, _>::new(VehicleAssignmentChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<DispatchAssignment, _>::new(DispatchAssignmentChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<DamageReport, _>::new(DamageReportChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ProofOfDelivery, _>::new(ProofOfDeliveryChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<OperationalException, _>::new(OperationalExceptionChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<PickupInstruction, _>::new(PickupInstructionChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<DeliveryInstruction, _>::new(DeliveryInstructionChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<MoveInventory, _>::new(MoveInventoryChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<PackagingItem, _>::new(PackagingItemChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<LogisticsProvider, _>::new(LogisticsProviderChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ThirdPartyDispatch, _>::new(ThirdPartyDispatchChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<FuelLog, _>::new(FuelLogChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<MaintenanceRecord, _>::new(MaintenanceRecordChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<TollReceipt, _>::new(TollReceiptChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ShiftLog, _>::new(ShiftLogChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<CustomerFeedback, _>::new(CustomerFeedbackChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<IncidentReport, _>::new(IncidentReportChecker::default()))
}

pub fn module() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<RouteStatusType>()
        .entity::<InventoryConditionType>()
        .entity::<ExceptionSeverity>()
        .entity::<OrderStatus>()
        .entity::<CrewRole>()
        .entity::<Platform>()
        .entity::<Merchant>()
        .entity::<MoveQuote>()
        .entity::<MoveOrder>()
        .entity::<RouteStop>()
        .entity::<Crew>()
        .entity::<CrewMemberAssignment>()
        .entity::<Vehicle>()
        .entity::<VehicleAssignment>()
        .entity::<DispatchAssignment>()
        .entity::<DamageReport>()
        .entity::<ProofOfDelivery>()
        .entity::<OperationalException>()
        .entity::<PickupInstruction>()
        .entity::<DeliveryInstruction>()
        .entity::<MoveInventory>()
        .entity::<PackagingItem>()
        .entity::<LogisticsProvider>()
        .entity::<ThirdPartyDispatch>()
        .entity::<FuelLog>()
        .entity::<MaintenanceRecord>()
        .entity::<TollReceipt>()
        .entity::<ShiftLog>()
        .entity::<CustomerFeedback>()
        .entity::<IncidentReport>()
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Moving Company Platform")
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("RouteStatusType")
            .value("id", 1001_u64)
            .value("name", "Pending")
            .value("code", "PENDING")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("RouteStatusType")
            .value("id", 1002_u64)
            .value("name", "In Progress")
            .value("code", "IN_PROGRESS")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("RouteStatusType")
            .value("id", 1003_u64)
            .value("name", "Completed")
            .value("code", "COMPLETED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryConditionType")
            .value("id", 1001_u64)
            .value("name", "New")
            .value("code", "NEW")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryConditionType")
            .value("id", 1002_u64)
            .value("name", "Used")
            .value("code", "USED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryConditionType")
            .value("id", 1003_u64)
            .value("name", "Damaged")
            .value("code", "DAMAGED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ExceptionSeverity")
            .value("id", 1001_u64)
            .value("name", "Low")
            .value("code", "LOW")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ExceptionSeverity")
            .value("id", 1002_u64)
            .value("name", "Medium")
            .value("code", "MEDIUM")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ExceptionSeverity")
            .value("id", 1003_u64)
            .value("name", "High")
            .value("code", "HIGH")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("OrderStatus")
            .value("id", 1001_u64)
            .value("name", "Draft")
            .value("code", "DRAFT")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("OrderStatus")
            .value("id", 1002_u64)
            .value("name", "Confirmed")
            .value("code", "CONFIRMED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("OrderStatus")
            .value("id", 1003_u64)
            .value("name", "Completed")
            .value("code", "COMPLETED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CrewRole")
            .value("id", 1001_u64)
            .value("name", "Driver")
            .value("code", "DRIVER")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CrewRole")
            .value("id", 1002_u64)
            .value("name", "Mover")
            .value("code", "MOVER")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CrewRole")
            .value("id", 1003_u64)
            .value("name", "Supervisor")
            .value("code", "SUPERVISOR")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
}

pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<RouteStatusType>()
        .checker(teaql_runtime::TypedEntityChecker::<RouteStatusType, _>::new(RouteStatusTypeChecker::default()))
        .entity::<InventoryConditionType>()
        .checker(teaql_runtime::TypedEntityChecker::<InventoryConditionType, _>::new(InventoryConditionTypeChecker::default()))
        .entity::<ExceptionSeverity>()
        .checker(teaql_runtime::TypedEntityChecker::<ExceptionSeverity, _>::new(ExceptionSeverityChecker::default()))
        .entity::<OrderStatus>()
        .checker(teaql_runtime::TypedEntityChecker::<OrderStatus, _>::new(OrderStatusChecker::default()))
        .entity::<CrewRole>()
        .checker(teaql_runtime::TypedEntityChecker::<CrewRole, _>::new(CrewRoleChecker::default()))
        .entity::<Platform>()
        .checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .entity::<Merchant>()
        .checker(teaql_runtime::TypedEntityChecker::<Merchant, _>::new(MerchantChecker::default()))
        .entity::<MoveQuote>()
        .checker(teaql_runtime::TypedEntityChecker::<MoveQuote, _>::new(MoveQuoteChecker::default()))
        .entity::<MoveOrder>()
        .checker(teaql_runtime::TypedEntityChecker::<MoveOrder, _>::new(MoveOrderChecker::default()))
        .entity::<RouteStop>()
        .checker(teaql_runtime::TypedEntityChecker::<RouteStop, _>::new(RouteStopChecker::default()))
        .entity::<Crew>()
        .checker(teaql_runtime::TypedEntityChecker::<Crew, _>::new(CrewChecker::default()))
        .entity::<CrewMemberAssignment>()
        .checker(teaql_runtime::TypedEntityChecker::<CrewMemberAssignment, _>::new(CrewMemberAssignmentChecker::default()))
        .entity::<Vehicle>()
        .checker(teaql_runtime::TypedEntityChecker::<Vehicle, _>::new(VehicleChecker::default()))
        .entity::<VehicleAssignment>()
        .checker(teaql_runtime::TypedEntityChecker::<VehicleAssignment, _>::new(VehicleAssignmentChecker::default()))
        .entity::<DispatchAssignment>()
        .checker(teaql_runtime::TypedEntityChecker::<DispatchAssignment, _>::new(DispatchAssignmentChecker::default()))
        .entity::<DamageReport>()
        .checker(teaql_runtime::TypedEntityChecker::<DamageReport, _>::new(DamageReportChecker::default()))
        .entity::<ProofOfDelivery>()
        .checker(teaql_runtime::TypedEntityChecker::<ProofOfDelivery, _>::new(ProofOfDeliveryChecker::default()))
        .entity::<OperationalException>()
        .checker(teaql_runtime::TypedEntityChecker::<OperationalException, _>::new(OperationalExceptionChecker::default()))
        .entity::<PickupInstruction>()
        .checker(teaql_runtime::TypedEntityChecker::<PickupInstruction, _>::new(PickupInstructionChecker::default()))
        .entity::<DeliveryInstruction>()
        .checker(teaql_runtime::TypedEntityChecker::<DeliveryInstruction, _>::new(DeliveryInstructionChecker::default()))
        .entity::<MoveInventory>()
        .checker(teaql_runtime::TypedEntityChecker::<MoveInventory, _>::new(MoveInventoryChecker::default()))
        .entity::<PackagingItem>()
        .checker(teaql_runtime::TypedEntityChecker::<PackagingItem, _>::new(PackagingItemChecker::default()))
        .entity::<LogisticsProvider>()
        .checker(teaql_runtime::TypedEntityChecker::<LogisticsProvider, _>::new(LogisticsProviderChecker::default()))
        .entity::<ThirdPartyDispatch>()
        .checker(teaql_runtime::TypedEntityChecker::<ThirdPartyDispatch, _>::new(ThirdPartyDispatchChecker::default()))
        .entity::<FuelLog>()
        .checker(teaql_runtime::TypedEntityChecker::<FuelLog, _>::new(FuelLogChecker::default()))
        .entity::<MaintenanceRecord>()
        .checker(teaql_runtime::TypedEntityChecker::<MaintenanceRecord, _>::new(MaintenanceRecordChecker::default()))
        .entity::<TollReceipt>()
        .checker(teaql_runtime::TypedEntityChecker::<TollReceipt, _>::new(TollReceiptChecker::default()))
        .entity::<ShiftLog>()
        .checker(teaql_runtime::TypedEntityChecker::<ShiftLog, _>::new(ShiftLogChecker::default()))
        .entity::<CustomerFeedback>()
        .checker(teaql_runtime::TypedEntityChecker::<CustomerFeedback, _>::new(CustomerFeedbackChecker::default()))
        .entity::<IncidentReport>()
        .checker(teaql_runtime::TypedEntityChecker::<IncidentReport, _>::new(IncidentReportChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Moving Company Platform")
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("RouteStatusType")
            .value("id", 1001_u64)
            .value("name", "Pending")
            .value("code", "PENDING")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("RouteStatusType")
            .value("id", 1002_u64)
            .value("name", "In Progress")
            .value("code", "IN_PROGRESS")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("RouteStatusType")
            .value("id", 1003_u64)
            .value("name", "Completed")
            .value("code", "COMPLETED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryConditionType")
            .value("id", 1001_u64)
            .value("name", "New")
            .value("code", "NEW")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryConditionType")
            .value("id", 1002_u64)
            .value("name", "Used")
            .value("code", "USED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryConditionType")
            .value("id", 1003_u64)
            .value("name", "Damaged")
            .value("code", "DAMAGED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ExceptionSeverity")
            .value("id", 1001_u64)
            .value("name", "Low")
            .value("code", "LOW")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ExceptionSeverity")
            .value("id", 1002_u64)
            .value("name", "Medium")
            .value("code", "MEDIUM")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ExceptionSeverity")
            .value("id", 1003_u64)
            .value("name", "High")
            .value("code", "HIGH")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("OrderStatus")
            .value("id", 1001_u64)
            .value("name", "Draft")
            .value("code", "DRAFT")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("OrderStatus")
            .value("id", 1002_u64)
            .value("name", "Confirmed")
            .value("code", "CONFIRMED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("OrderStatus")
            .value("id", 1003_u64)
            .value("name", "Completed")
            .value("code", "COMPLETED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CrewRole")
            .value("id", 1001_u64)
            .value("name", "Driver")
            .value("code", "DRIVER")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CrewRole")
            .value("id", 1002_u64)
            .value("name", "Mover")
            .value("code", "MOVER")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CrewRole")
            .value("id", 1003_u64)
            .value("name", "Supervisor")
            .value("code", "SUPERVISOR")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<RouteStatusType, _>(RouteStatusTypeBehavior::default())
        .entity_with_behavior::<InventoryConditionType, _>(InventoryConditionTypeBehavior::default())
        .entity_with_behavior::<ExceptionSeverity, _>(ExceptionSeverityBehavior::default())
        .entity_with_behavior::<OrderStatus, _>(OrderStatusBehavior::default())
        .entity_with_behavior::<CrewRole, _>(CrewRoleBehavior::default())
        .entity_with_behavior::<Platform, _>(PlatformBehavior::default())
        .entity_with_behavior::<Merchant, _>(MerchantBehavior::default())
        .entity_with_behavior::<MoveQuote, _>(MoveQuoteBehavior::default())
        .entity_with_behavior::<MoveOrder, _>(MoveOrderBehavior::default())
        .entity_with_behavior::<RouteStop, _>(RouteStopBehavior::default())
        .entity_with_behavior::<Crew, _>(CrewBehavior::default())
        .entity_with_behavior::<CrewMemberAssignment, _>(CrewMemberAssignmentBehavior::default())
        .entity_with_behavior::<Vehicle, _>(VehicleBehavior::default())
        .entity_with_behavior::<VehicleAssignment, _>(VehicleAssignmentBehavior::default())
        .entity_with_behavior::<DispatchAssignment, _>(DispatchAssignmentBehavior::default())
        .entity_with_behavior::<DamageReport, _>(DamageReportBehavior::default())
        .entity_with_behavior::<ProofOfDelivery, _>(ProofOfDeliveryBehavior::default())
        .entity_with_behavior::<OperationalException, _>(OperationalExceptionBehavior::default())
        .entity_with_behavior::<PickupInstruction, _>(PickupInstructionBehavior::default())
        .entity_with_behavior::<DeliveryInstruction, _>(DeliveryInstructionBehavior::default())
        .entity_with_behavior::<MoveInventory, _>(MoveInventoryBehavior::default())
        .entity_with_behavior::<PackagingItem, _>(PackagingItemBehavior::default())
        .entity_with_behavior::<LogisticsProvider, _>(LogisticsProviderBehavior::default())
        .entity_with_behavior::<ThirdPartyDispatch, _>(ThirdPartyDispatchBehavior::default())
        .entity_with_behavior::<FuelLog, _>(FuelLogBehavior::default())
        .entity_with_behavior::<MaintenanceRecord, _>(MaintenanceRecordBehavior::default())
        .entity_with_behavior::<TollReceipt, _>(TollReceiptBehavior::default())
        .entity_with_behavior::<ShiftLog, _>(ShiftLogBehavior::default())
        .entity_with_behavior::<CustomerFeedback, _>(CustomerFeedbackBehavior::default())
        .entity_with_behavior::<IncidentReport, _>(IncidentReportBehavior::default())
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Moving Company Platform")
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("RouteStatusType")
            .value("id", 1001_u64)
            .value("name", "Pending")
            .value("code", "PENDING")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("RouteStatusType")
            .value("id", 1002_u64)
            .value("name", "In Progress")
            .value("code", "IN_PROGRESS")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("RouteStatusType")
            .value("id", 1003_u64)
            .value("name", "Completed")
            .value("code", "COMPLETED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryConditionType")
            .value("id", 1001_u64)
            .value("name", "New")
            .value("code", "NEW")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryConditionType")
            .value("id", 1002_u64)
            .value("name", "Used")
            .value("code", "USED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryConditionType")
            .value("id", 1003_u64)
            .value("name", "Damaged")
            .value("code", "DAMAGED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ExceptionSeverity")
            .value("id", 1001_u64)
            .value("name", "Low")
            .value("code", "LOW")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ExceptionSeverity")
            .value("id", 1002_u64)
            .value("name", "Medium")
            .value("code", "MEDIUM")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ExceptionSeverity")
            .value("id", 1003_u64)
            .value("name", "High")
            .value("code", "HIGH")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("OrderStatus")
            .value("id", 1001_u64)
            .value("name", "Draft")
            .value("code", "DRAFT")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("OrderStatus")
            .value("id", 1002_u64)
            .value("name", "Confirmed")
            .value("code", "CONFIRMED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("OrderStatus")
            .value("id", 1003_u64)
            .value("name", "Completed")
            .value("code", "COMPLETED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CrewRole")
            .value("id", 1001_u64)
            .value("name", "Driver")
            .value("code", "DRIVER")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CrewRole")
            .value("id", 1002_u64)
            .value("name", "Mover")
            .value("code", "MOVER")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CrewRole")
            .value("id", 1003_u64)
            .value("name", "Supervisor")
            .value("code", "SUPERVISOR")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
}

pub fn module_with_behaviors_and_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<RouteStatusType, _>(RouteStatusTypeBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<RouteStatusType, _>::new(RouteStatusTypeChecker::default()))
        .entity_with_behavior::<InventoryConditionType, _>(InventoryConditionTypeBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<InventoryConditionType, _>::new(InventoryConditionTypeChecker::default()))
        .entity_with_behavior::<ExceptionSeverity, _>(ExceptionSeverityBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<ExceptionSeverity, _>::new(ExceptionSeverityChecker::default()))
        .entity_with_behavior::<OrderStatus, _>(OrderStatusBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<OrderStatus, _>::new(OrderStatusChecker::default()))
        .entity_with_behavior::<CrewRole, _>(CrewRoleBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<CrewRole, _>::new(CrewRoleChecker::default()))
        .entity_with_behavior::<Platform, _>(PlatformBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .entity_with_behavior::<Merchant, _>(MerchantBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Merchant, _>::new(MerchantChecker::default()))
        .entity_with_behavior::<MoveQuote, _>(MoveQuoteBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<MoveQuote, _>::new(MoveQuoteChecker::default()))
        .entity_with_behavior::<MoveOrder, _>(MoveOrderBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<MoveOrder, _>::new(MoveOrderChecker::default()))
        .entity_with_behavior::<RouteStop, _>(RouteStopBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<RouteStop, _>::new(RouteStopChecker::default()))
        .entity_with_behavior::<Crew, _>(CrewBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Crew, _>::new(CrewChecker::default()))
        .entity_with_behavior::<CrewMemberAssignment, _>(CrewMemberAssignmentBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<CrewMemberAssignment, _>::new(CrewMemberAssignmentChecker::default()))
        .entity_with_behavior::<Vehicle, _>(VehicleBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Vehicle, _>::new(VehicleChecker::default()))
        .entity_with_behavior::<VehicleAssignment, _>(VehicleAssignmentBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<VehicleAssignment, _>::new(VehicleAssignmentChecker::default()))
        .entity_with_behavior::<DispatchAssignment, _>(DispatchAssignmentBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<DispatchAssignment, _>::new(DispatchAssignmentChecker::default()))
        .entity_with_behavior::<DamageReport, _>(DamageReportBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<DamageReport, _>::new(DamageReportChecker::default()))
        .entity_with_behavior::<ProofOfDelivery, _>(ProofOfDeliveryBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<ProofOfDelivery, _>::new(ProofOfDeliveryChecker::default()))
        .entity_with_behavior::<OperationalException, _>(OperationalExceptionBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<OperationalException, _>::new(OperationalExceptionChecker::default()))
        .entity_with_behavior::<PickupInstruction, _>(PickupInstructionBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<PickupInstruction, _>::new(PickupInstructionChecker::default()))
        .entity_with_behavior::<DeliveryInstruction, _>(DeliveryInstructionBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<DeliveryInstruction, _>::new(DeliveryInstructionChecker::default()))
        .entity_with_behavior::<MoveInventory, _>(MoveInventoryBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<MoveInventory, _>::new(MoveInventoryChecker::default()))
        .entity_with_behavior::<PackagingItem, _>(PackagingItemBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<PackagingItem, _>::new(PackagingItemChecker::default()))
        .entity_with_behavior::<LogisticsProvider, _>(LogisticsProviderBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<LogisticsProvider, _>::new(LogisticsProviderChecker::default()))
        .entity_with_behavior::<ThirdPartyDispatch, _>(ThirdPartyDispatchBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<ThirdPartyDispatch, _>::new(ThirdPartyDispatchChecker::default()))
        .entity_with_behavior::<FuelLog, _>(FuelLogBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<FuelLog, _>::new(FuelLogChecker::default()))
        .entity_with_behavior::<MaintenanceRecord, _>(MaintenanceRecordBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<MaintenanceRecord, _>::new(MaintenanceRecordChecker::default()))
        .entity_with_behavior::<TollReceipt, _>(TollReceiptBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<TollReceipt, _>::new(TollReceiptChecker::default()))
        .entity_with_behavior::<ShiftLog, _>(ShiftLogBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<ShiftLog, _>::new(ShiftLogChecker::default()))
        .entity_with_behavior::<CustomerFeedback, _>(CustomerFeedbackBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<CustomerFeedback, _>::new(CustomerFeedbackChecker::default()))
        .entity_with_behavior::<IncidentReport, _>(IncidentReportBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<IncidentReport, _>::new(IncidentReportChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Moving Company Platform")
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("RouteStatusType")
            .value("id", 1001_u64)
            .value("name", "Pending")
            .value("code", "PENDING")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("RouteStatusType")
            .value("id", 1002_u64)
            .value("name", "In Progress")
            .value("code", "IN_PROGRESS")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("RouteStatusType")
            .value("id", 1003_u64)
            .value("name", "Completed")
            .value("code", "COMPLETED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryConditionType")
            .value("id", 1001_u64)
            .value("name", "New")
            .value("code", "NEW")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryConditionType")
            .value("id", 1002_u64)
            .value("name", "Used")
            .value("code", "USED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("InventoryConditionType")
            .value("id", 1003_u64)
            .value("name", "Damaged")
            .value("code", "DAMAGED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ExceptionSeverity")
            .value("id", 1001_u64)
            .value("name", "Low")
            .value("code", "LOW")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ExceptionSeverity")
            .value("id", 1002_u64)
            .value("name", "Medium")
            .value("code", "MEDIUM")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ExceptionSeverity")
            .value("id", 1003_u64)
            .value("name", "High")
            .value("code", "HIGH")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("OrderStatus")
            .value("id", 1001_u64)
            .value("name", "Draft")
            .value("code", "DRAFT")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("OrderStatus")
            .value("id", 1002_u64)
            .value("name", "Confirmed")
            .value("code", "CONFIRMED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("OrderStatus")
            .value("id", 1003_u64)
            .value("name", "Completed")
            .value("code", "COMPLETED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CrewRole")
            .value("id", 1001_u64)
            .value("name", "Driver")
            .value("code", "DRIVER")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CrewRole")
            .value("id", 1002_u64)
            .value("name", "Mover")
            .value("code", "MOVER")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CrewRole")
            .value("id", 1003_u64)
            .value("name", "Supervisor")
            .value("code", "SUPERVISOR")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
}