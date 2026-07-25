#![allow(unused_imports)]
#![allow(async_fn_in_trait)]
use std::{collections::BTreeMap, future::Future, marker::PhantomData};

use serde_json::Value as JsonValue;
use teaql_core::{
    BinaryOp, Expr, Record,
    RelationAggregate as RuntimeRelationAggregate, SelectQuery, SmartList,
};
use teaql_runtime::{ContextError, GraphNode, EntityDataServiceBehavior, DataServiceError, RuntimeError, UserContext};

// Re-export query builder types from teaql_core::request
pub use teaql_core::request::{
    COUNT_ALIAS, TYPE_FIELD, TYPE_GROUP_FIELD,
    FieldOperator, DateRange, EntityReference,
    QuerySelection, RelationSelection, RelationFilter, QueryOptions,
    UnsafeRawSqlSegment, RawDynamicProperty, RawProjection,
    RelationAggregate, FacetRequest, ObjectGroupBy,
    apply_relation_selections, apply_runtime_metadata,
    field_operator_expr, field_operator_column_expr,
    required_value, required_text,
    remove_default_live_filter, remove_filter_expr,
    dynamic_json_value_to_teaql_value, dynamic_json_values,
    dynamic_json_operator, dynamic_json_filter_expr,
    dynamic_json_u64_field,
    runtime_relation_aggregates,
    merge_outer_filter_into_facet_aggregates, attach_facets,
};


pub trait TeaqlRecordRepository {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn fetch_all(&self, query: &SelectQuery) -> Result<Vec<Record>, DataServiceError<Self::Error>>;

    async fn fetch_smart_list(&self, query: &SelectQuery) -> Result<SmartList<Record>, DataServiceError<Self::Error>>;

    async fn fetch_smart_list_with_relation_aggregates(
        &self,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
    ) -> Result<SmartList<Record>, DataServiceError<Self::Error>>;

    async fn fetch_stream(&self, query: &SelectQuery) -> Result<Vec<teaql_data_service::StreamChunk>, DataServiceError<Self::Error>>;
}

pub trait TeaqlEntityRepository: TeaqlRecordRepository {
    async fn fetch_enhanced_entities<T>(&self, query: &SelectQuery) -> Result<SmartList<T>, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity;

    async fn fetch_enhanced_entities_with_relation_aggregates<T>(
        &self,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
    ) -> Result<SmartList<T>, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity;

    async fn save_entity_graph<T>(&self, entity: T) -> Result<GraphNode, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity;

    async fn save_entity_ledger(&self, root: teaql_runtime::EntityRoot) -> Result<(), DataServiceError<Self::Error>>;
}

impl<'a, E> TeaqlRecordRepository for teaql_runtime::EntityDataService<'a, E>
where
    E: teaql_data_service::QueryExecutor + teaql_data_service::MutationExecutor + teaql_data_service::StreamQueryExecutor + Send + Sync + 'static,
{
    type Error = E::Error;

    async fn fetch_all(&self, query: &SelectQuery) -> Result<Vec<Record>, DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::fetch_all(self, query).await
    }

    async fn fetch_smart_list(&self, query: &SelectQuery) -> Result<SmartList<Record>, DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::fetch_smart_list(self, query).await
    }

    async fn fetch_smart_list_with_relation_aggregates(
        &self,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
    ) -> Result<SmartList<Record>, DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::fetch_smart_list_with_relation_aggregates(
            self,
            query,
            relation_aggregates,
        ).await
    }

    async fn fetch_stream(&self, query: &SelectQuery) -> Result<Vec<teaql_data_service::StreamChunk>, DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::fetch_stream(self, query).await
    }
}

impl<'a, E> TeaqlEntityRepository for teaql_runtime::EntityDataService<'a, E>
where
    E: teaql_data_service::QueryExecutor + teaql_data_service::MutationExecutor + teaql_data_service::StreamQueryExecutor + Send + Sync + 'static,
{
    async fn fetch_enhanced_entities<T>(&self, query: &SelectQuery) -> Result<SmartList<T>, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity,
    {
        teaql_runtime::EntityDataService::fetch_enhanced_entities(self, query).await
    }

    async fn fetch_enhanced_entities_with_relation_aggregates<T>(
        &self,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
    ) -> Result<SmartList<T>, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity,
    {
        teaql_runtime::EntityDataService::fetch_enhanced_entities_with_relation_aggregates(
            self,
            query,
            relation_aggregates,
        ).await
    }

    async fn save_entity_graph<T>(&self, entity: T) -> Result<GraphNode, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity,
    {
        teaql_runtime::EntityDataService::save_entity_graph(self, entity).await
    }

    async fn save_entity_ledger(&self, root: teaql_runtime::EntityRoot) -> Result<(), DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::execute_ledger_plan(self, root).await
    }
}

pub type TeaqlDataServiceError<R> = DataServiceError<<R as TeaqlRecordRepository>::Error>;

pub trait TeaqlRuntime {
    fn user_context(&self) -> &UserContext;

    fn fetch_facet_smart_list(
        &self,
        entity: &str,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
        trace_context: Vec<teaql_core::TraceNode>,
    ) -> impl std::future::Future<Output = Result<SmartList<Record>, RuntimeError>> + Send;
}

/// Internal trait for repository access. Application code should not use this trait directly.
#[doc(hidden)]
pub trait AuditedSave<'a, C>
where
    C: TeaqlRepositoryProvider + ?Sized + 'a,
{
    type Error;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>>;
}



pub trait TeaqlRepositoryProvider: TeaqlRuntime {
    type CustomerProfileRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_profile_repository(&self) -> Result<Self::CustomerProfileRepository<'_>, ContextError>;
    type AddressBookRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn address_book_repository(&self) -> Result<Self::AddressBookRepository<'_>, ContextError>;
    type ContactPersonRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn contact_person_repository(&self) -> Result<Self::ContactPersonRepository<'_>, ContextError>;
    type AccountSettingsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn account_settings_repository(&self) -> Result<Self::AccountSettingsRepository<'_>, ContextError>;
    type LoyaltyProgramRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn loyalty_program_repository(&self) -> Result<Self::LoyaltyProgramRepository<'_>, ContextError>;
    type ServiceHistoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_history_repository(&self) -> Result<Self::ServiceHistoryRepository<'_>, ContextError>;
    type FeedbackReviewRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn feedback_review_repository(&self) -> Result<Self::FeedbackReviewRepository<'_>, ContextError>;
    type DisputeCaseRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn dispute_case_repository(&self) -> Result<Self::DisputeCaseRepository<'_>, ContextError>;
    type DocumentUploadRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn document_upload_repository(&self) -> Result<Self::DocumentUploadRepository<'_>, ContextError>;
    type PreferenceCenterRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn preference_center_repository(&self) -> Result<Self::PreferenceCenterRepository<'_>, ContextError>;
    type NotificationPrefRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn notification_pref_repository(&self) -> Result<Self::NotificationPrefRepository<'_>, ContextError>;
    type BillingContactRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn billing_contact_repository(&self) -> Result<Self::BillingContactRepository<'_>, ContextError>;
    type VehicleRegistryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_registry_repository(&self) -> Result<Self::VehicleRegistryRepository<'_>, ContextError>;
    type DriverProfileRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn driver_profile_repository(&self) -> Result<Self::DriverProfileRepository<'_>, ContextError>;
    type MaintenanceLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn maintenance_log_repository(&self) -> Result<Self::MaintenanceLogRepository<'_>, ContextError>;
    type FuelRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fuel_record_repository(&self) -> Result<Self::FuelRecordRepository<'_>, ContextError>;
    type InspectionChecklistRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn inspection_checklist_repository(&self) -> Result<Self::InspectionChecklistRepository<'_>, ContextError>;
    type RoutePlanRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn route_plan_repository(&self) -> Result<Self::RoutePlanRepository<'_>, ContextError>;
    type LoadManifestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn load_manifest_repository(&self) -> Result<Self::LoadManifestRepository<'_>, ContextError>;
    type EquipmentInventoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn equipment_inventory_repository(&self) -> Result<Self::EquipmentInventoryRepository<'_>, ContextError>;
    type GarageAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn garage_assignment_repository(&self) -> Result<Self::GarageAssignmentRepository<'_>, ContextError>;
    type IncidentReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn incident_report_repository(&self) -> Result<Self::IncidentReportRepository<'_>, ContextError>;
    type ComplianceCertificateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn compliance_certificate_repository(&self) -> Result<Self::ComplianceCertificateRepository<'_>, ContextError>;
    type TelematicsDataRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn telematics_data_repository(&self) -> Result<Self::TelematicsDataRepository<'_>, ContextError>;
    type InvoiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn invoice_repository(&self) -> Result<Self::InvoiceRepository<'_>, ContextError>;
    type PaymentTransactionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payment_transaction_repository(&self) -> Result<Self::PaymentTransactionRepository<'_>, ContextError>;
    type TaxCalculationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tax_calculation_repository(&self) -> Result<Self::TaxCalculationRepository<'_>, ContextError>;
    type CreditMemoRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn credit_memo_repository(&self) -> Result<Self::CreditMemoRepository<'_>, ContextError>;
    type DepositReceiptRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn deposit_receipt_repository(&self) -> Result<Self::DepositReceiptRepository<'_>, ContextError>;
    type RefundRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn refund_request_repository(&self) -> Result<Self::RefundRequestRepository<'_>, ContextError>;
    type ExpenseReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn expense_report_repository(&self) -> Result<Self::ExpenseReportRepository<'_>, ContextError>;
    type BudgetAllocationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn budget_allocation_repository(&self) -> Result<Self::BudgetAllocationRepository<'_>, ContextError>;
    type FinancialStatementRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn financial_statement_repository(&self) -> Result<Self::FinancialStatementRepository<'_>, ContextError>;
    type AuditTrailRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn audit_trail_repository(&self) -> Result<Self::AuditTrailRepository<'_>, ContextError>;
    type CurrencyExchangeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn currency_exchange_repository(&self) -> Result<Self::CurrencyExchangeRepository<'_>, ContextError>;
    type ReceivableAgingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn receivable_aging_repository(&self) -> Result<Self::ReceivableAgingRepository<'_>, ContextError>;
    type EmployeeRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn employee_record_repository(&self) -> Result<Self::EmployeeRecordRepository<'_>, ContextError>;
    type PayrollRunRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payroll_run_repository(&self) -> Result<Self::PayrollRunRepository<'_>, ContextError>;
    type TimesheetEntryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn timesheet_entry_repository(&self) -> Result<Self::TimesheetEntryRepository<'_>, ContextError>;
    type BenefitPlanRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn benefit_plan_repository(&self) -> Result<Self::BenefitPlanRepository<'_>, ContextError>;
    type TaxWithholdingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tax_withholding_repository(&self) -> Result<Self::TaxWithholdingRepository<'_>, ContextError>;
    type LeaveRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn leave_request_repository(&self) -> Result<Self::LeaveRequestRepository<'_>, ContextError>;
    type TrainingRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn training_record_repository(&self) -> Result<Self::TrainingRecordRepository<'_>, ContextError>;
    type PerformanceReviewRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn performance_review_repository(&self) -> Result<Self::PerformanceReviewRepository<'_>, ContextError>;
    type CompensationAdjustmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn compensation_adjustment_repository(&self) -> Result<Self::CompensationAdjustmentRepository<'_>, ContextError>;
    type OnboardingChecklistRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn onboarding_checklist_repository(&self) -> Result<Self::OnboardingChecklistRepository<'_>, ContextError>;
    type OffboardingProcessRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn offboarding_process_repository(&self) -> Result<Self::OffboardingProcessRepository<'_>, ContextError>;
    type EmployeeHandbookRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn employee_handbook_repository(&self) -> Result<Self::EmployeeHandbookRepository<'_>, ContextError>;
    type MoveOrderRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_order_repository(&self) -> Result<Self::MoveOrderRepository<'_>, ContextError>;
    type JobScheduleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn job_schedule_repository(&self) -> Result<Self::JobScheduleRepository<'_>, ContextError>;
    type CrewAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn crew_assignment_repository(&self) -> Result<Self::CrewAssignmentRepository<'_>, ContextError>;
    type EquipmentAllocationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn equipment_allocation_repository(&self) -> Result<Self::EquipmentAllocationRepository<'_>, ContextError>;
    type TimeSlotRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn time_slot_repository(&self) -> Result<Self::TimeSlotRepository<'_>, ContextError>;
    type ServiceLocationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_location_repository(&self) -> Result<Self::ServiceLocationRepository<'_>, ContextError>;
    type SpecialInstructionsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn special_instructions_repository(&self) -> Result<Self::SpecialInstructionsRepository<'_>, ContextError>;
    type StatusUpdateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn status_update_repository(&self) -> Result<Self::StatusUpdateRepository<'_>, ContextError>;
    type CancellationPolicyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn cancellation_policy_repository(&self) -> Result<Self::CancellationPolicyRepository<'_>, ContextError>;
    type RescheduleRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn reschedule_request_repository(&self) -> Result<Self::RescheduleRequestRepository<'_>, ContextError>;
    type SatisfactionSurveyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn satisfaction_survey_repository(&self) -> Result<Self::SatisfactionSurveyRepository<'_>, ContextError>;
    type FollowUpTaskRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn follow_up_task_repository(&self) -> Result<Self::FollowUpTaskRepository<'_>, ContextError>;
}

#[allow(async_fn_in_trait)]
pub trait TeaqlUserContextExt {
    async fn commit_data(&self) -> Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>;

    async fn transaction_data<F, Fut>(&self, f: F) -> Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>>;
}

impl TeaqlUserContextExt for teaql_runtime::UserContext {
    async fn commit_data(&self) -> Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>> {
        self.commit_changes::<crate::runtime::DataServiceExecutor>().await
    }

    async fn transaction_data<F, Fut>(&self, f: F) -> Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>>,
    {
        let executor = self.require_resource::<crate::runtime::DataServiceExecutor>().map_err(|err| {
            DataServiceError::Runtime(RuntimeError::Graph(format!(
                "cannot start transaction without executor: {err}"
            )))
        })?;
        let root = self.entity_root();

        let tx = teaql_data_service::TransactionExecutor::begin(&*executor).await.map_err(DataServiceError::Executor)?;
        root.push_change_set();

        let result = f().await;
        match result {
            Ok(()) => {
                root.pop_change_set();
                teaql_data_service::Transaction::commit(tx).await.map_err(DataServiceError::Executor)?;
                Ok(())
            }
            Err(err) => {
                root.pop_change_set();
                teaql_data_service::Transaction::rollback(tx).await.map_err(DataServiceError::Executor)?;
                Err(err)
            }
        }
    }
}

impl TeaqlRuntime for teaql_runtime::UserContext {
    fn user_context(&self) -> &UserContext {
        self
    }

    async fn fetch_facet_smart_list(
        &self,
        entity: &str,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
        trace_context: Vec<teaql_core::TraceNode>,
    ) -> Result<SmartList<Record>, RuntimeError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>(entity)
            .map_err(|err| RuntimeError::Graph(err.to_string()))?
            .with_trace_context(trace_context)
            .fetch_smart_list_with_relation_aggregates(query, relation_aggregates)
            .await
            .map_err(|err| RuntimeError::Graph(err.to_string()))
    }
}

impl TeaqlRepositoryProvider for teaql_runtime::UserContext {
    type CustomerProfileRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_profile_repository(&self) -> Result<Self::CustomerProfileRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerProfile")
    }

    type AddressBookRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn address_book_repository(&self) -> Result<Self::AddressBookRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AddressBook")
    }

    type ContactPersonRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn contact_person_repository(&self) -> Result<Self::ContactPersonRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ContactPerson")
    }

    type AccountSettingsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn account_settings_repository(&self) -> Result<Self::AccountSettingsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AccountSettings")
    }

    type LoyaltyProgramRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn loyalty_program_repository(&self) -> Result<Self::LoyaltyProgramRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoyaltyProgram")
    }

    type ServiceHistoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_history_repository(&self) -> Result<Self::ServiceHistoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceHistory")
    }

    type FeedbackReviewRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn feedback_review_repository(&self) -> Result<Self::FeedbackReviewRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FeedbackReview")
    }

    type DisputeCaseRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn dispute_case_repository(&self) -> Result<Self::DisputeCaseRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DisputeCase")
    }

    type DocumentUploadRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn document_upload_repository(&self) -> Result<Self::DocumentUploadRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DocumentUpload")
    }

    type PreferenceCenterRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn preference_center_repository(&self) -> Result<Self::PreferenceCenterRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PreferenceCenter")
    }

    type NotificationPrefRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn notification_pref_repository(&self) -> Result<Self::NotificationPrefRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("NotificationPref")
    }

    type BillingContactRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn billing_contact_repository(&self) -> Result<Self::BillingContactRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BillingContact")
    }

    type VehicleRegistryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_registry_repository(&self) -> Result<Self::VehicleRegistryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleRegistry")
    }

    type DriverProfileRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn driver_profile_repository(&self) -> Result<Self::DriverProfileRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DriverProfile")
    }

    type MaintenanceLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn maintenance_log_repository(&self) -> Result<Self::MaintenanceLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MaintenanceLog")
    }

    type FuelRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fuel_record_repository(&self) -> Result<Self::FuelRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FuelRecord")
    }

    type InspectionChecklistRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn inspection_checklist_repository(&self) -> Result<Self::InspectionChecklistRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InspectionChecklist")
    }

    type RoutePlanRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn route_plan_repository(&self) -> Result<Self::RoutePlanRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RoutePlan")
    }

    type LoadManifestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn load_manifest_repository(&self) -> Result<Self::LoadManifestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoadManifest")
    }

    type EquipmentInventoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn equipment_inventory_repository(&self) -> Result<Self::EquipmentInventoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EquipmentInventory")
    }

    type GarageAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn garage_assignment_repository(&self) -> Result<Self::GarageAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("GarageAssignment")
    }

    type IncidentReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn incident_report_repository(&self) -> Result<Self::IncidentReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("IncidentReport")
    }

    type ComplianceCertificateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn compliance_certificate_repository(&self) -> Result<Self::ComplianceCertificateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ComplianceCertificate")
    }

    type TelematicsDataRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn telematics_data_repository(&self) -> Result<Self::TelematicsDataRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TelematicsData")
    }

    type InvoiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn invoice_repository(&self) -> Result<Self::InvoiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Invoice")
    }

    type PaymentTransactionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payment_transaction_repository(&self) -> Result<Self::PaymentTransactionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PaymentTransaction")
    }

    type TaxCalculationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tax_calculation_repository(&self) -> Result<Self::TaxCalculationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TaxCalculation")
    }

    type CreditMemoRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn credit_memo_repository(&self) -> Result<Self::CreditMemoRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CreditMemo")
    }

    type DepositReceiptRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn deposit_receipt_repository(&self) -> Result<Self::DepositReceiptRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DepositReceipt")
    }

    type RefundRequestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn refund_request_repository(&self) -> Result<Self::RefundRequestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RefundRequest")
    }

    type ExpenseReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn expense_report_repository(&self) -> Result<Self::ExpenseReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExpenseReport")
    }

    type BudgetAllocationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn budget_allocation_repository(&self) -> Result<Self::BudgetAllocationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BudgetAllocation")
    }

    type FinancialStatementRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn financial_statement_repository(&self) -> Result<Self::FinancialStatementRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FinancialStatement")
    }

    type AuditTrailRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn audit_trail_repository(&self) -> Result<Self::AuditTrailRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AuditTrail")
    }

    type CurrencyExchangeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn currency_exchange_repository(&self) -> Result<Self::CurrencyExchangeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CurrencyExchange")
    }

    type ReceivableAgingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn receivable_aging_repository(&self) -> Result<Self::ReceivableAgingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ReceivableAging")
    }

    type EmployeeRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn employee_record_repository(&self) -> Result<Self::EmployeeRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EmployeeRecord")
    }

    type PayrollRunRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payroll_run_repository(&self) -> Result<Self::PayrollRunRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PayrollRun")
    }

    type TimesheetEntryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn timesheet_entry_repository(&self) -> Result<Self::TimesheetEntryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TimesheetEntry")
    }

    type BenefitPlanRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn benefit_plan_repository(&self) -> Result<Self::BenefitPlanRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BenefitPlan")
    }

    type TaxWithholdingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tax_withholding_repository(&self) -> Result<Self::TaxWithholdingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TaxWithholding")
    }

    type LeaveRequestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn leave_request_repository(&self) -> Result<Self::LeaveRequestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LeaveRequest")
    }

    type TrainingRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn training_record_repository(&self) -> Result<Self::TrainingRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TrainingRecord")
    }

    type PerformanceReviewRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn performance_review_repository(&self) -> Result<Self::PerformanceReviewRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PerformanceReview")
    }

    type CompensationAdjustmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn compensation_adjustment_repository(&self) -> Result<Self::CompensationAdjustmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CompensationAdjustment")
    }

    type OnboardingChecklistRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn onboarding_checklist_repository(&self) -> Result<Self::OnboardingChecklistRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OnboardingChecklist")
    }

    type OffboardingProcessRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn offboarding_process_repository(&self) -> Result<Self::OffboardingProcessRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OffboardingProcess")
    }

    type EmployeeHandbookRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn employee_handbook_repository(&self) -> Result<Self::EmployeeHandbookRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EmployeeHandbook")
    }

    type MoveOrderRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_order_repository(&self) -> Result<Self::MoveOrderRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveOrder")
    }

    type JobScheduleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn job_schedule_repository(&self) -> Result<Self::JobScheduleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("JobSchedule")
    }

    type CrewAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn crew_assignment_repository(&self) -> Result<Self::CrewAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CrewAssignment")
    }

    type EquipmentAllocationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn equipment_allocation_repository(&self) -> Result<Self::EquipmentAllocationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EquipmentAllocation")
    }

    type TimeSlotRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn time_slot_repository(&self) -> Result<Self::TimeSlotRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TimeSlot")
    }

    type ServiceLocationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_location_repository(&self) -> Result<Self::ServiceLocationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceLocation")
    }

    type SpecialInstructionsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn special_instructions_repository(&self) -> Result<Self::SpecialInstructionsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SpecialInstructions")
    }

    type StatusUpdateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn status_update_repository(&self) -> Result<Self::StatusUpdateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("StatusUpdate")
    }

    type CancellationPolicyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn cancellation_policy_repository(&self) -> Result<Self::CancellationPolicyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CancellationPolicy")
    }

    type RescheduleRequestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn reschedule_request_repository(&self) -> Result<Self::RescheduleRequestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RescheduleRequest")
    }

    type SatisfactionSurveyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn satisfaction_survey_repository(&self) -> Result<Self::SatisfactionSurveyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SatisfactionSurvey")
    }

    type FollowUpTaskRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn follow_up_task_repository(&self) -> Result<Self::FollowUpTaskRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FollowUpTask")
    }
}

pub(crate) async fn execute_facets<C>(
    ctx: &C,
    outer_query: &SelectQuery,
    options: &QueryOptions,
) -> Result<BTreeMap<String, SmartList<Record>>, RuntimeError>
where
    C: TeaqlRuntime + ?Sized,
{
    let mut facets = BTreeMap::new();
    for facet in &options.facets {
        let mut selection = facet.query.clone();
        merge_outer_filter_into_facet_aggregates(&mut selection, outer_query);
        if !facet.include_all_facets {
            selection = restrict_facet_to_outer_query(ctx, selection, outer_query, &facet.relation_name)?;
        }
        let relation_aggregates = runtime_relation_aggregates(&selection.query_options);
        let query = apply_runtime_metadata(
            selection.query,
            &selection.query_options,
            &selection.child_enhancements,
        );
        let mut chain = outer_query.trace_chain.clone();
        chain.push(teaql_core::TraceNode::new(
            query.entity.clone(),
            None,
            facet.facet_name.clone(),
        ));

        let facet_rows = ctx.fetch_facet_smart_list(&query.entity, &query, &relation_aggregates, chain).await?;
        facets.insert(facet.facet_name.clone(), facet_rows);
    }
    Ok(facets)
}

pub(crate) fn restrict_facet_to_outer_query<C>(
    ctx: &C,
    mut selection: QuerySelection,
    outer_query: &SelectQuery,
    relation_name: &str,
) -> Result<QuerySelection, RuntimeError>
where
    C: TeaqlRuntime + ?Sized,
{
    let descriptor = ctx
        .user_context()
        .entity(&outer_query.entity)
        .cloned()
        .ok_or_else(|| RuntimeError::Graph(format!("missing entity: {}", outer_query.entity)))?;
    let relation = descriptor
        .relation_by_name(relation_name)
        .cloned()
        .ok_or_else(|| RuntimeError::MissingRelation {
            entity: outer_query.entity.clone(),
            relation: relation_name.to_owned(),
        })?;
    let mut subquery = outer_query.clone();
    subquery.projection.clear();
    subquery.expr_projection.clear();
    subquery.order_by.clear();
    subquery.slice = None;
    subquery.aggregates.clear();
    subquery.group_by.clear();
    subquery.relations.clear();
    selection.query = selection.query.and_filter(Expr::in_subquery(
        relation.foreign_key,
        descriptor,
        subquery,
        relation.local_key,
    ));
    Ok(selection)
}
