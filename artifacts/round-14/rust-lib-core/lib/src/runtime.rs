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

pub const DATABASE_URL_ENV: &str = "MOVING_COMPANY_SERVICE_CORE_DATABASE_URL";
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
            "Platform" => Some(std::sync::Arc::new(crate::Platform::entity_descriptor())),
            "Merchant" => Some(std::sync::Arc::new(crate::Merchant::entity_descriptor())),
            "MoveOrder" => Some(std::sync::Arc::new(crate::MoveOrder::entity_descriptor())),
            "Employee" => Some(std::sync::Arc::new(crate::Employee::entity_descriptor())),
            "Customer" => Some(std::sync::Arc::new(crate::Customer::entity_descriptor())),
            "Product" => Some(std::sync::Arc::new(crate::Product::entity_descriptor())),
            "Campaign" => Some(std::sync::Arc::new(crate::Campaign::entity_descriptor())),
            "Payment" => Some(std::sync::Arc::new(crate::Payment::entity_descriptor())),
            "Vehicle" => Some(std::sync::Arc::new(crate::Vehicle::entity_descriptor())),
            "Contract" => Some(std::sync::Arc::new(crate::Contract::entity_descriptor())),
            "UserAccount" => Some(std::sync::Arc::new(crate::UserAccount::entity_descriptor())),
            "ActivityLog" => Some(std::sync::Arc::new(crate::ActivityLog::entity_descriptor())),
            "Notification" => Some(std::sync::Arc::new(crate::Notification::entity_descriptor())),
            "ApiClient" => Some(std::sync::Arc::new(crate::ApiClient::entity_descriptor())),
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
        "platform_data", "merchant_data", "move_order_data", "employee_data", "customer_data", "product_data", "campaign_data", "payment_data", "vehicle_data", "contract_data", "user_account_data", "activity_log_data", "notification_data", "api_client_data"
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
        .with_entity("Platform")
        .with_entity("Merchant")
        .with_entity("MoveOrder")
        .with_entity("Employee")
        .with_entity("Customer")
        .with_entity("Product")
        .with_entity("Campaign")
        .with_entity("Payment")
        .with_entity("Vehicle")
        .with_entity("Contract")
        .with_entity("UserAccount")
        .with_entity("ActivityLog")
        .with_entity("Notification")
        .with_entity("ApiClient")
}

pub fn behavior_registry() -> teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry {
    teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry::new()
        .with_behavior("Platform", PlatformBehavior::default())
        .with_behavior("Merchant", MerchantBehavior::default())
        .with_behavior("MoveOrder", MoveOrderBehavior::default())
        .with_behavior("Employee", EmployeeBehavior::default())
        .with_behavior("Customer", CustomerBehavior::default())
        .with_behavior("Product", ProductBehavior::default())
        .with_behavior("Campaign", CampaignBehavior::default())
        .with_behavior("Payment", PaymentBehavior::default())
        .with_behavior("Vehicle", VehicleBehavior::default())
        .with_behavior("Contract", ContractBehavior::default())
        .with_behavior("UserAccount", UserAccountBehavior::default())
        .with_behavior("ActivityLog", ActivityLogBehavior::default())
        .with_behavior("Notification", NotificationBehavior::default())
        .with_behavior("ApiClient", ApiClientBehavior::default())
}

pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
    teaql_runtime::InMemoryCheckerRegistry::new()
        .with_checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Merchant, _>::new(MerchantChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<MoveOrder, _>::new(MoveOrderChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Employee, _>::new(EmployeeChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Customer, _>::new(CustomerChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Product, _>::new(ProductChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Campaign, _>::new(CampaignChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Payment, _>::new(PaymentChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Vehicle, _>::new(VehicleChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Contract, _>::new(ContractChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<UserAccount, _>::new(UserAccountChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ActivityLog, _>::new(ActivityLogChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Notification, _>::new(NotificationChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ApiClient, _>::new(ApiClientChecker::default()))
}

pub fn module() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<Platform>()
        .entity::<Merchant>()
        .entity::<MoveOrder>()
        .entity::<Employee>()
        .entity::<Customer>()
        .entity::<Product>()
        .entity::<Campaign>()
        .entity::<Payment>()
        .entity::<Vehicle>()
        .entity::<Contract>()
        .entity::<UserAccount>()
        .entity::<ActivityLog>()
        .entity::<Notification>()
        .entity::<ApiClient>()
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Merchant")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("MoveOrder")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Employee")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Product")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Campaign")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Payment")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Vehicle")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Contract")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("UserAccount")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("ActivityLog")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Notification")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("ApiClient")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
}

pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<Platform>()
        .checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .entity::<Merchant>()
        .checker(teaql_runtime::TypedEntityChecker::<Merchant, _>::new(MerchantChecker::default()))
        .entity::<MoveOrder>()
        .checker(teaql_runtime::TypedEntityChecker::<MoveOrder, _>::new(MoveOrderChecker::default()))
        .entity::<Employee>()
        .checker(teaql_runtime::TypedEntityChecker::<Employee, _>::new(EmployeeChecker::default()))
        .entity::<Customer>()
        .checker(teaql_runtime::TypedEntityChecker::<Customer, _>::new(CustomerChecker::default()))
        .entity::<Product>()
        .checker(teaql_runtime::TypedEntityChecker::<Product, _>::new(ProductChecker::default()))
        .entity::<Campaign>()
        .checker(teaql_runtime::TypedEntityChecker::<Campaign, _>::new(CampaignChecker::default()))
        .entity::<Payment>()
        .checker(teaql_runtime::TypedEntityChecker::<Payment, _>::new(PaymentChecker::default()))
        .entity::<Vehicle>()
        .checker(teaql_runtime::TypedEntityChecker::<Vehicle, _>::new(VehicleChecker::default()))
        .entity::<Contract>()
        .checker(teaql_runtime::TypedEntityChecker::<Contract, _>::new(ContractChecker::default()))
        .entity::<UserAccount>()
        .checker(teaql_runtime::TypedEntityChecker::<UserAccount, _>::new(UserAccountChecker::default()))
        .entity::<ActivityLog>()
        .checker(teaql_runtime::TypedEntityChecker::<ActivityLog, _>::new(ActivityLogChecker::default()))
        .entity::<Notification>()
        .checker(teaql_runtime::TypedEntityChecker::<Notification, _>::new(NotificationChecker::default()))
        .entity::<ApiClient>()
        .checker(teaql_runtime::TypedEntityChecker::<ApiClient, _>::new(ApiClientChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Merchant")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("MoveOrder")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Employee")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Product")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Campaign")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Payment")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Vehicle")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Contract")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("UserAccount")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("ActivityLog")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Notification")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("ApiClient")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<Platform, _>(PlatformBehavior::default())
        .entity_with_behavior::<Merchant, _>(MerchantBehavior::default())
        .entity_with_behavior::<MoveOrder, _>(MoveOrderBehavior::default())
        .entity_with_behavior::<Employee, _>(EmployeeBehavior::default())
        .entity_with_behavior::<Customer, _>(CustomerBehavior::default())
        .entity_with_behavior::<Product, _>(ProductBehavior::default())
        .entity_with_behavior::<Campaign, _>(CampaignBehavior::default())
        .entity_with_behavior::<Payment, _>(PaymentBehavior::default())
        .entity_with_behavior::<Vehicle, _>(VehicleBehavior::default())
        .entity_with_behavior::<Contract, _>(ContractBehavior::default())
        .entity_with_behavior::<UserAccount, _>(UserAccountBehavior::default())
        .entity_with_behavior::<ActivityLog, _>(ActivityLogBehavior::default())
        .entity_with_behavior::<Notification, _>(NotificationBehavior::default())
        .entity_with_behavior::<ApiClient, _>(ApiClientBehavior::default())
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Merchant")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("MoveOrder")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Employee")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Product")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Campaign")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Payment")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Vehicle")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Contract")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("UserAccount")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("ActivityLog")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Notification")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("ApiClient")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
}

pub fn module_with_behaviors_and_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<Platform, _>(PlatformBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .entity_with_behavior::<Merchant, _>(MerchantBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Merchant, _>::new(MerchantChecker::default()))
        .entity_with_behavior::<MoveOrder, _>(MoveOrderBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<MoveOrder, _>::new(MoveOrderChecker::default()))
        .entity_with_behavior::<Employee, _>(EmployeeBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Employee, _>::new(EmployeeChecker::default()))
        .entity_with_behavior::<Customer, _>(CustomerBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Customer, _>::new(CustomerChecker::default()))
        .entity_with_behavior::<Product, _>(ProductBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Product, _>::new(ProductChecker::default()))
        .entity_with_behavior::<Campaign, _>(CampaignBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Campaign, _>::new(CampaignChecker::default()))
        .entity_with_behavior::<Payment, _>(PaymentBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Payment, _>::new(PaymentChecker::default()))
        .entity_with_behavior::<Vehicle, _>(VehicleBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Vehicle, _>::new(VehicleChecker::default()))
        .entity_with_behavior::<Contract, _>(ContractBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Contract, _>::new(ContractChecker::default()))
        .entity_with_behavior::<UserAccount, _>(UserAccountBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<UserAccount, _>::new(UserAccountChecker::default()))
        .entity_with_behavior::<ActivityLog, _>(ActivityLogBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<ActivityLog, _>::new(ActivityLogChecker::default()))
        .entity_with_behavior::<Notification, _>(NotificationBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Notification, _>::new(NotificationChecker::default()))
        .entity_with_behavior::<ApiClient, _>(ApiClientBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<ApiClient, _>::new(ApiClientChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Merchant")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("MoveOrder")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Employee")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Customer")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Product")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Campaign")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Payment")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Vehicle")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Contract")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("UserAccount")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("ActivityLog")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Notification")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("ApiClient")
            .value("id", "0u64")
            .value("name", "Unknown")
            .value("merchant_id", "0u64")
            .value("version", 1_i64))
}