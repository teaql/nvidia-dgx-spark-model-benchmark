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

pub const DATABASE_URL_ENV: &str = "HR_PAYROLL_MICROSERVICE_CORE_DATABASE_URL";
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
            "LeaveType" => Some(std::sync::Arc::new(crate::LeaveType::entity_descriptor())),
            "EmployeeStatus" => Some(std::sync::Arc::new(crate::EmployeeStatus::entity_descriptor())),
            "ContractType" => Some(std::sync::Arc::new(crate::ContractType::entity_descriptor())),
            "ReviewGrade" => Some(std::sync::Arc::new(crate::ReviewGrade::entity_descriptor())),
            "ApplicationStatus" => Some(std::sync::Arc::new(crate::ApplicationStatus::entity_descriptor())),
            "Platform" => Some(std::sync::Arc::new(crate::Platform::entity_descriptor())),
            "Merchant" => Some(std::sync::Arc::new(crate::Merchant::entity_descriptor())),
            "Department" => Some(std::sync::Arc::new(crate::Department::entity_descriptor())),
            "Position" => Some(std::sync::Arc::new(crate::Position::entity_descriptor())),
            "Employee" => Some(std::sync::Arc::new(crate::Employee::entity_descriptor())),
            "SalaryRecord" => Some(std::sync::Arc::new(crate::SalaryRecord::entity_descriptor())),
            "AttendanceLog" => Some(std::sync::Arc::new(crate::AttendanceLog::entity_descriptor())),
            "LeaveRequest" => Some(std::sync::Arc::new(crate::LeaveRequest::entity_descriptor())),
            "PerformanceReview" => Some(std::sync::Arc::new(crate::PerformanceReview::entity_descriptor())),
            "TrainingRecord" => Some(std::sync::Arc::new(crate::TrainingRecord::entity_descriptor())),
            "BenefitPlan" => Some(std::sync::Arc::new(crate::BenefitPlan::entity_descriptor())),
            "ExpenseClaim" => Some(std::sync::Arc::new(crate::ExpenseClaim::entity_descriptor())),
            "PayrollRun" => Some(std::sync::Arc::new(crate::PayrollRun::entity_descriptor())),
            "TaxForm" => Some(std::sync::Arc::new(crate::TaxForm::entity_descriptor())),
            "Contract" => Some(std::sync::Arc::new(crate::Contract::entity_descriptor())),
            "Resignation" => Some(std::sync::Arc::new(crate::Resignation::entity_descriptor())),
            "WarningLetter" => Some(std::sync::Arc::new(crate::WarningLetter::entity_descriptor())),
            "BonusRecord" => Some(std::sync::Arc::new(crate::BonusRecord::entity_descriptor())),
            "ShiftSchedule" => Some(std::sync::Arc::new(crate::ShiftSchedule::entity_descriptor())),
            "TimeOffBalance" => Some(std::sync::Arc::new(crate::TimeOffBalance::entity_descriptor())),
            "RecruitmentPost" => Some(std::sync::Arc::new(crate::RecruitmentPost::entity_descriptor())),
            "JobApplication" => Some(std::sync::Arc::new(crate::JobApplication::entity_descriptor())),
            "Interview" => Some(std::sync::Arc::new(crate::Interview::entity_descriptor())),
            "OfferLetter" => Some(std::sync::Arc::new(crate::OfferLetter::entity_descriptor())),
            "OnboardingChecklist" => Some(std::sync::Arc::new(crate::OnboardingChecklist::entity_descriptor())),
            "OffboardingChecklist" => Some(std::sync::Arc::new(crate::OffboardingChecklist::entity_descriptor())),
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
        "leave_type_data", "employee_status_data", "contract_type_data", "review_grade_data", "application_status_data", "platform_data", "merchant_data", "department_data", "position_data", "employee_data", "salary_record_data", "attendance_log_data", "leave_request_data", "performance_review_data", "training_record_data", "benefit_plan_data", "expense_claim_data", "payroll_run_data", "tax_form_data", "contract_data", "resignation_data", "warning_letter_data", "bonus_record_data", "shift_schedule_data", "time_off_balance_data", "recruitment_post_data", "job_application_data", "interview_data", "offer_letter_data", "onboarding_checklist_data", "offboarding_checklist_data"
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
        .with_entity("LeaveType")
        .with_entity("EmployeeStatus")
        .with_entity("ContractType")
        .with_entity("ReviewGrade")
        .with_entity("ApplicationStatus")
        .with_entity("Platform")
        .with_entity("Merchant")
        .with_entity("Department")
        .with_entity("Position")
        .with_entity("Employee")
        .with_entity("SalaryRecord")
        .with_entity("AttendanceLog")
        .with_entity("LeaveRequest")
        .with_entity("PerformanceReview")
        .with_entity("TrainingRecord")
        .with_entity("BenefitPlan")
        .with_entity("ExpenseClaim")
        .with_entity("PayrollRun")
        .with_entity("TaxForm")
        .with_entity("Contract")
        .with_entity("Resignation")
        .with_entity("WarningLetter")
        .with_entity("BonusRecord")
        .with_entity("ShiftSchedule")
        .with_entity("TimeOffBalance")
        .with_entity("RecruitmentPost")
        .with_entity("JobApplication")
        .with_entity("Interview")
        .with_entity("OfferLetter")
        .with_entity("OnboardingChecklist")
        .with_entity("OffboardingChecklist")
}

pub fn behavior_registry() -> teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry {
    teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry::new()
        .with_behavior("LeaveType", LeaveTypeBehavior::default())
        .with_behavior("EmployeeStatus", EmployeeStatusBehavior::default())
        .with_behavior("ContractType", ContractTypeBehavior::default())
        .with_behavior("ReviewGrade", ReviewGradeBehavior::default())
        .with_behavior("ApplicationStatus", ApplicationStatusBehavior::default())
        .with_behavior("Platform", PlatformBehavior::default())
        .with_behavior("Merchant", MerchantBehavior::default())
        .with_behavior("Department", DepartmentBehavior::default())
        .with_behavior("Position", PositionBehavior::default())
        .with_behavior("Employee", EmployeeBehavior::default())
        .with_behavior("SalaryRecord", SalaryRecordBehavior::default())
        .with_behavior("AttendanceLog", AttendanceLogBehavior::default())
        .with_behavior("LeaveRequest", LeaveRequestBehavior::default())
        .with_behavior("PerformanceReview", PerformanceReviewBehavior::default())
        .with_behavior("TrainingRecord", TrainingRecordBehavior::default())
        .with_behavior("BenefitPlan", BenefitPlanBehavior::default())
        .with_behavior("ExpenseClaim", ExpenseClaimBehavior::default())
        .with_behavior("PayrollRun", PayrollRunBehavior::default())
        .with_behavior("TaxForm", TaxFormBehavior::default())
        .with_behavior("Contract", ContractBehavior::default())
        .with_behavior("Resignation", ResignationBehavior::default())
        .with_behavior("WarningLetter", WarningLetterBehavior::default())
        .with_behavior("BonusRecord", BonusRecordBehavior::default())
        .with_behavior("ShiftSchedule", ShiftScheduleBehavior::default())
        .with_behavior("TimeOffBalance", TimeOffBalanceBehavior::default())
        .with_behavior("RecruitmentPost", RecruitmentPostBehavior::default())
        .with_behavior("JobApplication", JobApplicationBehavior::default())
        .with_behavior("Interview", InterviewBehavior::default())
        .with_behavior("OfferLetter", OfferLetterBehavior::default())
        .with_behavior("OnboardingChecklist", OnboardingChecklistBehavior::default())
        .with_behavior("OffboardingChecklist", OffboardingChecklistBehavior::default())
}

pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
    teaql_runtime::InMemoryCheckerRegistry::new()
        .with_checker(teaql_runtime::TypedEntityChecker::<LeaveType, _>::new(LeaveTypeChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<EmployeeStatus, _>::new(EmployeeStatusChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ContractType, _>::new(ContractTypeChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ReviewGrade, _>::new(ReviewGradeChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ApplicationStatus, _>::new(ApplicationStatusChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Merchant, _>::new(MerchantChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Department, _>::new(DepartmentChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Position, _>::new(PositionChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Employee, _>::new(EmployeeChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<SalaryRecord, _>::new(SalaryRecordChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<AttendanceLog, _>::new(AttendanceLogChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<LeaveRequest, _>::new(LeaveRequestChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<PerformanceReview, _>::new(PerformanceReviewChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<TrainingRecord, _>::new(TrainingRecordChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<BenefitPlan, _>::new(BenefitPlanChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ExpenseClaim, _>::new(ExpenseClaimChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<PayrollRun, _>::new(PayrollRunChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<TaxForm, _>::new(TaxFormChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Contract, _>::new(ContractChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Resignation, _>::new(ResignationChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<WarningLetter, _>::new(WarningLetterChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<BonusRecord, _>::new(BonusRecordChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<ShiftSchedule, _>::new(ShiftScheduleChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<TimeOffBalance, _>::new(TimeOffBalanceChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<RecruitmentPost, _>::new(RecruitmentPostChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<JobApplication, _>::new(JobApplicationChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Interview, _>::new(InterviewChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<OfferLetter, _>::new(OfferLetterChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<OnboardingChecklist, _>::new(OnboardingChecklistChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<OffboardingChecklist, _>::new(OffboardingChecklistChecker::default()))
}

pub fn module() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<LeaveType>()
        .entity::<EmployeeStatus>()
        .entity::<ContractType>()
        .entity::<ReviewGrade>()
        .entity::<ApplicationStatus>()
        .entity::<Platform>()
        .entity::<Merchant>()
        .entity::<Department>()
        .entity::<Position>()
        .entity::<Employee>()
        .entity::<SalaryRecord>()
        .entity::<AttendanceLog>()
        .entity::<LeaveRequest>()
        .entity::<PerformanceReview>()
        .entity::<TrainingRecord>()
        .entity::<BenefitPlan>()
        .entity::<ExpenseClaim>()
        .entity::<PayrollRun>()
        .entity::<TaxForm>()
        .entity::<Contract>()
        .entity::<Resignation>()
        .entity::<WarningLetter>()
        .entity::<BonusRecord>()
        .entity::<ShiftSchedule>()
        .entity::<TimeOffBalance>()
        .entity::<RecruitmentPost>()
        .entity::<JobApplication>()
        .entity::<Interview>()
        .entity::<OfferLetter>()
        .entity::<OnboardingChecklist>()
        .entity::<OffboardingChecklist>()
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Moving Company Platform")
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("LeaveType")
            .value("id", 1001_u64)
            .value("name", "Annual")
            .value("code", "ANNUAL")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("LeaveType")
            .value("id", 1002_u64)
            .value("name", "Sick")
            .value("code", "SICK")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("LeaveType")
            .value("id", 1003_u64)
            .value("name", "Unpaid")
            .value("code", "UNPAID")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("EmployeeStatus")
            .value("id", 1001_u64)
            .value("name", "Active")
            .value("code", "ACTIVE")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("EmployeeStatus")
            .value("id", 1002_u64)
            .value("name", "Probation")
            .value("code", "PROBATION")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("EmployeeStatus")
            .value("id", 1003_u64)
            .value("name", "Terminated")
            .value("code", "TERMINATED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ContractType")
            .value("id", 1001_u64)
            .value("name", "Full Time")
            .value("code", "FULL_TIME")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ContractType")
            .value("id", 1002_u64)
            .value("name", "Part Time")
            .value("code", "PART_TIME")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ContractType")
            .value("id", 1003_u64)
            .value("name", "Contractor")
            .value("code", "CONTRACTOR")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ReviewGrade")
            .value("id", 1001_u64)
            .value("name", "Excellent")
            .value("code", "EXCELLENT")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ReviewGrade")
            .value("id", 1002_u64)
            .value("name", "Good")
            .value("code", "GOOD")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ReviewGrade")
            .value("id", 1003_u64)
            .value("name", "Needs Improvement")
            .value("code", "NEEDS_IMPROVEMENT")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ApplicationStatus")
            .value("id", 1001_u64)
            .value("name", "Applied")
            .value("code", "APPLIED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ApplicationStatus")
            .value("id", 1002_u64)
            .value("name", "Interviewing")
            .value("code", "INTERVIEWING")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ApplicationStatus")
            .value("id", 1003_u64)
            .value("name", "Offered")
            .value("code", "OFFERED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
}

pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<LeaveType>()
        .checker(teaql_runtime::TypedEntityChecker::<LeaveType, _>::new(LeaveTypeChecker::default()))
        .entity::<EmployeeStatus>()
        .checker(teaql_runtime::TypedEntityChecker::<EmployeeStatus, _>::new(EmployeeStatusChecker::default()))
        .entity::<ContractType>()
        .checker(teaql_runtime::TypedEntityChecker::<ContractType, _>::new(ContractTypeChecker::default()))
        .entity::<ReviewGrade>()
        .checker(teaql_runtime::TypedEntityChecker::<ReviewGrade, _>::new(ReviewGradeChecker::default()))
        .entity::<ApplicationStatus>()
        .checker(teaql_runtime::TypedEntityChecker::<ApplicationStatus, _>::new(ApplicationStatusChecker::default()))
        .entity::<Platform>()
        .checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .entity::<Merchant>()
        .checker(teaql_runtime::TypedEntityChecker::<Merchant, _>::new(MerchantChecker::default()))
        .entity::<Department>()
        .checker(teaql_runtime::TypedEntityChecker::<Department, _>::new(DepartmentChecker::default()))
        .entity::<Position>()
        .checker(teaql_runtime::TypedEntityChecker::<Position, _>::new(PositionChecker::default()))
        .entity::<Employee>()
        .checker(teaql_runtime::TypedEntityChecker::<Employee, _>::new(EmployeeChecker::default()))
        .entity::<SalaryRecord>()
        .checker(teaql_runtime::TypedEntityChecker::<SalaryRecord, _>::new(SalaryRecordChecker::default()))
        .entity::<AttendanceLog>()
        .checker(teaql_runtime::TypedEntityChecker::<AttendanceLog, _>::new(AttendanceLogChecker::default()))
        .entity::<LeaveRequest>()
        .checker(teaql_runtime::TypedEntityChecker::<LeaveRequest, _>::new(LeaveRequestChecker::default()))
        .entity::<PerformanceReview>()
        .checker(teaql_runtime::TypedEntityChecker::<PerformanceReview, _>::new(PerformanceReviewChecker::default()))
        .entity::<TrainingRecord>()
        .checker(teaql_runtime::TypedEntityChecker::<TrainingRecord, _>::new(TrainingRecordChecker::default()))
        .entity::<BenefitPlan>()
        .checker(teaql_runtime::TypedEntityChecker::<BenefitPlan, _>::new(BenefitPlanChecker::default()))
        .entity::<ExpenseClaim>()
        .checker(teaql_runtime::TypedEntityChecker::<ExpenseClaim, _>::new(ExpenseClaimChecker::default()))
        .entity::<PayrollRun>()
        .checker(teaql_runtime::TypedEntityChecker::<PayrollRun, _>::new(PayrollRunChecker::default()))
        .entity::<TaxForm>()
        .checker(teaql_runtime::TypedEntityChecker::<TaxForm, _>::new(TaxFormChecker::default()))
        .entity::<Contract>()
        .checker(teaql_runtime::TypedEntityChecker::<Contract, _>::new(ContractChecker::default()))
        .entity::<Resignation>()
        .checker(teaql_runtime::TypedEntityChecker::<Resignation, _>::new(ResignationChecker::default()))
        .entity::<WarningLetter>()
        .checker(teaql_runtime::TypedEntityChecker::<WarningLetter, _>::new(WarningLetterChecker::default()))
        .entity::<BonusRecord>()
        .checker(teaql_runtime::TypedEntityChecker::<BonusRecord, _>::new(BonusRecordChecker::default()))
        .entity::<ShiftSchedule>()
        .checker(teaql_runtime::TypedEntityChecker::<ShiftSchedule, _>::new(ShiftScheduleChecker::default()))
        .entity::<TimeOffBalance>()
        .checker(teaql_runtime::TypedEntityChecker::<TimeOffBalance, _>::new(TimeOffBalanceChecker::default()))
        .entity::<RecruitmentPost>()
        .checker(teaql_runtime::TypedEntityChecker::<RecruitmentPost, _>::new(RecruitmentPostChecker::default()))
        .entity::<JobApplication>()
        .checker(teaql_runtime::TypedEntityChecker::<JobApplication, _>::new(JobApplicationChecker::default()))
        .entity::<Interview>()
        .checker(teaql_runtime::TypedEntityChecker::<Interview, _>::new(InterviewChecker::default()))
        .entity::<OfferLetter>()
        .checker(teaql_runtime::TypedEntityChecker::<OfferLetter, _>::new(OfferLetterChecker::default()))
        .entity::<OnboardingChecklist>()
        .checker(teaql_runtime::TypedEntityChecker::<OnboardingChecklist, _>::new(OnboardingChecklistChecker::default()))
        .entity::<OffboardingChecklist>()
        .checker(teaql_runtime::TypedEntityChecker::<OffboardingChecklist, _>::new(OffboardingChecklistChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Moving Company Platform")
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("LeaveType")
            .value("id", 1001_u64)
            .value("name", "Annual")
            .value("code", "ANNUAL")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("LeaveType")
            .value("id", 1002_u64)
            .value("name", "Sick")
            .value("code", "SICK")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("LeaveType")
            .value("id", 1003_u64)
            .value("name", "Unpaid")
            .value("code", "UNPAID")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("EmployeeStatus")
            .value("id", 1001_u64)
            .value("name", "Active")
            .value("code", "ACTIVE")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("EmployeeStatus")
            .value("id", 1002_u64)
            .value("name", "Probation")
            .value("code", "PROBATION")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("EmployeeStatus")
            .value("id", 1003_u64)
            .value("name", "Terminated")
            .value("code", "TERMINATED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ContractType")
            .value("id", 1001_u64)
            .value("name", "Full Time")
            .value("code", "FULL_TIME")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ContractType")
            .value("id", 1002_u64)
            .value("name", "Part Time")
            .value("code", "PART_TIME")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ContractType")
            .value("id", 1003_u64)
            .value("name", "Contractor")
            .value("code", "CONTRACTOR")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ReviewGrade")
            .value("id", 1001_u64)
            .value("name", "Excellent")
            .value("code", "EXCELLENT")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ReviewGrade")
            .value("id", 1002_u64)
            .value("name", "Good")
            .value("code", "GOOD")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ReviewGrade")
            .value("id", 1003_u64)
            .value("name", "Needs Improvement")
            .value("code", "NEEDS_IMPROVEMENT")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ApplicationStatus")
            .value("id", 1001_u64)
            .value("name", "Applied")
            .value("code", "APPLIED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ApplicationStatus")
            .value("id", 1002_u64)
            .value("name", "Interviewing")
            .value("code", "INTERVIEWING")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ApplicationStatus")
            .value("id", 1003_u64)
            .value("name", "Offered")
            .value("code", "OFFERED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<LeaveType, _>(LeaveTypeBehavior::default())
        .entity_with_behavior::<EmployeeStatus, _>(EmployeeStatusBehavior::default())
        .entity_with_behavior::<ContractType, _>(ContractTypeBehavior::default())
        .entity_with_behavior::<ReviewGrade, _>(ReviewGradeBehavior::default())
        .entity_with_behavior::<ApplicationStatus, _>(ApplicationStatusBehavior::default())
        .entity_with_behavior::<Platform, _>(PlatformBehavior::default())
        .entity_with_behavior::<Merchant, _>(MerchantBehavior::default())
        .entity_with_behavior::<Department, _>(DepartmentBehavior::default())
        .entity_with_behavior::<Position, _>(PositionBehavior::default())
        .entity_with_behavior::<Employee, _>(EmployeeBehavior::default())
        .entity_with_behavior::<SalaryRecord, _>(SalaryRecordBehavior::default())
        .entity_with_behavior::<AttendanceLog, _>(AttendanceLogBehavior::default())
        .entity_with_behavior::<LeaveRequest, _>(LeaveRequestBehavior::default())
        .entity_with_behavior::<PerformanceReview, _>(PerformanceReviewBehavior::default())
        .entity_with_behavior::<TrainingRecord, _>(TrainingRecordBehavior::default())
        .entity_with_behavior::<BenefitPlan, _>(BenefitPlanBehavior::default())
        .entity_with_behavior::<ExpenseClaim, _>(ExpenseClaimBehavior::default())
        .entity_with_behavior::<PayrollRun, _>(PayrollRunBehavior::default())
        .entity_with_behavior::<TaxForm, _>(TaxFormBehavior::default())
        .entity_with_behavior::<Contract, _>(ContractBehavior::default())
        .entity_with_behavior::<Resignation, _>(ResignationBehavior::default())
        .entity_with_behavior::<WarningLetter, _>(WarningLetterBehavior::default())
        .entity_with_behavior::<BonusRecord, _>(BonusRecordBehavior::default())
        .entity_with_behavior::<ShiftSchedule, _>(ShiftScheduleBehavior::default())
        .entity_with_behavior::<TimeOffBalance, _>(TimeOffBalanceBehavior::default())
        .entity_with_behavior::<RecruitmentPost, _>(RecruitmentPostBehavior::default())
        .entity_with_behavior::<JobApplication, _>(JobApplicationBehavior::default())
        .entity_with_behavior::<Interview, _>(InterviewBehavior::default())
        .entity_with_behavior::<OfferLetter, _>(OfferLetterBehavior::default())
        .entity_with_behavior::<OnboardingChecklist, _>(OnboardingChecklistBehavior::default())
        .entity_with_behavior::<OffboardingChecklist, _>(OffboardingChecklistBehavior::default())
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Moving Company Platform")
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("LeaveType")
            .value("id", 1001_u64)
            .value("name", "Annual")
            .value("code", "ANNUAL")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("LeaveType")
            .value("id", 1002_u64)
            .value("name", "Sick")
            .value("code", "SICK")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("LeaveType")
            .value("id", 1003_u64)
            .value("name", "Unpaid")
            .value("code", "UNPAID")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("EmployeeStatus")
            .value("id", 1001_u64)
            .value("name", "Active")
            .value("code", "ACTIVE")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("EmployeeStatus")
            .value("id", 1002_u64)
            .value("name", "Probation")
            .value("code", "PROBATION")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("EmployeeStatus")
            .value("id", 1003_u64)
            .value("name", "Terminated")
            .value("code", "TERMINATED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ContractType")
            .value("id", 1001_u64)
            .value("name", "Full Time")
            .value("code", "FULL_TIME")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ContractType")
            .value("id", 1002_u64)
            .value("name", "Part Time")
            .value("code", "PART_TIME")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ContractType")
            .value("id", 1003_u64)
            .value("name", "Contractor")
            .value("code", "CONTRACTOR")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ReviewGrade")
            .value("id", 1001_u64)
            .value("name", "Excellent")
            .value("code", "EXCELLENT")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ReviewGrade")
            .value("id", 1002_u64)
            .value("name", "Good")
            .value("code", "GOOD")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ReviewGrade")
            .value("id", 1003_u64)
            .value("name", "Needs Improvement")
            .value("code", "NEEDS_IMPROVEMENT")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ApplicationStatus")
            .value("id", 1001_u64)
            .value("name", "Applied")
            .value("code", "APPLIED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ApplicationStatus")
            .value("id", 1002_u64)
            .value("name", "Interviewing")
            .value("code", "INTERVIEWING")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ApplicationStatus")
            .value("id", 1003_u64)
            .value("name", "Offered")
            .value("code", "OFFERED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
}

pub fn module_with_behaviors_and_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<LeaveType, _>(LeaveTypeBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<LeaveType, _>::new(LeaveTypeChecker::default()))
        .entity_with_behavior::<EmployeeStatus, _>(EmployeeStatusBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<EmployeeStatus, _>::new(EmployeeStatusChecker::default()))
        .entity_with_behavior::<ContractType, _>(ContractTypeBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<ContractType, _>::new(ContractTypeChecker::default()))
        .entity_with_behavior::<ReviewGrade, _>(ReviewGradeBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<ReviewGrade, _>::new(ReviewGradeChecker::default()))
        .entity_with_behavior::<ApplicationStatus, _>(ApplicationStatusBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<ApplicationStatus, _>::new(ApplicationStatusChecker::default()))
        .entity_with_behavior::<Platform, _>(PlatformBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .entity_with_behavior::<Merchant, _>(MerchantBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Merchant, _>::new(MerchantChecker::default()))
        .entity_with_behavior::<Department, _>(DepartmentBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Department, _>::new(DepartmentChecker::default()))
        .entity_with_behavior::<Position, _>(PositionBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Position, _>::new(PositionChecker::default()))
        .entity_with_behavior::<Employee, _>(EmployeeBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Employee, _>::new(EmployeeChecker::default()))
        .entity_with_behavior::<SalaryRecord, _>(SalaryRecordBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<SalaryRecord, _>::new(SalaryRecordChecker::default()))
        .entity_with_behavior::<AttendanceLog, _>(AttendanceLogBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<AttendanceLog, _>::new(AttendanceLogChecker::default()))
        .entity_with_behavior::<LeaveRequest, _>(LeaveRequestBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<LeaveRequest, _>::new(LeaveRequestChecker::default()))
        .entity_with_behavior::<PerformanceReview, _>(PerformanceReviewBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<PerformanceReview, _>::new(PerformanceReviewChecker::default()))
        .entity_with_behavior::<TrainingRecord, _>(TrainingRecordBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<TrainingRecord, _>::new(TrainingRecordChecker::default()))
        .entity_with_behavior::<BenefitPlan, _>(BenefitPlanBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<BenefitPlan, _>::new(BenefitPlanChecker::default()))
        .entity_with_behavior::<ExpenseClaim, _>(ExpenseClaimBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<ExpenseClaim, _>::new(ExpenseClaimChecker::default()))
        .entity_with_behavior::<PayrollRun, _>(PayrollRunBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<PayrollRun, _>::new(PayrollRunChecker::default()))
        .entity_with_behavior::<TaxForm, _>(TaxFormBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<TaxForm, _>::new(TaxFormChecker::default()))
        .entity_with_behavior::<Contract, _>(ContractBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Contract, _>::new(ContractChecker::default()))
        .entity_with_behavior::<Resignation, _>(ResignationBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Resignation, _>::new(ResignationChecker::default()))
        .entity_with_behavior::<WarningLetter, _>(WarningLetterBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<WarningLetter, _>::new(WarningLetterChecker::default()))
        .entity_with_behavior::<BonusRecord, _>(BonusRecordBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<BonusRecord, _>::new(BonusRecordChecker::default()))
        .entity_with_behavior::<ShiftSchedule, _>(ShiftScheduleBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<ShiftSchedule, _>::new(ShiftScheduleChecker::default()))
        .entity_with_behavior::<TimeOffBalance, _>(TimeOffBalanceBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<TimeOffBalance, _>::new(TimeOffBalanceChecker::default()))
        .entity_with_behavior::<RecruitmentPost, _>(RecruitmentPostBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<RecruitmentPost, _>::new(RecruitmentPostChecker::default()))
        .entity_with_behavior::<JobApplication, _>(JobApplicationBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<JobApplication, _>::new(JobApplicationChecker::default()))
        .entity_with_behavior::<Interview, _>(InterviewBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Interview, _>::new(InterviewChecker::default()))
        .entity_with_behavior::<OfferLetter, _>(OfferLetterBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<OfferLetter, _>::new(OfferLetterChecker::default()))
        .entity_with_behavior::<OnboardingChecklist, _>(OnboardingChecklistBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<OnboardingChecklist, _>::new(OnboardingChecklistChecker::default()))
        .entity_with_behavior::<OffboardingChecklist, _>(OffboardingChecklistBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<OffboardingChecklist, _>::new(OffboardingChecklistChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Moving Company Platform")
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("LeaveType")
            .value("id", 1001_u64)
            .value("name", "Annual")
            .value("code", "ANNUAL")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("LeaveType")
            .value("id", 1002_u64)
            .value("name", "Sick")
            .value("code", "SICK")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("LeaveType")
            .value("id", 1003_u64)
            .value("name", "Unpaid")
            .value("code", "UNPAID")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("EmployeeStatus")
            .value("id", 1001_u64)
            .value("name", "Active")
            .value("code", "ACTIVE")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("EmployeeStatus")
            .value("id", 1002_u64)
            .value("name", "Probation")
            .value("code", "PROBATION")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("EmployeeStatus")
            .value("id", 1003_u64)
            .value("name", "Terminated")
            .value("code", "TERMINATED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ContractType")
            .value("id", 1001_u64)
            .value("name", "Full Time")
            .value("code", "FULL_TIME")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ContractType")
            .value("id", 1002_u64)
            .value("name", "Part Time")
            .value("code", "PART_TIME")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ContractType")
            .value("id", 1003_u64)
            .value("name", "Contractor")
            .value("code", "CONTRACTOR")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ReviewGrade")
            .value("id", 1001_u64)
            .value("name", "Excellent")
            .value("code", "EXCELLENT")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ReviewGrade")
            .value("id", 1002_u64)
            .value("name", "Good")
            .value("code", "GOOD")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ReviewGrade")
            .value("id", 1003_u64)
            .value("name", "Needs Improvement")
            .value("code", "NEEDS_IMPROVEMENT")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ApplicationStatus")
            .value("id", 1001_u64)
            .value("name", "Applied")
            .value("code", "APPLIED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ApplicationStatus")
            .value("id", 1002_u64)
            .value("name", "Interviewing")
            .value("code", "INTERVIEWING")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("ApplicationStatus")
            .value("id", 1003_u64)
            .value("name", "Offered")
            .value("code", "OFFERED")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
}