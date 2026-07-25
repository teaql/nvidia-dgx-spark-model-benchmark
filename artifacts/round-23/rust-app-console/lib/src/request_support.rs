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
    type CustomerContactRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_contact_repository(&self) -> Result<Self::CustomerContactRepository<'_>, ContextError>;
    type CustomerAddressRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_address_repository(&self) -> Result<Self::CustomerAddressRepository<'_>, ContextError>;
    type CustomerPreferenceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_preference_repository(&self) -> Result<Self::CustomerPreferenceRepository<'_>, ContextError>;
    type LoyaltyProgramRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn loyalty_program_repository(&self) -> Result<Self::LoyaltyProgramRepository<'_>, ContextError>;
    type CustomerFeedbackRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_feedback_repository(&self) -> Result<Self::CustomerFeedbackRepository<'_>, ContextError>;
    type CustomerSegmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_segment_repository(&self) -> Result<Self::CustomerSegmentRepository<'_>, ContextError>;
    type CustomerAccountRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_account_repository(&self) -> Result<Self::CustomerAccountRepository<'_>, ContextError>;
    type PaymentMethodRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payment_method_repository(&self) -> Result<Self::PaymentMethodRepository<'_>, ContextError>;
    type InvoiceHistoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn invoice_history_repository(&self) -> Result<Self::InvoiceHistoryRepository<'_>, ContextError>;
    type DisputeRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn dispute_record_repository(&self) -> Result<Self::DisputeRecordRepository<'_>, ContextError>;
    type ServiceAgreementRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_agreement_repository(&self) -> Result<Self::ServiceAgreementRepository<'_>, ContextError>;
    type ContractTermsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn contract_terms_repository(&self) -> Result<Self::ContractTermsRepository<'_>, ContextError>;
    type RenewalNoticeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn renewal_notice_repository(&self) -> Result<Self::RenewalNoticeRepository<'_>, ContextError>;
    type CancellationRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn cancellation_request_repository(&self) -> Result<Self::CancellationRequestRepository<'_>, ContextError>;
    type ReferralCodeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn referral_code_repository(&self) -> Result<Self::ReferralCodeRepository<'_>, ContextError>;
    type MarketingCampaignRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn marketing_campaign_repository(&self) -> Result<Self::MarketingCampaignRepository<'_>, ContextError>;
    type LeadSourceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn lead_source_repository(&self) -> Result<Self::LeadSourceRepository<'_>, ContextError>;
    type VehicleRegistryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_registry_repository(&self) -> Result<Self::VehicleRegistryRepository<'_>, ContextError>;
    type VehicleSpecRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_spec_repository(&self) -> Result<Self::VehicleSpecRepository<'_>, ContextError>;
    type MaintenanceLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn maintenance_log_repository(&self) -> Result<Self::MaintenanceLogRepository<'_>, ContextError>;
    type FuelRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fuel_record_repository(&self) -> Result<Self::FuelRecordRepository<'_>, ContextError>;
    type TireInventoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tire_inventory_repository(&self) -> Result<Self::TireInventoryRepository<'_>, ContextError>;
    type DriverAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn driver_assignment_repository(&self) -> Result<Self::DriverAssignmentRepository<'_>, ContextError>;
    type DriverLicenseRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn driver_license_repository(&self) -> Result<Self::DriverLicenseRepository<'_>, ContextError>;
    type DriverTrainingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn driver_training_repository(&self) -> Result<Self::DriverTrainingRepository<'_>, ContextError>;
    type RoutePlanRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn route_plan_repository(&self) -> Result<Self::RoutePlanRepository<'_>, ContextError>;
    type LoadCapacityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn load_capacity_repository(&self) -> Result<Self::LoadCapacityRepository<'_>, ContextError>;
    type CargoSecurementRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn cargo_securement_repository(&self) -> Result<Self::CargoSecurementRepository<'_>, ContextError>;
    type GpsTrackingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn gps_tracking_repository(&self) -> Result<Self::GpsTrackingRepository<'_>, ContextError>;
    type TelematicsDataRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn telematics_data_repository(&self) -> Result<Self::TelematicsDataRepository<'_>, ContextError>;
    type IncidentReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn incident_report_repository(&self) -> Result<Self::IncidentReportRepository<'_>, ContextError>;
    type InspectionChecklistRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn inspection_checklist_repository(&self) -> Result<Self::InspectionChecklistRepository<'_>, ContextError>;
    type ServiceScheduleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_schedule_repository(&self) -> Result<Self::ServiceScheduleRepository<'_>, ContextError>;
    type WarrantyInfoRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn warranty_info_repository(&self) -> Result<Self::WarrantyInfoRepository<'_>, ContextError>;
    type DecommissionRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn decommission_record_repository(&self) -> Result<Self::DecommissionRecordRepository<'_>, ContextError>;
    type InvoiceTemplateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn invoice_template_repository(&self) -> Result<Self::InvoiceTemplateRepository<'_>, ContextError>;
    type BillingCycleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn billing_cycle_repository(&self) -> Result<Self::BillingCycleRepository<'_>, ContextError>;
    type TaxJurisdictionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tax_jurisdiction_repository(&self) -> Result<Self::TaxJurisdictionRepository<'_>, ContextError>;
    type TaxRateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tax_rate_repository(&self) -> Result<Self::TaxRateRepository<'_>, ContextError>;
    type DiscountPolicyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn discount_policy_repository(&self) -> Result<Self::DiscountPolicyRepository<'_>, ContextError>;
    type PaymentGatewayRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payment_gateway_repository(&self) -> Result<Self::PaymentGatewayRepository<'_>, ContextError>;
    type TransactionLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn transaction_log_repository(&self) -> Result<Self::TransactionLogRepository<'_>, ContextError>;
    type RefundProcessRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn refund_process_repository(&self) -> Result<Self::RefundProcessRepository<'_>, ContextError>;
    type CreditNoteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn credit_note_repository(&self) -> Result<Self::CreditNoteRepository<'_>, ContextError>;
    type DebitNoteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn debit_note_repository(&self) -> Result<Self::DebitNoteRepository<'_>, ContextError>;
    type ExpenseCategoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn expense_category_repository(&self) -> Result<Self::ExpenseCategoryRepository<'_>, ContextError>;
    type CostCenterRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn cost_center_repository(&self) -> Result<Self::CostCenterRepository<'_>, ContextError>;
    type BudgetAllocationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn budget_allocation_repository(&self) -> Result<Self::BudgetAllocationRepository<'_>, ContextError>;
    type FinancialReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn financial_report_repository(&self) -> Result<Self::FinancialReportRepository<'_>, ContextError>;
    type AuditTrailRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn audit_trail_repository(&self) -> Result<Self::AuditTrailRepository<'_>, ContextError>;
    type ReconciliationEntryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn reconciliation_entry_repository(&self) -> Result<Self::ReconciliationEntryRepository<'_>, ContextError>;
    type CurrencyConversionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn currency_conversion_repository(&self) -> Result<Self::CurrencyConversionRepository<'_>, ContextError>;
    type FiscalPeriodRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fiscal_period_repository(&self) -> Result<Self::FiscalPeriodRepository<'_>, ContextError>;
    type JobOrderRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn job_order_repository(&self) -> Result<Self::JobOrderRepository<'_>, ContextError>;
    type MoveScheduleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_schedule_repository(&self) -> Result<Self::MoveScheduleRepository<'_>, ContextError>;
    type CrewAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn crew_assignment_repository(&self) -> Result<Self::CrewAssignmentRepository<'_>, ContextError>;
    type EquipmentAllocationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn equipment_allocation_repository(&self) -> Result<Self::EquipmentAllocationRepository<'_>, ContextError>;
    type PickupLocationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn pickup_location_repository(&self) -> Result<Self::PickupLocationRepository<'_>, ContextError>;
    type DeliveryLocationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn delivery_location_repository(&self) -> Result<Self::DeliveryLocationRepository<'_>, ContextError>;
    type TransitTimeEstimateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn transit_time_estimate_repository(&self) -> Result<Self::TransitTimeEstimateRepository<'_>, ContextError>;
    type LoadingDockRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn loading_dock_repository(&self) -> Result<Self::LoadingDockRepository<'_>, ContextError>;
    type UnloadingDockRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn unloading_dock_repository(&self) -> Result<Self::UnloadingDockRepository<'_>, ContextError>;
    type CustomsDocumentationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customs_documentation_repository(&self) -> Result<Self::CustomsDocumentationRepository<'_>, ContextError>;
    type PermitRequiredRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn permit_required_repository(&self) -> Result<Self::PermitRequiredRepository<'_>, ContextError>;
    type InsuranceCoverageRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn insurance_coverage_repository(&self) -> Result<Self::InsuranceCoverageRepository<'_>, ContextError>;
    type LiabilityWaiverRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn liability_waiver_repository(&self) -> Result<Self::LiabilityWaiverRepository<'_>, ContextError>;
    type TrackingNumberRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tracking_number_repository(&self) -> Result<Self::TrackingNumberRepository<'_>, ContextError>;
    type StatusUpdateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn status_update_repository(&self) -> Result<Self::StatusUpdateRepository<'_>, ContextError>;
    type NotificationTemplateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn notification_template_repository(&self) -> Result<Self::NotificationTemplateRepository<'_>, ContextError>;
    type SlaMetricRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn sla_metric_repository(&self) -> Result<Self::SlaMetricRepository<'_>, ContextError>;
    type PerformanceKpiRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn performance_kpi_repository(&self) -> Result<Self::PerformanceKpiRepository<'_>, ContextError>;
    type EmployeeRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn employee_record_repository(&self) -> Result<Self::EmployeeRecordRepository<'_>, ContextError>;
    type PayrollInfoRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payroll_info_repository(&self) -> Result<Self::PayrollInfoRepository<'_>, ContextError>;
    type BenefitsPlanRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn benefits_plan_repository(&self) -> Result<Self::BenefitsPlanRepository<'_>, ContextError>;
    type TimeOffRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn time_off_request_repository(&self) -> Result<Self::TimeOffRequestRepository<'_>, ContextError>;
    type ShiftScheduleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn shift_schedule_repository(&self) -> Result<Self::ShiftScheduleRepository<'_>, ContextError>;
    type PerformanceReviewRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn performance_review_repository(&self) -> Result<Self::PerformanceReviewRepository<'_>, ContextError>;
    type CompetencyMatrixRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn competency_matrix_repository(&self) -> Result<Self::CompetencyMatrixRepository<'_>, ContextError>;
    type TrainingCourseRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn training_course_repository(&self) -> Result<Self::TrainingCourseRepository<'_>, ContextError>;
    type CertificationRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn certification_record_repository(&self) -> Result<Self::CertificationRecordRepository<'_>, ContextError>;
    type SafetyIncidentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn safety_incident_repository(&self) -> Result<Self::SafetyIncidentRepository<'_>, ContextError>;
    type HazardAssessmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn hazard_assessment_repository(&self) -> Result<Self::HazardAssessmentRepository<'_>, ContextError>;
    type PolicyAcknowledgmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn policy_acknowledgment_repository(&self) -> Result<Self::PolicyAcknowledgmentRepository<'_>, ContextError>;
    type GrievanceLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn grievance_log_repository(&self) -> Result<Self::GrievanceLogRepository<'_>, ContextError>;
    type DisciplinaryActionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn disciplinary_action_repository(&self) -> Result<Self::DisciplinaryActionRepository<'_>, ContextError>;
    type ExitInterviewRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn exit_interview_repository(&self) -> Result<Self::ExitInterviewRepository<'_>, ContextError>;
    type OnboardingChecklistRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn onboarding_checklist_repository(&self) -> Result<Self::OnboardingChecklistRepository<'_>, ContextError>;
    type OffboardingChecklistRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn offboarding_checklist_repository(&self) -> Result<Self::OffboardingChecklistRepository<'_>, ContextError>;
    type ComplianceAuditRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn compliance_audit_repository(&self) -> Result<Self::ComplianceAuditRepository<'_>, ContextError>;
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

    type CustomerContactRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_contact_repository(&self) -> Result<Self::CustomerContactRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerContact")
    }

    type CustomerAddressRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_address_repository(&self) -> Result<Self::CustomerAddressRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerAddress")
    }

    type CustomerPreferenceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_preference_repository(&self) -> Result<Self::CustomerPreferenceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerPreference")
    }

    type LoyaltyProgramRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn loyalty_program_repository(&self) -> Result<Self::LoyaltyProgramRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoyaltyProgram")
    }

    type CustomerFeedbackRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_feedback_repository(&self) -> Result<Self::CustomerFeedbackRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerFeedback")
    }

    type CustomerSegmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_segment_repository(&self) -> Result<Self::CustomerSegmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerSegment")
    }

    type CustomerAccountRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_account_repository(&self) -> Result<Self::CustomerAccountRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerAccount")
    }

    type PaymentMethodRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payment_method_repository(&self) -> Result<Self::PaymentMethodRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PaymentMethod")
    }

    type InvoiceHistoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn invoice_history_repository(&self) -> Result<Self::InvoiceHistoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InvoiceHistory")
    }

    type DisputeRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn dispute_record_repository(&self) -> Result<Self::DisputeRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DisputeRecord")
    }

    type ServiceAgreementRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_agreement_repository(&self) -> Result<Self::ServiceAgreementRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceAgreement")
    }

    type ContractTermsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn contract_terms_repository(&self) -> Result<Self::ContractTermsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ContractTerms")
    }

    type RenewalNoticeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn renewal_notice_repository(&self) -> Result<Self::RenewalNoticeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RenewalNotice")
    }

    type CancellationRequestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn cancellation_request_repository(&self) -> Result<Self::CancellationRequestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CancellationRequest")
    }

    type ReferralCodeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn referral_code_repository(&self) -> Result<Self::ReferralCodeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ReferralCode")
    }

    type MarketingCampaignRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn marketing_campaign_repository(&self) -> Result<Self::MarketingCampaignRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MarketingCampaign")
    }

    type LeadSourceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn lead_source_repository(&self) -> Result<Self::LeadSourceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LeadSource")
    }

    type VehicleRegistryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_registry_repository(&self) -> Result<Self::VehicleRegistryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleRegistry")
    }

    type VehicleSpecRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_spec_repository(&self) -> Result<Self::VehicleSpecRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleSpec")
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

    type TireInventoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tire_inventory_repository(&self) -> Result<Self::TireInventoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TireInventory")
    }

    type DriverAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn driver_assignment_repository(&self) -> Result<Self::DriverAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DriverAssignment")
    }

    type DriverLicenseRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn driver_license_repository(&self) -> Result<Self::DriverLicenseRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DriverLicense")
    }

    type DriverTrainingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn driver_training_repository(&self) -> Result<Self::DriverTrainingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DriverTraining")
    }

    type RoutePlanRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn route_plan_repository(&self) -> Result<Self::RoutePlanRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RoutePlan")
    }

    type LoadCapacityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn load_capacity_repository(&self) -> Result<Self::LoadCapacityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoadCapacity")
    }

    type CargoSecurementRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn cargo_securement_repository(&self) -> Result<Self::CargoSecurementRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CargoSecurement")
    }

    type GpsTrackingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn gps_tracking_repository(&self) -> Result<Self::GpsTrackingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("GpsTracking")
    }

    type TelematicsDataRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn telematics_data_repository(&self) -> Result<Self::TelematicsDataRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TelematicsData")
    }

    type IncidentReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn incident_report_repository(&self) -> Result<Self::IncidentReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("IncidentReport")
    }

    type InspectionChecklistRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn inspection_checklist_repository(&self) -> Result<Self::InspectionChecklistRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InspectionChecklist")
    }

    type ServiceScheduleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_schedule_repository(&self) -> Result<Self::ServiceScheduleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceSchedule")
    }

    type WarrantyInfoRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn warranty_info_repository(&self) -> Result<Self::WarrantyInfoRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("WarrantyInfo")
    }

    type DecommissionRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn decommission_record_repository(&self) -> Result<Self::DecommissionRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DecommissionRecord")
    }

    type InvoiceTemplateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn invoice_template_repository(&self) -> Result<Self::InvoiceTemplateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InvoiceTemplate")
    }

    type BillingCycleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn billing_cycle_repository(&self) -> Result<Self::BillingCycleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BillingCycle")
    }

    type TaxJurisdictionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tax_jurisdiction_repository(&self) -> Result<Self::TaxJurisdictionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TaxJurisdiction")
    }

    type TaxRateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tax_rate_repository(&self) -> Result<Self::TaxRateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TaxRate")
    }

    type DiscountPolicyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn discount_policy_repository(&self) -> Result<Self::DiscountPolicyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DiscountPolicy")
    }

    type PaymentGatewayRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payment_gateway_repository(&self) -> Result<Self::PaymentGatewayRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PaymentGateway")
    }

    type TransactionLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn transaction_log_repository(&self) -> Result<Self::TransactionLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TransactionLog")
    }

    type RefundProcessRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn refund_process_repository(&self) -> Result<Self::RefundProcessRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RefundProcess")
    }

    type CreditNoteRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn credit_note_repository(&self) -> Result<Self::CreditNoteRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CreditNote")
    }

    type DebitNoteRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn debit_note_repository(&self) -> Result<Self::DebitNoteRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DebitNote")
    }

    type ExpenseCategoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn expense_category_repository(&self) -> Result<Self::ExpenseCategoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExpenseCategory")
    }

    type CostCenterRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn cost_center_repository(&self) -> Result<Self::CostCenterRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CostCenter")
    }

    type BudgetAllocationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn budget_allocation_repository(&self) -> Result<Self::BudgetAllocationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BudgetAllocation")
    }

    type FinancialReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn financial_report_repository(&self) -> Result<Self::FinancialReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FinancialReport")
    }

    type AuditTrailRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn audit_trail_repository(&self) -> Result<Self::AuditTrailRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AuditTrail")
    }

    type ReconciliationEntryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn reconciliation_entry_repository(&self) -> Result<Self::ReconciliationEntryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ReconciliationEntry")
    }

    type CurrencyConversionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn currency_conversion_repository(&self) -> Result<Self::CurrencyConversionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CurrencyConversion")
    }

    type FiscalPeriodRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fiscal_period_repository(&self) -> Result<Self::FiscalPeriodRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FiscalPeriod")
    }

    type JobOrderRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn job_order_repository(&self) -> Result<Self::JobOrderRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("JobOrder")
    }

    type MoveScheduleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_schedule_repository(&self) -> Result<Self::MoveScheduleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveSchedule")
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

    type PickupLocationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn pickup_location_repository(&self) -> Result<Self::PickupLocationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PickupLocation")
    }

    type DeliveryLocationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn delivery_location_repository(&self) -> Result<Self::DeliveryLocationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DeliveryLocation")
    }

    type TransitTimeEstimateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn transit_time_estimate_repository(&self) -> Result<Self::TransitTimeEstimateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TransitTimeEstimate")
    }

    type LoadingDockRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn loading_dock_repository(&self) -> Result<Self::LoadingDockRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoadingDock")
    }

    type UnloadingDockRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn unloading_dock_repository(&self) -> Result<Self::UnloadingDockRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UnloadingDock")
    }

    type CustomsDocumentationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customs_documentation_repository(&self) -> Result<Self::CustomsDocumentationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomsDocumentation")
    }

    type PermitRequiredRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn permit_required_repository(&self) -> Result<Self::PermitRequiredRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PermitRequired")
    }

    type InsuranceCoverageRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn insurance_coverage_repository(&self) -> Result<Self::InsuranceCoverageRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InsuranceCoverage")
    }

    type LiabilityWaiverRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn liability_waiver_repository(&self) -> Result<Self::LiabilityWaiverRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LiabilityWaiver")
    }

    type TrackingNumberRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tracking_number_repository(&self) -> Result<Self::TrackingNumberRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TrackingNumber")
    }

    type StatusUpdateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn status_update_repository(&self) -> Result<Self::StatusUpdateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("StatusUpdate")
    }

    type NotificationTemplateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn notification_template_repository(&self) -> Result<Self::NotificationTemplateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("NotificationTemplate")
    }

    type SlaMetricRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn sla_metric_repository(&self) -> Result<Self::SlaMetricRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SlaMetric")
    }

    type PerformanceKpiRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn performance_kpi_repository(&self) -> Result<Self::PerformanceKpiRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PerformanceKpi")
    }

    type EmployeeRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn employee_record_repository(&self) -> Result<Self::EmployeeRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EmployeeRecord")
    }

    type PayrollInfoRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payroll_info_repository(&self) -> Result<Self::PayrollInfoRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PayrollInfo")
    }

    type BenefitsPlanRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn benefits_plan_repository(&self) -> Result<Self::BenefitsPlanRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BenefitsPlan")
    }

    type TimeOffRequestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn time_off_request_repository(&self) -> Result<Self::TimeOffRequestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TimeOffRequest")
    }

    type ShiftScheduleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn shift_schedule_repository(&self) -> Result<Self::ShiftScheduleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ShiftSchedule")
    }

    type PerformanceReviewRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn performance_review_repository(&self) -> Result<Self::PerformanceReviewRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PerformanceReview")
    }

    type CompetencyMatrixRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn competency_matrix_repository(&self) -> Result<Self::CompetencyMatrixRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CompetencyMatrix")
    }

    type TrainingCourseRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn training_course_repository(&self) -> Result<Self::TrainingCourseRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TrainingCourse")
    }

    type CertificationRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn certification_record_repository(&self) -> Result<Self::CertificationRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CertificationRecord")
    }

    type SafetyIncidentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn safety_incident_repository(&self) -> Result<Self::SafetyIncidentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SafetyIncident")
    }

    type HazardAssessmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn hazard_assessment_repository(&self) -> Result<Self::HazardAssessmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("HazardAssessment")
    }

    type PolicyAcknowledgmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn policy_acknowledgment_repository(&self) -> Result<Self::PolicyAcknowledgmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PolicyAcknowledgment")
    }

    type GrievanceLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn grievance_log_repository(&self) -> Result<Self::GrievanceLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("GrievanceLog")
    }

    type DisciplinaryActionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn disciplinary_action_repository(&self) -> Result<Self::DisciplinaryActionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DisciplinaryAction")
    }

    type ExitInterviewRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn exit_interview_repository(&self) -> Result<Self::ExitInterviewRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExitInterview")
    }

    type OnboardingChecklistRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn onboarding_checklist_repository(&self) -> Result<Self::OnboardingChecklistRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OnboardingChecklist")
    }

    type OffboardingChecklistRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn offboarding_checklist_repository(&self) -> Result<Self::OffboardingChecklistRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OffboardingChecklist")
    }

    type ComplianceAuditRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn compliance_audit_repository(&self) -> Result<Self::ComplianceAuditRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ComplianceAudit")
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
