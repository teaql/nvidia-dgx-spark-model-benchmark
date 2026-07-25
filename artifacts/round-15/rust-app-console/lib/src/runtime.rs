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
            "InventoryItem" => Some(std::sync::Arc::new(crate::InventoryItem::entity_descriptor())),
            "MoveOrder" => Some(std::sync::Arc::new(crate::MoveOrder::entity_descriptor())),
            "Route" => Some(std::sync::Arc::new(crate::Route::entity_descriptor())),
            "Payment" => Some(std::sync::Arc::new(crate::Payment::entity_descriptor())),
            "Invoice" => Some(std::sync::Arc::new(crate::Invoice::entity_descriptor())),
            "Feedback" => Some(std::sync::Arc::new(crate::Feedback::entity_descriptor())),
            "Schedule" => Some(std::sync::Arc::new(crate::Schedule::entity_descriptor())),
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
        "customer_data", "employee_data", "truck_data", "inventory_item_data", "move_order_data", "route_data", "payment_data", "invoice_data", "feedback_data", "schedule_data"
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
        .with_entity("InventoryItem")
        .with_entity("MoveOrder")
        .with_entity("Route")
        .with_entity("Payment")
        .with_entity("Invoice")
        .with_entity("Feedback")
        .with_entity("Schedule")
}

pub fn behavior_registry() -> teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry {
    teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry::new()
        .with_behavior("Customer", CustomerBehavior::default())
        .with_behavior("Employee", EmployeeBehavior::default())
        .with_behavior("Truck", TruckBehavior::default())
        .with_behavior("InventoryItem", InventoryItemBehavior::default())
        .with_behavior("MoveOrder", MoveOrderBehavior::default())
        .with_behavior("Route", RouteBehavior::default())
        .with_behavior("Payment", PaymentBehavior::default())
        .with_behavior("Invoice", InvoiceBehavior::default())
        .with_behavior("Feedback", FeedbackBehavior::default())
        .with_behavior("Schedule", ScheduleBehavior::default())
}

pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
    teaql_runtime::InMemoryCheckerRegistry::new()
        .with_checker(teaql_runtime::TypedEntityChecker::<Customer, _>::new(CustomerChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Employee, _>::new(EmployeeChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Truck, _>::new(TruckChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<InventoryItem, _>::new(InventoryItemChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<MoveOrder, _>::new(MoveOrderChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Route, _>::new(RouteChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Payment, _>::new(PaymentChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Invoice, _>::new(InvoiceChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Feedback, _>::new(FeedbackChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Schedule, _>::new(ScheduleChecker::default()))
}

pub fn module() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<Customer>()
        .entity::<Employee>()
        .entity::<Truck>()
        .entity::<InventoryItem>()
        .entity::<MoveOrder>()
        .entity::<Route>()
        .entity::<Payment>()
        .entity::<Invoice>()
        .entity::<Feedback>()
        .entity::<Schedule>()
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
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
        .entity::<InventoryItem>()
        .checker(teaql_runtime::TypedEntityChecker::<InventoryItem, _>::new(InventoryItemChecker::default()))
        .entity::<MoveOrder>()
        .checker(teaql_runtime::TypedEntityChecker::<MoveOrder, _>::new(MoveOrderChecker::default()))
        .entity::<Route>()
        .checker(teaql_runtime::TypedEntityChecker::<Route, _>::new(RouteChecker::default()))
        .entity::<Payment>()
        .checker(teaql_runtime::TypedEntityChecker::<Payment, _>::new(PaymentChecker::default()))
        .entity::<Invoice>()
        .checker(teaql_runtime::TypedEntityChecker::<Invoice, _>::new(InvoiceChecker::default()))
        .entity::<Feedback>()
        .checker(teaql_runtime::TypedEntityChecker::<Feedback, _>::new(FeedbackChecker::default()))
        .entity::<Schedule>()
        .checker(teaql_runtime::TypedEntityChecker::<Schedule, _>::new(ScheduleChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", 1_u64)
            .value("version", 1_i64))
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<Customer, _>(CustomerBehavior::default())
        .entity_with_behavior::<Employee, _>(EmployeeBehavior::default())
        .entity_with_behavior::<Truck, _>(TruckBehavior::default())
        .entity_with_behavior::<InventoryItem, _>(InventoryItemBehavior::default())
        .entity_with_behavior::<MoveOrder, _>(MoveOrderBehavior::default())
        .entity_with_behavior::<Route, _>(RouteBehavior::default())
        .entity_with_behavior::<Payment, _>(PaymentBehavior::default())
        .entity_with_behavior::<Invoice, _>(InvoiceBehavior::default())
        .entity_with_behavior::<Feedback, _>(FeedbackBehavior::default())
        .entity_with_behavior::<Schedule, _>(ScheduleBehavior::default())
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
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
        .entity_with_behavior::<InventoryItem, _>(InventoryItemBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<InventoryItem, _>::new(InventoryItemChecker::default()))
        .entity_with_behavior::<MoveOrder, _>(MoveOrderBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<MoveOrder, _>::new(MoveOrderChecker::default()))
        .entity_with_behavior::<Route, _>(RouteBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Route, _>::new(RouteChecker::default()))
        .entity_with_behavior::<Payment, _>(PaymentBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Payment, _>::new(PaymentChecker::default()))
        .entity_with_behavior::<Invoice, _>(InvoiceBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Invoice, _>::new(InvoiceChecker::default()))
        .entity_with_behavior::<Feedback, _>(FeedbackBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Feedback, _>::new(FeedbackChecker::default()))
        .entity_with_behavior::<Schedule, _>(ScheduleBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Schedule, _>::new(ScheduleChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", 1_u64)
            .value("version", 1_i64))
}