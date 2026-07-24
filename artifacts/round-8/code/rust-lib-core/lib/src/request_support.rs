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
    type TenantConfigurationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tenant_configuration_repository(&self) -> Result<Self::TenantConfigurationRepository<'_>, ContextError>;
    type OrganizationUnitRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn organization_unit_repository(&self) -> Result<Self::OrganizationUnitRepository<'_>, ContextError>;
    type DepartmentHierarchyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn department_hierarchy_repository(&self) -> Result<Self::DepartmentHierarchyRepository<'_>, ContextError>;
    type BranchOfficeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn branch_office_repository(&self) -> Result<Self::BranchOfficeRepository<'_>, ContextError>;
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
    type MoveItemRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_item_repository(&self) -> Result<Self::MoveItemRepository<'_>, ContextError>;
    type InventoryListRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn inventory_list_repository(&self) -> Result<Self::InventoryListRepository<'_>, ContextError>;
    type PackingMaterialRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn packing_material_repository(&self) -> Result<Self::PackingMaterialRepository<'_>, ContextError>;
    type LoadingZoneRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn loading_zone_repository(&self) -> Result<Self::LoadingZoneRepository<'_>, ContextError>;
    type UnloadingZoneRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn unloading_zone_repository(&self) -> Result<Self::UnloadingZoneRepository<'_>, ContextError>;
    type TransitLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn transit_log_repository(&self) -> Result<Self::TransitLogRepository<'_>, ContextError>;
    type DelayRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn delay_record_repository(&self) -> Result<Self::DelayRecordRepository<'_>, ContextError>;
    type RouteOptimizationRuleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn route_optimization_rule_repository(&self) -> Result<Self::RouteOptimizationRuleRepository<'_>, ContextError>;
    type VehicleAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_assignment_repository(&self) -> Result<Self::VehicleAssignmentRepository<'_>, ContextError>;
    type CargoWeightRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn cargo_weight_record_repository(&self) -> Result<Self::CargoWeightRecordRepository<'_>, ContextError>;
    type SpecialHandlingInstructionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn special_handling_instruction_repository(&self) -> Result<Self::SpecialHandlingInstructionRepository<'_>, ContextError>;
    type MoveStatusRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_status_repository(&self) -> Result<Self::MoveStatusRepository<'_>, ContextError>;
    type DeliveryWindowRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn delivery_window_repository(&self) -> Result<Self::DeliveryWindowRepository<'_>, ContextError>;
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
    type DeductionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn deduction_repository(&self) -> Result<Self::DeductionRepository<'_>, ContextError>;
    type LeaveRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn leave_request_repository(&self) -> Result<Self::LeaveRequestRepository<'_>, ContextError>;
    type EmployeeCertificationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn employee_certification_repository(&self) -> Result<Self::EmployeeCertificationRepository<'_>, ContextError>;
    type TrainingModuleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn training_module_repository(&self) -> Result<Self::TrainingModuleRepository<'_>, ContextError>;
    type AvailabilityScheduleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn availability_schedule_repository(&self) -> Result<Self::AvailabilityScheduleRepository<'_>, ContextError>;
    type SkillProfileRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn skill_profile_repository(&self) -> Result<Self::SkillProfileRepository<'_>, ContextError>;
    type PerformanceReviewRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn performance_review_repository(&self) -> Result<Self::PerformanceReviewRepository<'_>, ContextError>;
    type OvertimeRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn overtime_record_repository(&self) -> Result<Self::OvertimeRecordRepository<'_>, ContextError>;
    type TaxWithholdingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tax_withholding_repository(&self) -> Result<Self::TaxWithholdingRepository<'_>, ContextError>;
    type BenefitEnrollmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn benefit_enrollment_repository(&self) -> Result<Self::BenefitEnrollmentRepository<'_>, ContextError>;
    type ShiftSwapRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn shift_swap_request_repository(&self) -> Result<Self::ShiftSwapRequestRepository<'_>, ContextError>;
    type AttendanceRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn attendance_record_repository(&self) -> Result<Self::AttendanceRecordRepository<'_>, ContextError>;
    type PayrollAdjustmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payroll_adjustment_repository(&self) -> Result<Self::PayrollAdjustmentRepository<'_>, ContextError>;
    type CommissionRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn commission_record_repository(&self) -> Result<Self::CommissionRecordRepository<'_>, ContextError>;
    type CustomerRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_repository(&self) -> Result<Self::CustomerRepository<'_>, ContextError>;
    type PrivateCustomerProfileRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn private_customer_profile_repository(&self) -> Result<Self::PrivateCustomerProfileRepository<'_>, ContextError>;
    type CorporateCustomerProfileRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn corporate_customer_profile_repository(&self) -> Result<Self::CorporateCustomerProfileRepository<'_>, ContextError>;
    type CustomerContactRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_contact_repository(&self) -> Result<Self::CustomerContactRepository<'_>, ContextError>;
    type BillingProfileRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn billing_profile_repository(&self) -> Result<Self::BillingProfileRepository<'_>, ContextError>;
    type CustomerHistoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_history_repository(&self) -> Result<Self::CustomerHistoryRepository<'_>, ContextError>;
    type CustomerPreferenceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_preference_repository(&self) -> Result<Self::CustomerPreferenceRepository<'_>, ContextError>;
    type CustomerConsentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_consent_repository(&self) -> Result<Self::CustomerConsentRepository<'_>, ContextError>;
    type CustomerFeedbackRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_feedback_repository(&self) -> Result<Self::CustomerFeedbackRepository<'_>, ContextError>;
    type LoyaltyTierRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn loyalty_tier_repository(&self) -> Result<Self::LoyaltyTierRepository<'_>, ContextError>;
    type ReferralCodeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn referral_code_repository(&self) -> Result<Self::ReferralCodeRepository<'_>, ContextError>;
    type CommunicationLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn communication_log_repository(&self) -> Result<Self::CommunicationLogRepository<'_>, ContextError>;
    type ServiceRatingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_rating_repository(&self) -> Result<Self::ServiceRatingRepository<'_>, ContextError>;
    type AccountStatusRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn account_status_repository(&self) -> Result<Self::AccountStatusRepository<'_>, ContextError>;
    type ContactMethodRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn contact_method_repository(&self) -> Result<Self::ContactMethodRepository<'_>, ContextError>;
    type CustomerSegmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_segment_repository(&self) -> Result<Self::CustomerSegmentRepository<'_>, ContextError>;
    type ProductRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn product_repository(&self) -> Result<Self::ProductRepository<'_>, ContextError>;
    type ServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_repository(&self) -> Result<Self::ServiceRepository<'_>, ContextError>;
    type MovingServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn moving_service_repository(&self) -> Result<Self::MovingServiceRepository<'_>, ContextError>;
    type CleaningServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn cleaning_service_repository(&self) -> Result<Self::CleaningServiceRepository<'_>, ContextError>;
    type BoxRentalRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn box_rental_repository(&self) -> Result<Self::BoxRentalRepository<'_>, ContextError>;
    type ServiceConfigurationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_configuration_repository(&self) -> Result<Self::ServiceConfigurationRepository<'_>, ContextError>;
    type PriceListRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn price_list_repository(&self) -> Result<Self::PriceListRepository<'_>, ContextError>;
    type ServicePriceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_price_repository(&self) -> Result<Self::ServicePriceRepository<'_>, ContextError>;
    type ServiceBundleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_bundle_repository(&self) -> Result<Self::ServiceBundleRepository<'_>, ContextError>;
    type StorageUnitRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn storage_unit_repository(&self) -> Result<Self::StorageUnitRepository<'_>, ContextError>;
    type PackingKitRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn packing_kit_repository(&self) -> Result<Self::PackingKitRepository<'_>, ContextError>;
    type DisposalServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn disposal_service_repository(&self) -> Result<Self::DisposalServiceRepository<'_>, ContextError>;
    type ServiceAreaRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_area_repository(&self) -> Result<Self::ServiceAreaRepository<'_>, ContextError>;
    type AvailabilityCalendarRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn availability_calendar_repository(&self) -> Result<Self::AvailabilityCalendarRepository<'_>, ContextError>;
    type ServiceLevelAgreementRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_level_agreement_repository(&self) -> Result<Self::ServiceLevelAgreementRepository<'_>, ContextError>;
    type AddOnServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn add_on_service_repository(&self) -> Result<Self::AddOnServiceRepository<'_>, ContextError>;
    type InventoryItemRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn inventory_item_repository(&self) -> Result<Self::InventoryItemRepository<'_>, ContextError>;
    type ServiceCategoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_category_repository(&self) -> Result<Self::ServiceCategoryRepository<'_>, ContextError>;
    type CampaignRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn campaign_repository(&self) -> Result<Self::CampaignRepository<'_>, ContextError>;
    type DiscountCodeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn discount_code_repository(&self) -> Result<Self::DiscountCodeRepository<'_>, ContextError>;
    type LeadRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn lead_repository(&self) -> Result<Self::LeadRepository<'_>, ContextError>;
    type SalesOpportunityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn sales_opportunity_repository(&self) -> Result<Self::SalesOpportunityRepository<'_>, ContextError>;
    type LeadActivityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn lead_activity_repository(&self) -> Result<Self::LeadActivityRepository<'_>, ContextError>;
    type ConversionEventRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn conversion_event_repository(&self) -> Result<Self::ConversionEventRepository<'_>, ContextError>;
    type ConversionMetricRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn conversion_metric_repository(&self) -> Result<Self::ConversionMetricRepository<'_>, ContextError>;
    type MarketingChannelRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn marketing_channel_repository(&self) -> Result<Self::MarketingChannelRepository<'_>, ContextError>;
    type AudienceSegmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn audience_segment_repository(&self) -> Result<Self::AudienceSegmentRepository<'_>, ContextError>;
    type PromotionalOfferRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn promotional_offer_repository(&self) -> Result<Self::PromotionalOfferRepository<'_>, ContextError>;
    type SalesFunnelRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn sales_funnel_repository(&self) -> Result<Self::SalesFunnelRepository<'_>, ContextError>;
    type AttributionModelRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn attribution_model_repository(&self) -> Result<Self::AttributionModelRepository<'_>, ContextError>;
    type LeadScoreRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn lead_score_repository(&self) -> Result<Self::LeadScoreRepository<'_>, ContextError>;
    type CampaignBudgetRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn campaign_budget_repository(&self) -> Result<Self::CampaignBudgetRepository<'_>, ContextError>;
    type ConversionReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn conversion_report_repository(&self) -> Result<Self::ConversionReportRepository<'_>, ContextError>;
    type PaymentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payment_repository(&self) -> Result<Self::PaymentRepository<'_>, ContextError>;
    type InvoiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn invoice_repository(&self) -> Result<Self::InvoiceRepository<'_>, ContextError>;
    type InvoiceLineRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn invoice_line_repository(&self) -> Result<Self::InvoiceLineRepository<'_>, ContextError>;
    type RefundRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn refund_repository(&self) -> Result<Self::RefundRepository<'_>, ContextError>;
    type ExpenseRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn expense_repository(&self) -> Result<Self::ExpenseRepository<'_>, ContextError>;
    type VatRateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vat_rate_repository(&self) -> Result<Self::VatRateRepository<'_>, ContextError>;
    type JournalEntryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn journal_entry_repository(&self) -> Result<Self::JournalEntryRepository<'_>, ContextError>;
    type AccountRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn account_repository(&self) -> Result<Self::AccountRepository<'_>, ContextError>;
    type FinancialSummaryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn financial_summary_repository(&self) -> Result<Self::FinancialSummaryRepository<'_>, ContextError>;
    type BudgetRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn budget_repository(&self) -> Result<Self::BudgetRepository<'_>, ContextError>;
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
    type TaxRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tax_record_repository(&self) -> Result<Self::TaxRecordRepository<'_>, ContextError>;
    type CurrencyRateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn currency_rate_repository(&self) -> Result<Self::CurrencyRateRepository<'_>, ContextError>;
    type PaymentMethodRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payment_method_repository(&self) -> Result<Self::PaymentMethodRepository<'_>, ContextError>;
    type FinancialPeriodRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn financial_period_repository(&self) -> Result<Self::FinancialPeriodRepository<'_>, ContextError>;
    type VehicleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_repository(&self) -> Result<Self::VehicleRepository<'_>, ContextError>;
    type EquipmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn equipment_repository(&self) -> Result<Self::EquipmentRepository<'_>, ContextError>;
    type ConsumableRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn consumable_repository(&self) -> Result<Self::ConsumableRepository<'_>, ContextError>;
    type AssetAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn asset_assignment_repository(&self) -> Result<Self::AssetAssignmentRepository<'_>, ContextError>;
    type AssetInspectionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn asset_inspection_repository(&self) -> Result<Self::AssetInspectionRepository<'_>, ContextError>;
    type MaintenanceScheduleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn maintenance_schedule_repository(&self) -> Result<Self::MaintenanceScheduleRepository<'_>, ContextError>;
    type MaintenanceEventRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn maintenance_event_repository(&self) -> Result<Self::MaintenanceEventRepository<'_>, ContextError>;
    type FuelRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fuel_record_repository(&self) -> Result<Self::FuelRecordRepository<'_>, ContextError>;
    type SupplierRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn supplier_repository(&self) -> Result<Self::SupplierRepository<'_>, ContextError>;
    type InventoryStockRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn inventory_stock_repository(&self) -> Result<Self::InventoryStockRepository<'_>, ContextError>;
    type MaintenanceCostRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn maintenance_cost_repository(&self) -> Result<Self::MaintenanceCostRepository<'_>, ContextError>;
    type VehicleRegistrationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_registration_repository(&self) -> Result<Self::VehicleRegistrationRepository<'_>, ContextError>;
    type EquipmentSerialRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn equipment_serial_repository(&self) -> Result<Self::EquipmentSerialRepository<'_>, ContextError>;
    type SupplierContractRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn supplier_contract_repository(&self) -> Result<Self::SupplierContractRepository<'_>, ContextError>;
    type AssetConditionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn asset_condition_repository(&self) -> Result<Self::AssetConditionRepository<'_>, ContextError>;
    type DepreciationRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn depreciation_record_repository(&self) -> Result<Self::DepreciationRecordRepository<'_>, ContextError>;
    type WarrantyClaimRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn warranty_claim_repository(&self) -> Result<Self::WarrantyClaimRepository<'_>, ContextError>;
    type StorageLocationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn storage_location_repository(&self) -> Result<Self::StorageLocationRepository<'_>, ContextError>;
    type ContractRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn contract_repository(&self) -> Result<Self::ContractRepository<'_>, ContextError>;
    type InsurancePolicyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn insurance_policy_repository(&self) -> Result<Self::InsurancePolicyRepository<'_>, ContextError>;
    type InsuranceClaimRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn insurance_claim_repository(&self) -> Result<Self::InsuranceClaimRepository<'_>, ContextError>;
    type DocumentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn document_repository(&self) -> Result<Self::DocumentRepository<'_>, ContextError>;
    type DocumentVersionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn document_version_repository(&self) -> Result<Self::DocumentVersionRepository<'_>, ContextError>;
    type ComplianceCheckRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn compliance_check_repository(&self) -> Result<Self::ComplianceCheckRepository<'_>, ContextError>;
    type DataRetentionPolicyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn data_retention_policy_repository(&self) -> Result<Self::DataRetentionPolicyRepository<'_>, ContextError>;
    type RecoveryRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn recovery_request_repository(&self) -> Result<Self::RecoveryRequestRepository<'_>, ContextError>;
    type PolicyDocumentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn policy_document_repository(&self) -> Result<Self::PolicyDocumentRepository<'_>, ContextError>;
    type IncidentReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn incident_report_repository(&self) -> Result<Self::IncidentReportRepository<'_>, ContextError>;
    type AuditTrailRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn audit_trail_repository(&self) -> Result<Self::AuditTrailRepository<'_>, ContextError>;
    type LegalEntityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn legal_entity_repository(&self) -> Result<Self::LegalEntityRepository<'_>, ContextError>;
    type RegulatoryRequirementRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn regulatory_requirement_repository(&self) -> Result<Self::RegulatoryRequirementRepository<'_>, ContextError>;
    type ComplianceCertificateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn compliance_certificate_repository(&self) -> Result<Self::ComplianceCertificateRepository<'_>, ContextError>;
    type UserAccountRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn user_account_repository(&self) -> Result<Self::UserAccountRepository<'_>, ContextError>;
    type RoleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn role_repository(&self) -> Result<Self::RoleRepository<'_>, ContextError>;
    type PermissionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn permission_repository(&self) -> Result<Self::PermissionRepository<'_>, ContextError>;
    type UserRoleAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn user_role_assignment_repository(&self) -> Result<Self::UserRoleAssignmentRepository<'_>, ContextError>;
    type RolePermissionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn role_permission_repository(&self) -> Result<Self::RolePermissionRepository<'_>, ContextError>;
    type MagicLinkRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn magic_link_repository(&self) -> Result<Self::MagicLinkRepository<'_>, ContextError>;
    type UserSessionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn user_session_repository(&self) -> Result<Self::UserSessionRepository<'_>, ContextError>;
    type AccessTokenRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn access_token_repository(&self) -> Result<Self::AccessTokenRepository<'_>, ContextError>;
    type TwoFactorAuthRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn two_factor_auth_repository(&self) -> Result<Self::TwoFactorAuthRepository<'_>, ContextError>;
    type LoginAttemptRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn login_attempt_repository(&self) -> Result<Self::LoginAttemptRepository<'_>, ContextError>;
    type ActivityLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn activity_log_repository(&self) -> Result<Self::ActivityLogRepository<'_>, ContextError>;
    type AuditLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn audit_log_repository(&self) -> Result<Self::AuditLogRepository<'_>, ContextError>;
    type EntityChangeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn entity_change_repository(&self) -> Result<Self::EntityChangeRepository<'_>, ContextError>;
    type ChangeSetRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn change_set_repository(&self) -> Result<Self::ChangeSetRepository<'_>, ContextError>;
    type SystemEventRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn system_event_repository(&self) -> Result<Self::SystemEventRepository<'_>, ContextError>;
    type DataExportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn data_export_repository(&self) -> Result<Self::DataExportRepository<'_>, ContextError>;
    type NotificationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn notification_repository(&self) -> Result<Self::NotificationRepository<'_>, ContextError>;
    type NotificationTemplateRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn notification_template_repository(&self) -> Result<Self::NotificationTemplateRepository<'_>, ContextError>;
    type AutomationRuleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn automation_rule_repository(&self) -> Result<Self::AutomationRuleRepository<'_>, ContextError>;
    type AutomationTriggerRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn automation_trigger_repository(&self) -> Result<Self::AutomationTriggerRepository<'_>, ContextError>;
    type AutomationActionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn automation_action_repository(&self) -> Result<Self::AutomationActionRepository<'_>, ContextError>;
    type OperationalHookRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn operational_hook_repository(&self) -> Result<Self::OperationalHookRepository<'_>, ContextError>;
    type FinancialHookRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn financial_hook_repository(&self) -> Result<Self::FinancialHookRepository<'_>, ContextError>;
    type ApiClientRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn api_client_repository(&self) -> Result<Self::ApiClientRepository<'_>, ContextError>;
    type ApiEndpointRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn api_endpoint_repository(&self) -> Result<Self::ApiEndpointRepository<'_>, ContextError>;
    type WebhookRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn webhook_repository(&self) -> Result<Self::WebhookRepository<'_>, ContextError>;
    type WebhookDeliveryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn webhook_delivery_repository(&self) -> Result<Self::WebhookDeliveryRepository<'_>, ContextError>;
    type IntegrationMappingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn integration_mapping_repository(&self) -> Result<Self::IntegrationMappingRepository<'_>, ContextError>;
    type SynchronizationRunRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn synchronization_run_repository(&self) -> Result<Self::SynchronizationRunRepository<'_>, ContextError>;
    type ApiKeyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn api_key_repository(&self) -> Result<Self::ApiKeyRepository<'_>, ContextError>;
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

    type TenantConfigurationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tenant_configuration_repository(&self) -> Result<Self::TenantConfigurationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TenantConfiguration")
    }

    type OrganizationUnitRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn organization_unit_repository(&self) -> Result<Self::OrganizationUnitRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OrganizationUnit")
    }

    type DepartmentHierarchyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn department_hierarchy_repository(&self) -> Result<Self::DepartmentHierarchyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DepartmentHierarchy")
    }

    type BranchOfficeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn branch_office_repository(&self) -> Result<Self::BranchOfficeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BranchOffice")
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

    type MoveItemRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_item_repository(&self) -> Result<Self::MoveItemRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveItem")
    }

    type InventoryListRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn inventory_list_repository(&self) -> Result<Self::InventoryListRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InventoryList")
    }

    type PackingMaterialRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn packing_material_repository(&self) -> Result<Self::PackingMaterialRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PackingMaterial")
    }

    type LoadingZoneRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn loading_zone_repository(&self) -> Result<Self::LoadingZoneRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoadingZone")
    }

    type UnloadingZoneRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn unloading_zone_repository(&self) -> Result<Self::UnloadingZoneRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UnloadingZone")
    }

    type TransitLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn transit_log_repository(&self) -> Result<Self::TransitLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TransitLog")
    }

    type DelayRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn delay_record_repository(&self) -> Result<Self::DelayRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DelayRecord")
    }

    type RouteOptimizationRuleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn route_optimization_rule_repository(&self) -> Result<Self::RouteOptimizationRuleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RouteOptimizationRule")
    }

    type VehicleAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_assignment_repository(&self) -> Result<Self::VehicleAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleAssignment")
    }

    type CargoWeightRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn cargo_weight_record_repository(&self) -> Result<Self::CargoWeightRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CargoWeightRecord")
    }

    type SpecialHandlingInstructionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn special_handling_instruction_repository(&self) -> Result<Self::SpecialHandlingInstructionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SpecialHandlingInstruction")
    }

    type MoveStatusRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_status_repository(&self) -> Result<Self::MoveStatusRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveStatus")
    }

    type DeliveryWindowRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn delivery_window_repository(&self) -> Result<Self::DeliveryWindowRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DeliveryWindow")
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

    type DeductionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn deduction_repository(&self) -> Result<Self::DeductionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Deduction")
    }

    type LeaveRequestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn leave_request_repository(&self) -> Result<Self::LeaveRequestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LeaveRequest")
    }

    type EmployeeCertificationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn employee_certification_repository(&self) -> Result<Self::EmployeeCertificationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EmployeeCertification")
    }

    type TrainingModuleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn training_module_repository(&self) -> Result<Self::TrainingModuleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TrainingModule")
    }

    type AvailabilityScheduleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn availability_schedule_repository(&self) -> Result<Self::AvailabilityScheduleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AvailabilitySchedule")
    }

    type SkillProfileRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn skill_profile_repository(&self) -> Result<Self::SkillProfileRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SkillProfile")
    }

    type PerformanceReviewRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn performance_review_repository(&self) -> Result<Self::PerformanceReviewRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PerformanceReview")
    }

    type OvertimeRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn overtime_record_repository(&self) -> Result<Self::OvertimeRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OvertimeRecord")
    }

    type TaxWithholdingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tax_withholding_repository(&self) -> Result<Self::TaxWithholdingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TaxWithholding")
    }

    type BenefitEnrollmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn benefit_enrollment_repository(&self) -> Result<Self::BenefitEnrollmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BenefitEnrollment")
    }

    type ShiftSwapRequestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn shift_swap_request_repository(&self) -> Result<Self::ShiftSwapRequestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ShiftSwapRequest")
    }

    type AttendanceRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn attendance_record_repository(&self) -> Result<Self::AttendanceRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AttendanceRecord")
    }

    type PayrollAdjustmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payroll_adjustment_repository(&self) -> Result<Self::PayrollAdjustmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PayrollAdjustment")
    }

    type CommissionRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn commission_record_repository(&self) -> Result<Self::CommissionRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CommissionRecord")
    }

    type CustomerRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_repository(&self) -> Result<Self::CustomerRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Customer")
    }

    type PrivateCustomerProfileRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn private_customer_profile_repository(&self) -> Result<Self::PrivateCustomerProfileRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PrivateCustomerProfile")
    }

    type CorporateCustomerProfileRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn corporate_customer_profile_repository(&self) -> Result<Self::CorporateCustomerProfileRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CorporateCustomerProfile")
    }

    type CustomerContactRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_contact_repository(&self) -> Result<Self::CustomerContactRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerContact")
    }

    type BillingProfileRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn billing_profile_repository(&self) -> Result<Self::BillingProfileRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BillingProfile")
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

    type CustomerConsentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_consent_repository(&self) -> Result<Self::CustomerConsentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerConsent")
    }

    type CustomerFeedbackRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_feedback_repository(&self) -> Result<Self::CustomerFeedbackRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerFeedback")
    }

    type LoyaltyTierRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn loyalty_tier_repository(&self) -> Result<Self::LoyaltyTierRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoyaltyTier")
    }

    type ReferralCodeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn referral_code_repository(&self) -> Result<Self::ReferralCodeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ReferralCode")
    }

    type CommunicationLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn communication_log_repository(&self) -> Result<Self::CommunicationLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CommunicationLog")
    }

    type ServiceRatingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_rating_repository(&self) -> Result<Self::ServiceRatingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceRating")
    }

    type AccountStatusRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn account_status_repository(&self) -> Result<Self::AccountStatusRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AccountStatus")
    }

    type ContactMethodRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn contact_method_repository(&self) -> Result<Self::ContactMethodRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ContactMethod")
    }

    type CustomerSegmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_segment_repository(&self) -> Result<Self::CustomerSegmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerSegment")
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

    type MovingServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn moving_service_repository(&self) -> Result<Self::MovingServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MovingService")
    }

    type CleaningServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn cleaning_service_repository(&self) -> Result<Self::CleaningServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CleaningService")
    }

    type BoxRentalRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn box_rental_repository(&self) -> Result<Self::BoxRentalRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BoxRental")
    }

    type ServiceConfigurationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_configuration_repository(&self) -> Result<Self::ServiceConfigurationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceConfiguration")
    }

    type PriceListRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn price_list_repository(&self) -> Result<Self::PriceListRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PriceList")
    }

    type ServicePriceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_price_repository(&self) -> Result<Self::ServicePriceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServicePrice")
    }

    type ServiceBundleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_bundle_repository(&self) -> Result<Self::ServiceBundleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceBundle")
    }

    type StorageUnitRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn storage_unit_repository(&self) -> Result<Self::StorageUnitRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("StorageUnit")
    }

    type PackingKitRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn packing_kit_repository(&self) -> Result<Self::PackingKitRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PackingKit")
    }

    type DisposalServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn disposal_service_repository(&self) -> Result<Self::DisposalServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DisposalService")
    }

    type ServiceAreaRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_area_repository(&self) -> Result<Self::ServiceAreaRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceArea")
    }

    type AvailabilityCalendarRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn availability_calendar_repository(&self) -> Result<Self::AvailabilityCalendarRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AvailabilityCalendar")
    }

    type ServiceLevelAgreementRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_level_agreement_repository(&self) -> Result<Self::ServiceLevelAgreementRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceLevelAgreement")
    }

    type AddOnServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn add_on_service_repository(&self) -> Result<Self::AddOnServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AddOnService")
    }

    type InventoryItemRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn inventory_item_repository(&self) -> Result<Self::InventoryItemRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InventoryItem")
    }

    type ServiceCategoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_category_repository(&self) -> Result<Self::ServiceCategoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceCategory")
    }

    type CampaignRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn campaign_repository(&self) -> Result<Self::CampaignRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Campaign")
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

    type SalesOpportunityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn sales_opportunity_repository(&self) -> Result<Self::SalesOpportunityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SalesOpportunity")
    }

    type LeadActivityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn lead_activity_repository(&self) -> Result<Self::LeadActivityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LeadActivity")
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

    type MarketingChannelRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn marketing_channel_repository(&self) -> Result<Self::MarketingChannelRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MarketingChannel")
    }

    type AudienceSegmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn audience_segment_repository(&self) -> Result<Self::AudienceSegmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AudienceSegment")
    }

    type PromotionalOfferRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn promotional_offer_repository(&self) -> Result<Self::PromotionalOfferRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PromotionalOffer")
    }

    type SalesFunnelRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn sales_funnel_repository(&self) -> Result<Self::SalesFunnelRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SalesFunnel")
    }

    type AttributionModelRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn attribution_model_repository(&self) -> Result<Self::AttributionModelRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AttributionModel")
    }

    type LeadScoreRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn lead_score_repository(&self) -> Result<Self::LeadScoreRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LeadScore")
    }

    type CampaignBudgetRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn campaign_budget_repository(&self) -> Result<Self::CampaignBudgetRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CampaignBudget")
    }

    type ConversionReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn conversion_report_repository(&self) -> Result<Self::ConversionReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ConversionReport")
    }

    type PaymentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payment_repository(&self) -> Result<Self::PaymentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Payment")
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

    type RefundRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn refund_repository(&self) -> Result<Self::RefundRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Refund")
    }

    type ExpenseRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn expense_repository(&self) -> Result<Self::ExpenseRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Expense")
    }

    type VatRateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vat_rate_repository(&self) -> Result<Self::VatRateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VatRate")
    }

    type JournalEntryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn journal_entry_repository(&self) -> Result<Self::JournalEntryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("JournalEntry")
    }

    type AccountRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn account_repository(&self) -> Result<Self::AccountRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Account")
    }

    type FinancialSummaryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn financial_summary_repository(&self) -> Result<Self::FinancialSummaryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FinancialSummary")
    }

    type BudgetRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn budget_repository(&self) -> Result<Self::BudgetRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Budget")
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

    type TaxRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tax_record_repository(&self) -> Result<Self::TaxRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TaxRecord")
    }

    type CurrencyRateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn currency_rate_repository(&self) -> Result<Self::CurrencyRateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CurrencyRate")
    }

    type PaymentMethodRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payment_method_repository(&self) -> Result<Self::PaymentMethodRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PaymentMethod")
    }

    type FinancialPeriodRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn financial_period_repository(&self) -> Result<Self::FinancialPeriodRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FinancialPeriod")
    }

    type VehicleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_repository(&self) -> Result<Self::VehicleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Vehicle")
    }

    type EquipmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn equipment_repository(&self) -> Result<Self::EquipmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Equipment")
    }

    type ConsumableRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn consumable_repository(&self) -> Result<Self::ConsumableRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Consumable")
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

    type MaintenanceScheduleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn maintenance_schedule_repository(&self) -> Result<Self::MaintenanceScheduleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MaintenanceSchedule")
    }

    type MaintenanceEventRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn maintenance_event_repository(&self) -> Result<Self::MaintenanceEventRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MaintenanceEvent")
    }

    type FuelRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fuel_record_repository(&self) -> Result<Self::FuelRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FuelRecord")
    }

    type SupplierRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn supplier_repository(&self) -> Result<Self::SupplierRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Supplier")
    }

    type InventoryStockRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn inventory_stock_repository(&self) -> Result<Self::InventoryStockRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InventoryStock")
    }

    type MaintenanceCostRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn maintenance_cost_repository(&self) -> Result<Self::MaintenanceCostRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MaintenanceCost")
    }

    type VehicleRegistrationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_registration_repository(&self) -> Result<Self::VehicleRegistrationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleRegistration")
    }

    type EquipmentSerialRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn equipment_serial_repository(&self) -> Result<Self::EquipmentSerialRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EquipmentSerial")
    }

    type SupplierContractRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn supplier_contract_repository(&self) -> Result<Self::SupplierContractRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SupplierContract")
    }

    type AssetConditionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn asset_condition_repository(&self) -> Result<Self::AssetConditionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AssetCondition")
    }

    type DepreciationRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn depreciation_record_repository(&self) -> Result<Self::DepreciationRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DepreciationRecord")
    }

    type WarrantyClaimRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn warranty_claim_repository(&self) -> Result<Self::WarrantyClaimRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("WarrantyClaim")
    }

    type StorageLocationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn storage_location_repository(&self) -> Result<Self::StorageLocationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("StorageLocation")
    }

    type ContractRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn contract_repository(&self) -> Result<Self::ContractRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Contract")
    }

    type InsurancePolicyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn insurance_policy_repository(&self) -> Result<Self::InsurancePolicyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InsurancePolicy")
    }

    type InsuranceClaimRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn insurance_claim_repository(&self) -> Result<Self::InsuranceClaimRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InsuranceClaim")
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

    type ComplianceCheckRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn compliance_check_repository(&self) -> Result<Self::ComplianceCheckRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ComplianceCheck")
    }

    type DataRetentionPolicyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn data_retention_policy_repository(&self) -> Result<Self::DataRetentionPolicyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DataRetentionPolicy")
    }

    type RecoveryRequestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn recovery_request_repository(&self) -> Result<Self::RecoveryRequestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RecoveryRequest")
    }

    type PolicyDocumentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn policy_document_repository(&self) -> Result<Self::PolicyDocumentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PolicyDocument")
    }

    type IncidentReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn incident_report_repository(&self) -> Result<Self::IncidentReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("IncidentReport")
    }

    type AuditTrailRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn audit_trail_repository(&self) -> Result<Self::AuditTrailRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AuditTrail")
    }

    type LegalEntityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn legal_entity_repository(&self) -> Result<Self::LegalEntityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LegalEntity")
    }

    type RegulatoryRequirementRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn regulatory_requirement_repository(&self) -> Result<Self::RegulatoryRequirementRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RegulatoryRequirement")
    }

    type ComplianceCertificateRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn compliance_certificate_repository(&self) -> Result<Self::ComplianceCertificateRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ComplianceCertificate")
    }

    type UserAccountRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn user_account_repository(&self) -> Result<Self::UserAccountRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UserAccount")
    }

    type RoleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn role_repository(&self) -> Result<Self::RoleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Role")
    }

    type PermissionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn permission_repository(&self) -> Result<Self::PermissionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Permission")
    }

    type UserRoleAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn user_role_assignment_repository(&self) -> Result<Self::UserRoleAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UserRoleAssignment")
    }

    type RolePermissionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn role_permission_repository(&self) -> Result<Self::RolePermissionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RolePermission")
    }

    type MagicLinkRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn magic_link_repository(&self) -> Result<Self::MagicLinkRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MagicLink")
    }

    type UserSessionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn user_session_repository(&self) -> Result<Self::UserSessionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UserSession")
    }

    type AccessTokenRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn access_token_repository(&self) -> Result<Self::AccessTokenRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AccessToken")
    }

    type TwoFactorAuthRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn two_factor_auth_repository(&self) -> Result<Self::TwoFactorAuthRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TwoFactorAuth")
    }

    type LoginAttemptRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn login_attempt_repository(&self) -> Result<Self::LoginAttemptRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoginAttempt")
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

    type EntityChangeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn entity_change_repository(&self) -> Result<Self::EntityChangeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EntityChange")
    }

    type ChangeSetRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn change_set_repository(&self) -> Result<Self::ChangeSetRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ChangeSet")
    }

    type SystemEventRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn system_event_repository(&self) -> Result<Self::SystemEventRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SystemEvent")
    }

    type DataExportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn data_export_repository(&self) -> Result<Self::DataExportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DataExport")
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

    type AutomationActionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn automation_action_repository(&self) -> Result<Self::AutomationActionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AutomationAction")
    }

    type OperationalHookRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn operational_hook_repository(&self) -> Result<Self::OperationalHookRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OperationalHook")
    }

    type FinancialHookRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn financial_hook_repository(&self) -> Result<Self::FinancialHookRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FinancialHook")
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

    type IntegrationMappingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn integration_mapping_repository(&self) -> Result<Self::IntegrationMappingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("IntegrationMapping")
    }

    type SynchronizationRunRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn synchronization_run_repository(&self) -> Result<Self::SynchronizationRunRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SynchronizationRun")
    }

    type ApiKeyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn api_key_repository(&self) -> Result<Self::ApiKeyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ApiKey")
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
