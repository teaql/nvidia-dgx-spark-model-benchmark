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
    type PlatformConfigRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn platform_config_repository(&self) -> Result<Self::PlatformConfigRepository<'_>, ContextError>;
    type TenantRegistryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tenant_registry_repository(&self) -> Result<Self::TenantRegistryRepository<'_>, ContextError>;
    type MerchantRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn merchant_repository(&self) -> Result<Self::MerchantRepository<'_>, ContextError>;
    type BranchRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn branch_repository(&self) -> Result<Self::BranchRepository<'_>, ContextError>;
    type FranchiseRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn franchise_repository(&self) -> Result<Self::FranchiseRepository<'_>, ContextError>;
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
    type PackingListRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn packing_list_repository(&self) -> Result<Self::PackingListRepository<'_>, ContextError>;
    type InventoryItemRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn inventory_item_repository(&self) -> Result<Self::InventoryItemRepository<'_>, ContextError>;
    type VehicleLoadPlanRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_load_plan_repository(&self) -> Result<Self::VehicleLoadPlanRepository<'_>, ContextError>;
    type WeighStationTicketRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn weigh_station_ticket_repository(&self) -> Result<Self::WeighStationTicketRepository<'_>, ContextError>;
    type TollReceiptRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn toll_receipt_repository(&self) -> Result<Self::TollReceiptRepository<'_>, ContextError>;
    type ParkingPermitRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn parking_permit_repository(&self) -> Result<Self::ParkingPermitRepository<'_>, ContextError>;
    type TrafficViolationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn traffic_violation_repository(&self) -> Result<Self::TrafficViolationRepository<'_>, ContextError>;
    type DetourLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn detour_log_repository(&self) -> Result<Self::DetourLogRepository<'_>, ContextError>;
    type FuelStopRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fuel_stop_repository(&self) -> Result<Self::FuelStopRepository<'_>, ContextError>;
    type WeatherDelayRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn weather_delay_repository(&self) -> Result<Self::WeatherDelayRepository<'_>, ContextError>;
    type CustomerSignatureRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_signature_repository(&self) -> Result<Self::CustomerSignatureRepository<'_>, ContextError>;
    type WalkthroughChecklistRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn walkthrough_checklist_repository(&self) -> Result<Self::WalkthroughChecklistRepository<'_>, ContextError>;
    type PostMoveSurveyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn post_move_survey_repository(&self) -> Result<Self::PostMoveSurveyRepository<'_>, ContextError>;
    type OperationsManagerOverrideRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn operations_manager_override_repository(&self) -> Result<Self::OperationsManagerOverrideRepository<'_>, ContextError>;
    type EmployeeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn employee_repository(&self) -> Result<Self::EmployeeRepository<'_>, ContextError>;
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
    type LeaveRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn leave_request_repository(&self) -> Result<Self::LeaveRequestRepository<'_>, ContextError>;
    type EmployeeCertificationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn employee_certification_repository(&self) -> Result<Self::EmployeeCertificationRepository<'_>, ContextError>;
    type TaxWithholdingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tax_withholding_repository(&self) -> Result<Self::TaxWithholdingRepository<'_>, ContextError>;
    type DirectDepositInfoRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn direct_deposit_info_repository(&self) -> Result<Self::DirectDepositInfoRepository<'_>, ContextError>;
    type UnionDuesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn union_dues_repository(&self) -> Result<Self::UnionDuesRepository<'_>, ContextError>;
    type OvertimeApprovalRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn overtime_approval_repository(&self) -> Result<Self::OvertimeApprovalRepository<'_>, ContextError>;
    type ExpenseReimbursementRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn expense_reimbursement_repository(&self) -> Result<Self::ExpenseReimbursementRepository<'_>, ContextError>;
    type PerformanceReviewRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn performance_review_repository(&self) -> Result<Self::PerformanceReviewRepository<'_>, ContextError>;
    type WarningLetterRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn warning_letter_repository(&self) -> Result<Self::WarningLetterRepository<'_>, ContextError>;
    type TerminationRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn termination_record_repository(&self) -> Result<Self::TerminationRecordRepository<'_>, ContextError>;
    type EmergencyContactRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn emergency_contact_repository(&self) -> Result<Self::EmergencyContactRepository<'_>, ContextError>;
    type UniformAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn uniform_assignment_repository(&self) -> Result<Self::UniformAssignmentRepository<'_>, ContextError>;
    type BackgroundCheckRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn background_check_repository(&self) -> Result<Self::BackgroundCheckRepository<'_>, ContextError>;
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
    type ReferralCodeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn referral_code_repository(&self) -> Result<Self::ReferralCodeRepository<'_>, ContextError>;
    type LoyaltyTierRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn loyalty_tier_repository(&self) -> Result<Self::LoyaltyTierRepository<'_>, ContextError>;
    type ComplaintTicketRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn complaint_ticket_repository(&self) -> Result<Self::ComplaintTicketRepository<'_>, ContextError>;
    type ResolutionOfferRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn resolution_offer_repository(&self) -> Result<Self::ResolutionOfferRepository<'_>, ContextError>;
    type VipStatusRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vip_status_repository(&self) -> Result<Self::VipStatusRepository<'_>, ContextError>;
    type DoNotContactListRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn do_not_contact_list_repository(&self) -> Result<Self::DoNotContactListRepository<'_>, ContextError>;
    type CustomerNoteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_note_repository(&self) -> Result<Self::CustomerNoteRepository<'_>, ContextError>;
    type CommunicationLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn communication_log_repository(&self) -> Result<Self::CommunicationLogRepository<'_>, ContextError>;
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
    type PackingMaterialRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn packing_material_repository(&self) -> Result<Self::PackingMaterialRepository<'_>, ContextError>;
    type InsuranceAddonRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn insurance_addon_repository(&self) -> Result<Self::InsuranceAddonRepository<'_>, ContextError>;
    type PianoHandlingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn piano_handling_repository(&self) -> Result<Self::PianoHandlingRepository<'_>, ContextError>;
    type StairFeeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn stair_fee_repository(&self) -> Result<Self::StairFeeRepository<'_>, ContextError>;
    type LongCarryFeeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn long_carry_fee_repository(&self) -> Result<Self::LongCarryFeeRepository<'_>, ContextError>;
    type HoistingServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn hoisting_service_repository(&self) -> Result<Self::HoistingServiceRepository<'_>, ContextError>;
    type VehicleTransportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_transport_repository(&self) -> Result<Self::VehicleTransportRepository<'_>, ContextError>;
    type PetRelocationServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn pet_relocation_service_repository(&self) -> Result<Self::PetRelocationServiceRepository<'_>, ContextError>;
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
    type AdSpendRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn ad_spend_repository(&self) -> Result<Self::AdSpendRepository<'_>, ContextError>;
    type SocialMediaPostRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn social_media_post_repository(&self) -> Result<Self::SocialMediaPostRepository<'_>, ContextError>;
    type EmailBlastRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn email_blast_repository(&self) -> Result<Self::EmailBlastRepository<'_>, ContextError>;
    type SmsCampaignRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn sms_campaign_repository(&self) -> Result<Self::SmsCampaignRepository<'_>, ContextError>;
    type SalesScriptRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn sales_script_repository(&self) -> Result<Self::SalesScriptRepository<'_>, ContextError>;
    type ObjectionHandlingGuideRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn objection_handling_guide_repository(&self) -> Result<Self::ObjectionHandlingGuideRepository<'_>, ContextError>;
    type CompetitorAnalysisRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn competitor_analysis_repository(&self) -> Result<Self::CompetitorAnalysisRepository<'_>, ContextError>;
    type SalesTerritoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn sales_territory_repository(&self) -> Result<Self::SalesTerritoryRepository<'_>, ContextError>;
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
    type TaxDocumentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tax_document_repository(&self) -> Result<Self::TaxDocumentRepository<'_>, ContextError>;
    type BankTransactionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn bank_transaction_repository(&self) -> Result<Self::BankTransactionRepository<'_>, ContextError>;
    type MerchantFeeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn merchant_fee_repository(&self) -> Result<Self::MerchantFeeRepository<'_>, ContextError>;
    type ChargebackRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn chargeback_record_repository(&self) -> Result<Self::ChargebackRecordRepository<'_>, ContextError>;
    type CreditNoteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn credit_note_repository(&self) -> Result<Self::CreditNoteRepository<'_>, ContextError>;
    type DebitNoteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn debit_note_repository(&self) -> Result<Self::DebitNoteRepository<'_>, ContextError>;
    type AuditAdjustmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn audit_adjustment_repository(&self) -> Result<Self::AuditAdjustmentRepository<'_>, ContextError>;
    type FiscalYearRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fiscal_year_repository(&self) -> Result<Self::FiscalYearRepository<'_>, ContextError>;
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
    type GpsTrackerRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn gps_tracker_repository(&self) -> Result<Self::GpsTrackerRepository<'_>, ContextError>;
    type DashcamFootageRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn dashcam_footage_repository(&self) -> Result<Self::DashcamFootageRepository<'_>, ContextError>;
    type TireReplacementRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tire_replacement_repository(&self) -> Result<Self::TireReplacementRepository<'_>, ContextError>;
    type OilChangeLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn oil_change_log_repository(&self) -> Result<Self::OilChangeLogRepository<'_>, ContextError>;
    type RegistrationRenewalRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn registration_renewal_repository(&self) -> Result<Self::RegistrationRenewalRepository<'_>, ContextError>;
    type InsuranceCardRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn insurance_card_repository(&self) -> Result<Self::InsuranceCardRepository<'_>, ContextError>;
    type DepreciationScheduleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn depreciation_schedule_repository(&self) -> Result<Self::DepreciationScheduleRepository<'_>, ContextError>;
    type ScrapRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn scrap_record_repository(&self) -> Result<Self::ScrapRecordRepository<'_>, ContextError>;
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

    type PlatformConfigRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn platform_config_repository(&self) -> Result<Self::PlatformConfigRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PlatformConfig")
    }

    type TenantRegistryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tenant_registry_repository(&self) -> Result<Self::TenantRegistryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TenantRegistry")
    }

    type MerchantRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn merchant_repository(&self) -> Result<Self::MerchantRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Merchant")
    }

    type BranchRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn branch_repository(&self) -> Result<Self::BranchRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Branch")
    }

    type FranchiseRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn franchise_repository(&self) -> Result<Self::FranchiseRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Franchise")
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

    type PackingListRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn packing_list_repository(&self) -> Result<Self::PackingListRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PackingList")
    }

    type InventoryItemRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn inventory_item_repository(&self) -> Result<Self::InventoryItemRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InventoryItem")
    }

    type VehicleLoadPlanRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_load_plan_repository(&self) -> Result<Self::VehicleLoadPlanRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleLoadPlan")
    }

    type WeighStationTicketRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn weigh_station_ticket_repository(&self) -> Result<Self::WeighStationTicketRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("WeighStationTicket")
    }

    type TollReceiptRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn toll_receipt_repository(&self) -> Result<Self::TollReceiptRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TollReceipt")
    }

    type ParkingPermitRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn parking_permit_repository(&self) -> Result<Self::ParkingPermitRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ParkingPermit")
    }

    type TrafficViolationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn traffic_violation_repository(&self) -> Result<Self::TrafficViolationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TrafficViolation")
    }

    type DetourLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn detour_log_repository(&self) -> Result<Self::DetourLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DetourLog")
    }

    type FuelStopRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fuel_stop_repository(&self) -> Result<Self::FuelStopRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FuelStop")
    }

    type WeatherDelayRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn weather_delay_repository(&self) -> Result<Self::WeatherDelayRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("WeatherDelay")
    }

    type CustomerSignatureRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_signature_repository(&self) -> Result<Self::CustomerSignatureRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerSignature")
    }

    type WalkthroughChecklistRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn walkthrough_checklist_repository(&self) -> Result<Self::WalkthroughChecklistRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("WalkthroughChecklist")
    }

    type PostMoveSurveyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn post_move_survey_repository(&self) -> Result<Self::PostMoveSurveyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PostMoveSurvey")
    }

    type OperationsManagerOverrideRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn operations_manager_override_repository(&self) -> Result<Self::OperationsManagerOverrideRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OperationsManagerOverride")
    }

    type EmployeeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn employee_repository(&self) -> Result<Self::EmployeeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Employee")
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

    type TaxWithholdingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tax_withholding_repository(&self) -> Result<Self::TaxWithholdingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TaxWithholding")
    }

    type DirectDepositInfoRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn direct_deposit_info_repository(&self) -> Result<Self::DirectDepositInfoRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DirectDepositInfo")
    }

    type UnionDuesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn union_dues_repository(&self) -> Result<Self::UnionDuesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UnionDues")
    }

    type OvertimeApprovalRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn overtime_approval_repository(&self) -> Result<Self::OvertimeApprovalRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OvertimeApproval")
    }

    type ExpenseReimbursementRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn expense_reimbursement_repository(&self) -> Result<Self::ExpenseReimbursementRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExpenseReimbursement")
    }

    type PerformanceReviewRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn performance_review_repository(&self) -> Result<Self::PerformanceReviewRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PerformanceReview")
    }

    type WarningLetterRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn warning_letter_repository(&self) -> Result<Self::WarningLetterRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("WarningLetter")
    }

    type TerminationRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn termination_record_repository(&self) -> Result<Self::TerminationRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TerminationRecord")
    }

    type EmergencyContactRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn emergency_contact_repository(&self) -> Result<Self::EmergencyContactRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EmergencyContact")
    }

    type UniformAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn uniform_assignment_repository(&self) -> Result<Self::UniformAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UniformAssignment")
    }

    type BackgroundCheckRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn background_check_repository(&self) -> Result<Self::BackgroundCheckRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BackgroundCheck")
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

    type ReferralCodeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn referral_code_repository(&self) -> Result<Self::ReferralCodeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ReferralCode")
    }

    type LoyaltyTierRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn loyalty_tier_repository(&self) -> Result<Self::LoyaltyTierRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoyaltyTier")
    }

    type ComplaintTicketRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn complaint_ticket_repository(&self) -> Result<Self::ComplaintTicketRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ComplaintTicket")
    }

    type ResolutionOfferRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn resolution_offer_repository(&self) -> Result<Self::ResolutionOfferRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ResolutionOffer")
    }

    type VipStatusRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vip_status_repository(&self) -> Result<Self::VipStatusRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VipStatus")
    }

    type DoNotContactListRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn do_not_contact_list_repository(&self) -> Result<Self::DoNotContactListRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DoNotContactList")
    }

    type CustomerNoteRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_note_repository(&self) -> Result<Self::CustomerNoteRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerNote")
    }

    type CommunicationLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn communication_log_repository(&self) -> Result<Self::CommunicationLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CommunicationLog")
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

    type PackingMaterialRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn packing_material_repository(&self) -> Result<Self::PackingMaterialRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PackingMaterial")
    }

    type InsuranceAddonRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn insurance_addon_repository(&self) -> Result<Self::InsuranceAddonRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InsuranceAddon")
    }

    type PianoHandlingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn piano_handling_repository(&self) -> Result<Self::PianoHandlingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PianoHandling")
    }

    type StairFeeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn stair_fee_repository(&self) -> Result<Self::StairFeeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("StairFee")
    }

    type LongCarryFeeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn long_carry_fee_repository(&self) -> Result<Self::LongCarryFeeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LongCarryFee")
    }

    type HoistingServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn hoisting_service_repository(&self) -> Result<Self::HoistingServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("HoistingService")
    }

    type VehicleTransportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_transport_repository(&self) -> Result<Self::VehicleTransportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleTransport")
    }

    type PetRelocationServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn pet_relocation_service_repository(&self) -> Result<Self::PetRelocationServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PetRelocationService")
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

    type AdSpendRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn ad_spend_repository(&self) -> Result<Self::AdSpendRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AdSpend")
    }

    type SocialMediaPostRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn social_media_post_repository(&self) -> Result<Self::SocialMediaPostRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SocialMediaPost")
    }

    type EmailBlastRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn email_blast_repository(&self) -> Result<Self::EmailBlastRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EmailBlast")
    }

    type SmsCampaignRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn sms_campaign_repository(&self) -> Result<Self::SmsCampaignRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SmsCampaign")
    }

    type SalesScriptRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn sales_script_repository(&self) -> Result<Self::SalesScriptRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SalesScript")
    }

    type ObjectionHandlingGuideRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn objection_handling_guide_repository(&self) -> Result<Self::ObjectionHandlingGuideRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ObjectionHandlingGuide")
    }

    type CompetitorAnalysisRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn competitor_analysis_repository(&self) -> Result<Self::CompetitorAnalysisRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CompetitorAnalysis")
    }

    type SalesTerritoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn sales_territory_repository(&self) -> Result<Self::SalesTerritoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SalesTerritory")
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

    type TaxDocumentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tax_document_repository(&self) -> Result<Self::TaxDocumentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TaxDocument")
    }

    type BankTransactionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn bank_transaction_repository(&self) -> Result<Self::BankTransactionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("BankTransaction")
    }

    type MerchantFeeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn merchant_fee_repository(&self) -> Result<Self::MerchantFeeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MerchantFee")
    }

    type ChargebackRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn chargeback_record_repository(&self) -> Result<Self::ChargebackRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ChargebackRecord")
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

    type AuditAdjustmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn audit_adjustment_repository(&self) -> Result<Self::AuditAdjustmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AuditAdjustment")
    }

    type FiscalYearRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fiscal_year_repository(&self) -> Result<Self::FiscalYearRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FiscalYear")
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

    type GpsTrackerRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn gps_tracker_repository(&self) -> Result<Self::GpsTrackerRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("GpsTracker")
    }

    type DashcamFootageRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn dashcam_footage_repository(&self) -> Result<Self::DashcamFootageRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DashcamFootage")
    }

    type TireReplacementRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tire_replacement_repository(&self) -> Result<Self::TireReplacementRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TireReplacement")
    }

    type OilChangeLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn oil_change_log_repository(&self) -> Result<Self::OilChangeLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OilChangeLog")
    }

    type RegistrationRenewalRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn registration_renewal_repository(&self) -> Result<Self::RegistrationRenewalRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RegistrationRenewal")
    }

    type InsuranceCardRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn insurance_card_repository(&self) -> Result<Self::InsuranceCardRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InsuranceCard")
    }

    type DepreciationScheduleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn depreciation_schedule_repository(&self) -> Result<Self::DepreciationScheduleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DepreciationSchedule")
    }

    type ScrapRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn scrap_record_repository(&self) -> Result<Self::ScrapRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ScrapRecord")
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
