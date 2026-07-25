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
    type CustomerContractRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_contract_repository(&self) -> Result<Self::CustomerContractRepository<'_>, ContextError>;
    type CustomerFeedbackRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_feedback_repository(&self) -> Result<Self::CustomerFeedbackRepository<'_>, ContextError>;
    type CustomerSegmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_segment_repository(&self) -> Result<Self::CustomerSegmentRepository<'_>, ContextError>;
    type CustomerLoyaltyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_loyalty_repository(&self) -> Result<Self::CustomerLoyaltyRepository<'_>, ContextError>;
    type CustomerInvoiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_invoice_repository(&self) -> Result<Self::CustomerInvoiceRepository<'_>, ContextError>;
    type CustomerPaymentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_payment_repository(&self) -> Result<Self::CustomerPaymentRepository<'_>, ContextError>;
    type CustomerClaimRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_claim_repository(&self) -> Result<Self::CustomerClaimRepository<'_>, ContextError>;
    type CustomerNotificationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_notification_repository(&self) -> Result<Self::CustomerNotificationRepository<'_>, ContextError>;
    type CustomerAccountRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_account_repository(&self) -> Result<Self::CustomerAccountRepository<'_>, ContextError>;
    type CustomerLeadRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_lead_repository(&self) -> Result<Self::CustomerLeadRepository<'_>, ContextError>;
    type CustomerQuoteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_quote_repository(&self) -> Result<Self::CustomerQuoteRepository<'_>, ContextError>;
    type CustomerServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_service_repository(&self) -> Result<Self::CustomerServiceRepository<'_>, ContextError>;
    type CustomerSupportTicketRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_support_ticket_repository(&self) -> Result<Self::CustomerSupportTicketRepository<'_>, ContextError>;
    type CustomerVehicleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_vehicle_repository(&self) -> Result<Self::CustomerVehicleRepository<'_>, ContextError>;
    type CustomerMoveHistoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_move_history_repository(&self) -> Result<Self::CustomerMoveHistoryRepository<'_>, ContextError>;
    type CustomerPreferredTimeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_preferred_time_repository(&self) -> Result<Self::CustomerPreferredTimeRepository<'_>, ContextError>;
    type FleetVehicleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fleet_vehicle_repository(&self) -> Result<Self::FleetVehicleRepository<'_>, ContextError>;
    type VehicleSpecRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_spec_repository(&self) -> Result<Self::VehicleSpecRepository<'_>, ContextError>;
    type VehicleMaintenanceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_maintenance_repository(&self) -> Result<Self::VehicleMaintenanceRepository<'_>, ContextError>;
    type VehicleInspectionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_inspection_repository(&self) -> Result<Self::VehicleInspectionRepository<'_>, ContextError>;
    type VehicleAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_assignment_repository(&self) -> Result<Self::VehicleAssignmentRepository<'_>, ContextError>;
    type VehicleUtilizationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_utilization_repository(&self) -> Result<Self::VehicleUtilizationRepository<'_>, ContextError>;
    type VehicleFuelLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_fuel_log_repository(&self) -> Result<Self::VehicleFuelLogRepository<'_>, ContextError>;
    type VehicleOdometerRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_odometer_repository(&self) -> Result<Self::VehicleOdometerRepository<'_>, ContextError>;
    type VehicleInsuranceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_insurance_repository(&self) -> Result<Self::VehicleInsuranceRepository<'_>, ContextError>;
    type VehicleRegistrationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_registration_repository(&self) -> Result<Self::VehicleRegistrationRepository<'_>, ContextError>;
    type VehicleDamageReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_damage_report_repository(&self) -> Result<Self::VehicleDamageReportRepository<'_>, ContextError>;
    type VehicleCleaningScheduleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_cleaning_schedule_repository(&self) -> Result<Self::VehicleCleaningScheduleRepository<'_>, ContextError>;
    type FleetOperatorRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fleet_operator_repository(&self) -> Result<Self::FleetOperatorRepository<'_>, ContextError>;
    type DriverProfileRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn driver_profile_repository(&self) -> Result<Self::DriverProfileRepository<'_>, ContextError>;
    type DriverLicenseRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn driver_license_repository(&self) -> Result<Self::DriverLicenseRepository<'_>, ContextError>;
    type DriverCertificationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn driver_certification_repository(&self) -> Result<Self::DriverCertificationRepository<'_>, ContextError>;
    type DriverAvailabilityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn driver_availability_repository(&self) -> Result<Self::DriverAvailabilityRepository<'_>, ContextError>;
    type DriverPerformanceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn driver_performance_repository(&self) -> Result<Self::DriverPerformanceRepository<'_>, ContextError>;
    type DriverTrainingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn driver_training_repository(&self) -> Result<Self::DriverTrainingRepository<'_>, ContextError>;
    type FleetDispatchRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fleet_dispatch_repository(&self) -> Result<Self::FleetDispatchRepository<'_>, ContextError>;
    type InvoiceHeaderRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn invoice_header_repository(&self) -> Result<Self::InvoiceHeaderRepository<'_>, ContextError>;
    type InvoiceLineItemRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn invoice_line_item_repository(&self) -> Result<Self::InvoiceLineItemRepository<'_>, ContextError>;
    type PaymentMethodRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payment_method_repository(&self) -> Result<Self::PaymentMethodRepository<'_>, ContextError>;
    type PaymentTransactionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payment_transaction_repository(&self) -> Result<Self::PaymentTransactionRepository<'_>, ContextError>;
    type BillingCycleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn billing_cycle_repository(&self) -> Result<Self::BillingCycleRepository<'_>, ContextError>;
    type TaxCodeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tax_code_repository(&self) -> Result<Self::TaxCodeRepository<'_>, ContextError>;
    type DiscountRuleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn discount_rule_repository(&self) -> Result<Self::DiscountRuleRepository<'_>, ContextError>;
    type CreditNoteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn credit_note_repository(&self) -> Result<Self::CreditNoteRepository<'_>, ContextError>;
    type DebitNoteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn debit_note_repository(&self) -> Result<Self::DebitNoteRepository<'_>, ContextError>;
    type BillingAddressRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn billing_address_repository(&self) -> Result<Self::BillingAddressRepository<'_>, ContextError>;
    type OutstandingBalanceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn outstanding_balance_repository(&self) -> Result<Self::OutstandingBalanceRepository<'_>, ContextError>;
    type AgingReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn aging_report_repository(&self) -> Result<Self::AgingReportRepository<'_>, ContextError>;
    type PaymentReminderRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payment_reminder_repository(&self) -> Result<Self::PaymentReminderRepository<'_>, ContextError>;
    type RefundRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn refund_request_repository(&self) -> Result<Self::RefundRequestRepository<'_>, ContextError>;
    type BillingAdjustmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn billing_adjustment_repository(&self) -> Result<Self::BillingAdjustmentRepository<'_>, ContextError>;
    type RevenueRecognitionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn revenue_recognition_repository(&self) -> Result<Self::RevenueRecognitionRepository<'_>, ContextError>;
    type FinancialPeriodRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn financial_period_repository(&self) -> Result<Self::FinancialPeriodRepository<'_>, ContextError>;
    type AuditTrailRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn audit_trail_repository(&self) -> Result<Self::AuditTrailRepository<'_>, ContextError>;
    type CurrencyRateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn currency_rate_repository(&self) -> Result<Self::CurrencyRateRepository<'_>, ContextError>;
    type BillingApprovalRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn billing_approval_repository(&self) -> Result<Self::BillingApprovalRepository<'_>, ContextError>;
    type MoveOrderRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_order_repository(&self) -> Result<Self::MoveOrderRepository<'_>, ContextError>;
    type MoveScheduleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_schedule_repository(&self) -> Result<Self::MoveScheduleRepository<'_>, ContextError>;
    type RoutePlanRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn route_plan_repository(&self) -> Result<Self::RoutePlanRepository<'_>, ContextError>;
    type LoadPlanRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn load_plan_repository(&self) -> Result<Self::LoadPlanRepository<'_>, ContextError>;
    type CrewAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn crew_assignment_repository(&self) -> Result<Self::CrewAssignmentRepository<'_>, ContextError>;
    type EquipmentChecklistRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn equipment_checklist_repository(&self) -> Result<Self::EquipmentChecklistRepository<'_>, ContextError>;
    type LoadingProcedureRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn loading_procedure_repository(&self) -> Result<Self::LoadingProcedureRepository<'_>, ContextError>;
    type UnloadingProcedureRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn unloading_procedure_repository(&self) -> Result<Self::UnloadingProcedureRepository<'_>, ContextError>;
    type TransitMonitoringRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn transit_monitoring_repository(&self) -> Result<Self::TransitMonitoringRepository<'_>, ContextError>;
    type DeliveryConfirmationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn delivery_confirmation_repository(&self) -> Result<Self::DeliveryConfirmationRepository<'_>, ContextError>;
    type ExceptionHandlingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn exception_handling_repository(&self) -> Result<Self::ExceptionHandlingRepository<'_>, ContextError>;
    type CustomsDocumentationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customs_documentation_repository(&self) -> Result<Self::CustomsDocumentationRepository<'_>, ContextError>;
    type InventorySnapshotRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn inventory_snapshot_repository(&self) -> Result<Self::InventorySnapshotRepository<'_>, ContextError>;
    type WarehouseAllocationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn warehouse_allocation_repository(&self) -> Result<Self::WarehouseAllocationRepository<'_>, ContextError>;
    type DockSchedulingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn dock_scheduling_repository(&self) -> Result<Self::DockSchedulingRepository<'_>, ContextError>;
    type YardManagementRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn yard_management_repository(&self) -> Result<Self::YardManagementRepository<'_>, ContextError>;
    type SafetyIncidentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn safety_incident_repository(&self) -> Result<Self::SafetyIncidentRepository<'_>, ContextError>;
    type ComplianceCheckRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn compliance_check_repository(&self) -> Result<Self::ComplianceCheckRepository<'_>, ContextError>;
    type PerformanceMetricRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn performance_metric_repository(&self) -> Result<Self::PerformanceMetricRepository<'_>, ContextError>;
    type OperationsDashboardRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn operations_dashboard_repository(&self) -> Result<Self::OperationsDashboardRepository<'_>, ContextError>;
    type DailySummaryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn daily_summary_repository(&self) -> Result<Self::DailySummaryRepository<'_>, ContextError>;
    type WeeklyReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn weekly_report_repository(&self) -> Result<Self::WeeklyReportRepository<'_>, ContextError>;
    type MonthlyKpiRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn monthly_kpi_repository(&self) -> Result<Self::MonthlyKpiRepository<'_>, ContextError>;
    type AnnualPerformanceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn annual_performance_repository(&self) -> Result<Self::AnnualPerformanceRepository<'_>, ContextError>;
    type UtilizationReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn utilization_report_repository(&self) -> Result<Self::UtilizationReportRepository<'_>, ContextError>;
    type CostAnalysisRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn cost_analysis_repository(&self) -> Result<Self::CostAnalysisRepository<'_>, ContextError>;
    type ProfitMarginRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn profit_margin_repository(&self) -> Result<Self::ProfitMarginRepository<'_>, ContextError>;
    type CustomerSatisfactionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_satisfaction_repository(&self) -> Result<Self::CustomerSatisfactionRepository<'_>, ContextError>;
    type OnTimeDeliveryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn on_time_delivery_repository(&self) -> Result<Self::OnTimeDeliveryRepository<'_>, ContextError>;
    type ClaimRateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn claim_rate_repository(&self) -> Result<Self::ClaimRateRepository<'_>, ContextError>;
    type FleetEfficiencyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fleet_efficiency_repository(&self) -> Result<Self::FleetEfficiencyRepository<'_>, ContextError>;
    type DriverProductivityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn driver_productivity_repository(&self) -> Result<Self::DriverProductivityRepository<'_>, ContextError>;
    type BillingAccuracyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn billing_accuracy_repository(&self) -> Result<Self::BillingAccuracyRepository<'_>, ContextError>;
    type InvoiceAgingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn invoice_aging_repository(&self) -> Result<Self::InvoiceAgingRepository<'_>, ContextError>;
    type MoveVolumeTrendRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_volume_trend_repository(&self) -> Result<Self::MoveVolumeTrendRepository<'_>, ContextError>;
    type GeographicDistributionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn geographic_distribution_repository(&self) -> Result<Self::GeographicDistributionRepository<'_>, ContextError>;
    type ServiceLinePerformanceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_line_performance_repository(&self) -> Result<Self::ServiceLinePerformanceRepository<'_>, ContextError>;
    type ExpenseVarianceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn expense_variance_repository(&self) -> Result<Self::ExpenseVarianceRepository<'_>, ContextError>;
    type ForecastVsActualRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn forecast_vs_actual_repository(&self) -> Result<Self::ForecastVsActualRepository<'_>, ContextError>;
    type ExecutiveDashboardRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn executive_dashboard_repository(&self) -> Result<Self::ExecutiveDashboardRepository<'_>, ContextError>;
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

    type CustomerContractRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_contract_repository(&self) -> Result<Self::CustomerContractRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerContract")
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

    type CustomerLoyaltyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_loyalty_repository(&self) -> Result<Self::CustomerLoyaltyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerLoyalty")
    }

    type CustomerInvoiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_invoice_repository(&self) -> Result<Self::CustomerInvoiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerInvoice")
    }

    type CustomerPaymentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_payment_repository(&self) -> Result<Self::CustomerPaymentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerPayment")
    }

    type CustomerClaimRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_claim_repository(&self) -> Result<Self::CustomerClaimRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerClaim")
    }

    type CustomerNotificationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_notification_repository(&self) -> Result<Self::CustomerNotificationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerNotification")
    }

    type CustomerAccountRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_account_repository(&self) -> Result<Self::CustomerAccountRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerAccount")
    }

    type CustomerLeadRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_lead_repository(&self) -> Result<Self::CustomerLeadRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerLead")
    }

    type CustomerQuoteRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_quote_repository(&self) -> Result<Self::CustomerQuoteRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerQuote")
    }

    type CustomerServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_service_repository(&self) -> Result<Self::CustomerServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerService")
    }

    type CustomerSupportTicketRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_support_ticket_repository(&self) -> Result<Self::CustomerSupportTicketRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerSupportTicket")
    }

    type CustomerVehicleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_vehicle_repository(&self) -> Result<Self::CustomerVehicleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerVehicle")
    }

    type CustomerMoveHistoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_move_history_repository(&self) -> Result<Self::CustomerMoveHistoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerMoveHistory")
    }

    type CustomerPreferredTimeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_preferred_time_repository(&self) -> Result<Self::CustomerPreferredTimeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerPreferredTime")
    }

    type FleetVehicleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fleet_vehicle_repository(&self) -> Result<Self::FleetVehicleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FleetVehicle")
    }

    type VehicleSpecRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_spec_repository(&self) -> Result<Self::VehicleSpecRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleSpec")
    }

    type VehicleMaintenanceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_maintenance_repository(&self) -> Result<Self::VehicleMaintenanceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleMaintenance")
    }

    type VehicleInspectionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_inspection_repository(&self) -> Result<Self::VehicleInspectionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleInspection")
    }

    type VehicleAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_assignment_repository(&self) -> Result<Self::VehicleAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleAssignment")
    }

    type VehicleUtilizationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_utilization_repository(&self) -> Result<Self::VehicleUtilizationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleUtilization")
    }

    type VehicleFuelLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_fuel_log_repository(&self) -> Result<Self::VehicleFuelLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleFuelLog")
    }

    type VehicleOdometerRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_odometer_repository(&self) -> Result<Self::VehicleOdometerRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleOdometer")
    }

    type VehicleInsuranceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_insurance_repository(&self) -> Result<Self::VehicleInsuranceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleInsurance")
    }

    type VehicleRegistrationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_registration_repository(&self) -> Result<Self::VehicleRegistrationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleRegistration")
    }

    type VehicleDamageReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_damage_report_repository(&self) -> Result<Self::VehicleDamageReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleDamageReport")
    }

    type VehicleCleaningScheduleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_cleaning_schedule_repository(&self) -> Result<Self::VehicleCleaningScheduleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleCleaningSchedule")
    }

    type FleetOperatorRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fleet_operator_repository(&self) -> Result<Self::FleetOperatorRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FleetOperator")
    }

    type DriverProfileRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn driver_profile_repository(&self) -> Result<Self::DriverProfileRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DriverProfile")
    }

    type DriverLicenseRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn driver_license_repository(&self) -> Result<Self::DriverLicenseRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DriverLicense")
    }

    type DriverCertificationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn driver_certification_repository(&self) -> Result<Self::DriverCertificationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DriverCertification")
    }

    type DriverAvailabilityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn driver_availability_repository(&self) -> Result<Self::DriverAvailabilityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DriverAvailability")
    }

    type DriverPerformanceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn driver_performance_repository(&self) -> Result<Self::DriverPerformanceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DriverPerformance")
    }

    type DriverTrainingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn driver_training_repository(&self) -> Result<Self::DriverTrainingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DriverTraining")
    }

    type FleetDispatchRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fleet_dispatch_repository(&self) -> Result<Self::FleetDispatchRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FleetDispatch")
    }

    type InvoiceHeaderRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn invoice_header_repository(&self) -> Result<Self::InvoiceHeaderRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InvoiceHeader")
    }

    type InvoiceLineItemRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn invoice_line_item_repository(&self) -> Result<Self::InvoiceLineItemRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InvoiceLineItem")
    }

    type PaymentMethodRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payment_method_repository(&self) -> Result<Self::PaymentMethodRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PaymentMethod")
    }

    type PaymentTransactionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payment_transaction_repository(&self) -> Result<Self::PaymentTransactionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PaymentTransaction")
    }

    type BillingCycleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn billing_cycle_repository(&self) -> Result<Self::BillingCycleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BillingCycle")
    }

    type TaxCodeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tax_code_repository(&self) -> Result<Self::TaxCodeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TaxCode")
    }

    type DiscountRuleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn discount_rule_repository(&self) -> Result<Self::DiscountRuleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DiscountRule")
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

    type BillingAddressRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn billing_address_repository(&self) -> Result<Self::BillingAddressRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BillingAddress")
    }

    type OutstandingBalanceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn outstanding_balance_repository(&self) -> Result<Self::OutstandingBalanceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OutstandingBalance")
    }

    type AgingReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn aging_report_repository(&self) -> Result<Self::AgingReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AgingReport")
    }

    type PaymentReminderRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payment_reminder_repository(&self) -> Result<Self::PaymentReminderRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PaymentReminder")
    }

    type RefundRequestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn refund_request_repository(&self) -> Result<Self::RefundRequestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RefundRequest")
    }

    type BillingAdjustmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn billing_adjustment_repository(&self) -> Result<Self::BillingAdjustmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BillingAdjustment")
    }

    type RevenueRecognitionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn revenue_recognition_repository(&self) -> Result<Self::RevenueRecognitionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RevenueRecognition")
    }

    type FinancialPeriodRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn financial_period_repository(&self) -> Result<Self::FinancialPeriodRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FinancialPeriod")
    }

    type AuditTrailRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn audit_trail_repository(&self) -> Result<Self::AuditTrailRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AuditTrail")
    }

    type CurrencyRateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn currency_rate_repository(&self) -> Result<Self::CurrencyRateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CurrencyRate")
    }

    type BillingApprovalRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn billing_approval_repository(&self) -> Result<Self::BillingApprovalRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BillingApproval")
    }

    type MoveOrderRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_order_repository(&self) -> Result<Self::MoveOrderRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveOrder")
    }

    type MoveScheduleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_schedule_repository(&self) -> Result<Self::MoveScheduleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveSchedule")
    }

    type RoutePlanRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn route_plan_repository(&self) -> Result<Self::RoutePlanRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RoutePlan")
    }

    type LoadPlanRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn load_plan_repository(&self) -> Result<Self::LoadPlanRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoadPlan")
    }

    type CrewAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn crew_assignment_repository(&self) -> Result<Self::CrewAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CrewAssignment")
    }

    type EquipmentChecklistRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn equipment_checklist_repository(&self) -> Result<Self::EquipmentChecklistRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EquipmentChecklist")
    }

    type LoadingProcedureRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn loading_procedure_repository(&self) -> Result<Self::LoadingProcedureRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoadingProcedure")
    }

    type UnloadingProcedureRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn unloading_procedure_repository(&self) -> Result<Self::UnloadingProcedureRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UnloadingProcedure")
    }

    type TransitMonitoringRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn transit_monitoring_repository(&self) -> Result<Self::TransitMonitoringRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TransitMonitoring")
    }

    type DeliveryConfirmationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn delivery_confirmation_repository(&self) -> Result<Self::DeliveryConfirmationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DeliveryConfirmation")
    }

    type ExceptionHandlingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn exception_handling_repository(&self) -> Result<Self::ExceptionHandlingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExceptionHandling")
    }

    type CustomsDocumentationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customs_documentation_repository(&self) -> Result<Self::CustomsDocumentationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomsDocumentation")
    }

    type InventorySnapshotRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn inventory_snapshot_repository(&self) -> Result<Self::InventorySnapshotRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InventorySnapshot")
    }

    type WarehouseAllocationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn warehouse_allocation_repository(&self) -> Result<Self::WarehouseAllocationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("WarehouseAllocation")
    }

    type DockSchedulingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn dock_scheduling_repository(&self) -> Result<Self::DockSchedulingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DockScheduling")
    }

    type YardManagementRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn yard_management_repository(&self) -> Result<Self::YardManagementRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("YardManagement")
    }

    type SafetyIncidentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn safety_incident_repository(&self) -> Result<Self::SafetyIncidentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SafetyIncident")
    }

    type ComplianceCheckRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn compliance_check_repository(&self) -> Result<Self::ComplianceCheckRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ComplianceCheck")
    }

    type PerformanceMetricRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn performance_metric_repository(&self) -> Result<Self::PerformanceMetricRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PerformanceMetric")
    }

    type OperationsDashboardRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn operations_dashboard_repository(&self) -> Result<Self::OperationsDashboardRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OperationsDashboard")
    }

    type DailySummaryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn daily_summary_repository(&self) -> Result<Self::DailySummaryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DailySummary")
    }

    type WeeklyReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn weekly_report_repository(&self) -> Result<Self::WeeklyReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("WeeklyReport")
    }

    type MonthlyKpiRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn monthly_kpi_repository(&self) -> Result<Self::MonthlyKpiRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MonthlyKpi")
    }

    type AnnualPerformanceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn annual_performance_repository(&self) -> Result<Self::AnnualPerformanceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AnnualPerformance")
    }

    type UtilizationReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn utilization_report_repository(&self) -> Result<Self::UtilizationReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UtilizationReport")
    }

    type CostAnalysisRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn cost_analysis_repository(&self) -> Result<Self::CostAnalysisRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CostAnalysis")
    }

    type ProfitMarginRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn profit_margin_repository(&self) -> Result<Self::ProfitMarginRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ProfitMargin")
    }

    type CustomerSatisfactionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_satisfaction_repository(&self) -> Result<Self::CustomerSatisfactionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerSatisfaction")
    }

    type OnTimeDeliveryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn on_time_delivery_repository(&self) -> Result<Self::OnTimeDeliveryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OnTimeDelivery")
    }

    type ClaimRateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn claim_rate_repository(&self) -> Result<Self::ClaimRateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ClaimRate")
    }

    type FleetEfficiencyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fleet_efficiency_repository(&self) -> Result<Self::FleetEfficiencyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FleetEfficiency")
    }

    type DriverProductivityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn driver_productivity_repository(&self) -> Result<Self::DriverProductivityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DriverProductivity")
    }

    type BillingAccuracyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn billing_accuracy_repository(&self) -> Result<Self::BillingAccuracyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BillingAccuracy")
    }

    type InvoiceAgingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn invoice_aging_repository(&self) -> Result<Self::InvoiceAgingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InvoiceAging")
    }

    type MoveVolumeTrendRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_volume_trend_repository(&self) -> Result<Self::MoveVolumeTrendRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveVolumeTrend")
    }

    type GeographicDistributionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn geographic_distribution_repository(&self) -> Result<Self::GeographicDistributionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("GeographicDistribution")
    }

    type ServiceLinePerformanceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_line_performance_repository(&self) -> Result<Self::ServiceLinePerformanceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceLinePerformance")
    }

    type ExpenseVarianceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn expense_variance_repository(&self) -> Result<Self::ExpenseVarianceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExpenseVariance")
    }

    type ForecastVsActualRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn forecast_vs_actual_repository(&self) -> Result<Self::ForecastVsActualRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ForecastVsActual")
    }

    type ExecutiveDashboardRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn executive_dashboard_repository(&self) -> Result<Self::ExecutiveDashboardRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExecutiveDashboard")
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
