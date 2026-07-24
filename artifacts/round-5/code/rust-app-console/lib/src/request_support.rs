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
    type PlatformRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn platform_repository(&self) -> Result<Self::PlatformRepository<'_>, ContextError>;
    type MerchantRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn merchant_repository(&self) -> Result<Self::MerchantRepository<'_>, ContextError>;
    type EmployeeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn employee_repository(&self) -> Result<Self::EmployeeRepository<'_>, ContextError>;
    type PlatformSettingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn platform_setting_repository(&self) -> Result<Self::PlatformSettingRepository<'_>, ContextError>;
    type PlatformUserRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn platform_user_repository(&self) -> Result<Self::PlatformUserRepository<'_>, ContextError>;
    type PlatformAuditLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn platform_audit_log_repository(&self) -> Result<Self::PlatformAuditLogRepository<'_>, ContextError>;
    type OrganizationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn organization_repository(&self) -> Result<Self::OrganizationRepository<'_>, ContextError>;
    type OrganizationSettingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn organization_setting_repository(&self) -> Result<Self::OrganizationSettingRepository<'_>, ContextError>;
    type OrganizationMemberRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn organization_member_repository(&self) -> Result<Self::OrganizationMemberRepository<'_>, ContextError>;
    type MoveOrderRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_order_repository(&self) -> Result<Self::MoveOrderRepository<'_>, ContextError>;
    type MoveQuoteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_quote_repository(&self) -> Result<Self::MoveQuoteRepository<'_>, ContextError>;
    type RouteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn route_repository(&self) -> Result<Self::RouteRepository<'_>, ContextError>;
    type RouteStopRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn route_stop_repository(&self) -> Result<Self::RouteStopRepository<'_>, ContextError>;
    type TimeSlotRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn time_slot_repository(&self) -> Result<Self::TimeSlotRepository<'_>, ContextError>;
    type FulfillmentEventRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fulfillment_event_repository(&self) -> Result<Self::FulfillmentEventRepository<'_>, ContextError>;
    type AddressRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn address_repository(&self) -> Result<Self::AddressRepository<'_>, ContextError>;
    type CrewRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn crew_repository(&self) -> Result<Self::CrewRepository<'_>, ContextError>;
    type DispatchAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn dispatch_assignment_repository(&self) -> Result<Self::DispatchAssignmentRepository<'_>, ContextError>;
    type DamageReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn damage_report_repository(&self) -> Result<Self::DamageReportRepository<'_>, ContextError>;
    type ProofOfDeliveryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn proof_of_delivery_repository(&self) -> Result<Self::ProofOfDeliveryRepository<'_>, ContextError>;
    type InventoryItemRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn inventory_item_repository(&self) -> Result<Self::InventoryItemRepository<'_>, ContextError>;
    type PackingListRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn packing_list_repository(&self) -> Result<Self::PackingListRepository<'_>, ContextError>;
    type PackingItemRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn packing_item_repository(&self) -> Result<Self::PackingItemRepository<'_>, ContextError>;
    type LoadingPlanRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn loading_plan_repository(&self) -> Result<Self::LoadingPlanRepository<'_>, ContextError>;
    type UnloadingPlanRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn unloading_plan_repository(&self) -> Result<Self::UnloadingPlanRepository<'_>, ContextError>;
    type StorageFacilityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn storage_facility_repository(&self) -> Result<Self::StorageFacilityRepository<'_>, ContextError>;
    type StorageUnitRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn storage_unit_repository(&self) -> Result<Self::StorageUnitRepository<'_>, ContextError>;
    type StorageInventoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn storage_inventory_repository(&self) -> Result<Self::StorageInventoryRepository<'_>, ContextError>;
    type TransportManifestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn transport_manifest_repository(&self) -> Result<Self::TransportManifestRepository<'_>, ContextError>;
    type CustomsDeclarationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customs_declaration_repository(&self) -> Result<Self::CustomsDeclarationRepository<'_>, ContextError>;
    type EquipmentChecklistRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn equipment_checklist_repository(&self) -> Result<Self::EquipmentChecklistRepository<'_>, ContextError>;
    type FuelLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fuel_log_repository(&self) -> Result<Self::FuelLogRepository<'_>, ContextError>;
    type MaintenanceRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn maintenance_request_repository(&self) -> Result<Self::MaintenanceRequestRepository<'_>, ContextError>;
    type DepartmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn department_repository(&self) -> Result<Self::DepartmentRepository<'_>, ContextError>;
    type JobAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn job_assignment_repository(&self) -> Result<Self::JobAssignmentRepository<'_>, ContextError>;
    type WorkShiftRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn work_shift_repository(&self) -> Result<Self::WorkShiftRepository<'_>, ContextError>;
    type WorkedHoursRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn worked_hours_repository(&self) -> Result<Self::WorkedHoursRepository<'_>, ContextError>;
    type PayrollPeriodRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payroll_period_repository(&self) -> Result<Self::PayrollPeriodRepository<'_>, ContextError>;
    type PayrollCalculationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payroll_calculation_repository(&self) -> Result<Self::PayrollCalculationRepository<'_>, ContextError>;
    type PayslipRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payslip_repository(&self) -> Result<Self::PayslipRepository<'_>, ContextError>;
    type BonusRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn bonus_repository(&self) -> Result<Self::BonusRepository<'_>, ContextError>;
    type EmployeeCertificationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn employee_certification_repository(&self) -> Result<Self::EmployeeCertificationRepository<'_>, ContextError>;
    type LeaveRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn leave_request_repository(&self) -> Result<Self::LeaveRequestRepository<'_>, ContextError>;
    type BillingProfileRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn billing_profile_repository(&self) -> Result<Self::BillingProfileRepository<'_>, ContextError>;
    type CorporateCustomerProfileRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn corporate_customer_profile_repository(&self) -> Result<Self::CorporateCustomerProfileRepository<'_>, ContextError>;
    type CustomerRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_repository(&self) -> Result<Self::CustomerRepository<'_>, ContextError>;
    type CustomerConsentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_consent_repository(&self) -> Result<Self::CustomerConsentRepository<'_>, ContextError>;
    type CustomerContactRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_contact_repository(&self) -> Result<Self::CustomerContactRepository<'_>, ContextError>;
    type CustomerHistoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_history_repository(&self) -> Result<Self::CustomerHistoryRepository<'_>, ContextError>;
    type CustomerPreferenceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_preference_repository(&self) -> Result<Self::CustomerPreferenceRepository<'_>, ContextError>;
    type PrivateCustomerProfileRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn private_customer_profile_repository(&self) -> Result<Self::PrivateCustomerProfileRepository<'_>, ContextError>;
    type BoxRentalRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn box_rental_repository(&self) -> Result<Self::BoxRentalRepository<'_>, ContextError>;
    type CleaningServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn cleaning_service_repository(&self) -> Result<Self::CleaningServiceRepository<'_>, ContextError>;
    type MovingServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn moving_service_repository(&self) -> Result<Self::MovingServiceRepository<'_>, ContextError>;
    type PriceListRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn price_list_repository(&self) -> Result<Self::PriceListRepository<'_>, ContextError>;
    type ProductRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn product_repository(&self) -> Result<Self::ProductRepository<'_>, ContextError>;
    type ServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_repository(&self) -> Result<Self::ServiceRepository<'_>, ContextError>;
    type ServiceBundleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_bundle_repository(&self) -> Result<Self::ServiceBundleRepository<'_>, ContextError>;
    type ServiceConfigurationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_configuration_repository(&self) -> Result<Self::ServiceConfigurationRepository<'_>, ContextError>;
    type ServicePriceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_price_repository(&self) -> Result<Self::ServicePriceRepository<'_>, ContextError>;
    type CampaignRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn campaign_repository(&self) -> Result<Self::CampaignRepository<'_>, ContextError>;
    type ConversionEventRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn conversion_event_repository(&self) -> Result<Self::ConversionEventRepository<'_>, ContextError>;
    type ConversionMetricRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn conversion_metric_repository(&self) -> Result<Self::ConversionMetricRepository<'_>, ContextError>;
    type DiscountCodeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn discount_code_repository(&self) -> Result<Self::DiscountCodeRepository<'_>, ContextError>;
    type LeadRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn lead_repository(&self) -> Result<Self::LeadRepository<'_>, ContextError>;
    type LeadActivityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn lead_activity_repository(&self) -> Result<Self::LeadActivityRepository<'_>, ContextError>;
    type SalesOpportunityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn sales_opportunity_repository(&self) -> Result<Self::SalesOpportunityRepository<'_>, ContextError>;
    type AccountRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn account_repository(&self) -> Result<Self::AccountRepository<'_>, ContextError>;
    type ExpenseRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn expense_repository(&self) -> Result<Self::ExpenseRepository<'_>, ContextError>;
    type FinancialSummaryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn financial_summary_repository(&self) -> Result<Self::FinancialSummaryRepository<'_>, ContextError>;
    type InvoiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn invoice_repository(&self) -> Result<Self::InvoiceRepository<'_>, ContextError>;
    type InvoiceLineRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn invoice_line_repository(&self) -> Result<Self::InvoiceLineRepository<'_>, ContextError>;
    type JournalEntryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn journal_entry_repository(&self) -> Result<Self::JournalEntryRepository<'_>, ContextError>;
    type PaymentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payment_repository(&self) -> Result<Self::PaymentRepository<'_>, ContextError>;
    type RefundRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn refund_repository(&self) -> Result<Self::RefundRepository<'_>, ContextError>;
    type VatRateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vat_rate_repository(&self) -> Result<Self::VatRateRepository<'_>, ContextError>;
    type AssetAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn asset_assignment_repository(&self) -> Result<Self::AssetAssignmentRepository<'_>, ContextError>;
    type AssetInspectionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn asset_inspection_repository(&self) -> Result<Self::AssetInspectionRepository<'_>, ContextError>;
    type ConsumableRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn consumable_repository(&self) -> Result<Self::ConsumableRepository<'_>, ContextError>;
    type EquipmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn equipment_repository(&self) -> Result<Self::EquipmentRepository<'_>, ContextError>;
    type FuelRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fuel_record_repository(&self) -> Result<Self::FuelRecordRepository<'_>, ContextError>;
    type MaintenanceEventRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn maintenance_event_repository(&self) -> Result<Self::MaintenanceEventRepository<'_>, ContextError>;
    type MaintenanceScheduleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn maintenance_schedule_repository(&self) -> Result<Self::MaintenanceScheduleRepository<'_>, ContextError>;
    type SupplierRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn supplier_repository(&self) -> Result<Self::SupplierRepository<'_>, ContextError>;
    type VehicleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_repository(&self) -> Result<Self::VehicleRepository<'_>, ContextError>;
    type ComplianceCheckRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn compliance_check_repository(&self) -> Result<Self::ComplianceCheckRepository<'_>, ContextError>;
    type ContractRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn contract_repository(&self) -> Result<Self::ContractRepository<'_>, ContextError>;
    type DataRetentionPolicyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn data_retention_policy_repository(&self) -> Result<Self::DataRetentionPolicyRepository<'_>, ContextError>;
    type DocumentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn document_repository(&self) -> Result<Self::DocumentRepository<'_>, ContextError>;
    type DocumentVersionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn document_version_repository(&self) -> Result<Self::DocumentVersionRepository<'_>, ContextError>;
    type InsuranceClaimRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn insurance_claim_repository(&self) -> Result<Self::InsuranceClaimRepository<'_>, ContextError>;
    type InsurancePolicyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn insurance_policy_repository(&self) -> Result<Self::InsurancePolicyRepository<'_>, ContextError>;
    type RecoveryRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn recovery_request_repository(&self) -> Result<Self::RecoveryRequestRepository<'_>, ContextError>;
    type MagicLinkRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn magic_link_repository(&self) -> Result<Self::MagicLinkRepository<'_>, ContextError>;
    type PermissionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn permission_repository(&self) -> Result<Self::PermissionRepository<'_>, ContextError>;
    type RoleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn role_repository(&self) -> Result<Self::RoleRepository<'_>, ContextError>;
    type RolePermissionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn role_permission_repository(&self) -> Result<Self::RolePermissionRepository<'_>, ContextError>;
    type UserAccountRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn user_account_repository(&self) -> Result<Self::UserAccountRepository<'_>, ContextError>;
    type UserRoleAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn user_role_assignment_repository(&self) -> Result<Self::UserRoleAssignmentRepository<'_>, ContextError>;
    type UserSessionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn user_session_repository(&self) -> Result<Self::UserSessionRepository<'_>, ContextError>;
    type ActivityLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn activity_log_repository(&self) -> Result<Self::ActivityLogRepository<'_>, ContextError>;
    type AuditLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn audit_log_repository(&self) -> Result<Self::AuditLogRepository<'_>, ContextError>;
    type ChangeSetRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn change_set_repository(&self) -> Result<Self::ChangeSetRepository<'_>, ContextError>;
    type EntityChangeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn entity_change_repository(&self) -> Result<Self::EntityChangeRepository<'_>, ContextError>;
    type AutomationActionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn automation_action_repository(&self) -> Result<Self::AutomationActionRepository<'_>, ContextError>;
    type AutomationRuleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn automation_rule_repository(&self) -> Result<Self::AutomationRuleRepository<'_>, ContextError>;
    type AutomationTriggerRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn automation_trigger_repository(&self) -> Result<Self::AutomationTriggerRepository<'_>, ContextError>;
    type NotificationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn notification_repository(&self) -> Result<Self::NotificationRepository<'_>, ContextError>;
    type NotificationTemplateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn notification_template_repository(&self) -> Result<Self::NotificationTemplateRepository<'_>, ContextError>;
    type ApiClientRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn api_client_repository(&self) -> Result<Self::ApiClientRepository<'_>, ContextError>;
    type ApiEndpointRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn api_endpoint_repository(&self) -> Result<Self::ApiEndpointRepository<'_>, ContextError>;
    type IntegrationMappingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn integration_mapping_repository(&self) -> Result<Self::IntegrationMappingRepository<'_>, ContextError>;
    type WebhookRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn webhook_repository(&self) -> Result<Self::WebhookRepository<'_>, ContextError>;
    type WebhookDeliveryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn webhook_delivery_repository(&self) -> Result<Self::WebhookDeliveryRepository<'_>, ContextError>;
    type PlatformConfigurationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn platform_configuration_repository(&self) -> Result<Self::PlatformConfigurationRepository<'_>, ContextError>;
    type PlatformLocaleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn platform_locale_repository(&self) -> Result<Self::PlatformLocaleRepository<'_>, ContextError>;
    type MerchantBranchRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn merchant_branch_repository(&self) -> Result<Self::MerchantBranchRepository<'_>, ContextError>;
    type MerchantSettingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn merchant_setting_repository(&self) -> Result<Self::MerchantSettingRepository<'_>, ContextError>;
    type OperationalExceptionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn operational_exception_repository(&self) -> Result<Self::OperationalExceptionRepository<'_>, ContextError>;
    type CrewMemberAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn crew_member_assignment_repository(&self) -> Result<Self::CrewMemberAssignmentRepository<'_>, ContextError>;
    type PickupInstructionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn pickup_instruction_repository(&self) -> Result<Self::PickupInstructionRepository<'_>, ContextError>;
    type DeliveryInstructionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn delivery_instruction_repository(&self) -> Result<Self::DeliveryInstructionRepository<'_>, ContextError>;
    type MoveInventoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_inventory_repository(&self) -> Result<Self::MoveInventoryRepository<'_>, ContextError>;
    type ExtraOperationsLogistics1Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_operations_logistics_1_repository(&self) -> Result<Self::ExtraOperationsLogistics1Repository<'_>, ContextError>;
    type ExtraOperationsLogistics2Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_operations_logistics_2_repository(&self) -> Result<Self::ExtraOperationsLogistics2Repository<'_>, ContextError>;
    type ExtraOperationsLogistics3Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_operations_logistics_3_repository(&self) -> Result<Self::ExtraOperationsLogistics3Repository<'_>, ContextError>;
    type ExtraOperationsLogistics4Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_operations_logistics_4_repository(&self) -> Result<Self::ExtraOperationsLogistics4Repository<'_>, ContextError>;
    type ExtraOperationsLogistics5Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_operations_logistics_5_repository(&self) -> Result<Self::ExtraOperationsLogistics5Repository<'_>, ContextError>;
    type ExtraOperationsLogistics6Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_operations_logistics_6_repository(&self) -> Result<Self::ExtraOperationsLogistics6Repository<'_>, ContextError>;
    type ExtraOperationsLogistics7Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_operations_logistics_7_repository(&self) -> Result<Self::ExtraOperationsLogistics7Repository<'_>, ContextError>;
    type ExtraOperationsLogistics8Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_operations_logistics_8_repository(&self) -> Result<Self::ExtraOperationsLogistics8Repository<'_>, ContextError>;
    type ExtraOperationsLogistics9Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_operations_logistics_9_repository(&self) -> Result<Self::ExtraOperationsLogistics9Repository<'_>, ContextError>;
    type EmployeeAvailabilityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn employee_availability_repository(&self) -> Result<Self::EmployeeAvailabilityRepository<'_>, ContextError>;
    type PayrollDeductionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payroll_deduction_repository(&self) -> Result<Self::PayrollDeductionRepository<'_>, ContextError>;
    type TrainingSessionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn training_session_repository(&self) -> Result<Self::TrainingSessionRepository<'_>, ContextError>;
    type ShiftAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn shift_assignment_repository(&self) -> Result<Self::ShiftAssignmentRepository<'_>, ContextError>;
    type ExtraEmployeesPayroll1Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_employees_payroll_1_repository(&self) -> Result<Self::ExtraEmployeesPayroll1Repository<'_>, ContextError>;
    type ExtraEmployeesPayroll2Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_employees_payroll_2_repository(&self) -> Result<Self::ExtraEmployeesPayroll2Repository<'_>, ContextError>;
    type ExtraEmployeesPayroll3Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_employees_payroll_3_repository(&self) -> Result<Self::ExtraEmployeesPayroll3Repository<'_>, ContextError>;
    type ExtraEmployeesPayroll4Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_employees_payroll_4_repository(&self) -> Result<Self::ExtraEmployeesPayroll4Repository<'_>, ContextError>;
    type ExtraEmployeesPayroll5Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_employees_payroll_5_repository(&self) -> Result<Self::ExtraEmployeesPayroll5Repository<'_>, ContextError>;
    type ExtraEmployeesPayroll6Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_employees_payroll_6_repository(&self) -> Result<Self::ExtraEmployeesPayroll6Repository<'_>, ContextError>;
    type ExtraEmployeesPayroll7Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_employees_payroll_7_repository(&self) -> Result<Self::ExtraEmployeesPayroll7Repository<'_>, ContextError>;
    type CustomerComplaintRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_complaint_repository(&self) -> Result<Self::CustomerComplaintRepository<'_>, ContextError>;
    type CustomerNoteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_note_repository(&self) -> Result<Self::CustomerNoteRepository<'_>, ContextError>;
    type ExtraCustomerManagement1Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_customer_management_1_repository(&self) -> Result<Self::ExtraCustomerManagement1Repository<'_>, ContextError>;
    type ExtraCustomerManagement2Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_customer_management_2_repository(&self) -> Result<Self::ExtraCustomerManagement2Repository<'_>, ContextError>;
    type ExtraCustomerManagement3Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_customer_management_3_repository(&self) -> Result<Self::ExtraCustomerManagement3Repository<'_>, ContextError>;
    type ExtraCustomerManagement4Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_customer_management_4_repository(&self) -> Result<Self::ExtraCustomerManagement4Repository<'_>, ContextError>;
    type ExtraCustomerManagement5Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_customer_management_5_repository(&self) -> Result<Self::ExtraCustomerManagement5Repository<'_>, ContextError>;
    type ExtraCustomerManagement6Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_customer_management_6_repository(&self) -> Result<Self::ExtraCustomerManagement6Repository<'_>, ContextError>;
    type StorageServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn storage_service_repository(&self) -> Result<Self::StorageServiceRepository<'_>, ContextError>;
    type PackingServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn packing_service_repository(&self) -> Result<Self::PackingServiceRepository<'_>, ContextError>;
    type DisposalServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn disposal_service_repository(&self) -> Result<Self::DisposalServiceRepository<'_>, ContextError>;
    type RentalPeriodRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn rental_period_repository(&self) -> Result<Self::RentalPeriodRepository<'_>, ContextError>;
    type ServiceAreaRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_area_repository(&self) -> Result<Self::ServiceAreaRepository<'_>, ContextError>;
    type ExtraProductsServices1Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_products_services_1_repository(&self) -> Result<Self::ExtraProductsServices1Repository<'_>, ContextError>;
    type ExtraProductsServices2Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_products_services_2_repository(&self) -> Result<Self::ExtraProductsServices2Repository<'_>, ContextError>;
    type ExtraProductsServices3Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_products_services_3_repository(&self) -> Result<Self::ExtraProductsServices3Repository<'_>, ContextError>;
    type ExtraProductsServices4Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_products_services_4_repository(&self) -> Result<Self::ExtraProductsServices4Repository<'_>, ContextError>;
    type CampaignAudienceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn campaign_audience_repository(&self) -> Result<Self::CampaignAudienceRepository<'_>, ContextError>;
    type CampaignChannelRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn campaign_channel_repository(&self) -> Result<Self::CampaignChannelRepository<'_>, ContextError>;
    type LeadAttributionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn lead_attribution_repository(&self) -> Result<Self::LeadAttributionRepository<'_>, ContextError>;
    type SalesFunnelRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn sales_funnel_repository(&self) -> Result<Self::SalesFunnelRepository<'_>, ContextError>;
    type ExtraMarketingSales1Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_marketing_sales_1_repository(&self) -> Result<Self::ExtraMarketingSales1Repository<'_>, ContextError>;
    type ExtraMarketingSales2Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_marketing_sales_2_repository(&self) -> Result<Self::ExtraMarketingSales2Repository<'_>, ContextError>;
    type ExtraMarketingSales3Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_marketing_sales_3_repository(&self) -> Result<Self::ExtraMarketingSales3Repository<'_>, ContextError>;
    type ExtraMarketingSales4Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_marketing_sales_4_repository(&self) -> Result<Self::ExtraMarketingSales4Repository<'_>, ContextError>;
    type ExpenseClaimRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn expense_claim_repository(&self) -> Result<Self::ExpenseClaimRepository<'_>, ContextError>;
    type SettlementRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn settlement_repository(&self) -> Result<Self::SettlementRepository<'_>, ContextError>;
    type ReceivableRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn receivable_repository(&self) -> Result<Self::ReceivableRepository<'_>, ContextError>;
    type PayableRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payable_repository(&self) -> Result<Self::PayableRepository<'_>, ContextError>;
    type ExtraFinanceAccounting1Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_finance_accounting_1_repository(&self) -> Result<Self::ExtraFinanceAccounting1Repository<'_>, ContextError>;
    type ExtraFinanceAccounting2Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_finance_accounting_2_repository(&self) -> Result<Self::ExtraFinanceAccounting2Repository<'_>, ContextError>;
    type ExtraFinanceAccounting3Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_finance_accounting_3_repository(&self) -> Result<Self::ExtraFinanceAccounting3Repository<'_>, ContextError>;
    type ExtraFinanceAccounting4Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_finance_accounting_4_repository(&self) -> Result<Self::ExtraFinanceAccounting4Repository<'_>, ContextError>;
    type VehicleInspectionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_inspection_repository(&self) -> Result<Self::VehicleInspectionRepository<'_>, ContextError>;
    type EquipmentCheckoutRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn equipment_checkout_repository(&self) -> Result<Self::EquipmentCheckoutRepository<'_>, ContextError>;
    type ConsumableReorderRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn consumable_reorder_repository(&self) -> Result<Self::ConsumableReorderRepository<'_>, ContextError>;
    type ExtraAssetManagement1Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_asset_management_1_repository(&self) -> Result<Self::ExtraAssetManagement1Repository<'_>, ContextError>;
    type ExtraAssetManagement2Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_asset_management_2_repository(&self) -> Result<Self::ExtraAssetManagement2Repository<'_>, ContextError>;
    type ExtraAssetManagement3Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_asset_management_3_repository(&self) -> Result<Self::ExtraAssetManagement3Repository<'_>, ContextError>;
    type ExtraAssetManagement4Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_asset_management_4_repository(&self) -> Result<Self::ExtraAssetManagement4Repository<'_>, ContextError>;
    type ExtraAssetManagement5Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_asset_management_5_repository(&self) -> Result<Self::ExtraAssetManagement5Repository<'_>, ContextError>;
    type AuthenticationAttemptRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn authentication_attempt_repository(&self) -> Result<Self::AuthenticationAttemptRepository<'_>, ContextError>;
    type AccessPolicyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn access_policy_repository(&self) -> Result<Self::AccessPolicyRepository<'_>, ContextError>;
    type ExtraIdentityAccess1Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_identity_access_1_repository(&self) -> Result<Self::ExtraIdentityAccess1Repository<'_>, ContextError>;
    type AuditExportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn audit_export_repository(&self) -> Result<Self::AuditExportRepository<'_>, ContextError>;
    type ExtraActivityAudit1Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_activity_audit_1_repository(&self) -> Result<Self::ExtraActivityAudit1Repository<'_>, ContextError>;
    type NotificationPreferenceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn notification_preference_repository(&self) -> Result<Self::NotificationPreferenceRepository<'_>, ContextError>;
    type NotificationDeliveryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn notification_delivery_repository(&self) -> Result<Self::NotificationDeliveryRepository<'_>, ContextError>;
    type SynchronizationRunRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn synchronization_run_repository(&self) -> Result<Self::SynchronizationRunRepository<'_>, ContextError>;
    type ExtraApiIntegrations1Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn extra_api_integrations_1_repository(&self) -> Result<Self::ExtraApiIntegrations1Repository<'_>, ContextError>;
    type GenderTypeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn gender_type_repository(&self) -> Result<Self::GenderTypeRepository<'_>, ContextError>;
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
    type PlatformRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn platform_repository(&self) -> Result<Self::PlatformRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Platform")
    }

    type MerchantRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn merchant_repository(&self) -> Result<Self::MerchantRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Merchant")
    }

    type EmployeeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn employee_repository(&self) -> Result<Self::EmployeeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Employee")
    }

    type PlatformSettingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn platform_setting_repository(&self) -> Result<Self::PlatformSettingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PlatformSetting")
    }

    type PlatformUserRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn platform_user_repository(&self) -> Result<Self::PlatformUserRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PlatformUser")
    }

    type PlatformAuditLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn platform_audit_log_repository(&self) -> Result<Self::PlatformAuditLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PlatformAuditLog")
    }

    type OrganizationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn organization_repository(&self) -> Result<Self::OrganizationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Organization")
    }

    type OrganizationSettingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn organization_setting_repository(&self) -> Result<Self::OrganizationSettingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OrganizationSetting")
    }

    type OrganizationMemberRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn organization_member_repository(&self) -> Result<Self::OrganizationMemberRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OrganizationMember")
    }

    type MoveOrderRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_order_repository(&self) -> Result<Self::MoveOrderRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveOrder")
    }

    type MoveQuoteRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_quote_repository(&self) -> Result<Self::MoveQuoteRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveQuote")
    }

    type RouteRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn route_repository(&self) -> Result<Self::RouteRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Route")
    }

    type RouteStopRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn route_stop_repository(&self) -> Result<Self::RouteStopRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RouteStop")
    }

    type TimeSlotRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn time_slot_repository(&self) -> Result<Self::TimeSlotRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TimeSlot")
    }

    type FulfillmentEventRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fulfillment_event_repository(&self) -> Result<Self::FulfillmentEventRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FulfillmentEvent")
    }

    type AddressRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn address_repository(&self) -> Result<Self::AddressRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Address")
    }

    type CrewRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn crew_repository(&self) -> Result<Self::CrewRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Crew")
    }

    type DispatchAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn dispatch_assignment_repository(&self) -> Result<Self::DispatchAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DispatchAssignment")
    }

    type DamageReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn damage_report_repository(&self) -> Result<Self::DamageReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DamageReport")
    }

    type ProofOfDeliveryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn proof_of_delivery_repository(&self) -> Result<Self::ProofOfDeliveryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ProofOfDelivery")
    }

    type InventoryItemRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn inventory_item_repository(&self) -> Result<Self::InventoryItemRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InventoryItem")
    }

    type PackingListRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn packing_list_repository(&self) -> Result<Self::PackingListRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PackingList")
    }

    type PackingItemRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn packing_item_repository(&self) -> Result<Self::PackingItemRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PackingItem")
    }

    type LoadingPlanRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn loading_plan_repository(&self) -> Result<Self::LoadingPlanRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoadingPlan")
    }

    type UnloadingPlanRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn unloading_plan_repository(&self) -> Result<Self::UnloadingPlanRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UnloadingPlan")
    }

    type StorageFacilityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn storage_facility_repository(&self) -> Result<Self::StorageFacilityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("StorageFacility")
    }

    type StorageUnitRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn storage_unit_repository(&self) -> Result<Self::StorageUnitRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("StorageUnit")
    }

    type StorageInventoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn storage_inventory_repository(&self) -> Result<Self::StorageInventoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("StorageInventory")
    }

    type TransportManifestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn transport_manifest_repository(&self) -> Result<Self::TransportManifestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TransportManifest")
    }

    type CustomsDeclarationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customs_declaration_repository(&self) -> Result<Self::CustomsDeclarationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomsDeclaration")
    }

    type EquipmentChecklistRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn equipment_checklist_repository(&self) -> Result<Self::EquipmentChecklistRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EquipmentChecklist")
    }

    type FuelLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fuel_log_repository(&self) -> Result<Self::FuelLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FuelLog")
    }

    type MaintenanceRequestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn maintenance_request_repository(&self) -> Result<Self::MaintenanceRequestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MaintenanceRequest")
    }

    type DepartmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn department_repository(&self) -> Result<Self::DepartmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Department")
    }

    type JobAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn job_assignment_repository(&self) -> Result<Self::JobAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("JobAssignment")
    }

    type WorkShiftRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn work_shift_repository(&self) -> Result<Self::WorkShiftRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("WorkShift")
    }

    type WorkedHoursRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn worked_hours_repository(&self) -> Result<Self::WorkedHoursRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("WorkedHours")
    }

    type PayrollPeriodRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payroll_period_repository(&self) -> Result<Self::PayrollPeriodRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PayrollPeriod")
    }

    type PayrollCalculationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payroll_calculation_repository(&self) -> Result<Self::PayrollCalculationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PayrollCalculation")
    }

    type PayslipRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payslip_repository(&self) -> Result<Self::PayslipRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Payslip")
    }

    type BonusRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn bonus_repository(&self) -> Result<Self::BonusRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Bonus")
    }

    type EmployeeCertificationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn employee_certification_repository(&self) -> Result<Self::EmployeeCertificationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EmployeeCertification")
    }

    type LeaveRequestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn leave_request_repository(&self) -> Result<Self::LeaveRequestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LeaveRequest")
    }

    type BillingProfileRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn billing_profile_repository(&self) -> Result<Self::BillingProfileRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BillingProfile")
    }

    type CorporateCustomerProfileRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn corporate_customer_profile_repository(&self) -> Result<Self::CorporateCustomerProfileRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CorporateCustomerProfile")
    }

    type CustomerRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_repository(&self) -> Result<Self::CustomerRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Customer")
    }

    type CustomerConsentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_consent_repository(&self) -> Result<Self::CustomerConsentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerConsent")
    }

    type CustomerContactRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_contact_repository(&self) -> Result<Self::CustomerContactRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerContact")
    }

    type CustomerHistoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_history_repository(&self) -> Result<Self::CustomerHistoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerHistory")
    }

    type CustomerPreferenceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_preference_repository(&self) -> Result<Self::CustomerPreferenceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerPreference")
    }

    type PrivateCustomerProfileRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn private_customer_profile_repository(&self) -> Result<Self::PrivateCustomerProfileRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PrivateCustomerProfile")
    }

    type BoxRentalRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn box_rental_repository(&self) -> Result<Self::BoxRentalRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BoxRental")
    }

    type CleaningServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn cleaning_service_repository(&self) -> Result<Self::CleaningServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CleaningService")
    }

    type MovingServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn moving_service_repository(&self) -> Result<Self::MovingServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MovingService")
    }

    type PriceListRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn price_list_repository(&self) -> Result<Self::PriceListRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PriceList")
    }

    type ProductRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn product_repository(&self) -> Result<Self::ProductRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Product")
    }

    type ServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_repository(&self) -> Result<Self::ServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Service")
    }

    type ServiceBundleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_bundle_repository(&self) -> Result<Self::ServiceBundleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceBundle")
    }

    type ServiceConfigurationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_configuration_repository(&self) -> Result<Self::ServiceConfigurationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceConfiguration")
    }

    type ServicePriceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_price_repository(&self) -> Result<Self::ServicePriceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServicePrice")
    }

    type CampaignRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn campaign_repository(&self) -> Result<Self::CampaignRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Campaign")
    }

    type ConversionEventRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn conversion_event_repository(&self) -> Result<Self::ConversionEventRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ConversionEvent")
    }

    type ConversionMetricRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn conversion_metric_repository(&self) -> Result<Self::ConversionMetricRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ConversionMetric")
    }

    type DiscountCodeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn discount_code_repository(&self) -> Result<Self::DiscountCodeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DiscountCode")
    }

    type LeadRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn lead_repository(&self) -> Result<Self::LeadRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Lead")
    }

    type LeadActivityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn lead_activity_repository(&self) -> Result<Self::LeadActivityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LeadActivity")
    }

    type SalesOpportunityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn sales_opportunity_repository(&self) -> Result<Self::SalesOpportunityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SalesOpportunity")
    }

    type AccountRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn account_repository(&self) -> Result<Self::AccountRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Account")
    }

    type ExpenseRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn expense_repository(&self) -> Result<Self::ExpenseRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Expense")
    }

    type FinancialSummaryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn financial_summary_repository(&self) -> Result<Self::FinancialSummaryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FinancialSummary")
    }

    type InvoiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn invoice_repository(&self) -> Result<Self::InvoiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Invoice")
    }

    type InvoiceLineRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn invoice_line_repository(&self) -> Result<Self::InvoiceLineRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InvoiceLine")
    }

    type JournalEntryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn journal_entry_repository(&self) -> Result<Self::JournalEntryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("JournalEntry")
    }

    type PaymentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payment_repository(&self) -> Result<Self::PaymentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Payment")
    }

    type RefundRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn refund_repository(&self) -> Result<Self::RefundRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Refund")
    }

    type VatRateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vat_rate_repository(&self) -> Result<Self::VatRateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VatRate")
    }

    type AssetAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn asset_assignment_repository(&self) -> Result<Self::AssetAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AssetAssignment")
    }

    type AssetInspectionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn asset_inspection_repository(&self) -> Result<Self::AssetInspectionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AssetInspection")
    }

    type ConsumableRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn consumable_repository(&self) -> Result<Self::ConsumableRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Consumable")
    }

    type EquipmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn equipment_repository(&self) -> Result<Self::EquipmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Equipment")
    }

    type FuelRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fuel_record_repository(&self) -> Result<Self::FuelRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FuelRecord")
    }

    type MaintenanceEventRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn maintenance_event_repository(&self) -> Result<Self::MaintenanceEventRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MaintenanceEvent")
    }

    type MaintenanceScheduleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn maintenance_schedule_repository(&self) -> Result<Self::MaintenanceScheduleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MaintenanceSchedule")
    }

    type SupplierRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn supplier_repository(&self) -> Result<Self::SupplierRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Supplier")
    }

    type VehicleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_repository(&self) -> Result<Self::VehicleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Vehicle")
    }

    type ComplianceCheckRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn compliance_check_repository(&self) -> Result<Self::ComplianceCheckRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ComplianceCheck")
    }

    type ContractRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn contract_repository(&self) -> Result<Self::ContractRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Contract")
    }

    type DataRetentionPolicyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn data_retention_policy_repository(&self) -> Result<Self::DataRetentionPolicyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DataRetentionPolicy")
    }

    type DocumentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn document_repository(&self) -> Result<Self::DocumentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Document")
    }

    type DocumentVersionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn document_version_repository(&self) -> Result<Self::DocumentVersionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DocumentVersion")
    }

    type InsuranceClaimRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn insurance_claim_repository(&self) -> Result<Self::InsuranceClaimRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InsuranceClaim")
    }

    type InsurancePolicyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn insurance_policy_repository(&self) -> Result<Self::InsurancePolicyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InsurancePolicy")
    }

    type RecoveryRequestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn recovery_request_repository(&self) -> Result<Self::RecoveryRequestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RecoveryRequest")
    }

    type MagicLinkRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn magic_link_repository(&self) -> Result<Self::MagicLinkRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MagicLink")
    }

    type PermissionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn permission_repository(&self) -> Result<Self::PermissionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Permission")
    }

    type RoleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn role_repository(&self) -> Result<Self::RoleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Role")
    }

    type RolePermissionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn role_permission_repository(&self) -> Result<Self::RolePermissionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RolePermission")
    }

    type UserAccountRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn user_account_repository(&self) -> Result<Self::UserAccountRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UserAccount")
    }

    type UserRoleAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn user_role_assignment_repository(&self) -> Result<Self::UserRoleAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UserRoleAssignment")
    }

    type UserSessionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn user_session_repository(&self) -> Result<Self::UserSessionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UserSession")
    }

    type ActivityLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn activity_log_repository(&self) -> Result<Self::ActivityLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ActivityLog")
    }

    type AuditLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn audit_log_repository(&self) -> Result<Self::AuditLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AuditLog")
    }

    type ChangeSetRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn change_set_repository(&self) -> Result<Self::ChangeSetRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ChangeSet")
    }

    type EntityChangeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn entity_change_repository(&self) -> Result<Self::EntityChangeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EntityChange")
    }

    type AutomationActionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn automation_action_repository(&self) -> Result<Self::AutomationActionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AutomationAction")
    }

    type AutomationRuleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn automation_rule_repository(&self) -> Result<Self::AutomationRuleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AutomationRule")
    }

    type AutomationTriggerRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn automation_trigger_repository(&self) -> Result<Self::AutomationTriggerRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AutomationTrigger")
    }

    type NotificationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn notification_repository(&self) -> Result<Self::NotificationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Notification")
    }

    type NotificationTemplateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn notification_template_repository(&self) -> Result<Self::NotificationTemplateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("NotificationTemplate")
    }

    type ApiClientRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn api_client_repository(&self) -> Result<Self::ApiClientRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ApiClient")
    }

    type ApiEndpointRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn api_endpoint_repository(&self) -> Result<Self::ApiEndpointRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ApiEndpoint")
    }

    type IntegrationMappingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn integration_mapping_repository(&self) -> Result<Self::IntegrationMappingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("IntegrationMapping")
    }

    type WebhookRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn webhook_repository(&self) -> Result<Self::WebhookRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Webhook")
    }

    type WebhookDeliveryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn webhook_delivery_repository(&self) -> Result<Self::WebhookDeliveryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("WebhookDelivery")
    }

    type PlatformConfigurationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn platform_configuration_repository(&self) -> Result<Self::PlatformConfigurationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PlatformConfiguration")
    }

    type PlatformLocaleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn platform_locale_repository(&self) -> Result<Self::PlatformLocaleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PlatformLocale")
    }

    type MerchantBranchRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn merchant_branch_repository(&self) -> Result<Self::MerchantBranchRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MerchantBranch")
    }

    type MerchantSettingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn merchant_setting_repository(&self) -> Result<Self::MerchantSettingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MerchantSetting")
    }

    type OperationalExceptionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn operational_exception_repository(&self) -> Result<Self::OperationalExceptionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OperationalException")
    }

    type CrewMemberAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn crew_member_assignment_repository(&self) -> Result<Self::CrewMemberAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CrewMemberAssignment")
    }

    type PickupInstructionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn pickup_instruction_repository(&self) -> Result<Self::PickupInstructionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PickupInstruction")
    }

    type DeliveryInstructionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn delivery_instruction_repository(&self) -> Result<Self::DeliveryInstructionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DeliveryInstruction")
    }

    type MoveInventoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_inventory_repository(&self) -> Result<Self::MoveInventoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveInventory")
    }

    type ExtraOperationsLogistics1Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_operations_logistics_1_repository(&self) -> Result<Self::ExtraOperationsLogistics1Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraOperationsLogistics1")
    }

    type ExtraOperationsLogistics2Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_operations_logistics_2_repository(&self) -> Result<Self::ExtraOperationsLogistics2Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraOperationsLogistics2")
    }

    type ExtraOperationsLogistics3Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_operations_logistics_3_repository(&self) -> Result<Self::ExtraOperationsLogistics3Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraOperationsLogistics3")
    }

    type ExtraOperationsLogistics4Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_operations_logistics_4_repository(&self) -> Result<Self::ExtraOperationsLogistics4Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraOperationsLogistics4")
    }

    type ExtraOperationsLogistics5Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_operations_logistics_5_repository(&self) -> Result<Self::ExtraOperationsLogistics5Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraOperationsLogistics5")
    }

    type ExtraOperationsLogistics6Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_operations_logistics_6_repository(&self) -> Result<Self::ExtraOperationsLogistics6Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraOperationsLogistics6")
    }

    type ExtraOperationsLogistics7Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_operations_logistics_7_repository(&self) -> Result<Self::ExtraOperationsLogistics7Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraOperationsLogistics7")
    }

    type ExtraOperationsLogistics8Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_operations_logistics_8_repository(&self) -> Result<Self::ExtraOperationsLogistics8Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraOperationsLogistics8")
    }

    type ExtraOperationsLogistics9Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_operations_logistics_9_repository(&self) -> Result<Self::ExtraOperationsLogistics9Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraOperationsLogistics9")
    }

    type EmployeeAvailabilityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn employee_availability_repository(&self) -> Result<Self::EmployeeAvailabilityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EmployeeAvailability")
    }

    type PayrollDeductionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payroll_deduction_repository(&self) -> Result<Self::PayrollDeductionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PayrollDeduction")
    }

    type TrainingSessionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn training_session_repository(&self) -> Result<Self::TrainingSessionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TrainingSession")
    }

    type ShiftAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn shift_assignment_repository(&self) -> Result<Self::ShiftAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ShiftAssignment")
    }

    type ExtraEmployeesPayroll1Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_employees_payroll_1_repository(&self) -> Result<Self::ExtraEmployeesPayroll1Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraEmployeesPayroll1")
    }

    type ExtraEmployeesPayroll2Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_employees_payroll_2_repository(&self) -> Result<Self::ExtraEmployeesPayroll2Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraEmployeesPayroll2")
    }

    type ExtraEmployeesPayroll3Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_employees_payroll_3_repository(&self) -> Result<Self::ExtraEmployeesPayroll3Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraEmployeesPayroll3")
    }

    type ExtraEmployeesPayroll4Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_employees_payroll_4_repository(&self) -> Result<Self::ExtraEmployeesPayroll4Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraEmployeesPayroll4")
    }

    type ExtraEmployeesPayroll5Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_employees_payroll_5_repository(&self) -> Result<Self::ExtraEmployeesPayroll5Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraEmployeesPayroll5")
    }

    type ExtraEmployeesPayroll6Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_employees_payroll_6_repository(&self) -> Result<Self::ExtraEmployeesPayroll6Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraEmployeesPayroll6")
    }

    type ExtraEmployeesPayroll7Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_employees_payroll_7_repository(&self) -> Result<Self::ExtraEmployeesPayroll7Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraEmployeesPayroll7")
    }

    type CustomerComplaintRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_complaint_repository(&self) -> Result<Self::CustomerComplaintRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerComplaint")
    }

    type CustomerNoteRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_note_repository(&self) -> Result<Self::CustomerNoteRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerNote")
    }

    type ExtraCustomerManagement1Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_customer_management_1_repository(&self) -> Result<Self::ExtraCustomerManagement1Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraCustomerManagement1")
    }

    type ExtraCustomerManagement2Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_customer_management_2_repository(&self) -> Result<Self::ExtraCustomerManagement2Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraCustomerManagement2")
    }

    type ExtraCustomerManagement3Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_customer_management_3_repository(&self) -> Result<Self::ExtraCustomerManagement3Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraCustomerManagement3")
    }

    type ExtraCustomerManagement4Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_customer_management_4_repository(&self) -> Result<Self::ExtraCustomerManagement4Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraCustomerManagement4")
    }

    type ExtraCustomerManagement5Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_customer_management_5_repository(&self) -> Result<Self::ExtraCustomerManagement5Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraCustomerManagement5")
    }

    type ExtraCustomerManagement6Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_customer_management_6_repository(&self) -> Result<Self::ExtraCustomerManagement6Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraCustomerManagement6")
    }

    type StorageServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn storage_service_repository(&self) -> Result<Self::StorageServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("StorageService")
    }

    type PackingServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn packing_service_repository(&self) -> Result<Self::PackingServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PackingService")
    }

    type DisposalServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn disposal_service_repository(&self) -> Result<Self::DisposalServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DisposalService")
    }

    type RentalPeriodRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn rental_period_repository(&self) -> Result<Self::RentalPeriodRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RentalPeriod")
    }

    type ServiceAreaRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_area_repository(&self) -> Result<Self::ServiceAreaRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceArea")
    }

    type ExtraProductsServices1Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_products_services_1_repository(&self) -> Result<Self::ExtraProductsServices1Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraProductsServices1")
    }

    type ExtraProductsServices2Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_products_services_2_repository(&self) -> Result<Self::ExtraProductsServices2Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraProductsServices2")
    }

    type ExtraProductsServices3Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_products_services_3_repository(&self) -> Result<Self::ExtraProductsServices3Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraProductsServices3")
    }

    type ExtraProductsServices4Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_products_services_4_repository(&self) -> Result<Self::ExtraProductsServices4Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraProductsServices4")
    }

    type CampaignAudienceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn campaign_audience_repository(&self) -> Result<Self::CampaignAudienceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CampaignAudience")
    }

    type CampaignChannelRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn campaign_channel_repository(&self) -> Result<Self::CampaignChannelRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CampaignChannel")
    }

    type LeadAttributionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn lead_attribution_repository(&self) -> Result<Self::LeadAttributionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LeadAttribution")
    }

    type SalesFunnelRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn sales_funnel_repository(&self) -> Result<Self::SalesFunnelRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SalesFunnel")
    }

    type ExtraMarketingSales1Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_marketing_sales_1_repository(&self) -> Result<Self::ExtraMarketingSales1Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraMarketingSales1")
    }

    type ExtraMarketingSales2Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_marketing_sales_2_repository(&self) -> Result<Self::ExtraMarketingSales2Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraMarketingSales2")
    }

    type ExtraMarketingSales3Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_marketing_sales_3_repository(&self) -> Result<Self::ExtraMarketingSales3Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraMarketingSales3")
    }

    type ExtraMarketingSales4Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_marketing_sales_4_repository(&self) -> Result<Self::ExtraMarketingSales4Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraMarketingSales4")
    }

    type ExpenseClaimRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn expense_claim_repository(&self) -> Result<Self::ExpenseClaimRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExpenseClaim")
    }

    type SettlementRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn settlement_repository(&self) -> Result<Self::SettlementRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Settlement")
    }

    type ReceivableRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn receivable_repository(&self) -> Result<Self::ReceivableRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Receivable")
    }

    type PayableRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payable_repository(&self) -> Result<Self::PayableRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Payable")
    }

    type ExtraFinanceAccounting1Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_finance_accounting_1_repository(&self) -> Result<Self::ExtraFinanceAccounting1Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraFinanceAccounting1")
    }

    type ExtraFinanceAccounting2Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_finance_accounting_2_repository(&self) -> Result<Self::ExtraFinanceAccounting2Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraFinanceAccounting2")
    }

    type ExtraFinanceAccounting3Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_finance_accounting_3_repository(&self) -> Result<Self::ExtraFinanceAccounting3Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraFinanceAccounting3")
    }

    type ExtraFinanceAccounting4Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_finance_accounting_4_repository(&self) -> Result<Self::ExtraFinanceAccounting4Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraFinanceAccounting4")
    }

    type VehicleInspectionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_inspection_repository(&self) -> Result<Self::VehicleInspectionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleInspection")
    }

    type EquipmentCheckoutRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn equipment_checkout_repository(&self) -> Result<Self::EquipmentCheckoutRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EquipmentCheckout")
    }

    type ConsumableReorderRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn consumable_reorder_repository(&self) -> Result<Self::ConsumableReorderRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ConsumableReorder")
    }

    type ExtraAssetManagement1Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_asset_management_1_repository(&self) -> Result<Self::ExtraAssetManagement1Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraAssetManagement1")
    }

    type ExtraAssetManagement2Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_asset_management_2_repository(&self) -> Result<Self::ExtraAssetManagement2Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraAssetManagement2")
    }

    type ExtraAssetManagement3Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_asset_management_3_repository(&self) -> Result<Self::ExtraAssetManagement3Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraAssetManagement3")
    }

    type ExtraAssetManagement4Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_asset_management_4_repository(&self) -> Result<Self::ExtraAssetManagement4Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraAssetManagement4")
    }

    type ExtraAssetManagement5Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_asset_management_5_repository(&self) -> Result<Self::ExtraAssetManagement5Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraAssetManagement5")
    }

    type AuthenticationAttemptRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn authentication_attempt_repository(&self) -> Result<Self::AuthenticationAttemptRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AuthenticationAttempt")
    }

    type AccessPolicyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn access_policy_repository(&self) -> Result<Self::AccessPolicyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AccessPolicy")
    }

    type ExtraIdentityAccess1Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_identity_access_1_repository(&self) -> Result<Self::ExtraIdentityAccess1Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraIdentityAccess1")
    }

    type AuditExportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn audit_export_repository(&self) -> Result<Self::AuditExportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AuditExport")
    }

    type ExtraActivityAudit1Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_activity_audit_1_repository(&self) -> Result<Self::ExtraActivityAudit1Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraActivityAudit1")
    }

    type NotificationPreferenceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn notification_preference_repository(&self) -> Result<Self::NotificationPreferenceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("NotificationPreference")
    }

    type NotificationDeliveryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn notification_delivery_repository(&self) -> Result<Self::NotificationDeliveryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("NotificationDelivery")
    }

    type SynchronizationRunRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn synchronization_run_repository(&self) -> Result<Self::SynchronizationRunRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SynchronizationRun")
    }

    type ExtraApiIntegrations1Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn extra_api_integrations_1_repository(&self) -> Result<Self::ExtraApiIntegrations1Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExtraApiIntegrations1")
    }

    type GenderTypeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn gender_type_repository(&self) -> Result<Self::GenderTypeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("GenderType")
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
