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

pub const DATABASE_URL_ENV: &str = "MAIN_SERVICE_CORE_DATABASE_URL";
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
            "Employee" => Some(std::sync::Arc::new(crate::Employee::entity_descriptor())),
            "Truck" => Some(std::sync::Arc::new(crate::Truck::entity_descriptor())),
            "Route" => Some(std::sync::Arc::new(crate::Route::entity_descriptor())),
            "Invoice" => Some(std::sync::Arc::new(crate::Invoice::entity_descriptor())),
            "Payment" => Some(std::sync::Arc::new(crate::Payment::entity_descriptor())),
            "Schedule" => Some(std::sync::Arc::new(crate::Schedule::entity_descriptor())),
            "Warehouse" => Some(std::sync::Arc::new(crate::Warehouse::entity_descriptor())),
            "Inventory" => Some(std::sync::Arc::new(crate::Inventory::entity_descriptor())),
            "Quote" => Some(std::sync::Arc::new(crate::Quote::entity_descriptor())),
            "Contract" => Some(std::sync::Arc::new(crate::Contract::entity_descriptor())),
            "Feedback" => Some(std::sync::Arc::new(crate::Feedback::entity_descriptor())),
            "Insurance" => Some(std::sync::Arc::new(crate::Insurance::entity_descriptor())),
            "Maintenance" => Some(std::sync::Arc::new(crate::Maintenance::entity_descriptor())),
            "Driver" => Some(std::sync::Arc::new(crate::Driver::entity_descriptor())),
            "Address" => Some(std::sync::Arc::new(crate::Address::entity_descriptor())),
            "Service" => Some(std::sync::Arc::new(crate::Service::entity_descriptor())),
            "Equipment" => Some(std::sync::Arc::new(crate::Equipment::entity_descriptor())),
            "Tracking" => Some(std::sync::Arc::new(crate::Tracking::entity_descriptor())),
            "Report" => Some(std::sync::Arc::new(crate::Report::entity_descriptor())),
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
        "customer_data", "employee_data", "truck_data", "route_data", "invoice_data", "payment_data", "schedule_data", "warehouse_data", "inventory_data", "quote_data", "contract_data", "feedback_data", "insurance_data", "maintenance_data", "driver_data", "address_data", "service_data", "equipment_data", "tracking_data", "report_data"
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
        .with_entity("Employee")
        .with_entity("Truck")
        .with_entity("Route")
        .with_entity("Invoice")
        .with_entity("Payment")
        .with_entity("Schedule")
        .with_entity("Warehouse")
        .with_entity("Inventory")
        .with_entity("Quote")
        .with_entity("Contract")
        .with_entity("Feedback")
        .with_entity("Insurance")
        .with_entity("Maintenance")
        .with_entity("Driver")
        .with_entity("Address")
        .with_entity("Service")
        .with_entity("Equipment")
        .with_entity("Tracking")
        .with_entity("Report")
}

pub fn behavior_registry() -> teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry {
    teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry::new()
        .with_behavior("Customer", CustomerBehavior::default())
        .with_behavior("Employee", EmployeeBehavior::default())
        .with_behavior("Truck", TruckBehavior::default())
        .with_behavior("Route", RouteBehavior::default())
        .with_behavior("Invoice", InvoiceBehavior::default())
        .with_behavior("Payment", PaymentBehavior::default())
        .with_behavior("Schedule", ScheduleBehavior::default())
        .with_behavior("Warehouse", WarehouseBehavior::default())
        .with_behavior("Inventory", InventoryBehavior::default())
        .with_behavior("Quote", QuoteBehavior::default())
        .with_behavior("Contract", ContractBehavior::default())
        .with_behavior("Feedback", FeedbackBehavior::default())
        .with_behavior("Insurance", InsuranceBehavior::default())
        .with_behavior("Maintenance", MaintenanceBehavior::default())
        .with_behavior("Driver", DriverBehavior::default())
        .with_behavior("Address", AddressBehavior::default())
        .with_behavior("Service", ServiceBehavior::default())
        .with_behavior("Equipment", EquipmentBehavior::default())
        .with_behavior("Tracking", TrackingBehavior::default())
        .with_behavior("Report", ReportBehavior::default())
}

pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
    teaql_runtime::InMemoryCheckerRegistry::new()
        .with_checker(teaql_runtime::TypedEntityChecker::<Customer, _>::new(CustomerChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Employee, _>::new(EmployeeChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Truck, _>::new(TruckChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Route, _>::new(RouteChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Invoice, _>::new(InvoiceChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Payment, _>::new(PaymentChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Schedule, _>::new(ScheduleChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Warehouse, _>::new(WarehouseChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Inventory, _>::new(InventoryChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Quote, _>::new(QuoteChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Contract, _>::new(ContractChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Feedback, _>::new(FeedbackChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Insurance, _>::new(InsuranceChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Maintenance, _>::new(MaintenanceChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Driver, _>::new(DriverChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Address, _>::new(AddressChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Service, _>::new(ServiceChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Equipment, _>::new(EquipmentChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Tracking, _>::new(TrackingChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Report, _>::new(ReportChecker::default()))
}

pub fn module() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<Customer>()
        .entity::<Employee>()
        .entity::<Truck>()
        .entity::<Route>()
        .entity::<Invoice>()
        .entity::<Payment>()
        .entity::<Schedule>()
        .entity::<Warehouse>()
        .entity::<Inventory>()
        .entity::<Quote>()
        .entity::<Contract>()
        .entity::<Feedback>()
        .entity::<Insurance>()
        .entity::<Maintenance>()
        .entity::<Driver>()
        .entity::<Address>()
        .entity::<Service>()
        .entity::<Equipment>()
        .entity::<Tracking>()
        .entity::<Report>()
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Employee")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Truck")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Route")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Invoice")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Payment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Schedule")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Warehouse")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Inventory")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Quote")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Contract")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Feedback")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Insurance")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Maintenance")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Driver")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Address")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Service")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Equipment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Tracking")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Report")
            .value("id", 1_u64)
            .value("version", 1_i64))
}

pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<Customer>()
        .checker(teaql_runtime::TypedEntityChecker::<Customer, _>::new(CustomerChecker::default()))
        .entity::<Employee>()
        .checker(teaql_runtime::TypedEntityChecker::<Employee, _>::new(EmployeeChecker::default()))
        .entity::<Truck>()
        .checker(teaql_runtime::TypedEntityChecker::<Truck, _>::new(TruckChecker::default()))
        .entity::<Route>()
        .checker(teaql_runtime::TypedEntityChecker::<Route, _>::new(RouteChecker::default()))
        .entity::<Invoice>()
        .checker(teaql_runtime::TypedEntityChecker::<Invoice, _>::new(InvoiceChecker::default()))
        .entity::<Payment>()
        .checker(teaql_runtime::TypedEntityChecker::<Payment, _>::new(PaymentChecker::default()))
        .entity::<Schedule>()
        .checker(teaql_runtime::TypedEntityChecker::<Schedule, _>::new(ScheduleChecker::default()))
        .entity::<Warehouse>()
        .checker(teaql_runtime::TypedEntityChecker::<Warehouse, _>::new(WarehouseChecker::default()))
        .entity::<Inventory>()
        .checker(teaql_runtime::TypedEntityChecker::<Inventory, _>::new(InventoryChecker::default()))
        .entity::<Quote>()
        .checker(teaql_runtime::TypedEntityChecker::<Quote, _>::new(QuoteChecker::default()))
        .entity::<Contract>()
        .checker(teaql_runtime::TypedEntityChecker::<Contract, _>::new(ContractChecker::default()))
        .entity::<Feedback>()
        .checker(teaql_runtime::TypedEntityChecker::<Feedback, _>::new(FeedbackChecker::default()))
        .entity::<Insurance>()
        .checker(teaql_runtime::TypedEntityChecker::<Insurance, _>::new(InsuranceChecker::default()))
        .entity::<Maintenance>()
        .checker(teaql_runtime::TypedEntityChecker::<Maintenance, _>::new(MaintenanceChecker::default()))
        .entity::<Driver>()
        .checker(teaql_runtime::TypedEntityChecker::<Driver, _>::new(DriverChecker::default()))
        .entity::<Address>()
        .checker(teaql_runtime::TypedEntityChecker::<Address, _>::new(AddressChecker::default()))
        .entity::<Service>()
        .checker(teaql_runtime::TypedEntityChecker::<Service, _>::new(ServiceChecker::default()))
        .entity::<Equipment>()
        .checker(teaql_runtime::TypedEntityChecker::<Equipment, _>::new(EquipmentChecker::default()))
        .entity::<Tracking>()
        .checker(teaql_runtime::TypedEntityChecker::<Tracking, _>::new(TrackingChecker::default()))
        .entity::<Report>()
        .checker(teaql_runtime::TypedEntityChecker::<Report, _>::new(ReportChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Employee")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Truck")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Route")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Invoice")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Payment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Schedule")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Warehouse")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Inventory")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Quote")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Contract")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Feedback")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Insurance")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Maintenance")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Driver")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Address")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Service")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Equipment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Tracking")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Report")
            .value("id", 1_u64)
            .value("version", 1_i64))
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<Customer, _>(CustomerBehavior::default())
        .entity_with_behavior::<Employee, _>(EmployeeBehavior::default())
        .entity_with_behavior::<Truck, _>(TruckBehavior::default())
        .entity_with_behavior::<Route, _>(RouteBehavior::default())
        .entity_with_behavior::<Invoice, _>(InvoiceBehavior::default())
        .entity_with_behavior::<Payment, _>(PaymentBehavior::default())
        .entity_with_behavior::<Schedule, _>(ScheduleBehavior::default())
        .entity_with_behavior::<Warehouse, _>(WarehouseBehavior::default())
        .entity_with_behavior::<Inventory, _>(InventoryBehavior::default())
        .entity_with_behavior::<Quote, _>(QuoteBehavior::default())
        .entity_with_behavior::<Contract, _>(ContractBehavior::default())
        .entity_with_behavior::<Feedback, _>(FeedbackBehavior::default())
        .entity_with_behavior::<Insurance, _>(InsuranceBehavior::default())
        .entity_with_behavior::<Maintenance, _>(MaintenanceBehavior::default())
        .entity_with_behavior::<Driver, _>(DriverBehavior::default())
        .entity_with_behavior::<Address, _>(AddressBehavior::default())
        .entity_with_behavior::<Service, _>(ServiceBehavior::default())
        .entity_with_behavior::<Equipment, _>(EquipmentBehavior::default())
        .entity_with_behavior::<Tracking, _>(TrackingBehavior::default())
        .entity_with_behavior::<Report, _>(ReportBehavior::default())
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Employee")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Truck")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Route")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Invoice")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Payment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Schedule")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Warehouse")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Inventory")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Quote")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Contract")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Feedback")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Insurance")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Maintenance")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Driver")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Address")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Service")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Equipment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Tracking")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Report")
            .value("id", 1_u64)
            .value("version", 1_i64))
}

pub fn module_with_behaviors_and_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<Customer, _>(CustomerBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Customer, _>::new(CustomerChecker::default()))
        .entity_with_behavior::<Employee, _>(EmployeeBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Employee, _>::new(EmployeeChecker::default()))
        .entity_with_behavior::<Truck, _>(TruckBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Truck, _>::new(TruckChecker::default()))
        .entity_with_behavior::<Route, _>(RouteBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Route, _>::new(RouteChecker::default()))
        .entity_with_behavior::<Invoice, _>(InvoiceBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Invoice, _>::new(InvoiceChecker::default()))
        .entity_with_behavior::<Payment, _>(PaymentBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Payment, _>::new(PaymentChecker::default()))
        .entity_with_behavior::<Schedule, _>(ScheduleBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Schedule, _>::new(ScheduleChecker::default()))
        .entity_with_behavior::<Warehouse, _>(WarehouseBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Warehouse, _>::new(WarehouseChecker::default()))
        .entity_with_behavior::<Inventory, _>(InventoryBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Inventory, _>::new(InventoryChecker::default()))
        .entity_with_behavior::<Quote, _>(QuoteBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Quote, _>::new(QuoteChecker::default()))
        .entity_with_behavior::<Contract, _>(ContractBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Contract, _>::new(ContractChecker::default()))
        .entity_with_behavior::<Feedback, _>(FeedbackBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Feedback, _>::new(FeedbackChecker::default()))
        .entity_with_behavior::<Insurance, _>(InsuranceBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Insurance, _>::new(InsuranceChecker::default()))
        .entity_with_behavior::<Maintenance, _>(MaintenanceBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Maintenance, _>::new(MaintenanceChecker::default()))
        .entity_with_behavior::<Driver, _>(DriverBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Driver, _>::new(DriverChecker::default()))
        .entity_with_behavior::<Address, _>(AddressBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Address, _>::new(AddressChecker::default()))
        .entity_with_behavior::<Service, _>(ServiceBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Service, _>::new(ServiceChecker::default()))
        .entity_with_behavior::<Equipment, _>(EquipmentBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Equipment, _>::new(EquipmentChecker::default()))
        .entity_with_behavior::<Tracking, _>(TrackingBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Tracking, _>::new(TrackingChecker::default()))
        .entity_with_behavior::<Report, _>(ReportBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Report, _>::new(ReportChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Employee")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Truck")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Route")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Invoice")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Payment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Schedule")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Warehouse")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Inventory")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Quote")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Contract")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Feedback")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Insurance")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Maintenance")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Driver")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Address")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Service")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Equipment")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Tracking")
            .value("id", 1_u64)
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Report")
            .value("id", 1_u64)
            .value("version", 1_i64))
}