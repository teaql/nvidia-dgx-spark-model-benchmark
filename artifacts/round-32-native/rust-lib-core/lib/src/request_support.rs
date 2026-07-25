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
    type NdaAgreementRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn nda_agreement_repository(&self) -> Result<Self::NdaAgreementRepository<'_>, ContextError>;
    type TermsOfServiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn terms_of_service_repository(&self) -> Result<Self::TermsOfServiceRepository<'_>, ContextError>;
    type PrivacyPolicyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn privacy_policy_repository(&self) -> Result<Self::PrivacyPolicyRepository<'_>, ContextError>;
    type CookieConsentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn cookie_consent_repository(&self) -> Result<Self::CookieConsentRepository<'_>, ContextError>;
    type GdprRequestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn gdpr_request_repository(&self) -> Result<Self::GdprRequestRepository<'_>, ContextError>;
    type OshaIncidentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn osha_incident_repository(&self) -> Result<Self::OshaIncidentRepository<'_>, ContextError>;
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
    type PasswordResetRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn password_reset_repository(&self) -> Result<Self::PasswordResetRepository<'_>, ContextError>;
    type TwoFactorAuthRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn two_factor_auth_repository(&self) -> Result<Self::TwoFactorAuthRepository<'_>, ContextError>;
    type AccessTokenRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn access_token_repository(&self) -> Result<Self::AccessTokenRepository<'_>, ContextError>;
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
    type LoginAttemptRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn login_attempt_repository(&self) -> Result<Self::LoginAttemptRepository<'_>, ContextError>;
    type FailedAuthLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn failed_auth_log_repository(&self) -> Result<Self::FailedAuthLogRepository<'_>, ContextError>;
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
    type SmsDeliveryReceiptRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn sms_delivery_receipt_repository(&self) -> Result<Self::SmsDeliveryReceiptRepository<'_>, ContextError>;
    type EmailBounceLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn email_bounce_log_repository(&self) -> Result<Self::EmailBounceLogRepository<'_>, ContextError>;
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
    type SyncJobRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn sync_job_repository(&self) -> Result<Self::SyncJobRepository<'_>, ContextError>;
    type ApiRateLimitRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn api_rate_limit_repository(&self) -> Result<Self::ApiRateLimitRepository<'_>, ContextError>;
    type CustomEntity180Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_180_repository(&self) -> Result<Self::CustomEntity180Repository<'_>, ContextError>;
    type CustomEntity181Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_181_repository(&self) -> Result<Self::CustomEntity181Repository<'_>, ContextError>;
    type CustomEntity182Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_182_repository(&self) -> Result<Self::CustomEntity182Repository<'_>, ContextError>;
    type CustomEntity183Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_183_repository(&self) -> Result<Self::CustomEntity183Repository<'_>, ContextError>;
    type CustomEntity184Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_184_repository(&self) -> Result<Self::CustomEntity184Repository<'_>, ContextError>;
    type CustomEntity185Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_185_repository(&self) -> Result<Self::CustomEntity185Repository<'_>, ContextError>;
    type CustomEntity186Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_186_repository(&self) -> Result<Self::CustomEntity186Repository<'_>, ContextError>;
    type CustomEntity187Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_187_repository(&self) -> Result<Self::CustomEntity187Repository<'_>, ContextError>;
    type CustomEntity188Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_188_repository(&self) -> Result<Self::CustomEntity188Repository<'_>, ContextError>;
    type CustomEntity189Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_189_repository(&self) -> Result<Self::CustomEntity189Repository<'_>, ContextError>;
    type CustomEntity190Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_190_repository(&self) -> Result<Self::CustomEntity190Repository<'_>, ContextError>;
    type CustomEntity191Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_191_repository(&self) -> Result<Self::CustomEntity191Repository<'_>, ContextError>;
    type CustomEntity192Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_192_repository(&self) -> Result<Self::CustomEntity192Repository<'_>, ContextError>;
    type CustomEntity193Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_193_repository(&self) -> Result<Self::CustomEntity193Repository<'_>, ContextError>;
    type CustomEntity194Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_194_repository(&self) -> Result<Self::CustomEntity194Repository<'_>, ContextError>;
    type CustomEntity195Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_195_repository(&self) -> Result<Self::CustomEntity195Repository<'_>, ContextError>;
    type CustomEntity196Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_196_repository(&self) -> Result<Self::CustomEntity196Repository<'_>, ContextError>;
    type CustomEntity197Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_197_repository(&self) -> Result<Self::CustomEntity197Repository<'_>, ContextError>;
    type CustomEntity198Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_198_repository(&self) -> Result<Self::CustomEntity198Repository<'_>, ContextError>;
    type CustomEntity199Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_199_repository(&self) -> Result<Self::CustomEntity199Repository<'_>, ContextError>;
    type CustomEntity200Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_200_repository(&self) -> Result<Self::CustomEntity200Repository<'_>, ContextError>;
    type CustomEntity201Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_201_repository(&self) -> Result<Self::CustomEntity201Repository<'_>, ContextError>;
    type CustomEntity202Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_202_repository(&self) -> Result<Self::CustomEntity202Repository<'_>, ContextError>;
    type CustomEntity203Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_203_repository(&self) -> Result<Self::CustomEntity203Repository<'_>, ContextError>;
    type CustomEntity204Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_204_repository(&self) -> Result<Self::CustomEntity204Repository<'_>, ContextError>;
    type CustomEntity205Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_205_repository(&self) -> Result<Self::CustomEntity205Repository<'_>, ContextError>;
    type CustomEntity206Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_206_repository(&self) -> Result<Self::CustomEntity206Repository<'_>, ContextError>;
    type CustomEntity207Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_207_repository(&self) -> Result<Self::CustomEntity207Repository<'_>, ContextError>;
    type CustomEntity208Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_208_repository(&self) -> Result<Self::CustomEntity208Repository<'_>, ContextError>;
    type CustomEntity209Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_209_repository(&self) -> Result<Self::CustomEntity209Repository<'_>, ContextError>;
    type CustomEntity210Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_210_repository(&self) -> Result<Self::CustomEntity210Repository<'_>, ContextError>;
    type CustomEntity211Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_211_repository(&self) -> Result<Self::CustomEntity211Repository<'_>, ContextError>;
    type CustomEntity212Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_212_repository(&self) -> Result<Self::CustomEntity212Repository<'_>, ContextError>;
    type CustomEntity213Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_213_repository(&self) -> Result<Self::CustomEntity213Repository<'_>, ContextError>;
    type CustomEntity214Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_214_repository(&self) -> Result<Self::CustomEntity214Repository<'_>, ContextError>;
    type CustomEntity215Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_215_repository(&self) -> Result<Self::CustomEntity215Repository<'_>, ContextError>;
    type CustomEntity216Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_216_repository(&self) -> Result<Self::CustomEntity216Repository<'_>, ContextError>;
    type CustomEntity217Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_217_repository(&self) -> Result<Self::CustomEntity217Repository<'_>, ContextError>;
    type CustomEntity218Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_218_repository(&self) -> Result<Self::CustomEntity218Repository<'_>, ContextError>;
    type CustomEntity219Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_219_repository(&self) -> Result<Self::CustomEntity219Repository<'_>, ContextError>;
    type CustomEntity220Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_220_repository(&self) -> Result<Self::CustomEntity220Repository<'_>, ContextError>;
    type CustomEntity221Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_221_repository(&self) -> Result<Self::CustomEntity221Repository<'_>, ContextError>;
    type CustomEntity222Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_222_repository(&self) -> Result<Self::CustomEntity222Repository<'_>, ContextError>;
    type CustomEntity223Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_223_repository(&self) -> Result<Self::CustomEntity223Repository<'_>, ContextError>;
    type CustomEntity224Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_224_repository(&self) -> Result<Self::CustomEntity224Repository<'_>, ContextError>;
    type CustomEntity225Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_225_repository(&self) -> Result<Self::CustomEntity225Repository<'_>, ContextError>;
    type CustomEntity226Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_226_repository(&self) -> Result<Self::CustomEntity226Repository<'_>, ContextError>;
    type CustomEntity227Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_227_repository(&self) -> Result<Self::CustomEntity227Repository<'_>, ContextError>;
    type CustomEntity228Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_228_repository(&self) -> Result<Self::CustomEntity228Repository<'_>, ContextError>;
    type CustomEntity229Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_229_repository(&self) -> Result<Self::CustomEntity229Repository<'_>, ContextError>;
    type CustomEntity230Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_230_repository(&self) -> Result<Self::CustomEntity230Repository<'_>, ContextError>;
    type CustomEntity231Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_231_repository(&self) -> Result<Self::CustomEntity231Repository<'_>, ContextError>;
    type CustomEntity232Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_232_repository(&self) -> Result<Self::CustomEntity232Repository<'_>, ContextError>;
    type CustomEntity233Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_233_repository(&self) -> Result<Self::CustomEntity233Repository<'_>, ContextError>;
    type CustomEntity234Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_234_repository(&self) -> Result<Self::CustomEntity234Repository<'_>, ContextError>;
    type CustomEntity235Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_235_repository(&self) -> Result<Self::CustomEntity235Repository<'_>, ContextError>;
    type CustomEntity236Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_236_repository(&self) -> Result<Self::CustomEntity236Repository<'_>, ContextError>;
    type CustomEntity237Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_237_repository(&self) -> Result<Self::CustomEntity237Repository<'_>, ContextError>;
    type CustomEntity238Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_238_repository(&self) -> Result<Self::CustomEntity238Repository<'_>, ContextError>;
    type CustomEntity239Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_239_repository(&self) -> Result<Self::CustomEntity239Repository<'_>, ContextError>;
    type CustomEntity240Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_240_repository(&self) -> Result<Self::CustomEntity240Repository<'_>, ContextError>;
    type CustomEntity241Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_241_repository(&self) -> Result<Self::CustomEntity241Repository<'_>, ContextError>;
    type CustomEntity242Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_242_repository(&self) -> Result<Self::CustomEntity242Repository<'_>, ContextError>;
    type CustomEntity243Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_243_repository(&self) -> Result<Self::CustomEntity243Repository<'_>, ContextError>;
    type CustomEntity244Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_244_repository(&self) -> Result<Self::CustomEntity244Repository<'_>, ContextError>;
    type CustomEntity245Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_245_repository(&self) -> Result<Self::CustomEntity245Repository<'_>, ContextError>;
    type CustomEntity246Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_246_repository(&self) -> Result<Self::CustomEntity246Repository<'_>, ContextError>;
    type CustomEntity247Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_247_repository(&self) -> Result<Self::CustomEntity247Repository<'_>, ContextError>;
    type CustomEntity248Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_248_repository(&self) -> Result<Self::CustomEntity248Repository<'_>, ContextError>;
    type CustomEntity249Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_249_repository(&self) -> Result<Self::CustomEntity249Repository<'_>, ContextError>;
    type CustomEntity250Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_250_repository(&self) -> Result<Self::CustomEntity250Repository<'_>, ContextError>;
    type CustomEntity251Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_251_repository(&self) -> Result<Self::CustomEntity251Repository<'_>, ContextError>;
    type CustomEntity252Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_252_repository(&self) -> Result<Self::CustomEntity252Repository<'_>, ContextError>;
    type CustomEntity253Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_253_repository(&self) -> Result<Self::CustomEntity253Repository<'_>, ContextError>;
    type CustomEntity254Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_254_repository(&self) -> Result<Self::CustomEntity254Repository<'_>, ContextError>;
    type CustomEntity255Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_255_repository(&self) -> Result<Self::CustomEntity255Repository<'_>, ContextError>;
    type CustomEntity256Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_256_repository(&self) -> Result<Self::CustomEntity256Repository<'_>, ContextError>;
    type CustomEntity257Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_257_repository(&self) -> Result<Self::CustomEntity257Repository<'_>, ContextError>;
    type CustomEntity258Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_258_repository(&self) -> Result<Self::CustomEntity258Repository<'_>, ContextError>;
    type CustomEntity259Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_259_repository(&self) -> Result<Self::CustomEntity259Repository<'_>, ContextError>;
    type CustomEntity260Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_260_repository(&self) -> Result<Self::CustomEntity260Repository<'_>, ContextError>;
    type CustomEntity261Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_261_repository(&self) -> Result<Self::CustomEntity261Repository<'_>, ContextError>;
    type CustomEntity262Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_262_repository(&self) -> Result<Self::CustomEntity262Repository<'_>, ContextError>;
    type CustomEntity263Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_263_repository(&self) -> Result<Self::CustomEntity263Repository<'_>, ContextError>;
    type CustomEntity264Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_264_repository(&self) -> Result<Self::CustomEntity264Repository<'_>, ContextError>;
    type CustomEntity265Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_265_repository(&self) -> Result<Self::CustomEntity265Repository<'_>, ContextError>;
    type CustomEntity266Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_266_repository(&self) -> Result<Self::CustomEntity266Repository<'_>, ContextError>;
    type CustomEntity267Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_267_repository(&self) -> Result<Self::CustomEntity267Repository<'_>, ContextError>;
    type CustomEntity268Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_268_repository(&self) -> Result<Self::CustomEntity268Repository<'_>, ContextError>;
    type CustomEntity269Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_269_repository(&self) -> Result<Self::CustomEntity269Repository<'_>, ContextError>;
    type CustomEntity270Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_270_repository(&self) -> Result<Self::CustomEntity270Repository<'_>, ContextError>;
    type CustomEntity271Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_271_repository(&self) -> Result<Self::CustomEntity271Repository<'_>, ContextError>;
    type CustomEntity272Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_272_repository(&self) -> Result<Self::CustomEntity272Repository<'_>, ContextError>;
    type CustomEntity273Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_273_repository(&self) -> Result<Self::CustomEntity273Repository<'_>, ContextError>;
    type CustomEntity274Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_274_repository(&self) -> Result<Self::CustomEntity274Repository<'_>, ContextError>;
    type CustomEntity275Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_275_repository(&self) -> Result<Self::CustomEntity275Repository<'_>, ContextError>;
    type CustomEntity276Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_276_repository(&self) -> Result<Self::CustomEntity276Repository<'_>, ContextError>;
    type CustomEntity277Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_277_repository(&self) -> Result<Self::CustomEntity277Repository<'_>, ContextError>;
    type CustomEntity278Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_278_repository(&self) -> Result<Self::CustomEntity278Repository<'_>, ContextError>;
    type CustomEntity279Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_279_repository(&self) -> Result<Self::CustomEntity279Repository<'_>, ContextError>;
    type CustomEntity280Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_280_repository(&self) -> Result<Self::CustomEntity280Repository<'_>, ContextError>;
    type CustomEntity281Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_281_repository(&self) -> Result<Self::CustomEntity281Repository<'_>, ContextError>;
    type CustomEntity282Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_282_repository(&self) -> Result<Self::CustomEntity282Repository<'_>, ContextError>;
    type CustomEntity283Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_283_repository(&self) -> Result<Self::CustomEntity283Repository<'_>, ContextError>;
    type CustomEntity284Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_284_repository(&self) -> Result<Self::CustomEntity284Repository<'_>, ContextError>;
    type CustomEntity285Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_285_repository(&self) -> Result<Self::CustomEntity285Repository<'_>, ContextError>;
    type CustomEntity286Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_286_repository(&self) -> Result<Self::CustomEntity286Repository<'_>, ContextError>;
    type CustomEntity287Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_287_repository(&self) -> Result<Self::CustomEntity287Repository<'_>, ContextError>;
    type CustomEntity288Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_288_repository(&self) -> Result<Self::CustomEntity288Repository<'_>, ContextError>;
    type CustomEntity289Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_289_repository(&self) -> Result<Self::CustomEntity289Repository<'_>, ContextError>;
    type CustomEntity290Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_290_repository(&self) -> Result<Self::CustomEntity290Repository<'_>, ContextError>;
    type CustomEntity291Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_291_repository(&self) -> Result<Self::CustomEntity291Repository<'_>, ContextError>;
    type CustomEntity292Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_292_repository(&self) -> Result<Self::CustomEntity292Repository<'_>, ContextError>;
    type CustomEntity293Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_293_repository(&self) -> Result<Self::CustomEntity293Repository<'_>, ContextError>;
    type CustomEntity294Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_294_repository(&self) -> Result<Self::CustomEntity294Repository<'_>, ContextError>;
    type CustomEntity295Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_295_repository(&self) -> Result<Self::CustomEntity295Repository<'_>, ContextError>;
    type CustomEntity296Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_296_repository(&self) -> Result<Self::CustomEntity296Repository<'_>, ContextError>;
    type CustomEntity297Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_297_repository(&self) -> Result<Self::CustomEntity297Repository<'_>, ContextError>;
    type CustomEntity298Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_298_repository(&self) -> Result<Self::CustomEntity298Repository<'_>, ContextError>;
    type CustomEntity299Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_299_repository(&self) -> Result<Self::CustomEntity299Repository<'_>, ContextError>;
    type CustomEntity300Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_300_repository(&self) -> Result<Self::CustomEntity300Repository<'_>, ContextError>;
    type CustomEntity301Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_301_repository(&self) -> Result<Self::CustomEntity301Repository<'_>, ContextError>;
    type CustomEntity302Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_302_repository(&self) -> Result<Self::CustomEntity302Repository<'_>, ContextError>;
    type CustomEntity303Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_303_repository(&self) -> Result<Self::CustomEntity303Repository<'_>, ContextError>;
    type CustomEntity304Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_304_repository(&self) -> Result<Self::CustomEntity304Repository<'_>, ContextError>;
    type CustomEntity305Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_305_repository(&self) -> Result<Self::CustomEntity305Repository<'_>, ContextError>;
    type CustomEntity306Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_306_repository(&self) -> Result<Self::CustomEntity306Repository<'_>, ContextError>;
    type CustomEntity307Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_307_repository(&self) -> Result<Self::CustomEntity307Repository<'_>, ContextError>;
    type CustomEntity308Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_308_repository(&self) -> Result<Self::CustomEntity308Repository<'_>, ContextError>;
    type CustomEntity309Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_309_repository(&self) -> Result<Self::CustomEntity309Repository<'_>, ContextError>;
    type CustomEntity310Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_310_repository(&self) -> Result<Self::CustomEntity310Repository<'_>, ContextError>;
    type CustomEntity311Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_311_repository(&self) -> Result<Self::CustomEntity311Repository<'_>, ContextError>;
    type CustomEntity312Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_312_repository(&self) -> Result<Self::CustomEntity312Repository<'_>, ContextError>;
    type CustomEntity313Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_313_repository(&self) -> Result<Self::CustomEntity313Repository<'_>, ContextError>;
    type CustomEntity314Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_314_repository(&self) -> Result<Self::CustomEntity314Repository<'_>, ContextError>;
    type CustomEntity315Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_315_repository(&self) -> Result<Self::CustomEntity315Repository<'_>, ContextError>;
    type CustomEntity316Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_316_repository(&self) -> Result<Self::CustomEntity316Repository<'_>, ContextError>;
    type CustomEntity317Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_317_repository(&self) -> Result<Self::CustomEntity317Repository<'_>, ContextError>;
    type CustomEntity318Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_318_repository(&self) -> Result<Self::CustomEntity318Repository<'_>, ContextError>;
    type CustomEntity319Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_319_repository(&self) -> Result<Self::CustomEntity319Repository<'_>, ContextError>;
    type CustomEntity320Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_320_repository(&self) -> Result<Self::CustomEntity320Repository<'_>, ContextError>;
    type CustomEntity321Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_321_repository(&self) -> Result<Self::CustomEntity321Repository<'_>, ContextError>;
    type CustomEntity322Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_322_repository(&self) -> Result<Self::CustomEntity322Repository<'_>, ContextError>;
    type CustomEntity323Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_323_repository(&self) -> Result<Self::CustomEntity323Repository<'_>, ContextError>;
    type CustomEntity324Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_324_repository(&self) -> Result<Self::CustomEntity324Repository<'_>, ContextError>;
    type CustomEntity325Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_325_repository(&self) -> Result<Self::CustomEntity325Repository<'_>, ContextError>;
    type CustomEntity326Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_326_repository(&self) -> Result<Self::CustomEntity326Repository<'_>, ContextError>;
    type CustomEntity327Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_327_repository(&self) -> Result<Self::CustomEntity327Repository<'_>, ContextError>;
    type CustomEntity328Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_328_repository(&self) -> Result<Self::CustomEntity328Repository<'_>, ContextError>;
    type CustomEntity329Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_329_repository(&self) -> Result<Self::CustomEntity329Repository<'_>, ContextError>;
    type CustomEntity330Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_330_repository(&self) -> Result<Self::CustomEntity330Repository<'_>, ContextError>;
    type CustomEntity331Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_331_repository(&self) -> Result<Self::CustomEntity331Repository<'_>, ContextError>;
    type CustomEntity332Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_332_repository(&self) -> Result<Self::CustomEntity332Repository<'_>, ContextError>;
    type CustomEntity333Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_333_repository(&self) -> Result<Self::CustomEntity333Repository<'_>, ContextError>;
    type CustomEntity334Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_334_repository(&self) -> Result<Self::CustomEntity334Repository<'_>, ContextError>;
    type CustomEntity335Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_335_repository(&self) -> Result<Self::CustomEntity335Repository<'_>, ContextError>;
    type CustomEntity336Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_336_repository(&self) -> Result<Self::CustomEntity336Repository<'_>, ContextError>;
    type CustomEntity337Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_337_repository(&self) -> Result<Self::CustomEntity337Repository<'_>, ContextError>;
    type CustomEntity338Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_338_repository(&self) -> Result<Self::CustomEntity338Repository<'_>, ContextError>;
    type CustomEntity339Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_339_repository(&self) -> Result<Self::CustomEntity339Repository<'_>, ContextError>;
    type CustomEntity340Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_340_repository(&self) -> Result<Self::CustomEntity340Repository<'_>, ContextError>;
    type CustomEntity341Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_341_repository(&self) -> Result<Self::CustomEntity341Repository<'_>, ContextError>;
    type CustomEntity342Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_342_repository(&self) -> Result<Self::CustomEntity342Repository<'_>, ContextError>;
    type CustomEntity343Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_343_repository(&self) -> Result<Self::CustomEntity343Repository<'_>, ContextError>;
    type CustomEntity344Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_344_repository(&self) -> Result<Self::CustomEntity344Repository<'_>, ContextError>;
    type CustomEntity345Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_345_repository(&self) -> Result<Self::CustomEntity345Repository<'_>, ContextError>;
    type CustomEntity346Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_346_repository(&self) -> Result<Self::CustomEntity346Repository<'_>, ContextError>;
    type CustomEntity347Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_347_repository(&self) -> Result<Self::CustomEntity347Repository<'_>, ContextError>;
    type CustomEntity348Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_348_repository(&self) -> Result<Self::CustomEntity348Repository<'_>, ContextError>;
    type CustomEntity349Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_349_repository(&self) -> Result<Self::CustomEntity349Repository<'_>, ContextError>;
    type CustomEntity350Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_350_repository(&self) -> Result<Self::CustomEntity350Repository<'_>, ContextError>;
    type CustomEntity351Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_351_repository(&self) -> Result<Self::CustomEntity351Repository<'_>, ContextError>;
    type CustomEntity352Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_352_repository(&self) -> Result<Self::CustomEntity352Repository<'_>, ContextError>;
    type CustomEntity353Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_353_repository(&self) -> Result<Self::CustomEntity353Repository<'_>, ContextError>;
    type CustomEntity354Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_354_repository(&self) -> Result<Self::CustomEntity354Repository<'_>, ContextError>;
    type CustomEntity355Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_355_repository(&self) -> Result<Self::CustomEntity355Repository<'_>, ContextError>;
    type CustomEntity356Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_356_repository(&self) -> Result<Self::CustomEntity356Repository<'_>, ContextError>;
    type CustomEntity357Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_357_repository(&self) -> Result<Self::CustomEntity357Repository<'_>, ContextError>;
    type CustomEntity358Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_358_repository(&self) -> Result<Self::CustomEntity358Repository<'_>, ContextError>;
    type CustomEntity359Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_359_repository(&self) -> Result<Self::CustomEntity359Repository<'_>, ContextError>;
    type CustomEntity360Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_360_repository(&self) -> Result<Self::CustomEntity360Repository<'_>, ContextError>;
    type CustomEntity361Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_361_repository(&self) -> Result<Self::CustomEntity361Repository<'_>, ContextError>;
    type CustomEntity362Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_362_repository(&self) -> Result<Self::CustomEntity362Repository<'_>, ContextError>;
    type CustomEntity363Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_363_repository(&self) -> Result<Self::CustomEntity363Repository<'_>, ContextError>;
    type CustomEntity364Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_364_repository(&self) -> Result<Self::CustomEntity364Repository<'_>, ContextError>;
    type CustomEntity365Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_365_repository(&self) -> Result<Self::CustomEntity365Repository<'_>, ContextError>;
    type CustomEntity366Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_366_repository(&self) -> Result<Self::CustomEntity366Repository<'_>, ContextError>;
    type CustomEntity367Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_367_repository(&self) -> Result<Self::CustomEntity367Repository<'_>, ContextError>;
    type CustomEntity368Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_368_repository(&self) -> Result<Self::CustomEntity368Repository<'_>, ContextError>;
    type CustomEntity369Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_369_repository(&self) -> Result<Self::CustomEntity369Repository<'_>, ContextError>;
    type CustomEntity370Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_370_repository(&self) -> Result<Self::CustomEntity370Repository<'_>, ContextError>;
    type CustomEntity371Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_371_repository(&self) -> Result<Self::CustomEntity371Repository<'_>, ContextError>;
    type CustomEntity372Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_372_repository(&self) -> Result<Self::CustomEntity372Repository<'_>, ContextError>;
    type CustomEntity373Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_373_repository(&self) -> Result<Self::CustomEntity373Repository<'_>, ContextError>;
    type CustomEntity374Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_374_repository(&self) -> Result<Self::CustomEntity374Repository<'_>, ContextError>;
    type CustomEntity375Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_375_repository(&self) -> Result<Self::CustomEntity375Repository<'_>, ContextError>;
    type CustomEntity376Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_376_repository(&self) -> Result<Self::CustomEntity376Repository<'_>, ContextError>;
    type CustomEntity377Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_377_repository(&self) -> Result<Self::CustomEntity377Repository<'_>, ContextError>;
    type CustomEntity378Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_378_repository(&self) -> Result<Self::CustomEntity378Repository<'_>, ContextError>;
    type CustomEntity379Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_379_repository(&self) -> Result<Self::CustomEntity379Repository<'_>, ContextError>;
    type CustomEntity380Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_380_repository(&self) -> Result<Self::CustomEntity380Repository<'_>, ContextError>;
    type CustomEntity381Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_381_repository(&self) -> Result<Self::CustomEntity381Repository<'_>, ContextError>;
    type CustomEntity382Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_382_repository(&self) -> Result<Self::CustomEntity382Repository<'_>, ContextError>;
    type CustomEntity383Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_383_repository(&self) -> Result<Self::CustomEntity383Repository<'_>, ContextError>;
    type CustomEntity384Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_384_repository(&self) -> Result<Self::CustomEntity384Repository<'_>, ContextError>;
    type CustomEntity385Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_385_repository(&self) -> Result<Self::CustomEntity385Repository<'_>, ContextError>;
    type CustomEntity386Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_386_repository(&self) -> Result<Self::CustomEntity386Repository<'_>, ContextError>;
    type CustomEntity387Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_387_repository(&self) -> Result<Self::CustomEntity387Repository<'_>, ContextError>;
    type CustomEntity388Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_388_repository(&self) -> Result<Self::CustomEntity388Repository<'_>, ContextError>;
    type CustomEntity389Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_389_repository(&self) -> Result<Self::CustomEntity389Repository<'_>, ContextError>;
    type CustomEntity390Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_390_repository(&self) -> Result<Self::CustomEntity390Repository<'_>, ContextError>;
    type CustomEntity391Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_391_repository(&self) -> Result<Self::CustomEntity391Repository<'_>, ContextError>;
    type CustomEntity392Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_392_repository(&self) -> Result<Self::CustomEntity392Repository<'_>, ContextError>;
    type CustomEntity393Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_393_repository(&self) -> Result<Self::CustomEntity393Repository<'_>, ContextError>;
    type CustomEntity394Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_394_repository(&self) -> Result<Self::CustomEntity394Repository<'_>, ContextError>;
    type CustomEntity395Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_395_repository(&self) -> Result<Self::CustomEntity395Repository<'_>, ContextError>;
    type CustomEntity396Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_396_repository(&self) -> Result<Self::CustomEntity396Repository<'_>, ContextError>;
    type CustomEntity397Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_397_repository(&self) -> Result<Self::CustomEntity397Repository<'_>, ContextError>;
    type CustomEntity398Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_398_repository(&self) -> Result<Self::CustomEntity398Repository<'_>, ContextError>;
    type CustomEntity399Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_399_repository(&self) -> Result<Self::CustomEntity399Repository<'_>, ContextError>;
    type CustomEntity400Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_400_repository(&self) -> Result<Self::CustomEntity400Repository<'_>, ContextError>;
    type CustomEntity401Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_401_repository(&self) -> Result<Self::CustomEntity401Repository<'_>, ContextError>;
    type CustomEntity402Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_402_repository(&self) -> Result<Self::CustomEntity402Repository<'_>, ContextError>;
    type CustomEntity403Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_403_repository(&self) -> Result<Self::CustomEntity403Repository<'_>, ContextError>;
    type CustomEntity404Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_404_repository(&self) -> Result<Self::CustomEntity404Repository<'_>, ContextError>;
    type CustomEntity405Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_405_repository(&self) -> Result<Self::CustomEntity405Repository<'_>, ContextError>;
    type CustomEntity406Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_406_repository(&self) -> Result<Self::CustomEntity406Repository<'_>, ContextError>;
    type CustomEntity407Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_407_repository(&self) -> Result<Self::CustomEntity407Repository<'_>, ContextError>;
    type CustomEntity408Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_408_repository(&self) -> Result<Self::CustomEntity408Repository<'_>, ContextError>;
    type CustomEntity409Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_409_repository(&self) -> Result<Self::CustomEntity409Repository<'_>, ContextError>;
    type CustomEntity410Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_410_repository(&self) -> Result<Self::CustomEntity410Repository<'_>, ContextError>;
    type CustomEntity411Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_411_repository(&self) -> Result<Self::CustomEntity411Repository<'_>, ContextError>;
    type CustomEntity412Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_412_repository(&self) -> Result<Self::CustomEntity412Repository<'_>, ContextError>;
    type CustomEntity413Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_413_repository(&self) -> Result<Self::CustomEntity413Repository<'_>, ContextError>;
    type CustomEntity414Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_414_repository(&self) -> Result<Self::CustomEntity414Repository<'_>, ContextError>;
    type CustomEntity415Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_415_repository(&self) -> Result<Self::CustomEntity415Repository<'_>, ContextError>;
    type CustomEntity416Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_416_repository(&self) -> Result<Self::CustomEntity416Repository<'_>, ContextError>;
    type CustomEntity417Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_417_repository(&self) -> Result<Self::CustomEntity417Repository<'_>, ContextError>;
    type CustomEntity418Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_418_repository(&self) -> Result<Self::CustomEntity418Repository<'_>, ContextError>;
    type CustomEntity419Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_419_repository(&self) -> Result<Self::CustomEntity419Repository<'_>, ContextError>;
    type CustomEntity420Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_420_repository(&self) -> Result<Self::CustomEntity420Repository<'_>, ContextError>;
    type CustomEntity421Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_421_repository(&self) -> Result<Self::CustomEntity421Repository<'_>, ContextError>;
    type CustomEntity422Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_422_repository(&self) -> Result<Self::CustomEntity422Repository<'_>, ContextError>;
    type CustomEntity423Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_423_repository(&self) -> Result<Self::CustomEntity423Repository<'_>, ContextError>;
    type CustomEntity424Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_424_repository(&self) -> Result<Self::CustomEntity424Repository<'_>, ContextError>;
    type CustomEntity425Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_425_repository(&self) -> Result<Self::CustomEntity425Repository<'_>, ContextError>;
    type CustomEntity426Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_426_repository(&self) -> Result<Self::CustomEntity426Repository<'_>, ContextError>;
    type CustomEntity427Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_427_repository(&self) -> Result<Self::CustomEntity427Repository<'_>, ContextError>;
    type CustomEntity428Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_428_repository(&self) -> Result<Self::CustomEntity428Repository<'_>, ContextError>;
    type CustomEntity429Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_429_repository(&self) -> Result<Self::CustomEntity429Repository<'_>, ContextError>;
    type CustomEntity430Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_430_repository(&self) -> Result<Self::CustomEntity430Repository<'_>, ContextError>;
    type CustomEntity431Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_431_repository(&self) -> Result<Self::CustomEntity431Repository<'_>, ContextError>;
    type CustomEntity432Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_432_repository(&self) -> Result<Self::CustomEntity432Repository<'_>, ContextError>;
    type CustomEntity433Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_433_repository(&self) -> Result<Self::CustomEntity433Repository<'_>, ContextError>;
    type CustomEntity434Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_434_repository(&self) -> Result<Self::CustomEntity434Repository<'_>, ContextError>;
    type CustomEntity435Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_435_repository(&self) -> Result<Self::CustomEntity435Repository<'_>, ContextError>;
    type CustomEntity436Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_436_repository(&self) -> Result<Self::CustomEntity436Repository<'_>, ContextError>;
    type CustomEntity437Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_437_repository(&self) -> Result<Self::CustomEntity437Repository<'_>, ContextError>;
    type CustomEntity438Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_438_repository(&self) -> Result<Self::CustomEntity438Repository<'_>, ContextError>;
    type CustomEntity439Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_439_repository(&self) -> Result<Self::CustomEntity439Repository<'_>, ContextError>;
    type CustomEntity440Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_440_repository(&self) -> Result<Self::CustomEntity440Repository<'_>, ContextError>;
    type CustomEntity441Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_441_repository(&self) -> Result<Self::CustomEntity441Repository<'_>, ContextError>;
    type CustomEntity442Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_442_repository(&self) -> Result<Self::CustomEntity442Repository<'_>, ContextError>;
    type CustomEntity443Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_443_repository(&self) -> Result<Self::CustomEntity443Repository<'_>, ContextError>;
    type CustomEntity444Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_444_repository(&self) -> Result<Self::CustomEntity444Repository<'_>, ContextError>;
    type CustomEntity445Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_445_repository(&self) -> Result<Self::CustomEntity445Repository<'_>, ContextError>;
    type CustomEntity446Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_446_repository(&self) -> Result<Self::CustomEntity446Repository<'_>, ContextError>;
    type CustomEntity447Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_447_repository(&self) -> Result<Self::CustomEntity447Repository<'_>, ContextError>;
    type CustomEntity448Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_448_repository(&self) -> Result<Self::CustomEntity448Repository<'_>, ContextError>;
    type CustomEntity449Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_449_repository(&self) -> Result<Self::CustomEntity449Repository<'_>, ContextError>;
    type CustomEntity450Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_450_repository(&self) -> Result<Self::CustomEntity450Repository<'_>, ContextError>;
    type CustomEntity451Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_451_repository(&self) -> Result<Self::CustomEntity451Repository<'_>, ContextError>;
    type CustomEntity452Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_452_repository(&self) -> Result<Self::CustomEntity452Repository<'_>, ContextError>;
    type CustomEntity453Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_453_repository(&self) -> Result<Self::CustomEntity453Repository<'_>, ContextError>;
    type CustomEntity454Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_454_repository(&self) -> Result<Self::CustomEntity454Repository<'_>, ContextError>;
    type CustomEntity455Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_455_repository(&self) -> Result<Self::CustomEntity455Repository<'_>, ContextError>;
    type CustomEntity456Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_456_repository(&self) -> Result<Self::CustomEntity456Repository<'_>, ContextError>;
    type CustomEntity457Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_457_repository(&self) -> Result<Self::CustomEntity457Repository<'_>, ContextError>;
    type CustomEntity458Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_458_repository(&self) -> Result<Self::CustomEntity458Repository<'_>, ContextError>;
    type CustomEntity459Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_459_repository(&self) -> Result<Self::CustomEntity459Repository<'_>, ContextError>;
    type CustomEntity460Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_460_repository(&self) -> Result<Self::CustomEntity460Repository<'_>, ContextError>;
    type CustomEntity461Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_461_repository(&self) -> Result<Self::CustomEntity461Repository<'_>, ContextError>;
    type CustomEntity462Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_462_repository(&self) -> Result<Self::CustomEntity462Repository<'_>, ContextError>;
    type CustomEntity463Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_463_repository(&self) -> Result<Self::CustomEntity463Repository<'_>, ContextError>;
    type CustomEntity464Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_464_repository(&self) -> Result<Self::CustomEntity464Repository<'_>, ContextError>;
    type CustomEntity465Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_465_repository(&self) -> Result<Self::CustomEntity465Repository<'_>, ContextError>;
    type CustomEntity466Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_466_repository(&self) -> Result<Self::CustomEntity466Repository<'_>, ContextError>;
    type CustomEntity467Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_467_repository(&self) -> Result<Self::CustomEntity467Repository<'_>, ContextError>;
    type CustomEntity468Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_468_repository(&self) -> Result<Self::CustomEntity468Repository<'_>, ContextError>;
    type CustomEntity469Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_469_repository(&self) -> Result<Self::CustomEntity469Repository<'_>, ContextError>;
    type CustomEntity470Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_470_repository(&self) -> Result<Self::CustomEntity470Repository<'_>, ContextError>;
    type CustomEntity471Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_471_repository(&self) -> Result<Self::CustomEntity471Repository<'_>, ContextError>;
    type CustomEntity472Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_472_repository(&self) -> Result<Self::CustomEntity472Repository<'_>, ContextError>;
    type CustomEntity473Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_473_repository(&self) -> Result<Self::CustomEntity473Repository<'_>, ContextError>;
    type CustomEntity474Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_474_repository(&self) -> Result<Self::CustomEntity474Repository<'_>, ContextError>;
    type CustomEntity475Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_475_repository(&self) -> Result<Self::CustomEntity475Repository<'_>, ContextError>;
    type CustomEntity476Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_476_repository(&self) -> Result<Self::CustomEntity476Repository<'_>, ContextError>;
    type CustomEntity477Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_477_repository(&self) -> Result<Self::CustomEntity477Repository<'_>, ContextError>;
    type CustomEntity478Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_478_repository(&self) -> Result<Self::CustomEntity478Repository<'_>, ContextError>;
    type CustomEntity479Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_479_repository(&self) -> Result<Self::CustomEntity479Repository<'_>, ContextError>;
    type CustomEntity480Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_480_repository(&self) -> Result<Self::CustomEntity480Repository<'_>, ContextError>;
    type CustomEntity481Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_481_repository(&self) -> Result<Self::CustomEntity481Repository<'_>, ContextError>;
    type CustomEntity482Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_482_repository(&self) -> Result<Self::CustomEntity482Repository<'_>, ContextError>;
    type CustomEntity483Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_483_repository(&self) -> Result<Self::CustomEntity483Repository<'_>, ContextError>;
    type CustomEntity484Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_484_repository(&self) -> Result<Self::CustomEntity484Repository<'_>, ContextError>;
    type CustomEntity485Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_485_repository(&self) -> Result<Self::CustomEntity485Repository<'_>, ContextError>;
    type CustomEntity486Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_486_repository(&self) -> Result<Self::CustomEntity486Repository<'_>, ContextError>;
    type CustomEntity487Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_487_repository(&self) -> Result<Self::CustomEntity487Repository<'_>, ContextError>;
    type CustomEntity488Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_488_repository(&self) -> Result<Self::CustomEntity488Repository<'_>, ContextError>;
    type CustomEntity489Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_489_repository(&self) -> Result<Self::CustomEntity489Repository<'_>, ContextError>;
    type CustomEntity490Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_490_repository(&self) -> Result<Self::CustomEntity490Repository<'_>, ContextError>;
    type CustomEntity491Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_491_repository(&self) -> Result<Self::CustomEntity491Repository<'_>, ContextError>;
    type CustomEntity492Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_492_repository(&self) -> Result<Self::CustomEntity492Repository<'_>, ContextError>;
    type CustomEntity493Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_493_repository(&self) -> Result<Self::CustomEntity493Repository<'_>, ContextError>;
    type CustomEntity494Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_494_repository(&self) -> Result<Self::CustomEntity494Repository<'_>, ContextError>;
    type CustomEntity495Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_495_repository(&self) -> Result<Self::CustomEntity495Repository<'_>, ContextError>;
    type CustomEntity496Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_496_repository(&self) -> Result<Self::CustomEntity496Repository<'_>, ContextError>;
    type CustomEntity497Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_497_repository(&self) -> Result<Self::CustomEntity497Repository<'_>, ContextError>;
    type CustomEntity498Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_498_repository(&self) -> Result<Self::CustomEntity498Repository<'_>, ContextError>;
    type CustomEntity499Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_499_repository(&self) -> Result<Self::CustomEntity499Repository<'_>, ContextError>;
    type CustomEntity500Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_500_repository(&self) -> Result<Self::CustomEntity500Repository<'_>, ContextError>;
    type CustomEntity501Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_501_repository(&self) -> Result<Self::CustomEntity501Repository<'_>, ContextError>;
    type CustomEntity502Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_502_repository(&self) -> Result<Self::CustomEntity502Repository<'_>, ContextError>;
    type CustomEntity503Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_503_repository(&self) -> Result<Self::CustomEntity503Repository<'_>, ContextError>;
    type CustomEntity504Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_504_repository(&self) -> Result<Self::CustomEntity504Repository<'_>, ContextError>;
    type CustomEntity505Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_505_repository(&self) -> Result<Self::CustomEntity505Repository<'_>, ContextError>;
    type CustomEntity506Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_506_repository(&self) -> Result<Self::CustomEntity506Repository<'_>, ContextError>;
    type CustomEntity507Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_507_repository(&self) -> Result<Self::CustomEntity507Repository<'_>, ContextError>;
    type CustomEntity508Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_508_repository(&self) -> Result<Self::CustomEntity508Repository<'_>, ContextError>;
    type CustomEntity509Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_509_repository(&self) -> Result<Self::CustomEntity509Repository<'_>, ContextError>;
    type CustomEntity510Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_510_repository(&self) -> Result<Self::CustomEntity510Repository<'_>, ContextError>;
    type CustomEntity511Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_511_repository(&self) -> Result<Self::CustomEntity511Repository<'_>, ContextError>;
    type CustomEntity512Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_512_repository(&self) -> Result<Self::CustomEntity512Repository<'_>, ContextError>;
    type CustomEntity513Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_513_repository(&self) -> Result<Self::CustomEntity513Repository<'_>, ContextError>;
    type CustomEntity514Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_514_repository(&self) -> Result<Self::CustomEntity514Repository<'_>, ContextError>;
    type CustomEntity515Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_515_repository(&self) -> Result<Self::CustomEntity515Repository<'_>, ContextError>;
    type CustomEntity516Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_516_repository(&self) -> Result<Self::CustomEntity516Repository<'_>, ContextError>;
    type CustomEntity517Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_517_repository(&self) -> Result<Self::CustomEntity517Repository<'_>, ContextError>;
    type CustomEntity518Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_518_repository(&self) -> Result<Self::CustomEntity518Repository<'_>, ContextError>;
    type CustomEntity519Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_519_repository(&self) -> Result<Self::CustomEntity519Repository<'_>, ContextError>;
    type CustomEntity520Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_520_repository(&self) -> Result<Self::CustomEntity520Repository<'_>, ContextError>;
    type CustomEntity521Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_521_repository(&self) -> Result<Self::CustomEntity521Repository<'_>, ContextError>;
    type CustomEntity522Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_522_repository(&self) -> Result<Self::CustomEntity522Repository<'_>, ContextError>;
    type CustomEntity523Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_523_repository(&self) -> Result<Self::CustomEntity523Repository<'_>, ContextError>;
    type CustomEntity524Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_524_repository(&self) -> Result<Self::CustomEntity524Repository<'_>, ContextError>;
    type CustomEntity525Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_525_repository(&self) -> Result<Self::CustomEntity525Repository<'_>, ContextError>;
    type CustomEntity526Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_526_repository(&self) -> Result<Self::CustomEntity526Repository<'_>, ContextError>;
    type CustomEntity527Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_527_repository(&self) -> Result<Self::CustomEntity527Repository<'_>, ContextError>;
    type CustomEntity528Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_528_repository(&self) -> Result<Self::CustomEntity528Repository<'_>, ContextError>;
    type CustomEntity529Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_529_repository(&self) -> Result<Self::CustomEntity529Repository<'_>, ContextError>;
    type CustomEntity530Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_530_repository(&self) -> Result<Self::CustomEntity530Repository<'_>, ContextError>;
    type CustomEntity531Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_531_repository(&self) -> Result<Self::CustomEntity531Repository<'_>, ContextError>;
    type CustomEntity532Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_532_repository(&self) -> Result<Self::CustomEntity532Repository<'_>, ContextError>;
    type CustomEntity533Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_533_repository(&self) -> Result<Self::CustomEntity533Repository<'_>, ContextError>;
    type CustomEntity534Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_534_repository(&self) -> Result<Self::CustomEntity534Repository<'_>, ContextError>;
    type CustomEntity535Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_535_repository(&self) -> Result<Self::CustomEntity535Repository<'_>, ContextError>;
    type CustomEntity536Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_536_repository(&self) -> Result<Self::CustomEntity536Repository<'_>, ContextError>;
    type CustomEntity537Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_537_repository(&self) -> Result<Self::CustomEntity537Repository<'_>, ContextError>;
    type CustomEntity538Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_538_repository(&self) -> Result<Self::CustomEntity538Repository<'_>, ContextError>;
    type CustomEntity539Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_539_repository(&self) -> Result<Self::CustomEntity539Repository<'_>, ContextError>;
    type CustomEntity540Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_540_repository(&self) -> Result<Self::CustomEntity540Repository<'_>, ContextError>;
    type CustomEntity541Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_541_repository(&self) -> Result<Self::CustomEntity541Repository<'_>, ContextError>;
    type CustomEntity542Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_542_repository(&self) -> Result<Self::CustomEntity542Repository<'_>, ContextError>;
    type CustomEntity543Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_543_repository(&self) -> Result<Self::CustomEntity543Repository<'_>, ContextError>;
    type CustomEntity544Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_544_repository(&self) -> Result<Self::CustomEntity544Repository<'_>, ContextError>;
    type CustomEntity545Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_545_repository(&self) -> Result<Self::CustomEntity545Repository<'_>, ContextError>;
    type CustomEntity546Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_546_repository(&self) -> Result<Self::CustomEntity546Repository<'_>, ContextError>;
    type CustomEntity547Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_547_repository(&self) -> Result<Self::CustomEntity547Repository<'_>, ContextError>;
    type CustomEntity548Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_548_repository(&self) -> Result<Self::CustomEntity548Repository<'_>, ContextError>;
    type CustomEntity549Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_549_repository(&self) -> Result<Self::CustomEntity549Repository<'_>, ContextError>;
    type CustomEntity550Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_550_repository(&self) -> Result<Self::CustomEntity550Repository<'_>, ContextError>;
    type CustomEntity551Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_551_repository(&self) -> Result<Self::CustomEntity551Repository<'_>, ContextError>;
    type CustomEntity552Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_552_repository(&self) -> Result<Self::CustomEntity552Repository<'_>, ContextError>;
    type CustomEntity553Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_553_repository(&self) -> Result<Self::CustomEntity553Repository<'_>, ContextError>;
    type CustomEntity554Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_554_repository(&self) -> Result<Self::CustomEntity554Repository<'_>, ContextError>;
    type CustomEntity555Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_555_repository(&self) -> Result<Self::CustomEntity555Repository<'_>, ContextError>;
    type CustomEntity556Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_556_repository(&self) -> Result<Self::CustomEntity556Repository<'_>, ContextError>;
    type CustomEntity557Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_557_repository(&self) -> Result<Self::CustomEntity557Repository<'_>, ContextError>;
    type CustomEntity558Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_558_repository(&self) -> Result<Self::CustomEntity558Repository<'_>, ContextError>;
    type CustomEntity559Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_559_repository(&self) -> Result<Self::CustomEntity559Repository<'_>, ContextError>;
    type CustomEntity560Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_560_repository(&self) -> Result<Self::CustomEntity560Repository<'_>, ContextError>;
    type CustomEntity561Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_561_repository(&self) -> Result<Self::CustomEntity561Repository<'_>, ContextError>;
    type CustomEntity562Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_562_repository(&self) -> Result<Self::CustomEntity562Repository<'_>, ContextError>;
    type CustomEntity563Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_563_repository(&self) -> Result<Self::CustomEntity563Repository<'_>, ContextError>;
    type CustomEntity564Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_564_repository(&self) -> Result<Self::CustomEntity564Repository<'_>, ContextError>;
    type CustomEntity565Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_565_repository(&self) -> Result<Self::CustomEntity565Repository<'_>, ContextError>;
    type CustomEntity566Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_566_repository(&self) -> Result<Self::CustomEntity566Repository<'_>, ContextError>;
    type CustomEntity567Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_567_repository(&self) -> Result<Self::CustomEntity567Repository<'_>, ContextError>;
    type CustomEntity568Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_568_repository(&self) -> Result<Self::CustomEntity568Repository<'_>, ContextError>;
    type CustomEntity569Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_569_repository(&self) -> Result<Self::CustomEntity569Repository<'_>, ContextError>;
    type CustomEntity570Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_570_repository(&self) -> Result<Self::CustomEntity570Repository<'_>, ContextError>;
    type CustomEntity571Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_571_repository(&self) -> Result<Self::CustomEntity571Repository<'_>, ContextError>;
    type CustomEntity572Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_572_repository(&self) -> Result<Self::CustomEntity572Repository<'_>, ContextError>;
    type CustomEntity573Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_573_repository(&self) -> Result<Self::CustomEntity573Repository<'_>, ContextError>;
    type CustomEntity574Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_574_repository(&self) -> Result<Self::CustomEntity574Repository<'_>, ContextError>;
    type CustomEntity575Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_575_repository(&self) -> Result<Self::CustomEntity575Repository<'_>, ContextError>;
    type CustomEntity576Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_576_repository(&self) -> Result<Self::CustomEntity576Repository<'_>, ContextError>;
    type CustomEntity577Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_577_repository(&self) -> Result<Self::CustomEntity577Repository<'_>, ContextError>;
    type CustomEntity578Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_578_repository(&self) -> Result<Self::CustomEntity578Repository<'_>, ContextError>;
    type CustomEntity579Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_579_repository(&self) -> Result<Self::CustomEntity579Repository<'_>, ContextError>;
    type CustomEntity580Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_580_repository(&self) -> Result<Self::CustomEntity580Repository<'_>, ContextError>;
    type CustomEntity581Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_581_repository(&self) -> Result<Self::CustomEntity581Repository<'_>, ContextError>;
    type CustomEntity582Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_582_repository(&self) -> Result<Self::CustomEntity582Repository<'_>, ContextError>;
    type CustomEntity583Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_583_repository(&self) -> Result<Self::CustomEntity583Repository<'_>, ContextError>;
    type CustomEntity584Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_584_repository(&self) -> Result<Self::CustomEntity584Repository<'_>, ContextError>;
    type CustomEntity585Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_585_repository(&self) -> Result<Self::CustomEntity585Repository<'_>, ContextError>;
    type CustomEntity586Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_586_repository(&self) -> Result<Self::CustomEntity586Repository<'_>, ContextError>;
    type CustomEntity587Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_587_repository(&self) -> Result<Self::CustomEntity587Repository<'_>, ContextError>;
    type CustomEntity588Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_588_repository(&self) -> Result<Self::CustomEntity588Repository<'_>, ContextError>;
    type CustomEntity589Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_589_repository(&self) -> Result<Self::CustomEntity589Repository<'_>, ContextError>;
    type CustomEntity590Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_590_repository(&self) -> Result<Self::CustomEntity590Repository<'_>, ContextError>;
    type CustomEntity591Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_591_repository(&self) -> Result<Self::CustomEntity591Repository<'_>, ContextError>;
    type CustomEntity592Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_592_repository(&self) -> Result<Self::CustomEntity592Repository<'_>, ContextError>;
    type CustomEntity593Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_593_repository(&self) -> Result<Self::CustomEntity593Repository<'_>, ContextError>;
    type CustomEntity594Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_594_repository(&self) -> Result<Self::CustomEntity594Repository<'_>, ContextError>;
    type CustomEntity595Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_595_repository(&self) -> Result<Self::CustomEntity595Repository<'_>, ContextError>;
    type CustomEntity596Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_596_repository(&self) -> Result<Self::CustomEntity596Repository<'_>, ContextError>;
    type CustomEntity597Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_597_repository(&self) -> Result<Self::CustomEntity597Repository<'_>, ContextError>;
    type CustomEntity598Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_598_repository(&self) -> Result<Self::CustomEntity598Repository<'_>, ContextError>;
    type CustomEntity599Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_599_repository(&self) -> Result<Self::CustomEntity599Repository<'_>, ContextError>;
    type CustomEntity600Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_600_repository(&self) -> Result<Self::CustomEntity600Repository<'_>, ContextError>;
    type CustomEntity601Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_601_repository(&self) -> Result<Self::CustomEntity601Repository<'_>, ContextError>;
    type CustomEntity602Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_602_repository(&self) -> Result<Self::CustomEntity602Repository<'_>, ContextError>;
    type CustomEntity603Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_603_repository(&self) -> Result<Self::CustomEntity603Repository<'_>, ContextError>;
    type CustomEntity604Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_604_repository(&self) -> Result<Self::CustomEntity604Repository<'_>, ContextError>;
    type CustomEntity605Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_605_repository(&self) -> Result<Self::CustomEntity605Repository<'_>, ContextError>;
    type CustomEntity606Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_606_repository(&self) -> Result<Self::CustomEntity606Repository<'_>, ContextError>;
    type CustomEntity607Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_607_repository(&self) -> Result<Self::CustomEntity607Repository<'_>, ContextError>;
    type CustomEntity608Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_608_repository(&self) -> Result<Self::CustomEntity608Repository<'_>, ContextError>;
    type CustomEntity609Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_609_repository(&self) -> Result<Self::CustomEntity609Repository<'_>, ContextError>;
    type CustomEntity610Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_610_repository(&self) -> Result<Self::CustomEntity610Repository<'_>, ContextError>;
    type CustomEntity611Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_611_repository(&self) -> Result<Self::CustomEntity611Repository<'_>, ContextError>;
    type CustomEntity612Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_612_repository(&self) -> Result<Self::CustomEntity612Repository<'_>, ContextError>;
    type CustomEntity613Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_613_repository(&self) -> Result<Self::CustomEntity613Repository<'_>, ContextError>;
    type CustomEntity614Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_614_repository(&self) -> Result<Self::CustomEntity614Repository<'_>, ContextError>;
    type CustomEntity615Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_615_repository(&self) -> Result<Self::CustomEntity615Repository<'_>, ContextError>;
    type CustomEntity616Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_616_repository(&self) -> Result<Self::CustomEntity616Repository<'_>, ContextError>;
    type CustomEntity617Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_617_repository(&self) -> Result<Self::CustomEntity617Repository<'_>, ContextError>;
    type CustomEntity618Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_618_repository(&self) -> Result<Self::CustomEntity618Repository<'_>, ContextError>;
    type CustomEntity619Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_619_repository(&self) -> Result<Self::CustomEntity619Repository<'_>, ContextError>;
    type CustomEntity620Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_620_repository(&self) -> Result<Self::CustomEntity620Repository<'_>, ContextError>;
    type CustomEntity621Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_621_repository(&self) -> Result<Self::CustomEntity621Repository<'_>, ContextError>;
    type CustomEntity622Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_622_repository(&self) -> Result<Self::CustomEntity622Repository<'_>, ContextError>;
    type CustomEntity623Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_623_repository(&self) -> Result<Self::CustomEntity623Repository<'_>, ContextError>;
    type CustomEntity624Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_624_repository(&self) -> Result<Self::CustomEntity624Repository<'_>, ContextError>;
    type CustomEntity625Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_625_repository(&self) -> Result<Self::CustomEntity625Repository<'_>, ContextError>;
    type CustomEntity626Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_626_repository(&self) -> Result<Self::CustomEntity626Repository<'_>, ContextError>;
    type CustomEntity627Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_627_repository(&self) -> Result<Self::CustomEntity627Repository<'_>, ContextError>;
    type CustomEntity628Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_628_repository(&self) -> Result<Self::CustomEntity628Repository<'_>, ContextError>;
    type CustomEntity629Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_629_repository(&self) -> Result<Self::CustomEntity629Repository<'_>, ContextError>;
    type CustomEntity630Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_630_repository(&self) -> Result<Self::CustomEntity630Repository<'_>, ContextError>;
    type CustomEntity631Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_631_repository(&self) -> Result<Self::CustomEntity631Repository<'_>, ContextError>;
    type CustomEntity632Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_632_repository(&self) -> Result<Self::CustomEntity632Repository<'_>, ContextError>;
    type CustomEntity633Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_633_repository(&self) -> Result<Self::CustomEntity633Repository<'_>, ContextError>;
    type CustomEntity634Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_634_repository(&self) -> Result<Self::CustomEntity634Repository<'_>, ContextError>;
    type CustomEntity635Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_635_repository(&self) -> Result<Self::CustomEntity635Repository<'_>, ContextError>;
    type CustomEntity636Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_636_repository(&self) -> Result<Self::CustomEntity636Repository<'_>, ContextError>;
    type CustomEntity637Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_637_repository(&self) -> Result<Self::CustomEntity637Repository<'_>, ContextError>;
    type CustomEntity638Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_638_repository(&self) -> Result<Self::CustomEntity638Repository<'_>, ContextError>;
    type CustomEntity639Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_639_repository(&self) -> Result<Self::CustomEntity639Repository<'_>, ContextError>;
    type CustomEntity640Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_640_repository(&self) -> Result<Self::CustomEntity640Repository<'_>, ContextError>;
    type CustomEntity641Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_641_repository(&self) -> Result<Self::CustomEntity641Repository<'_>, ContextError>;
    type CustomEntity642Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_642_repository(&self) -> Result<Self::CustomEntity642Repository<'_>, ContextError>;
    type CustomEntity643Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_643_repository(&self) -> Result<Self::CustomEntity643Repository<'_>, ContextError>;
    type CustomEntity644Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_644_repository(&self) -> Result<Self::CustomEntity644Repository<'_>, ContextError>;
    type CustomEntity645Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_645_repository(&self) -> Result<Self::CustomEntity645Repository<'_>, ContextError>;
    type CustomEntity646Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_646_repository(&self) -> Result<Self::CustomEntity646Repository<'_>, ContextError>;
    type CustomEntity647Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_647_repository(&self) -> Result<Self::CustomEntity647Repository<'_>, ContextError>;
    type CustomEntity648Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_648_repository(&self) -> Result<Self::CustomEntity648Repository<'_>, ContextError>;
    type CustomEntity649Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_649_repository(&self) -> Result<Self::CustomEntity649Repository<'_>, ContextError>;
    type CustomEntity650Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_650_repository(&self) -> Result<Self::CustomEntity650Repository<'_>, ContextError>;
    type CustomEntity651Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_651_repository(&self) -> Result<Self::CustomEntity651Repository<'_>, ContextError>;
    type CustomEntity652Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_652_repository(&self) -> Result<Self::CustomEntity652Repository<'_>, ContextError>;
    type CustomEntity653Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_653_repository(&self) -> Result<Self::CustomEntity653Repository<'_>, ContextError>;
    type CustomEntity654Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_654_repository(&self) -> Result<Self::CustomEntity654Repository<'_>, ContextError>;
    type CustomEntity655Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_655_repository(&self) -> Result<Self::CustomEntity655Repository<'_>, ContextError>;
    type CustomEntity656Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_656_repository(&self) -> Result<Self::CustomEntity656Repository<'_>, ContextError>;
    type CustomEntity657Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_657_repository(&self) -> Result<Self::CustomEntity657Repository<'_>, ContextError>;
    type CustomEntity658Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_658_repository(&self) -> Result<Self::CustomEntity658Repository<'_>, ContextError>;
    type CustomEntity659Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_659_repository(&self) -> Result<Self::CustomEntity659Repository<'_>, ContextError>;
    type CustomEntity660Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_660_repository(&self) -> Result<Self::CustomEntity660Repository<'_>, ContextError>;
    type CustomEntity661Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_661_repository(&self) -> Result<Self::CustomEntity661Repository<'_>, ContextError>;
    type CustomEntity662Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_662_repository(&self) -> Result<Self::CustomEntity662Repository<'_>, ContextError>;
    type CustomEntity663Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_663_repository(&self) -> Result<Self::CustomEntity663Repository<'_>, ContextError>;
    type CustomEntity664Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_664_repository(&self) -> Result<Self::CustomEntity664Repository<'_>, ContextError>;
    type CustomEntity665Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_665_repository(&self) -> Result<Self::CustomEntity665Repository<'_>, ContextError>;
    type CustomEntity666Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_666_repository(&self) -> Result<Self::CustomEntity666Repository<'_>, ContextError>;
    type CustomEntity667Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_667_repository(&self) -> Result<Self::CustomEntity667Repository<'_>, ContextError>;
    type CustomEntity668Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_668_repository(&self) -> Result<Self::CustomEntity668Repository<'_>, ContextError>;
    type CustomEntity669Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_669_repository(&self) -> Result<Self::CustomEntity669Repository<'_>, ContextError>;
    type CustomEntity670Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_670_repository(&self) -> Result<Self::CustomEntity670Repository<'_>, ContextError>;
    type CustomEntity671Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_671_repository(&self) -> Result<Self::CustomEntity671Repository<'_>, ContextError>;
    type CustomEntity672Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_672_repository(&self) -> Result<Self::CustomEntity672Repository<'_>, ContextError>;
    type CustomEntity673Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_673_repository(&self) -> Result<Self::CustomEntity673Repository<'_>, ContextError>;
    type CustomEntity674Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_674_repository(&self) -> Result<Self::CustomEntity674Repository<'_>, ContextError>;
    type CustomEntity675Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_675_repository(&self) -> Result<Self::CustomEntity675Repository<'_>, ContextError>;
    type CustomEntity676Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_676_repository(&self) -> Result<Self::CustomEntity676Repository<'_>, ContextError>;
    type CustomEntity677Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_677_repository(&self) -> Result<Self::CustomEntity677Repository<'_>, ContextError>;
    type CustomEntity678Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_678_repository(&self) -> Result<Self::CustomEntity678Repository<'_>, ContextError>;
    type CustomEntity679Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_679_repository(&self) -> Result<Self::CustomEntity679Repository<'_>, ContextError>;
    type CustomEntity680Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_680_repository(&self) -> Result<Self::CustomEntity680Repository<'_>, ContextError>;
    type CustomEntity681Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_681_repository(&self) -> Result<Self::CustomEntity681Repository<'_>, ContextError>;
    type CustomEntity682Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_682_repository(&self) -> Result<Self::CustomEntity682Repository<'_>, ContextError>;
    type CustomEntity683Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_683_repository(&self) -> Result<Self::CustomEntity683Repository<'_>, ContextError>;
    type CustomEntity684Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_684_repository(&self) -> Result<Self::CustomEntity684Repository<'_>, ContextError>;
    type CustomEntity685Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_685_repository(&self) -> Result<Self::CustomEntity685Repository<'_>, ContextError>;
    type CustomEntity686Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_686_repository(&self) -> Result<Self::CustomEntity686Repository<'_>, ContextError>;
    type CustomEntity687Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_687_repository(&self) -> Result<Self::CustomEntity687Repository<'_>, ContextError>;
    type CustomEntity688Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_688_repository(&self) -> Result<Self::CustomEntity688Repository<'_>, ContextError>;
    type CustomEntity689Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_689_repository(&self) -> Result<Self::CustomEntity689Repository<'_>, ContextError>;
    type CustomEntity690Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_690_repository(&self) -> Result<Self::CustomEntity690Repository<'_>, ContextError>;
    type CustomEntity691Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_691_repository(&self) -> Result<Self::CustomEntity691Repository<'_>, ContextError>;
    type CustomEntity692Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_692_repository(&self) -> Result<Self::CustomEntity692Repository<'_>, ContextError>;
    type CustomEntity693Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_693_repository(&self) -> Result<Self::CustomEntity693Repository<'_>, ContextError>;
    type CustomEntity694Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_694_repository(&self) -> Result<Self::CustomEntity694Repository<'_>, ContextError>;
    type CustomEntity695Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_695_repository(&self) -> Result<Self::CustomEntity695Repository<'_>, ContextError>;
    type CustomEntity696Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_696_repository(&self) -> Result<Self::CustomEntity696Repository<'_>, ContextError>;
    type CustomEntity697Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_697_repository(&self) -> Result<Self::CustomEntity697Repository<'_>, ContextError>;
    type CustomEntity698Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_698_repository(&self) -> Result<Self::CustomEntity698Repository<'_>, ContextError>;
    type CustomEntity699Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_699_repository(&self) -> Result<Self::CustomEntity699Repository<'_>, ContextError>;
    type CustomEntity700Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_700_repository(&self) -> Result<Self::CustomEntity700Repository<'_>, ContextError>;
    type CustomEntity701Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_701_repository(&self) -> Result<Self::CustomEntity701Repository<'_>, ContextError>;
    type CustomEntity702Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_702_repository(&self) -> Result<Self::CustomEntity702Repository<'_>, ContextError>;
    type CustomEntity703Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_703_repository(&self) -> Result<Self::CustomEntity703Repository<'_>, ContextError>;
    type CustomEntity704Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_704_repository(&self) -> Result<Self::CustomEntity704Repository<'_>, ContextError>;
    type CustomEntity705Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_705_repository(&self) -> Result<Self::CustomEntity705Repository<'_>, ContextError>;
    type CustomEntity706Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_706_repository(&self) -> Result<Self::CustomEntity706Repository<'_>, ContextError>;
    type CustomEntity707Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_707_repository(&self) -> Result<Self::CustomEntity707Repository<'_>, ContextError>;
    type CustomEntity708Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_708_repository(&self) -> Result<Self::CustomEntity708Repository<'_>, ContextError>;
    type CustomEntity709Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_709_repository(&self) -> Result<Self::CustomEntity709Repository<'_>, ContextError>;
    type CustomEntity710Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_710_repository(&self) -> Result<Self::CustomEntity710Repository<'_>, ContextError>;
    type CustomEntity711Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_711_repository(&self) -> Result<Self::CustomEntity711Repository<'_>, ContextError>;
    type CustomEntity712Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_712_repository(&self) -> Result<Self::CustomEntity712Repository<'_>, ContextError>;
    type CustomEntity713Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_713_repository(&self) -> Result<Self::CustomEntity713Repository<'_>, ContextError>;
    type CustomEntity714Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_714_repository(&self) -> Result<Self::CustomEntity714Repository<'_>, ContextError>;
    type CustomEntity715Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_715_repository(&self) -> Result<Self::CustomEntity715Repository<'_>, ContextError>;
    type CustomEntity716Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_716_repository(&self) -> Result<Self::CustomEntity716Repository<'_>, ContextError>;
    type CustomEntity717Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_717_repository(&self) -> Result<Self::CustomEntity717Repository<'_>, ContextError>;
    type CustomEntity718Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_718_repository(&self) -> Result<Self::CustomEntity718Repository<'_>, ContextError>;
    type CustomEntity719Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_719_repository(&self) -> Result<Self::CustomEntity719Repository<'_>, ContextError>;
    type CustomEntity720Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_720_repository(&self) -> Result<Self::CustomEntity720Repository<'_>, ContextError>;
    type CustomEntity721Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_721_repository(&self) -> Result<Self::CustomEntity721Repository<'_>, ContextError>;
    type CustomEntity722Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_722_repository(&self) -> Result<Self::CustomEntity722Repository<'_>, ContextError>;
    type CustomEntity723Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_723_repository(&self) -> Result<Self::CustomEntity723Repository<'_>, ContextError>;
    type CustomEntity724Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_724_repository(&self) -> Result<Self::CustomEntity724Repository<'_>, ContextError>;
    type CustomEntity725Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_725_repository(&self) -> Result<Self::CustomEntity725Repository<'_>, ContextError>;
    type CustomEntity726Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_726_repository(&self) -> Result<Self::CustomEntity726Repository<'_>, ContextError>;
    type CustomEntity727Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_727_repository(&self) -> Result<Self::CustomEntity727Repository<'_>, ContextError>;
    type CustomEntity728Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_728_repository(&self) -> Result<Self::CustomEntity728Repository<'_>, ContextError>;
    type CustomEntity729Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_729_repository(&self) -> Result<Self::CustomEntity729Repository<'_>, ContextError>;
    type CustomEntity730Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_730_repository(&self) -> Result<Self::CustomEntity730Repository<'_>, ContextError>;
    type CustomEntity731Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_731_repository(&self) -> Result<Self::CustomEntity731Repository<'_>, ContextError>;
    type CustomEntity732Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_732_repository(&self) -> Result<Self::CustomEntity732Repository<'_>, ContextError>;
    type CustomEntity733Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_733_repository(&self) -> Result<Self::CustomEntity733Repository<'_>, ContextError>;
    type CustomEntity734Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_734_repository(&self) -> Result<Self::CustomEntity734Repository<'_>, ContextError>;
    type CustomEntity735Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_735_repository(&self) -> Result<Self::CustomEntity735Repository<'_>, ContextError>;
    type CustomEntity736Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_736_repository(&self) -> Result<Self::CustomEntity736Repository<'_>, ContextError>;
    type CustomEntity737Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_737_repository(&self) -> Result<Self::CustomEntity737Repository<'_>, ContextError>;
    type CustomEntity738Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_738_repository(&self) -> Result<Self::CustomEntity738Repository<'_>, ContextError>;
    type CustomEntity739Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_739_repository(&self) -> Result<Self::CustomEntity739Repository<'_>, ContextError>;
    type CustomEntity740Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_740_repository(&self) -> Result<Self::CustomEntity740Repository<'_>, ContextError>;
    type CustomEntity741Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_741_repository(&self) -> Result<Self::CustomEntity741Repository<'_>, ContextError>;
    type CustomEntity742Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_742_repository(&self) -> Result<Self::CustomEntity742Repository<'_>, ContextError>;
    type CustomEntity743Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_743_repository(&self) -> Result<Self::CustomEntity743Repository<'_>, ContextError>;
    type CustomEntity744Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_744_repository(&self) -> Result<Self::CustomEntity744Repository<'_>, ContextError>;
    type CustomEntity745Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_745_repository(&self) -> Result<Self::CustomEntity745Repository<'_>, ContextError>;
    type CustomEntity746Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_746_repository(&self) -> Result<Self::CustomEntity746Repository<'_>, ContextError>;
    type CustomEntity747Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_747_repository(&self) -> Result<Self::CustomEntity747Repository<'_>, ContextError>;
    type CustomEntity748Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_748_repository(&self) -> Result<Self::CustomEntity748Repository<'_>, ContextError>;
    type CustomEntity749Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_749_repository(&self) -> Result<Self::CustomEntity749Repository<'_>, ContextError>;
    type CustomEntity750Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_750_repository(&self) -> Result<Self::CustomEntity750Repository<'_>, ContextError>;
    type CustomEntity751Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_751_repository(&self) -> Result<Self::CustomEntity751Repository<'_>, ContextError>;
    type CustomEntity752Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_752_repository(&self) -> Result<Self::CustomEntity752Repository<'_>, ContextError>;
    type CustomEntity753Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_753_repository(&self) -> Result<Self::CustomEntity753Repository<'_>, ContextError>;
    type CustomEntity754Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_754_repository(&self) -> Result<Self::CustomEntity754Repository<'_>, ContextError>;
    type CustomEntity755Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_755_repository(&self) -> Result<Self::CustomEntity755Repository<'_>, ContextError>;
    type CustomEntity756Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_756_repository(&self) -> Result<Self::CustomEntity756Repository<'_>, ContextError>;
    type CustomEntity757Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_757_repository(&self) -> Result<Self::CustomEntity757Repository<'_>, ContextError>;
    type CustomEntity758Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_758_repository(&self) -> Result<Self::CustomEntity758Repository<'_>, ContextError>;
    type CustomEntity759Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_759_repository(&self) -> Result<Self::CustomEntity759Repository<'_>, ContextError>;
    type CustomEntity760Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_760_repository(&self) -> Result<Self::CustomEntity760Repository<'_>, ContextError>;
    type CustomEntity761Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_761_repository(&self) -> Result<Self::CustomEntity761Repository<'_>, ContextError>;
    type CustomEntity762Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_762_repository(&self) -> Result<Self::CustomEntity762Repository<'_>, ContextError>;
    type CustomEntity763Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_763_repository(&self) -> Result<Self::CustomEntity763Repository<'_>, ContextError>;
    type CustomEntity764Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_764_repository(&self) -> Result<Self::CustomEntity764Repository<'_>, ContextError>;
    type CustomEntity765Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_765_repository(&self) -> Result<Self::CustomEntity765Repository<'_>, ContextError>;
    type CustomEntity766Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_766_repository(&self) -> Result<Self::CustomEntity766Repository<'_>, ContextError>;
    type CustomEntity767Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_767_repository(&self) -> Result<Self::CustomEntity767Repository<'_>, ContextError>;
    type CustomEntity768Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_768_repository(&self) -> Result<Self::CustomEntity768Repository<'_>, ContextError>;
    type CustomEntity769Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_769_repository(&self) -> Result<Self::CustomEntity769Repository<'_>, ContextError>;
    type CustomEntity770Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_770_repository(&self) -> Result<Self::CustomEntity770Repository<'_>, ContextError>;
    type CustomEntity771Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_771_repository(&self) -> Result<Self::CustomEntity771Repository<'_>, ContextError>;
    type CustomEntity772Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_772_repository(&self) -> Result<Self::CustomEntity772Repository<'_>, ContextError>;
    type CustomEntity773Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_773_repository(&self) -> Result<Self::CustomEntity773Repository<'_>, ContextError>;
    type CustomEntity774Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_774_repository(&self) -> Result<Self::CustomEntity774Repository<'_>, ContextError>;
    type CustomEntity775Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_775_repository(&self) -> Result<Self::CustomEntity775Repository<'_>, ContextError>;
    type CustomEntity776Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_776_repository(&self) -> Result<Self::CustomEntity776Repository<'_>, ContextError>;
    type CustomEntity777Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_777_repository(&self) -> Result<Self::CustomEntity777Repository<'_>, ContextError>;
    type CustomEntity778Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_778_repository(&self) -> Result<Self::CustomEntity778Repository<'_>, ContextError>;
    type CustomEntity779Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_779_repository(&self) -> Result<Self::CustomEntity779Repository<'_>, ContextError>;
    type CustomEntity780Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_780_repository(&self) -> Result<Self::CustomEntity780Repository<'_>, ContextError>;
    type CustomEntity781Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_781_repository(&self) -> Result<Self::CustomEntity781Repository<'_>, ContextError>;
    type CustomEntity782Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_782_repository(&self) -> Result<Self::CustomEntity782Repository<'_>, ContextError>;
    type CustomEntity783Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_783_repository(&self) -> Result<Self::CustomEntity783Repository<'_>, ContextError>;
    type CustomEntity784Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_784_repository(&self) -> Result<Self::CustomEntity784Repository<'_>, ContextError>;
    type CustomEntity785Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_785_repository(&self) -> Result<Self::CustomEntity785Repository<'_>, ContextError>;
    type CustomEntity786Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_786_repository(&self) -> Result<Self::CustomEntity786Repository<'_>, ContextError>;
    type CustomEntity787Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_787_repository(&self) -> Result<Self::CustomEntity787Repository<'_>, ContextError>;
    type CustomEntity788Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_788_repository(&self) -> Result<Self::CustomEntity788Repository<'_>, ContextError>;
    type CustomEntity789Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_789_repository(&self) -> Result<Self::CustomEntity789Repository<'_>, ContextError>;
    type CustomEntity790Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_790_repository(&self) -> Result<Self::CustomEntity790Repository<'_>, ContextError>;
    type CustomEntity791Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_791_repository(&self) -> Result<Self::CustomEntity791Repository<'_>, ContextError>;
    type CustomEntity792Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_792_repository(&self) -> Result<Self::CustomEntity792Repository<'_>, ContextError>;
    type CustomEntity793Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_793_repository(&self) -> Result<Self::CustomEntity793Repository<'_>, ContextError>;
    type CustomEntity794Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_794_repository(&self) -> Result<Self::CustomEntity794Repository<'_>, ContextError>;
    type CustomEntity795Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_795_repository(&self) -> Result<Self::CustomEntity795Repository<'_>, ContextError>;
    type CustomEntity796Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_796_repository(&self) -> Result<Self::CustomEntity796Repository<'_>, ContextError>;
    type CustomEntity797Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_797_repository(&self) -> Result<Self::CustomEntity797Repository<'_>, ContextError>;
    type CustomEntity798Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_798_repository(&self) -> Result<Self::CustomEntity798Repository<'_>, ContextError>;
    type CustomEntity799Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_799_repository(&self) -> Result<Self::CustomEntity799Repository<'_>, ContextError>;
    type CustomEntity800Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_800_repository(&self) -> Result<Self::CustomEntity800Repository<'_>, ContextError>;
    type CustomEntity801Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_801_repository(&self) -> Result<Self::CustomEntity801Repository<'_>, ContextError>;
    type CustomEntity802Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_802_repository(&self) -> Result<Self::CustomEntity802Repository<'_>, ContextError>;
    type CustomEntity803Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_803_repository(&self) -> Result<Self::CustomEntity803Repository<'_>, ContextError>;
    type CustomEntity804Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_804_repository(&self) -> Result<Self::CustomEntity804Repository<'_>, ContextError>;
    type CustomEntity805Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_805_repository(&self) -> Result<Self::CustomEntity805Repository<'_>, ContextError>;
    type CustomEntity806Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_806_repository(&self) -> Result<Self::CustomEntity806Repository<'_>, ContextError>;
    type CustomEntity807Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_807_repository(&self) -> Result<Self::CustomEntity807Repository<'_>, ContextError>;
    type CustomEntity808Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_808_repository(&self) -> Result<Self::CustomEntity808Repository<'_>, ContextError>;
    type CustomEntity809Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_809_repository(&self) -> Result<Self::CustomEntity809Repository<'_>, ContextError>;
    type CustomEntity810Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_810_repository(&self) -> Result<Self::CustomEntity810Repository<'_>, ContextError>;
    type CustomEntity811Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_811_repository(&self) -> Result<Self::CustomEntity811Repository<'_>, ContextError>;
    type CustomEntity812Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_812_repository(&self) -> Result<Self::CustomEntity812Repository<'_>, ContextError>;
    type CustomEntity813Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_813_repository(&self) -> Result<Self::CustomEntity813Repository<'_>, ContextError>;
    type CustomEntity814Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_814_repository(&self) -> Result<Self::CustomEntity814Repository<'_>, ContextError>;
    type CustomEntity815Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_815_repository(&self) -> Result<Self::CustomEntity815Repository<'_>, ContextError>;
    type CustomEntity816Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_816_repository(&self) -> Result<Self::CustomEntity816Repository<'_>, ContextError>;
    type CustomEntity817Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_817_repository(&self) -> Result<Self::CustomEntity817Repository<'_>, ContextError>;
    type CustomEntity818Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_818_repository(&self) -> Result<Self::CustomEntity818Repository<'_>, ContextError>;
    type CustomEntity819Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_819_repository(&self) -> Result<Self::CustomEntity819Repository<'_>, ContextError>;
    type CustomEntity820Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_820_repository(&self) -> Result<Self::CustomEntity820Repository<'_>, ContextError>;
    type CustomEntity821Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_821_repository(&self) -> Result<Self::CustomEntity821Repository<'_>, ContextError>;
    type CustomEntity822Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_822_repository(&self) -> Result<Self::CustomEntity822Repository<'_>, ContextError>;
    type CustomEntity823Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_823_repository(&self) -> Result<Self::CustomEntity823Repository<'_>, ContextError>;
    type CustomEntity824Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_824_repository(&self) -> Result<Self::CustomEntity824Repository<'_>, ContextError>;
    type CustomEntity825Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_825_repository(&self) -> Result<Self::CustomEntity825Repository<'_>, ContextError>;
    type CustomEntity826Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_826_repository(&self) -> Result<Self::CustomEntity826Repository<'_>, ContextError>;
    type CustomEntity827Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_827_repository(&self) -> Result<Self::CustomEntity827Repository<'_>, ContextError>;
    type CustomEntity828Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_828_repository(&self) -> Result<Self::CustomEntity828Repository<'_>, ContextError>;
    type CustomEntity829Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_829_repository(&self) -> Result<Self::CustomEntity829Repository<'_>, ContextError>;
    type CustomEntity830Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_830_repository(&self) -> Result<Self::CustomEntity830Repository<'_>, ContextError>;
    type CustomEntity831Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_831_repository(&self) -> Result<Self::CustomEntity831Repository<'_>, ContextError>;
    type CustomEntity832Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_832_repository(&self) -> Result<Self::CustomEntity832Repository<'_>, ContextError>;
    type CustomEntity833Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_833_repository(&self) -> Result<Self::CustomEntity833Repository<'_>, ContextError>;
    type CustomEntity834Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_834_repository(&self) -> Result<Self::CustomEntity834Repository<'_>, ContextError>;
    type CustomEntity835Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_835_repository(&self) -> Result<Self::CustomEntity835Repository<'_>, ContextError>;
    type CustomEntity836Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_836_repository(&self) -> Result<Self::CustomEntity836Repository<'_>, ContextError>;
    type CustomEntity837Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_837_repository(&self) -> Result<Self::CustomEntity837Repository<'_>, ContextError>;
    type CustomEntity838Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_838_repository(&self) -> Result<Self::CustomEntity838Repository<'_>, ContextError>;
    type CustomEntity839Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_839_repository(&self) -> Result<Self::CustomEntity839Repository<'_>, ContextError>;
    type CustomEntity840Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_840_repository(&self) -> Result<Self::CustomEntity840Repository<'_>, ContextError>;
    type CustomEntity841Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_841_repository(&self) -> Result<Self::CustomEntity841Repository<'_>, ContextError>;
    type CustomEntity842Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_842_repository(&self) -> Result<Self::CustomEntity842Repository<'_>, ContextError>;
    type CustomEntity843Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_843_repository(&self) -> Result<Self::CustomEntity843Repository<'_>, ContextError>;
    type CustomEntity844Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_844_repository(&self) -> Result<Self::CustomEntity844Repository<'_>, ContextError>;
    type CustomEntity845Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_845_repository(&self) -> Result<Self::CustomEntity845Repository<'_>, ContextError>;
    type CustomEntity846Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_846_repository(&self) -> Result<Self::CustomEntity846Repository<'_>, ContextError>;
    type CustomEntity847Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_847_repository(&self) -> Result<Self::CustomEntity847Repository<'_>, ContextError>;
    type CustomEntity848Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_848_repository(&self) -> Result<Self::CustomEntity848Repository<'_>, ContextError>;
    type CustomEntity849Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_849_repository(&self) -> Result<Self::CustomEntity849Repository<'_>, ContextError>;
    type CustomEntity850Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_850_repository(&self) -> Result<Self::CustomEntity850Repository<'_>, ContextError>;
    type CustomEntity851Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_851_repository(&self) -> Result<Self::CustomEntity851Repository<'_>, ContextError>;
    type CustomEntity852Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_852_repository(&self) -> Result<Self::CustomEntity852Repository<'_>, ContextError>;
    type CustomEntity853Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_853_repository(&self) -> Result<Self::CustomEntity853Repository<'_>, ContextError>;
    type CustomEntity854Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_854_repository(&self) -> Result<Self::CustomEntity854Repository<'_>, ContextError>;
    type CustomEntity855Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_855_repository(&self) -> Result<Self::CustomEntity855Repository<'_>, ContextError>;
    type CustomEntity856Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_856_repository(&self) -> Result<Self::CustomEntity856Repository<'_>, ContextError>;
    type CustomEntity857Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_857_repository(&self) -> Result<Self::CustomEntity857Repository<'_>, ContextError>;
    type CustomEntity858Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_858_repository(&self) -> Result<Self::CustomEntity858Repository<'_>, ContextError>;
    type CustomEntity859Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_859_repository(&self) -> Result<Self::CustomEntity859Repository<'_>, ContextError>;
    type CustomEntity860Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_860_repository(&self) -> Result<Self::CustomEntity860Repository<'_>, ContextError>;
    type CustomEntity861Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_861_repository(&self) -> Result<Self::CustomEntity861Repository<'_>, ContextError>;
    type CustomEntity862Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_862_repository(&self) -> Result<Self::CustomEntity862Repository<'_>, ContextError>;
    type CustomEntity863Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_863_repository(&self) -> Result<Self::CustomEntity863Repository<'_>, ContextError>;
    type CustomEntity864Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_864_repository(&self) -> Result<Self::CustomEntity864Repository<'_>, ContextError>;
    type CustomEntity865Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_865_repository(&self) -> Result<Self::CustomEntity865Repository<'_>, ContextError>;
    type CustomEntity866Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_866_repository(&self) -> Result<Self::CustomEntity866Repository<'_>, ContextError>;
    type CustomEntity867Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_867_repository(&self) -> Result<Self::CustomEntity867Repository<'_>, ContextError>;
    type CustomEntity868Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_868_repository(&self) -> Result<Self::CustomEntity868Repository<'_>, ContextError>;
    type CustomEntity869Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_869_repository(&self) -> Result<Self::CustomEntity869Repository<'_>, ContextError>;
    type CustomEntity870Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_870_repository(&self) -> Result<Self::CustomEntity870Repository<'_>, ContextError>;
    type CustomEntity871Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_871_repository(&self) -> Result<Self::CustomEntity871Repository<'_>, ContextError>;
    type CustomEntity872Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_872_repository(&self) -> Result<Self::CustomEntity872Repository<'_>, ContextError>;
    type CustomEntity873Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_873_repository(&self) -> Result<Self::CustomEntity873Repository<'_>, ContextError>;
    type CustomEntity874Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_874_repository(&self) -> Result<Self::CustomEntity874Repository<'_>, ContextError>;
    type CustomEntity875Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_875_repository(&self) -> Result<Self::CustomEntity875Repository<'_>, ContextError>;
    type CustomEntity876Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_876_repository(&self) -> Result<Self::CustomEntity876Repository<'_>, ContextError>;
    type CustomEntity877Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_877_repository(&self) -> Result<Self::CustomEntity877Repository<'_>, ContextError>;
    type CustomEntity878Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_878_repository(&self) -> Result<Self::CustomEntity878Repository<'_>, ContextError>;
    type CustomEntity879Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_879_repository(&self) -> Result<Self::CustomEntity879Repository<'_>, ContextError>;
    type CustomEntity880Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_880_repository(&self) -> Result<Self::CustomEntity880Repository<'_>, ContextError>;
    type CustomEntity881Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_881_repository(&self) -> Result<Self::CustomEntity881Repository<'_>, ContextError>;
    type CustomEntity882Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_882_repository(&self) -> Result<Self::CustomEntity882Repository<'_>, ContextError>;
    type CustomEntity883Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_883_repository(&self) -> Result<Self::CustomEntity883Repository<'_>, ContextError>;
    type CustomEntity884Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_884_repository(&self) -> Result<Self::CustomEntity884Repository<'_>, ContextError>;
    type CustomEntity885Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_885_repository(&self) -> Result<Self::CustomEntity885Repository<'_>, ContextError>;
    type CustomEntity886Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_886_repository(&self) -> Result<Self::CustomEntity886Repository<'_>, ContextError>;
    type CustomEntity887Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_887_repository(&self) -> Result<Self::CustomEntity887Repository<'_>, ContextError>;
    type CustomEntity888Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_888_repository(&self) -> Result<Self::CustomEntity888Repository<'_>, ContextError>;
    type CustomEntity889Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_889_repository(&self) -> Result<Self::CustomEntity889Repository<'_>, ContextError>;
    type CustomEntity890Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_890_repository(&self) -> Result<Self::CustomEntity890Repository<'_>, ContextError>;
    type CustomEntity891Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_891_repository(&self) -> Result<Self::CustomEntity891Repository<'_>, ContextError>;
    type CustomEntity892Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_892_repository(&self) -> Result<Self::CustomEntity892Repository<'_>, ContextError>;
    type CustomEntity893Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_893_repository(&self) -> Result<Self::CustomEntity893Repository<'_>, ContextError>;
    type CustomEntity894Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_894_repository(&self) -> Result<Self::CustomEntity894Repository<'_>, ContextError>;
    type CustomEntity895Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_895_repository(&self) -> Result<Self::CustomEntity895Repository<'_>, ContextError>;
    type CustomEntity896Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_896_repository(&self) -> Result<Self::CustomEntity896Repository<'_>, ContextError>;
    type CustomEntity897Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_897_repository(&self) -> Result<Self::CustomEntity897Repository<'_>, ContextError>;
    type CustomEntity898Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_898_repository(&self) -> Result<Self::CustomEntity898Repository<'_>, ContextError>;
    type CustomEntity899Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_899_repository(&self) -> Result<Self::CustomEntity899Repository<'_>, ContextError>;
    type CustomEntity900Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_900_repository(&self) -> Result<Self::CustomEntity900Repository<'_>, ContextError>;
    type CustomEntity901Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_901_repository(&self) -> Result<Self::CustomEntity901Repository<'_>, ContextError>;
    type CustomEntity902Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_902_repository(&self) -> Result<Self::CustomEntity902Repository<'_>, ContextError>;
    type CustomEntity903Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_903_repository(&self) -> Result<Self::CustomEntity903Repository<'_>, ContextError>;
    type CustomEntity904Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_904_repository(&self) -> Result<Self::CustomEntity904Repository<'_>, ContextError>;
    type CustomEntity905Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_905_repository(&self) -> Result<Self::CustomEntity905Repository<'_>, ContextError>;
    type CustomEntity906Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_906_repository(&self) -> Result<Self::CustomEntity906Repository<'_>, ContextError>;
    type CustomEntity907Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_907_repository(&self) -> Result<Self::CustomEntity907Repository<'_>, ContextError>;
    type CustomEntity908Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_908_repository(&self) -> Result<Self::CustomEntity908Repository<'_>, ContextError>;
    type CustomEntity909Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_909_repository(&self) -> Result<Self::CustomEntity909Repository<'_>, ContextError>;
    type CustomEntity910Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_910_repository(&self) -> Result<Self::CustomEntity910Repository<'_>, ContextError>;
    type CustomEntity911Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_911_repository(&self) -> Result<Self::CustomEntity911Repository<'_>, ContextError>;
    type CustomEntity912Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_912_repository(&self) -> Result<Self::CustomEntity912Repository<'_>, ContextError>;
    type CustomEntity913Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_913_repository(&self) -> Result<Self::CustomEntity913Repository<'_>, ContextError>;
    type CustomEntity914Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_914_repository(&self) -> Result<Self::CustomEntity914Repository<'_>, ContextError>;
    type CustomEntity915Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_915_repository(&self) -> Result<Self::CustomEntity915Repository<'_>, ContextError>;
    type CustomEntity916Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_916_repository(&self) -> Result<Self::CustomEntity916Repository<'_>, ContextError>;
    type CustomEntity917Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_917_repository(&self) -> Result<Self::CustomEntity917Repository<'_>, ContextError>;
    type CustomEntity918Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_918_repository(&self) -> Result<Self::CustomEntity918Repository<'_>, ContextError>;
    type CustomEntity919Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_919_repository(&self) -> Result<Self::CustomEntity919Repository<'_>, ContextError>;
    type CustomEntity920Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_920_repository(&self) -> Result<Self::CustomEntity920Repository<'_>, ContextError>;
    type CustomEntity921Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_921_repository(&self) -> Result<Self::CustomEntity921Repository<'_>, ContextError>;
    type CustomEntity922Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_922_repository(&self) -> Result<Self::CustomEntity922Repository<'_>, ContextError>;
    type CustomEntity923Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_923_repository(&self) -> Result<Self::CustomEntity923Repository<'_>, ContextError>;
    type CustomEntity924Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_924_repository(&self) -> Result<Self::CustomEntity924Repository<'_>, ContextError>;
    type CustomEntity925Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_925_repository(&self) -> Result<Self::CustomEntity925Repository<'_>, ContextError>;
    type CustomEntity926Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_926_repository(&self) -> Result<Self::CustomEntity926Repository<'_>, ContextError>;
    type CustomEntity927Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_927_repository(&self) -> Result<Self::CustomEntity927Repository<'_>, ContextError>;
    type CustomEntity928Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_928_repository(&self) -> Result<Self::CustomEntity928Repository<'_>, ContextError>;
    type CustomEntity929Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_929_repository(&self) -> Result<Self::CustomEntity929Repository<'_>, ContextError>;
    type CustomEntity930Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_930_repository(&self) -> Result<Self::CustomEntity930Repository<'_>, ContextError>;
    type CustomEntity931Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_931_repository(&self) -> Result<Self::CustomEntity931Repository<'_>, ContextError>;
    type CustomEntity932Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_932_repository(&self) -> Result<Self::CustomEntity932Repository<'_>, ContextError>;
    type CustomEntity933Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_933_repository(&self) -> Result<Self::CustomEntity933Repository<'_>, ContextError>;
    type CustomEntity934Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_934_repository(&self) -> Result<Self::CustomEntity934Repository<'_>, ContextError>;
    type CustomEntity935Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_935_repository(&self) -> Result<Self::CustomEntity935Repository<'_>, ContextError>;
    type CustomEntity936Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_936_repository(&self) -> Result<Self::CustomEntity936Repository<'_>, ContextError>;
    type CustomEntity937Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_937_repository(&self) -> Result<Self::CustomEntity937Repository<'_>, ContextError>;
    type CustomEntity938Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_938_repository(&self) -> Result<Self::CustomEntity938Repository<'_>, ContextError>;
    type CustomEntity939Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_939_repository(&self) -> Result<Self::CustomEntity939Repository<'_>, ContextError>;
    type CustomEntity940Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_940_repository(&self) -> Result<Self::CustomEntity940Repository<'_>, ContextError>;
    type CustomEntity941Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_941_repository(&self) -> Result<Self::CustomEntity941Repository<'_>, ContextError>;
    type CustomEntity942Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_942_repository(&self) -> Result<Self::CustomEntity942Repository<'_>, ContextError>;
    type CustomEntity943Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_943_repository(&self) -> Result<Self::CustomEntity943Repository<'_>, ContextError>;
    type CustomEntity944Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_944_repository(&self) -> Result<Self::CustomEntity944Repository<'_>, ContextError>;
    type CustomEntity945Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_945_repository(&self) -> Result<Self::CustomEntity945Repository<'_>, ContextError>;
    type CustomEntity946Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_946_repository(&self) -> Result<Self::CustomEntity946Repository<'_>, ContextError>;
    type CustomEntity947Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_947_repository(&self) -> Result<Self::CustomEntity947Repository<'_>, ContextError>;
    type CustomEntity948Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_948_repository(&self) -> Result<Self::CustomEntity948Repository<'_>, ContextError>;
    type CustomEntity949Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_949_repository(&self) -> Result<Self::CustomEntity949Repository<'_>, ContextError>;
    type CustomEntity950Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_950_repository(&self) -> Result<Self::CustomEntity950Repository<'_>, ContextError>;
    type CustomEntity951Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_951_repository(&self) -> Result<Self::CustomEntity951Repository<'_>, ContextError>;
    type CustomEntity952Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_952_repository(&self) -> Result<Self::CustomEntity952Repository<'_>, ContextError>;
    type CustomEntity953Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_953_repository(&self) -> Result<Self::CustomEntity953Repository<'_>, ContextError>;
    type CustomEntity954Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_954_repository(&self) -> Result<Self::CustomEntity954Repository<'_>, ContextError>;
    type CustomEntity955Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_955_repository(&self) -> Result<Self::CustomEntity955Repository<'_>, ContextError>;
    type CustomEntity956Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_956_repository(&self) -> Result<Self::CustomEntity956Repository<'_>, ContextError>;
    type CustomEntity957Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_957_repository(&self) -> Result<Self::CustomEntity957Repository<'_>, ContextError>;
    type CustomEntity958Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_958_repository(&self) -> Result<Self::CustomEntity958Repository<'_>, ContextError>;
    type CustomEntity959Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_959_repository(&self) -> Result<Self::CustomEntity959Repository<'_>, ContextError>;
    type CustomEntity960Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_960_repository(&self) -> Result<Self::CustomEntity960Repository<'_>, ContextError>;
    type CustomEntity961Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_961_repository(&self) -> Result<Self::CustomEntity961Repository<'_>, ContextError>;
    type CustomEntity962Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_962_repository(&self) -> Result<Self::CustomEntity962Repository<'_>, ContextError>;
    type CustomEntity963Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_963_repository(&self) -> Result<Self::CustomEntity963Repository<'_>, ContextError>;
    type CustomEntity964Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_964_repository(&self) -> Result<Self::CustomEntity964Repository<'_>, ContextError>;
    type CustomEntity965Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_965_repository(&self) -> Result<Self::CustomEntity965Repository<'_>, ContextError>;
    type CustomEntity966Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_966_repository(&self) -> Result<Self::CustomEntity966Repository<'_>, ContextError>;
    type CustomEntity967Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_967_repository(&self) -> Result<Self::CustomEntity967Repository<'_>, ContextError>;
    type CustomEntity968Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_968_repository(&self) -> Result<Self::CustomEntity968Repository<'_>, ContextError>;
    type CustomEntity969Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_969_repository(&self) -> Result<Self::CustomEntity969Repository<'_>, ContextError>;
    type CustomEntity970Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_970_repository(&self) -> Result<Self::CustomEntity970Repository<'_>, ContextError>;
    type CustomEntity971Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_971_repository(&self) -> Result<Self::CustomEntity971Repository<'_>, ContextError>;
    type CustomEntity972Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_972_repository(&self) -> Result<Self::CustomEntity972Repository<'_>, ContextError>;
    type CustomEntity973Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_973_repository(&self) -> Result<Self::CustomEntity973Repository<'_>, ContextError>;
    type CustomEntity974Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_974_repository(&self) -> Result<Self::CustomEntity974Repository<'_>, ContextError>;
    type CustomEntity975Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_975_repository(&self) -> Result<Self::CustomEntity975Repository<'_>, ContextError>;
    type CustomEntity976Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_976_repository(&self) -> Result<Self::CustomEntity976Repository<'_>, ContextError>;
    type CustomEntity977Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_977_repository(&self) -> Result<Self::CustomEntity977Repository<'_>, ContextError>;
    type CustomEntity978Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_978_repository(&self) -> Result<Self::CustomEntity978Repository<'_>, ContextError>;
    type CustomEntity979Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_979_repository(&self) -> Result<Self::CustomEntity979Repository<'_>, ContextError>;
    type CustomEntity980Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_980_repository(&self) -> Result<Self::CustomEntity980Repository<'_>, ContextError>;
    type CustomEntity981Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_981_repository(&self) -> Result<Self::CustomEntity981Repository<'_>, ContextError>;
    type CustomEntity982Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_982_repository(&self) -> Result<Self::CustomEntity982Repository<'_>, ContextError>;
    type CustomEntity983Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_983_repository(&self) -> Result<Self::CustomEntity983Repository<'_>, ContextError>;
    type CustomEntity984Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_984_repository(&self) -> Result<Self::CustomEntity984Repository<'_>, ContextError>;
    type CustomEntity985Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_985_repository(&self) -> Result<Self::CustomEntity985Repository<'_>, ContextError>;
    type CustomEntity986Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_986_repository(&self) -> Result<Self::CustomEntity986Repository<'_>, ContextError>;
    type CustomEntity987Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_987_repository(&self) -> Result<Self::CustomEntity987Repository<'_>, ContextError>;
    type CustomEntity988Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_988_repository(&self) -> Result<Self::CustomEntity988Repository<'_>, ContextError>;
    type CustomEntity989Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_989_repository(&self) -> Result<Self::CustomEntity989Repository<'_>, ContextError>;
    type CustomEntity990Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_990_repository(&self) -> Result<Self::CustomEntity990Repository<'_>, ContextError>;
    type CustomEntity991Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_991_repository(&self) -> Result<Self::CustomEntity991Repository<'_>, ContextError>;
    type CustomEntity992Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_992_repository(&self) -> Result<Self::CustomEntity992Repository<'_>, ContextError>;
    type CustomEntity993Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_993_repository(&self) -> Result<Self::CustomEntity993Repository<'_>, ContextError>;
    type CustomEntity994Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_994_repository(&self) -> Result<Self::CustomEntity994Repository<'_>, ContextError>;
    type CustomEntity995Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_995_repository(&self) -> Result<Self::CustomEntity995Repository<'_>, ContextError>;
    type CustomEntity996Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_996_repository(&self) -> Result<Self::CustomEntity996Repository<'_>, ContextError>;
    type CustomEntity997Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_997_repository(&self) -> Result<Self::CustomEntity997Repository<'_>, ContextError>;
    type CustomEntity998Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_998_repository(&self) -> Result<Self::CustomEntity998Repository<'_>, ContextError>;
    type CustomEntity999Repository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn custom_entity_999_repository(&self) -> Result<Self::CustomEntity999Repository<'_>, ContextError>;
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

    type NdaAgreementRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn nda_agreement_repository(&self) -> Result<Self::NdaAgreementRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("NdaAgreement")
    }

    type TermsOfServiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn terms_of_service_repository(&self) -> Result<Self::TermsOfServiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TermsOfService")
    }

    type PrivacyPolicyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn privacy_policy_repository(&self) -> Result<Self::PrivacyPolicyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PrivacyPolicy")
    }

    type CookieConsentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn cookie_consent_repository(&self) -> Result<Self::CookieConsentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CookieConsent")
    }

    type GdprRequestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn gdpr_request_repository(&self) -> Result<Self::GdprRequestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("GdprRequest")
    }

    type OshaIncidentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn osha_incident_repository(&self) -> Result<Self::OshaIncidentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OshaIncident")
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

    type PasswordResetRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn password_reset_repository(&self) -> Result<Self::PasswordResetRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PasswordReset")
    }

    type TwoFactorAuthRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn two_factor_auth_repository(&self) -> Result<Self::TwoFactorAuthRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TwoFactorAuth")
    }

    type AccessTokenRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn access_token_repository(&self) -> Result<Self::AccessTokenRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AccessToken")
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

    type LoginAttemptRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn login_attempt_repository(&self) -> Result<Self::LoginAttemptRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoginAttempt")
    }

    type FailedAuthLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn failed_auth_log_repository(&self) -> Result<Self::FailedAuthLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FailedAuthLog")
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

    type SmsDeliveryReceiptRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn sms_delivery_receipt_repository(&self) -> Result<Self::SmsDeliveryReceiptRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SmsDeliveryReceipt")
    }

    type EmailBounceLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn email_bounce_log_repository(&self) -> Result<Self::EmailBounceLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("EmailBounceLog")
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

    type SyncJobRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn sync_job_repository(&self) -> Result<Self::SyncJobRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SyncJob")
    }

    type ApiRateLimitRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn api_rate_limit_repository(&self) -> Result<Self::ApiRateLimitRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ApiRateLimit")
    }

    type CustomEntity180Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_180_repository(&self) -> Result<Self::CustomEntity180Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity180")
    }

    type CustomEntity181Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_181_repository(&self) -> Result<Self::CustomEntity181Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity181")
    }

    type CustomEntity182Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_182_repository(&self) -> Result<Self::CustomEntity182Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity182")
    }

    type CustomEntity183Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_183_repository(&self) -> Result<Self::CustomEntity183Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity183")
    }

    type CustomEntity184Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_184_repository(&self) -> Result<Self::CustomEntity184Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity184")
    }

    type CustomEntity185Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_185_repository(&self) -> Result<Self::CustomEntity185Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity185")
    }

    type CustomEntity186Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_186_repository(&self) -> Result<Self::CustomEntity186Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity186")
    }

    type CustomEntity187Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_187_repository(&self) -> Result<Self::CustomEntity187Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity187")
    }

    type CustomEntity188Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_188_repository(&self) -> Result<Self::CustomEntity188Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity188")
    }

    type CustomEntity189Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_189_repository(&self) -> Result<Self::CustomEntity189Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity189")
    }

    type CustomEntity190Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_190_repository(&self) -> Result<Self::CustomEntity190Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity190")
    }

    type CustomEntity191Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_191_repository(&self) -> Result<Self::CustomEntity191Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity191")
    }

    type CustomEntity192Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_192_repository(&self) -> Result<Self::CustomEntity192Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity192")
    }

    type CustomEntity193Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_193_repository(&self) -> Result<Self::CustomEntity193Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity193")
    }

    type CustomEntity194Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_194_repository(&self) -> Result<Self::CustomEntity194Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity194")
    }

    type CustomEntity195Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_195_repository(&self) -> Result<Self::CustomEntity195Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity195")
    }

    type CustomEntity196Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_196_repository(&self) -> Result<Self::CustomEntity196Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity196")
    }

    type CustomEntity197Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_197_repository(&self) -> Result<Self::CustomEntity197Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity197")
    }

    type CustomEntity198Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_198_repository(&self) -> Result<Self::CustomEntity198Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity198")
    }

    type CustomEntity199Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_199_repository(&self) -> Result<Self::CustomEntity199Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity199")
    }

    type CustomEntity200Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_200_repository(&self) -> Result<Self::CustomEntity200Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity200")
    }

    type CustomEntity201Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_201_repository(&self) -> Result<Self::CustomEntity201Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity201")
    }

    type CustomEntity202Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_202_repository(&self) -> Result<Self::CustomEntity202Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity202")
    }

    type CustomEntity203Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_203_repository(&self) -> Result<Self::CustomEntity203Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity203")
    }

    type CustomEntity204Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_204_repository(&self) -> Result<Self::CustomEntity204Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity204")
    }

    type CustomEntity205Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_205_repository(&self) -> Result<Self::CustomEntity205Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity205")
    }

    type CustomEntity206Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_206_repository(&self) -> Result<Self::CustomEntity206Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity206")
    }

    type CustomEntity207Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_207_repository(&self) -> Result<Self::CustomEntity207Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity207")
    }

    type CustomEntity208Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_208_repository(&self) -> Result<Self::CustomEntity208Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity208")
    }

    type CustomEntity209Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_209_repository(&self) -> Result<Self::CustomEntity209Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity209")
    }

    type CustomEntity210Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_210_repository(&self) -> Result<Self::CustomEntity210Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity210")
    }

    type CustomEntity211Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_211_repository(&self) -> Result<Self::CustomEntity211Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity211")
    }

    type CustomEntity212Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_212_repository(&self) -> Result<Self::CustomEntity212Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity212")
    }

    type CustomEntity213Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_213_repository(&self) -> Result<Self::CustomEntity213Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity213")
    }

    type CustomEntity214Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_214_repository(&self) -> Result<Self::CustomEntity214Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity214")
    }

    type CustomEntity215Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_215_repository(&self) -> Result<Self::CustomEntity215Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity215")
    }

    type CustomEntity216Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_216_repository(&self) -> Result<Self::CustomEntity216Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity216")
    }

    type CustomEntity217Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_217_repository(&self) -> Result<Self::CustomEntity217Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity217")
    }

    type CustomEntity218Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_218_repository(&self) -> Result<Self::CustomEntity218Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity218")
    }

    type CustomEntity219Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_219_repository(&self) -> Result<Self::CustomEntity219Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity219")
    }

    type CustomEntity220Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_220_repository(&self) -> Result<Self::CustomEntity220Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity220")
    }

    type CustomEntity221Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_221_repository(&self) -> Result<Self::CustomEntity221Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity221")
    }

    type CustomEntity222Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_222_repository(&self) -> Result<Self::CustomEntity222Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity222")
    }

    type CustomEntity223Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_223_repository(&self) -> Result<Self::CustomEntity223Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity223")
    }

    type CustomEntity224Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_224_repository(&self) -> Result<Self::CustomEntity224Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity224")
    }

    type CustomEntity225Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_225_repository(&self) -> Result<Self::CustomEntity225Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity225")
    }

    type CustomEntity226Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_226_repository(&self) -> Result<Self::CustomEntity226Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity226")
    }

    type CustomEntity227Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_227_repository(&self) -> Result<Self::CustomEntity227Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity227")
    }

    type CustomEntity228Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_228_repository(&self) -> Result<Self::CustomEntity228Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity228")
    }

    type CustomEntity229Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_229_repository(&self) -> Result<Self::CustomEntity229Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity229")
    }

    type CustomEntity230Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_230_repository(&self) -> Result<Self::CustomEntity230Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity230")
    }

    type CustomEntity231Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_231_repository(&self) -> Result<Self::CustomEntity231Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity231")
    }

    type CustomEntity232Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_232_repository(&self) -> Result<Self::CustomEntity232Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity232")
    }

    type CustomEntity233Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_233_repository(&self) -> Result<Self::CustomEntity233Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity233")
    }

    type CustomEntity234Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_234_repository(&self) -> Result<Self::CustomEntity234Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity234")
    }

    type CustomEntity235Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_235_repository(&self) -> Result<Self::CustomEntity235Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity235")
    }

    type CustomEntity236Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_236_repository(&self) -> Result<Self::CustomEntity236Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity236")
    }

    type CustomEntity237Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_237_repository(&self) -> Result<Self::CustomEntity237Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity237")
    }

    type CustomEntity238Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_238_repository(&self) -> Result<Self::CustomEntity238Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity238")
    }

    type CustomEntity239Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_239_repository(&self) -> Result<Self::CustomEntity239Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity239")
    }

    type CustomEntity240Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_240_repository(&self) -> Result<Self::CustomEntity240Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity240")
    }

    type CustomEntity241Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_241_repository(&self) -> Result<Self::CustomEntity241Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity241")
    }

    type CustomEntity242Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_242_repository(&self) -> Result<Self::CustomEntity242Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity242")
    }

    type CustomEntity243Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_243_repository(&self) -> Result<Self::CustomEntity243Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity243")
    }

    type CustomEntity244Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_244_repository(&self) -> Result<Self::CustomEntity244Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity244")
    }

    type CustomEntity245Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_245_repository(&self) -> Result<Self::CustomEntity245Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity245")
    }

    type CustomEntity246Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_246_repository(&self) -> Result<Self::CustomEntity246Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity246")
    }

    type CustomEntity247Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_247_repository(&self) -> Result<Self::CustomEntity247Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity247")
    }

    type CustomEntity248Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_248_repository(&self) -> Result<Self::CustomEntity248Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity248")
    }

    type CustomEntity249Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_249_repository(&self) -> Result<Self::CustomEntity249Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity249")
    }

    type CustomEntity250Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_250_repository(&self) -> Result<Self::CustomEntity250Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity250")
    }

    type CustomEntity251Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_251_repository(&self) -> Result<Self::CustomEntity251Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity251")
    }

    type CustomEntity252Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_252_repository(&self) -> Result<Self::CustomEntity252Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity252")
    }

    type CustomEntity253Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_253_repository(&self) -> Result<Self::CustomEntity253Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity253")
    }

    type CustomEntity254Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_254_repository(&self) -> Result<Self::CustomEntity254Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity254")
    }

    type CustomEntity255Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_255_repository(&self) -> Result<Self::CustomEntity255Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity255")
    }

    type CustomEntity256Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_256_repository(&self) -> Result<Self::CustomEntity256Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity256")
    }

    type CustomEntity257Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_257_repository(&self) -> Result<Self::CustomEntity257Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity257")
    }

    type CustomEntity258Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_258_repository(&self) -> Result<Self::CustomEntity258Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity258")
    }

    type CustomEntity259Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_259_repository(&self) -> Result<Self::CustomEntity259Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity259")
    }

    type CustomEntity260Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_260_repository(&self) -> Result<Self::CustomEntity260Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity260")
    }

    type CustomEntity261Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_261_repository(&self) -> Result<Self::CustomEntity261Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity261")
    }

    type CustomEntity262Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_262_repository(&self) -> Result<Self::CustomEntity262Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity262")
    }

    type CustomEntity263Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_263_repository(&self) -> Result<Self::CustomEntity263Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity263")
    }

    type CustomEntity264Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_264_repository(&self) -> Result<Self::CustomEntity264Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity264")
    }

    type CustomEntity265Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_265_repository(&self) -> Result<Self::CustomEntity265Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity265")
    }

    type CustomEntity266Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_266_repository(&self) -> Result<Self::CustomEntity266Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity266")
    }

    type CustomEntity267Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_267_repository(&self) -> Result<Self::CustomEntity267Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity267")
    }

    type CustomEntity268Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_268_repository(&self) -> Result<Self::CustomEntity268Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity268")
    }

    type CustomEntity269Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_269_repository(&self) -> Result<Self::CustomEntity269Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity269")
    }

    type CustomEntity270Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_270_repository(&self) -> Result<Self::CustomEntity270Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity270")
    }

    type CustomEntity271Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_271_repository(&self) -> Result<Self::CustomEntity271Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity271")
    }

    type CustomEntity272Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_272_repository(&self) -> Result<Self::CustomEntity272Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity272")
    }

    type CustomEntity273Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_273_repository(&self) -> Result<Self::CustomEntity273Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity273")
    }

    type CustomEntity274Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_274_repository(&self) -> Result<Self::CustomEntity274Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity274")
    }

    type CustomEntity275Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_275_repository(&self) -> Result<Self::CustomEntity275Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity275")
    }

    type CustomEntity276Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_276_repository(&self) -> Result<Self::CustomEntity276Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity276")
    }

    type CustomEntity277Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_277_repository(&self) -> Result<Self::CustomEntity277Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity277")
    }

    type CustomEntity278Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_278_repository(&self) -> Result<Self::CustomEntity278Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity278")
    }

    type CustomEntity279Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_279_repository(&self) -> Result<Self::CustomEntity279Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity279")
    }

    type CustomEntity280Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_280_repository(&self) -> Result<Self::CustomEntity280Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity280")
    }

    type CustomEntity281Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_281_repository(&self) -> Result<Self::CustomEntity281Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity281")
    }

    type CustomEntity282Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_282_repository(&self) -> Result<Self::CustomEntity282Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity282")
    }

    type CustomEntity283Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_283_repository(&self) -> Result<Self::CustomEntity283Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity283")
    }

    type CustomEntity284Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_284_repository(&self) -> Result<Self::CustomEntity284Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity284")
    }

    type CustomEntity285Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_285_repository(&self) -> Result<Self::CustomEntity285Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity285")
    }

    type CustomEntity286Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_286_repository(&self) -> Result<Self::CustomEntity286Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity286")
    }

    type CustomEntity287Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_287_repository(&self) -> Result<Self::CustomEntity287Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity287")
    }

    type CustomEntity288Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_288_repository(&self) -> Result<Self::CustomEntity288Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity288")
    }

    type CustomEntity289Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_289_repository(&self) -> Result<Self::CustomEntity289Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity289")
    }

    type CustomEntity290Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_290_repository(&self) -> Result<Self::CustomEntity290Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity290")
    }

    type CustomEntity291Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_291_repository(&self) -> Result<Self::CustomEntity291Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity291")
    }

    type CustomEntity292Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_292_repository(&self) -> Result<Self::CustomEntity292Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity292")
    }

    type CustomEntity293Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_293_repository(&self) -> Result<Self::CustomEntity293Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity293")
    }

    type CustomEntity294Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_294_repository(&self) -> Result<Self::CustomEntity294Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity294")
    }

    type CustomEntity295Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_295_repository(&self) -> Result<Self::CustomEntity295Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity295")
    }

    type CustomEntity296Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_296_repository(&self) -> Result<Self::CustomEntity296Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity296")
    }

    type CustomEntity297Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_297_repository(&self) -> Result<Self::CustomEntity297Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity297")
    }

    type CustomEntity298Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_298_repository(&self) -> Result<Self::CustomEntity298Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity298")
    }

    type CustomEntity299Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_299_repository(&self) -> Result<Self::CustomEntity299Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity299")
    }

    type CustomEntity300Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_300_repository(&self) -> Result<Self::CustomEntity300Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity300")
    }

    type CustomEntity301Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_301_repository(&self) -> Result<Self::CustomEntity301Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity301")
    }

    type CustomEntity302Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_302_repository(&self) -> Result<Self::CustomEntity302Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity302")
    }

    type CustomEntity303Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_303_repository(&self) -> Result<Self::CustomEntity303Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity303")
    }

    type CustomEntity304Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_304_repository(&self) -> Result<Self::CustomEntity304Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity304")
    }

    type CustomEntity305Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_305_repository(&self) -> Result<Self::CustomEntity305Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity305")
    }

    type CustomEntity306Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_306_repository(&self) -> Result<Self::CustomEntity306Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity306")
    }

    type CustomEntity307Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_307_repository(&self) -> Result<Self::CustomEntity307Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity307")
    }

    type CustomEntity308Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_308_repository(&self) -> Result<Self::CustomEntity308Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity308")
    }

    type CustomEntity309Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_309_repository(&self) -> Result<Self::CustomEntity309Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity309")
    }

    type CustomEntity310Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_310_repository(&self) -> Result<Self::CustomEntity310Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity310")
    }

    type CustomEntity311Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_311_repository(&self) -> Result<Self::CustomEntity311Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity311")
    }

    type CustomEntity312Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_312_repository(&self) -> Result<Self::CustomEntity312Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity312")
    }

    type CustomEntity313Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_313_repository(&self) -> Result<Self::CustomEntity313Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity313")
    }

    type CustomEntity314Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_314_repository(&self) -> Result<Self::CustomEntity314Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity314")
    }

    type CustomEntity315Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_315_repository(&self) -> Result<Self::CustomEntity315Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity315")
    }

    type CustomEntity316Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_316_repository(&self) -> Result<Self::CustomEntity316Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity316")
    }

    type CustomEntity317Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_317_repository(&self) -> Result<Self::CustomEntity317Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity317")
    }

    type CustomEntity318Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_318_repository(&self) -> Result<Self::CustomEntity318Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity318")
    }

    type CustomEntity319Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_319_repository(&self) -> Result<Self::CustomEntity319Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity319")
    }

    type CustomEntity320Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_320_repository(&self) -> Result<Self::CustomEntity320Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity320")
    }

    type CustomEntity321Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_321_repository(&self) -> Result<Self::CustomEntity321Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity321")
    }

    type CustomEntity322Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_322_repository(&self) -> Result<Self::CustomEntity322Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity322")
    }

    type CustomEntity323Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_323_repository(&self) -> Result<Self::CustomEntity323Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity323")
    }

    type CustomEntity324Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_324_repository(&self) -> Result<Self::CustomEntity324Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity324")
    }

    type CustomEntity325Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_325_repository(&self) -> Result<Self::CustomEntity325Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity325")
    }

    type CustomEntity326Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_326_repository(&self) -> Result<Self::CustomEntity326Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity326")
    }

    type CustomEntity327Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_327_repository(&self) -> Result<Self::CustomEntity327Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity327")
    }

    type CustomEntity328Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_328_repository(&self) -> Result<Self::CustomEntity328Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity328")
    }

    type CustomEntity329Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_329_repository(&self) -> Result<Self::CustomEntity329Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity329")
    }

    type CustomEntity330Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_330_repository(&self) -> Result<Self::CustomEntity330Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity330")
    }

    type CustomEntity331Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_331_repository(&self) -> Result<Self::CustomEntity331Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity331")
    }

    type CustomEntity332Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_332_repository(&self) -> Result<Self::CustomEntity332Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity332")
    }

    type CustomEntity333Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_333_repository(&self) -> Result<Self::CustomEntity333Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity333")
    }

    type CustomEntity334Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_334_repository(&self) -> Result<Self::CustomEntity334Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity334")
    }

    type CustomEntity335Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_335_repository(&self) -> Result<Self::CustomEntity335Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity335")
    }

    type CustomEntity336Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_336_repository(&self) -> Result<Self::CustomEntity336Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity336")
    }

    type CustomEntity337Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_337_repository(&self) -> Result<Self::CustomEntity337Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity337")
    }

    type CustomEntity338Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_338_repository(&self) -> Result<Self::CustomEntity338Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity338")
    }

    type CustomEntity339Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_339_repository(&self) -> Result<Self::CustomEntity339Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity339")
    }

    type CustomEntity340Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_340_repository(&self) -> Result<Self::CustomEntity340Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity340")
    }

    type CustomEntity341Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_341_repository(&self) -> Result<Self::CustomEntity341Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity341")
    }

    type CustomEntity342Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_342_repository(&self) -> Result<Self::CustomEntity342Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity342")
    }

    type CustomEntity343Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_343_repository(&self) -> Result<Self::CustomEntity343Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity343")
    }

    type CustomEntity344Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_344_repository(&self) -> Result<Self::CustomEntity344Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity344")
    }

    type CustomEntity345Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_345_repository(&self) -> Result<Self::CustomEntity345Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity345")
    }

    type CustomEntity346Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_346_repository(&self) -> Result<Self::CustomEntity346Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity346")
    }

    type CustomEntity347Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_347_repository(&self) -> Result<Self::CustomEntity347Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity347")
    }

    type CustomEntity348Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_348_repository(&self) -> Result<Self::CustomEntity348Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity348")
    }

    type CustomEntity349Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_349_repository(&self) -> Result<Self::CustomEntity349Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity349")
    }

    type CustomEntity350Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_350_repository(&self) -> Result<Self::CustomEntity350Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity350")
    }

    type CustomEntity351Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_351_repository(&self) -> Result<Self::CustomEntity351Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity351")
    }

    type CustomEntity352Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_352_repository(&self) -> Result<Self::CustomEntity352Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity352")
    }

    type CustomEntity353Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_353_repository(&self) -> Result<Self::CustomEntity353Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity353")
    }

    type CustomEntity354Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_354_repository(&self) -> Result<Self::CustomEntity354Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity354")
    }

    type CustomEntity355Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_355_repository(&self) -> Result<Self::CustomEntity355Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity355")
    }

    type CustomEntity356Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_356_repository(&self) -> Result<Self::CustomEntity356Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity356")
    }

    type CustomEntity357Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_357_repository(&self) -> Result<Self::CustomEntity357Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity357")
    }

    type CustomEntity358Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_358_repository(&self) -> Result<Self::CustomEntity358Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity358")
    }

    type CustomEntity359Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_359_repository(&self) -> Result<Self::CustomEntity359Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity359")
    }

    type CustomEntity360Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_360_repository(&self) -> Result<Self::CustomEntity360Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity360")
    }

    type CustomEntity361Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_361_repository(&self) -> Result<Self::CustomEntity361Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity361")
    }

    type CustomEntity362Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_362_repository(&self) -> Result<Self::CustomEntity362Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity362")
    }

    type CustomEntity363Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_363_repository(&self) -> Result<Self::CustomEntity363Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity363")
    }

    type CustomEntity364Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_364_repository(&self) -> Result<Self::CustomEntity364Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity364")
    }

    type CustomEntity365Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_365_repository(&self) -> Result<Self::CustomEntity365Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity365")
    }

    type CustomEntity366Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_366_repository(&self) -> Result<Self::CustomEntity366Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity366")
    }

    type CustomEntity367Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_367_repository(&self) -> Result<Self::CustomEntity367Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity367")
    }

    type CustomEntity368Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_368_repository(&self) -> Result<Self::CustomEntity368Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity368")
    }

    type CustomEntity369Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_369_repository(&self) -> Result<Self::CustomEntity369Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity369")
    }

    type CustomEntity370Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_370_repository(&self) -> Result<Self::CustomEntity370Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity370")
    }

    type CustomEntity371Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_371_repository(&self) -> Result<Self::CustomEntity371Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity371")
    }

    type CustomEntity372Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_372_repository(&self) -> Result<Self::CustomEntity372Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity372")
    }

    type CustomEntity373Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_373_repository(&self) -> Result<Self::CustomEntity373Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity373")
    }

    type CustomEntity374Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_374_repository(&self) -> Result<Self::CustomEntity374Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity374")
    }

    type CustomEntity375Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_375_repository(&self) -> Result<Self::CustomEntity375Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity375")
    }

    type CustomEntity376Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_376_repository(&self) -> Result<Self::CustomEntity376Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity376")
    }

    type CustomEntity377Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_377_repository(&self) -> Result<Self::CustomEntity377Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity377")
    }

    type CustomEntity378Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_378_repository(&self) -> Result<Self::CustomEntity378Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity378")
    }

    type CustomEntity379Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_379_repository(&self) -> Result<Self::CustomEntity379Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity379")
    }

    type CustomEntity380Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_380_repository(&self) -> Result<Self::CustomEntity380Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity380")
    }

    type CustomEntity381Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_381_repository(&self) -> Result<Self::CustomEntity381Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity381")
    }

    type CustomEntity382Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_382_repository(&self) -> Result<Self::CustomEntity382Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity382")
    }

    type CustomEntity383Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_383_repository(&self) -> Result<Self::CustomEntity383Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity383")
    }

    type CustomEntity384Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_384_repository(&self) -> Result<Self::CustomEntity384Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity384")
    }

    type CustomEntity385Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_385_repository(&self) -> Result<Self::CustomEntity385Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity385")
    }

    type CustomEntity386Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_386_repository(&self) -> Result<Self::CustomEntity386Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity386")
    }

    type CustomEntity387Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_387_repository(&self) -> Result<Self::CustomEntity387Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity387")
    }

    type CustomEntity388Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_388_repository(&self) -> Result<Self::CustomEntity388Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity388")
    }

    type CustomEntity389Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_389_repository(&self) -> Result<Self::CustomEntity389Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity389")
    }

    type CustomEntity390Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_390_repository(&self) -> Result<Self::CustomEntity390Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity390")
    }

    type CustomEntity391Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_391_repository(&self) -> Result<Self::CustomEntity391Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity391")
    }

    type CustomEntity392Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_392_repository(&self) -> Result<Self::CustomEntity392Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity392")
    }

    type CustomEntity393Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_393_repository(&self) -> Result<Self::CustomEntity393Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity393")
    }

    type CustomEntity394Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_394_repository(&self) -> Result<Self::CustomEntity394Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity394")
    }

    type CustomEntity395Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_395_repository(&self) -> Result<Self::CustomEntity395Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity395")
    }

    type CustomEntity396Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_396_repository(&self) -> Result<Self::CustomEntity396Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity396")
    }

    type CustomEntity397Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_397_repository(&self) -> Result<Self::CustomEntity397Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity397")
    }

    type CustomEntity398Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_398_repository(&self) -> Result<Self::CustomEntity398Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity398")
    }

    type CustomEntity399Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_399_repository(&self) -> Result<Self::CustomEntity399Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity399")
    }

    type CustomEntity400Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_400_repository(&self) -> Result<Self::CustomEntity400Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity400")
    }

    type CustomEntity401Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_401_repository(&self) -> Result<Self::CustomEntity401Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity401")
    }

    type CustomEntity402Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_402_repository(&self) -> Result<Self::CustomEntity402Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity402")
    }

    type CustomEntity403Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_403_repository(&self) -> Result<Self::CustomEntity403Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity403")
    }

    type CustomEntity404Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_404_repository(&self) -> Result<Self::CustomEntity404Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity404")
    }

    type CustomEntity405Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_405_repository(&self) -> Result<Self::CustomEntity405Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity405")
    }

    type CustomEntity406Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_406_repository(&self) -> Result<Self::CustomEntity406Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity406")
    }

    type CustomEntity407Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_407_repository(&self) -> Result<Self::CustomEntity407Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity407")
    }

    type CustomEntity408Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_408_repository(&self) -> Result<Self::CustomEntity408Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity408")
    }

    type CustomEntity409Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_409_repository(&self) -> Result<Self::CustomEntity409Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity409")
    }

    type CustomEntity410Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_410_repository(&self) -> Result<Self::CustomEntity410Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity410")
    }

    type CustomEntity411Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_411_repository(&self) -> Result<Self::CustomEntity411Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity411")
    }

    type CustomEntity412Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_412_repository(&self) -> Result<Self::CustomEntity412Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity412")
    }

    type CustomEntity413Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_413_repository(&self) -> Result<Self::CustomEntity413Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity413")
    }

    type CustomEntity414Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_414_repository(&self) -> Result<Self::CustomEntity414Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity414")
    }

    type CustomEntity415Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_415_repository(&self) -> Result<Self::CustomEntity415Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity415")
    }

    type CustomEntity416Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_416_repository(&self) -> Result<Self::CustomEntity416Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity416")
    }

    type CustomEntity417Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_417_repository(&self) -> Result<Self::CustomEntity417Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity417")
    }

    type CustomEntity418Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_418_repository(&self) -> Result<Self::CustomEntity418Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity418")
    }

    type CustomEntity419Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_419_repository(&self) -> Result<Self::CustomEntity419Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity419")
    }

    type CustomEntity420Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_420_repository(&self) -> Result<Self::CustomEntity420Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity420")
    }

    type CustomEntity421Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_421_repository(&self) -> Result<Self::CustomEntity421Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity421")
    }

    type CustomEntity422Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_422_repository(&self) -> Result<Self::CustomEntity422Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity422")
    }

    type CustomEntity423Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_423_repository(&self) -> Result<Self::CustomEntity423Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity423")
    }

    type CustomEntity424Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_424_repository(&self) -> Result<Self::CustomEntity424Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity424")
    }

    type CustomEntity425Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_425_repository(&self) -> Result<Self::CustomEntity425Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity425")
    }

    type CustomEntity426Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_426_repository(&self) -> Result<Self::CustomEntity426Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity426")
    }

    type CustomEntity427Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_427_repository(&self) -> Result<Self::CustomEntity427Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity427")
    }

    type CustomEntity428Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_428_repository(&self) -> Result<Self::CustomEntity428Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity428")
    }

    type CustomEntity429Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_429_repository(&self) -> Result<Self::CustomEntity429Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity429")
    }

    type CustomEntity430Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_430_repository(&self) -> Result<Self::CustomEntity430Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity430")
    }

    type CustomEntity431Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_431_repository(&self) -> Result<Self::CustomEntity431Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity431")
    }

    type CustomEntity432Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_432_repository(&self) -> Result<Self::CustomEntity432Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity432")
    }

    type CustomEntity433Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_433_repository(&self) -> Result<Self::CustomEntity433Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity433")
    }

    type CustomEntity434Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_434_repository(&self) -> Result<Self::CustomEntity434Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity434")
    }

    type CustomEntity435Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_435_repository(&self) -> Result<Self::CustomEntity435Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity435")
    }

    type CustomEntity436Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_436_repository(&self) -> Result<Self::CustomEntity436Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity436")
    }

    type CustomEntity437Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_437_repository(&self) -> Result<Self::CustomEntity437Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity437")
    }

    type CustomEntity438Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_438_repository(&self) -> Result<Self::CustomEntity438Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity438")
    }

    type CustomEntity439Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_439_repository(&self) -> Result<Self::CustomEntity439Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity439")
    }

    type CustomEntity440Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_440_repository(&self) -> Result<Self::CustomEntity440Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity440")
    }

    type CustomEntity441Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_441_repository(&self) -> Result<Self::CustomEntity441Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity441")
    }

    type CustomEntity442Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_442_repository(&self) -> Result<Self::CustomEntity442Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity442")
    }

    type CustomEntity443Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_443_repository(&self) -> Result<Self::CustomEntity443Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity443")
    }

    type CustomEntity444Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_444_repository(&self) -> Result<Self::CustomEntity444Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity444")
    }

    type CustomEntity445Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_445_repository(&self) -> Result<Self::CustomEntity445Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity445")
    }

    type CustomEntity446Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_446_repository(&self) -> Result<Self::CustomEntity446Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity446")
    }

    type CustomEntity447Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_447_repository(&self) -> Result<Self::CustomEntity447Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity447")
    }

    type CustomEntity448Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_448_repository(&self) -> Result<Self::CustomEntity448Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity448")
    }

    type CustomEntity449Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_449_repository(&self) -> Result<Self::CustomEntity449Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity449")
    }

    type CustomEntity450Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_450_repository(&self) -> Result<Self::CustomEntity450Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity450")
    }

    type CustomEntity451Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_451_repository(&self) -> Result<Self::CustomEntity451Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity451")
    }

    type CustomEntity452Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_452_repository(&self) -> Result<Self::CustomEntity452Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity452")
    }

    type CustomEntity453Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_453_repository(&self) -> Result<Self::CustomEntity453Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity453")
    }

    type CustomEntity454Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_454_repository(&self) -> Result<Self::CustomEntity454Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity454")
    }

    type CustomEntity455Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_455_repository(&self) -> Result<Self::CustomEntity455Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity455")
    }

    type CustomEntity456Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_456_repository(&self) -> Result<Self::CustomEntity456Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity456")
    }

    type CustomEntity457Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_457_repository(&self) -> Result<Self::CustomEntity457Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity457")
    }

    type CustomEntity458Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_458_repository(&self) -> Result<Self::CustomEntity458Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity458")
    }

    type CustomEntity459Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_459_repository(&self) -> Result<Self::CustomEntity459Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity459")
    }

    type CustomEntity460Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_460_repository(&self) -> Result<Self::CustomEntity460Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity460")
    }

    type CustomEntity461Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_461_repository(&self) -> Result<Self::CustomEntity461Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity461")
    }

    type CustomEntity462Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_462_repository(&self) -> Result<Self::CustomEntity462Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity462")
    }

    type CustomEntity463Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_463_repository(&self) -> Result<Self::CustomEntity463Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity463")
    }

    type CustomEntity464Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_464_repository(&self) -> Result<Self::CustomEntity464Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity464")
    }

    type CustomEntity465Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_465_repository(&self) -> Result<Self::CustomEntity465Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity465")
    }

    type CustomEntity466Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_466_repository(&self) -> Result<Self::CustomEntity466Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity466")
    }

    type CustomEntity467Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_467_repository(&self) -> Result<Self::CustomEntity467Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity467")
    }

    type CustomEntity468Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_468_repository(&self) -> Result<Self::CustomEntity468Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity468")
    }

    type CustomEntity469Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_469_repository(&self) -> Result<Self::CustomEntity469Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity469")
    }

    type CustomEntity470Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_470_repository(&self) -> Result<Self::CustomEntity470Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity470")
    }

    type CustomEntity471Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_471_repository(&self) -> Result<Self::CustomEntity471Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity471")
    }

    type CustomEntity472Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_472_repository(&self) -> Result<Self::CustomEntity472Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity472")
    }

    type CustomEntity473Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_473_repository(&self) -> Result<Self::CustomEntity473Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity473")
    }

    type CustomEntity474Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_474_repository(&self) -> Result<Self::CustomEntity474Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity474")
    }

    type CustomEntity475Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_475_repository(&self) -> Result<Self::CustomEntity475Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity475")
    }

    type CustomEntity476Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_476_repository(&self) -> Result<Self::CustomEntity476Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity476")
    }

    type CustomEntity477Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_477_repository(&self) -> Result<Self::CustomEntity477Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity477")
    }

    type CustomEntity478Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_478_repository(&self) -> Result<Self::CustomEntity478Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity478")
    }

    type CustomEntity479Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_479_repository(&self) -> Result<Self::CustomEntity479Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity479")
    }

    type CustomEntity480Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_480_repository(&self) -> Result<Self::CustomEntity480Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity480")
    }

    type CustomEntity481Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_481_repository(&self) -> Result<Self::CustomEntity481Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity481")
    }

    type CustomEntity482Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_482_repository(&self) -> Result<Self::CustomEntity482Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity482")
    }

    type CustomEntity483Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_483_repository(&self) -> Result<Self::CustomEntity483Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity483")
    }

    type CustomEntity484Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_484_repository(&self) -> Result<Self::CustomEntity484Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity484")
    }

    type CustomEntity485Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_485_repository(&self) -> Result<Self::CustomEntity485Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity485")
    }

    type CustomEntity486Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_486_repository(&self) -> Result<Self::CustomEntity486Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity486")
    }

    type CustomEntity487Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_487_repository(&self) -> Result<Self::CustomEntity487Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity487")
    }

    type CustomEntity488Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_488_repository(&self) -> Result<Self::CustomEntity488Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity488")
    }

    type CustomEntity489Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_489_repository(&self) -> Result<Self::CustomEntity489Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity489")
    }

    type CustomEntity490Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_490_repository(&self) -> Result<Self::CustomEntity490Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity490")
    }

    type CustomEntity491Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_491_repository(&self) -> Result<Self::CustomEntity491Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity491")
    }

    type CustomEntity492Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_492_repository(&self) -> Result<Self::CustomEntity492Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity492")
    }

    type CustomEntity493Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_493_repository(&self) -> Result<Self::CustomEntity493Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity493")
    }

    type CustomEntity494Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_494_repository(&self) -> Result<Self::CustomEntity494Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity494")
    }

    type CustomEntity495Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_495_repository(&self) -> Result<Self::CustomEntity495Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity495")
    }

    type CustomEntity496Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_496_repository(&self) -> Result<Self::CustomEntity496Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity496")
    }

    type CustomEntity497Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_497_repository(&self) -> Result<Self::CustomEntity497Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity497")
    }

    type CustomEntity498Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_498_repository(&self) -> Result<Self::CustomEntity498Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity498")
    }

    type CustomEntity499Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_499_repository(&self) -> Result<Self::CustomEntity499Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity499")
    }

    type CustomEntity500Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_500_repository(&self) -> Result<Self::CustomEntity500Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity500")
    }

    type CustomEntity501Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_501_repository(&self) -> Result<Self::CustomEntity501Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity501")
    }

    type CustomEntity502Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_502_repository(&self) -> Result<Self::CustomEntity502Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity502")
    }

    type CustomEntity503Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_503_repository(&self) -> Result<Self::CustomEntity503Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity503")
    }

    type CustomEntity504Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_504_repository(&self) -> Result<Self::CustomEntity504Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity504")
    }

    type CustomEntity505Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_505_repository(&self) -> Result<Self::CustomEntity505Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity505")
    }

    type CustomEntity506Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_506_repository(&self) -> Result<Self::CustomEntity506Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity506")
    }

    type CustomEntity507Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_507_repository(&self) -> Result<Self::CustomEntity507Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity507")
    }

    type CustomEntity508Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_508_repository(&self) -> Result<Self::CustomEntity508Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity508")
    }

    type CustomEntity509Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_509_repository(&self) -> Result<Self::CustomEntity509Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity509")
    }

    type CustomEntity510Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_510_repository(&self) -> Result<Self::CustomEntity510Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity510")
    }

    type CustomEntity511Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_511_repository(&self) -> Result<Self::CustomEntity511Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity511")
    }

    type CustomEntity512Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_512_repository(&self) -> Result<Self::CustomEntity512Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity512")
    }

    type CustomEntity513Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_513_repository(&self) -> Result<Self::CustomEntity513Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity513")
    }

    type CustomEntity514Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_514_repository(&self) -> Result<Self::CustomEntity514Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity514")
    }

    type CustomEntity515Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_515_repository(&self) -> Result<Self::CustomEntity515Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity515")
    }

    type CustomEntity516Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_516_repository(&self) -> Result<Self::CustomEntity516Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity516")
    }

    type CustomEntity517Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_517_repository(&self) -> Result<Self::CustomEntity517Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity517")
    }

    type CustomEntity518Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_518_repository(&self) -> Result<Self::CustomEntity518Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity518")
    }

    type CustomEntity519Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_519_repository(&self) -> Result<Self::CustomEntity519Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity519")
    }

    type CustomEntity520Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_520_repository(&self) -> Result<Self::CustomEntity520Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity520")
    }

    type CustomEntity521Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_521_repository(&self) -> Result<Self::CustomEntity521Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity521")
    }

    type CustomEntity522Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_522_repository(&self) -> Result<Self::CustomEntity522Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity522")
    }

    type CustomEntity523Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_523_repository(&self) -> Result<Self::CustomEntity523Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity523")
    }

    type CustomEntity524Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_524_repository(&self) -> Result<Self::CustomEntity524Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity524")
    }

    type CustomEntity525Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_525_repository(&self) -> Result<Self::CustomEntity525Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity525")
    }

    type CustomEntity526Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_526_repository(&self) -> Result<Self::CustomEntity526Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity526")
    }

    type CustomEntity527Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_527_repository(&self) -> Result<Self::CustomEntity527Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity527")
    }

    type CustomEntity528Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_528_repository(&self) -> Result<Self::CustomEntity528Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity528")
    }

    type CustomEntity529Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_529_repository(&self) -> Result<Self::CustomEntity529Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity529")
    }

    type CustomEntity530Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_530_repository(&self) -> Result<Self::CustomEntity530Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity530")
    }

    type CustomEntity531Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_531_repository(&self) -> Result<Self::CustomEntity531Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity531")
    }

    type CustomEntity532Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_532_repository(&self) -> Result<Self::CustomEntity532Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity532")
    }

    type CustomEntity533Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_533_repository(&self) -> Result<Self::CustomEntity533Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity533")
    }

    type CustomEntity534Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_534_repository(&self) -> Result<Self::CustomEntity534Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity534")
    }

    type CustomEntity535Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_535_repository(&self) -> Result<Self::CustomEntity535Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity535")
    }

    type CustomEntity536Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_536_repository(&self) -> Result<Self::CustomEntity536Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity536")
    }

    type CustomEntity537Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_537_repository(&self) -> Result<Self::CustomEntity537Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity537")
    }

    type CustomEntity538Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_538_repository(&self) -> Result<Self::CustomEntity538Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity538")
    }

    type CustomEntity539Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_539_repository(&self) -> Result<Self::CustomEntity539Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity539")
    }

    type CustomEntity540Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_540_repository(&self) -> Result<Self::CustomEntity540Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity540")
    }

    type CustomEntity541Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_541_repository(&self) -> Result<Self::CustomEntity541Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity541")
    }

    type CustomEntity542Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_542_repository(&self) -> Result<Self::CustomEntity542Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity542")
    }

    type CustomEntity543Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_543_repository(&self) -> Result<Self::CustomEntity543Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity543")
    }

    type CustomEntity544Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_544_repository(&self) -> Result<Self::CustomEntity544Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity544")
    }

    type CustomEntity545Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_545_repository(&self) -> Result<Self::CustomEntity545Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity545")
    }

    type CustomEntity546Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_546_repository(&self) -> Result<Self::CustomEntity546Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity546")
    }

    type CustomEntity547Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_547_repository(&self) -> Result<Self::CustomEntity547Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity547")
    }

    type CustomEntity548Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_548_repository(&self) -> Result<Self::CustomEntity548Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity548")
    }

    type CustomEntity549Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_549_repository(&self) -> Result<Self::CustomEntity549Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity549")
    }

    type CustomEntity550Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_550_repository(&self) -> Result<Self::CustomEntity550Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity550")
    }

    type CustomEntity551Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_551_repository(&self) -> Result<Self::CustomEntity551Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity551")
    }

    type CustomEntity552Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_552_repository(&self) -> Result<Self::CustomEntity552Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity552")
    }

    type CustomEntity553Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_553_repository(&self) -> Result<Self::CustomEntity553Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity553")
    }

    type CustomEntity554Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_554_repository(&self) -> Result<Self::CustomEntity554Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity554")
    }

    type CustomEntity555Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_555_repository(&self) -> Result<Self::CustomEntity555Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity555")
    }

    type CustomEntity556Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_556_repository(&self) -> Result<Self::CustomEntity556Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity556")
    }

    type CustomEntity557Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_557_repository(&self) -> Result<Self::CustomEntity557Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity557")
    }

    type CustomEntity558Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_558_repository(&self) -> Result<Self::CustomEntity558Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity558")
    }

    type CustomEntity559Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_559_repository(&self) -> Result<Self::CustomEntity559Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity559")
    }

    type CustomEntity560Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_560_repository(&self) -> Result<Self::CustomEntity560Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity560")
    }

    type CustomEntity561Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_561_repository(&self) -> Result<Self::CustomEntity561Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity561")
    }

    type CustomEntity562Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_562_repository(&self) -> Result<Self::CustomEntity562Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity562")
    }

    type CustomEntity563Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_563_repository(&self) -> Result<Self::CustomEntity563Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity563")
    }

    type CustomEntity564Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_564_repository(&self) -> Result<Self::CustomEntity564Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity564")
    }

    type CustomEntity565Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_565_repository(&self) -> Result<Self::CustomEntity565Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity565")
    }

    type CustomEntity566Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_566_repository(&self) -> Result<Self::CustomEntity566Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity566")
    }

    type CustomEntity567Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_567_repository(&self) -> Result<Self::CustomEntity567Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity567")
    }

    type CustomEntity568Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_568_repository(&self) -> Result<Self::CustomEntity568Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity568")
    }

    type CustomEntity569Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_569_repository(&self) -> Result<Self::CustomEntity569Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity569")
    }

    type CustomEntity570Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_570_repository(&self) -> Result<Self::CustomEntity570Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity570")
    }

    type CustomEntity571Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_571_repository(&self) -> Result<Self::CustomEntity571Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity571")
    }

    type CustomEntity572Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_572_repository(&self) -> Result<Self::CustomEntity572Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity572")
    }

    type CustomEntity573Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_573_repository(&self) -> Result<Self::CustomEntity573Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity573")
    }

    type CustomEntity574Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_574_repository(&self) -> Result<Self::CustomEntity574Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity574")
    }

    type CustomEntity575Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_575_repository(&self) -> Result<Self::CustomEntity575Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity575")
    }

    type CustomEntity576Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_576_repository(&self) -> Result<Self::CustomEntity576Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity576")
    }

    type CustomEntity577Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_577_repository(&self) -> Result<Self::CustomEntity577Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity577")
    }

    type CustomEntity578Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_578_repository(&self) -> Result<Self::CustomEntity578Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity578")
    }

    type CustomEntity579Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_579_repository(&self) -> Result<Self::CustomEntity579Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity579")
    }

    type CustomEntity580Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_580_repository(&self) -> Result<Self::CustomEntity580Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity580")
    }

    type CustomEntity581Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_581_repository(&self) -> Result<Self::CustomEntity581Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity581")
    }

    type CustomEntity582Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_582_repository(&self) -> Result<Self::CustomEntity582Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity582")
    }

    type CustomEntity583Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_583_repository(&self) -> Result<Self::CustomEntity583Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity583")
    }

    type CustomEntity584Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_584_repository(&self) -> Result<Self::CustomEntity584Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity584")
    }

    type CustomEntity585Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_585_repository(&self) -> Result<Self::CustomEntity585Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity585")
    }

    type CustomEntity586Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_586_repository(&self) -> Result<Self::CustomEntity586Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity586")
    }

    type CustomEntity587Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_587_repository(&self) -> Result<Self::CustomEntity587Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity587")
    }

    type CustomEntity588Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_588_repository(&self) -> Result<Self::CustomEntity588Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity588")
    }

    type CustomEntity589Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_589_repository(&self) -> Result<Self::CustomEntity589Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity589")
    }

    type CustomEntity590Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_590_repository(&self) -> Result<Self::CustomEntity590Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity590")
    }

    type CustomEntity591Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_591_repository(&self) -> Result<Self::CustomEntity591Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity591")
    }

    type CustomEntity592Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_592_repository(&self) -> Result<Self::CustomEntity592Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity592")
    }

    type CustomEntity593Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_593_repository(&self) -> Result<Self::CustomEntity593Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity593")
    }

    type CustomEntity594Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_594_repository(&self) -> Result<Self::CustomEntity594Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity594")
    }

    type CustomEntity595Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_595_repository(&self) -> Result<Self::CustomEntity595Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity595")
    }

    type CustomEntity596Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_596_repository(&self) -> Result<Self::CustomEntity596Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity596")
    }

    type CustomEntity597Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_597_repository(&self) -> Result<Self::CustomEntity597Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity597")
    }

    type CustomEntity598Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_598_repository(&self) -> Result<Self::CustomEntity598Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity598")
    }

    type CustomEntity599Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_599_repository(&self) -> Result<Self::CustomEntity599Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity599")
    }

    type CustomEntity600Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_600_repository(&self) -> Result<Self::CustomEntity600Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity600")
    }

    type CustomEntity601Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_601_repository(&self) -> Result<Self::CustomEntity601Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity601")
    }

    type CustomEntity602Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_602_repository(&self) -> Result<Self::CustomEntity602Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity602")
    }

    type CustomEntity603Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_603_repository(&self) -> Result<Self::CustomEntity603Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity603")
    }

    type CustomEntity604Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_604_repository(&self) -> Result<Self::CustomEntity604Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity604")
    }

    type CustomEntity605Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_605_repository(&self) -> Result<Self::CustomEntity605Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity605")
    }

    type CustomEntity606Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_606_repository(&self) -> Result<Self::CustomEntity606Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity606")
    }

    type CustomEntity607Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_607_repository(&self) -> Result<Self::CustomEntity607Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity607")
    }

    type CustomEntity608Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_608_repository(&self) -> Result<Self::CustomEntity608Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity608")
    }

    type CustomEntity609Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_609_repository(&self) -> Result<Self::CustomEntity609Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity609")
    }

    type CustomEntity610Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_610_repository(&self) -> Result<Self::CustomEntity610Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity610")
    }

    type CustomEntity611Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_611_repository(&self) -> Result<Self::CustomEntity611Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity611")
    }

    type CustomEntity612Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_612_repository(&self) -> Result<Self::CustomEntity612Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity612")
    }

    type CustomEntity613Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_613_repository(&self) -> Result<Self::CustomEntity613Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity613")
    }

    type CustomEntity614Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_614_repository(&self) -> Result<Self::CustomEntity614Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity614")
    }

    type CustomEntity615Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_615_repository(&self) -> Result<Self::CustomEntity615Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity615")
    }

    type CustomEntity616Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_616_repository(&self) -> Result<Self::CustomEntity616Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity616")
    }

    type CustomEntity617Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_617_repository(&self) -> Result<Self::CustomEntity617Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity617")
    }

    type CustomEntity618Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_618_repository(&self) -> Result<Self::CustomEntity618Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity618")
    }

    type CustomEntity619Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_619_repository(&self) -> Result<Self::CustomEntity619Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity619")
    }

    type CustomEntity620Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_620_repository(&self) -> Result<Self::CustomEntity620Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity620")
    }

    type CustomEntity621Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_621_repository(&self) -> Result<Self::CustomEntity621Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity621")
    }

    type CustomEntity622Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_622_repository(&self) -> Result<Self::CustomEntity622Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity622")
    }

    type CustomEntity623Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_623_repository(&self) -> Result<Self::CustomEntity623Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity623")
    }

    type CustomEntity624Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_624_repository(&self) -> Result<Self::CustomEntity624Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity624")
    }

    type CustomEntity625Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_625_repository(&self) -> Result<Self::CustomEntity625Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity625")
    }

    type CustomEntity626Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_626_repository(&self) -> Result<Self::CustomEntity626Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity626")
    }

    type CustomEntity627Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_627_repository(&self) -> Result<Self::CustomEntity627Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity627")
    }

    type CustomEntity628Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_628_repository(&self) -> Result<Self::CustomEntity628Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity628")
    }

    type CustomEntity629Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_629_repository(&self) -> Result<Self::CustomEntity629Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity629")
    }

    type CustomEntity630Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_630_repository(&self) -> Result<Self::CustomEntity630Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity630")
    }

    type CustomEntity631Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_631_repository(&self) -> Result<Self::CustomEntity631Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity631")
    }

    type CustomEntity632Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_632_repository(&self) -> Result<Self::CustomEntity632Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity632")
    }

    type CustomEntity633Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_633_repository(&self) -> Result<Self::CustomEntity633Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity633")
    }

    type CustomEntity634Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_634_repository(&self) -> Result<Self::CustomEntity634Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity634")
    }

    type CustomEntity635Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_635_repository(&self) -> Result<Self::CustomEntity635Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity635")
    }

    type CustomEntity636Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_636_repository(&self) -> Result<Self::CustomEntity636Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity636")
    }

    type CustomEntity637Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_637_repository(&self) -> Result<Self::CustomEntity637Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity637")
    }

    type CustomEntity638Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_638_repository(&self) -> Result<Self::CustomEntity638Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity638")
    }

    type CustomEntity639Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_639_repository(&self) -> Result<Self::CustomEntity639Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity639")
    }

    type CustomEntity640Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_640_repository(&self) -> Result<Self::CustomEntity640Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity640")
    }

    type CustomEntity641Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_641_repository(&self) -> Result<Self::CustomEntity641Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity641")
    }

    type CustomEntity642Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_642_repository(&self) -> Result<Self::CustomEntity642Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity642")
    }

    type CustomEntity643Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_643_repository(&self) -> Result<Self::CustomEntity643Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity643")
    }

    type CustomEntity644Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_644_repository(&self) -> Result<Self::CustomEntity644Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity644")
    }

    type CustomEntity645Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_645_repository(&self) -> Result<Self::CustomEntity645Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity645")
    }

    type CustomEntity646Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_646_repository(&self) -> Result<Self::CustomEntity646Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity646")
    }

    type CustomEntity647Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_647_repository(&self) -> Result<Self::CustomEntity647Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity647")
    }

    type CustomEntity648Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_648_repository(&self) -> Result<Self::CustomEntity648Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity648")
    }

    type CustomEntity649Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_649_repository(&self) -> Result<Self::CustomEntity649Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity649")
    }

    type CustomEntity650Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_650_repository(&self) -> Result<Self::CustomEntity650Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity650")
    }

    type CustomEntity651Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_651_repository(&self) -> Result<Self::CustomEntity651Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity651")
    }

    type CustomEntity652Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_652_repository(&self) -> Result<Self::CustomEntity652Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity652")
    }

    type CustomEntity653Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_653_repository(&self) -> Result<Self::CustomEntity653Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity653")
    }

    type CustomEntity654Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_654_repository(&self) -> Result<Self::CustomEntity654Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity654")
    }

    type CustomEntity655Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_655_repository(&self) -> Result<Self::CustomEntity655Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity655")
    }

    type CustomEntity656Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_656_repository(&self) -> Result<Self::CustomEntity656Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity656")
    }

    type CustomEntity657Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_657_repository(&self) -> Result<Self::CustomEntity657Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity657")
    }

    type CustomEntity658Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_658_repository(&self) -> Result<Self::CustomEntity658Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity658")
    }

    type CustomEntity659Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_659_repository(&self) -> Result<Self::CustomEntity659Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity659")
    }

    type CustomEntity660Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_660_repository(&self) -> Result<Self::CustomEntity660Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity660")
    }

    type CustomEntity661Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_661_repository(&self) -> Result<Self::CustomEntity661Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity661")
    }

    type CustomEntity662Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_662_repository(&self) -> Result<Self::CustomEntity662Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity662")
    }

    type CustomEntity663Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_663_repository(&self) -> Result<Self::CustomEntity663Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity663")
    }

    type CustomEntity664Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_664_repository(&self) -> Result<Self::CustomEntity664Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity664")
    }

    type CustomEntity665Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_665_repository(&self) -> Result<Self::CustomEntity665Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity665")
    }

    type CustomEntity666Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_666_repository(&self) -> Result<Self::CustomEntity666Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity666")
    }

    type CustomEntity667Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_667_repository(&self) -> Result<Self::CustomEntity667Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity667")
    }

    type CustomEntity668Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_668_repository(&self) -> Result<Self::CustomEntity668Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity668")
    }

    type CustomEntity669Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_669_repository(&self) -> Result<Self::CustomEntity669Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity669")
    }

    type CustomEntity670Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_670_repository(&self) -> Result<Self::CustomEntity670Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity670")
    }

    type CustomEntity671Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_671_repository(&self) -> Result<Self::CustomEntity671Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity671")
    }

    type CustomEntity672Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_672_repository(&self) -> Result<Self::CustomEntity672Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity672")
    }

    type CustomEntity673Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_673_repository(&self) -> Result<Self::CustomEntity673Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity673")
    }

    type CustomEntity674Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_674_repository(&self) -> Result<Self::CustomEntity674Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity674")
    }

    type CustomEntity675Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_675_repository(&self) -> Result<Self::CustomEntity675Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity675")
    }

    type CustomEntity676Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_676_repository(&self) -> Result<Self::CustomEntity676Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity676")
    }

    type CustomEntity677Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_677_repository(&self) -> Result<Self::CustomEntity677Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity677")
    }

    type CustomEntity678Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_678_repository(&self) -> Result<Self::CustomEntity678Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity678")
    }

    type CustomEntity679Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_679_repository(&self) -> Result<Self::CustomEntity679Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity679")
    }

    type CustomEntity680Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_680_repository(&self) -> Result<Self::CustomEntity680Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity680")
    }

    type CustomEntity681Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_681_repository(&self) -> Result<Self::CustomEntity681Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity681")
    }

    type CustomEntity682Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_682_repository(&self) -> Result<Self::CustomEntity682Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity682")
    }

    type CustomEntity683Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_683_repository(&self) -> Result<Self::CustomEntity683Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity683")
    }

    type CustomEntity684Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_684_repository(&self) -> Result<Self::CustomEntity684Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity684")
    }

    type CustomEntity685Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_685_repository(&self) -> Result<Self::CustomEntity685Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity685")
    }

    type CustomEntity686Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_686_repository(&self) -> Result<Self::CustomEntity686Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity686")
    }

    type CustomEntity687Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_687_repository(&self) -> Result<Self::CustomEntity687Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity687")
    }

    type CustomEntity688Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_688_repository(&self) -> Result<Self::CustomEntity688Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity688")
    }

    type CustomEntity689Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_689_repository(&self) -> Result<Self::CustomEntity689Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity689")
    }

    type CustomEntity690Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_690_repository(&self) -> Result<Self::CustomEntity690Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity690")
    }

    type CustomEntity691Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_691_repository(&self) -> Result<Self::CustomEntity691Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity691")
    }

    type CustomEntity692Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_692_repository(&self) -> Result<Self::CustomEntity692Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity692")
    }

    type CustomEntity693Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_693_repository(&self) -> Result<Self::CustomEntity693Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity693")
    }

    type CustomEntity694Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_694_repository(&self) -> Result<Self::CustomEntity694Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity694")
    }

    type CustomEntity695Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_695_repository(&self) -> Result<Self::CustomEntity695Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity695")
    }

    type CustomEntity696Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_696_repository(&self) -> Result<Self::CustomEntity696Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity696")
    }

    type CustomEntity697Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_697_repository(&self) -> Result<Self::CustomEntity697Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity697")
    }

    type CustomEntity698Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_698_repository(&self) -> Result<Self::CustomEntity698Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity698")
    }

    type CustomEntity699Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_699_repository(&self) -> Result<Self::CustomEntity699Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity699")
    }

    type CustomEntity700Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_700_repository(&self) -> Result<Self::CustomEntity700Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity700")
    }

    type CustomEntity701Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_701_repository(&self) -> Result<Self::CustomEntity701Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity701")
    }

    type CustomEntity702Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_702_repository(&self) -> Result<Self::CustomEntity702Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity702")
    }

    type CustomEntity703Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_703_repository(&self) -> Result<Self::CustomEntity703Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity703")
    }

    type CustomEntity704Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_704_repository(&self) -> Result<Self::CustomEntity704Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity704")
    }

    type CustomEntity705Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_705_repository(&self) -> Result<Self::CustomEntity705Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity705")
    }

    type CustomEntity706Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_706_repository(&self) -> Result<Self::CustomEntity706Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity706")
    }

    type CustomEntity707Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_707_repository(&self) -> Result<Self::CustomEntity707Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity707")
    }

    type CustomEntity708Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_708_repository(&self) -> Result<Self::CustomEntity708Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity708")
    }

    type CustomEntity709Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_709_repository(&self) -> Result<Self::CustomEntity709Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity709")
    }

    type CustomEntity710Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_710_repository(&self) -> Result<Self::CustomEntity710Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity710")
    }

    type CustomEntity711Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_711_repository(&self) -> Result<Self::CustomEntity711Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity711")
    }

    type CustomEntity712Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_712_repository(&self) -> Result<Self::CustomEntity712Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity712")
    }

    type CustomEntity713Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_713_repository(&self) -> Result<Self::CustomEntity713Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity713")
    }

    type CustomEntity714Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_714_repository(&self) -> Result<Self::CustomEntity714Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity714")
    }

    type CustomEntity715Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_715_repository(&self) -> Result<Self::CustomEntity715Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity715")
    }

    type CustomEntity716Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_716_repository(&self) -> Result<Self::CustomEntity716Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity716")
    }

    type CustomEntity717Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_717_repository(&self) -> Result<Self::CustomEntity717Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity717")
    }

    type CustomEntity718Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_718_repository(&self) -> Result<Self::CustomEntity718Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity718")
    }

    type CustomEntity719Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_719_repository(&self) -> Result<Self::CustomEntity719Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity719")
    }

    type CustomEntity720Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_720_repository(&self) -> Result<Self::CustomEntity720Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity720")
    }

    type CustomEntity721Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_721_repository(&self) -> Result<Self::CustomEntity721Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity721")
    }

    type CustomEntity722Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_722_repository(&self) -> Result<Self::CustomEntity722Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity722")
    }

    type CustomEntity723Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_723_repository(&self) -> Result<Self::CustomEntity723Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity723")
    }

    type CustomEntity724Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_724_repository(&self) -> Result<Self::CustomEntity724Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity724")
    }

    type CustomEntity725Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_725_repository(&self) -> Result<Self::CustomEntity725Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity725")
    }

    type CustomEntity726Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_726_repository(&self) -> Result<Self::CustomEntity726Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity726")
    }

    type CustomEntity727Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_727_repository(&self) -> Result<Self::CustomEntity727Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity727")
    }

    type CustomEntity728Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_728_repository(&self) -> Result<Self::CustomEntity728Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity728")
    }

    type CustomEntity729Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_729_repository(&self) -> Result<Self::CustomEntity729Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity729")
    }

    type CustomEntity730Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_730_repository(&self) -> Result<Self::CustomEntity730Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity730")
    }

    type CustomEntity731Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_731_repository(&self) -> Result<Self::CustomEntity731Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity731")
    }

    type CustomEntity732Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_732_repository(&self) -> Result<Self::CustomEntity732Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity732")
    }

    type CustomEntity733Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_733_repository(&self) -> Result<Self::CustomEntity733Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity733")
    }

    type CustomEntity734Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_734_repository(&self) -> Result<Self::CustomEntity734Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity734")
    }

    type CustomEntity735Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_735_repository(&self) -> Result<Self::CustomEntity735Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity735")
    }

    type CustomEntity736Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_736_repository(&self) -> Result<Self::CustomEntity736Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity736")
    }

    type CustomEntity737Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_737_repository(&self) -> Result<Self::CustomEntity737Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity737")
    }

    type CustomEntity738Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_738_repository(&self) -> Result<Self::CustomEntity738Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity738")
    }

    type CustomEntity739Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_739_repository(&self) -> Result<Self::CustomEntity739Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity739")
    }

    type CustomEntity740Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_740_repository(&self) -> Result<Self::CustomEntity740Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity740")
    }

    type CustomEntity741Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_741_repository(&self) -> Result<Self::CustomEntity741Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity741")
    }

    type CustomEntity742Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_742_repository(&self) -> Result<Self::CustomEntity742Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity742")
    }

    type CustomEntity743Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_743_repository(&self) -> Result<Self::CustomEntity743Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity743")
    }

    type CustomEntity744Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_744_repository(&self) -> Result<Self::CustomEntity744Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity744")
    }

    type CustomEntity745Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_745_repository(&self) -> Result<Self::CustomEntity745Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity745")
    }

    type CustomEntity746Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_746_repository(&self) -> Result<Self::CustomEntity746Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity746")
    }

    type CustomEntity747Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_747_repository(&self) -> Result<Self::CustomEntity747Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity747")
    }

    type CustomEntity748Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_748_repository(&self) -> Result<Self::CustomEntity748Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity748")
    }

    type CustomEntity749Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_749_repository(&self) -> Result<Self::CustomEntity749Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity749")
    }

    type CustomEntity750Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_750_repository(&self) -> Result<Self::CustomEntity750Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity750")
    }

    type CustomEntity751Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_751_repository(&self) -> Result<Self::CustomEntity751Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity751")
    }

    type CustomEntity752Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_752_repository(&self) -> Result<Self::CustomEntity752Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity752")
    }

    type CustomEntity753Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_753_repository(&self) -> Result<Self::CustomEntity753Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity753")
    }

    type CustomEntity754Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_754_repository(&self) -> Result<Self::CustomEntity754Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity754")
    }

    type CustomEntity755Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_755_repository(&self) -> Result<Self::CustomEntity755Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity755")
    }

    type CustomEntity756Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_756_repository(&self) -> Result<Self::CustomEntity756Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity756")
    }

    type CustomEntity757Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_757_repository(&self) -> Result<Self::CustomEntity757Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity757")
    }

    type CustomEntity758Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_758_repository(&self) -> Result<Self::CustomEntity758Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity758")
    }

    type CustomEntity759Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_759_repository(&self) -> Result<Self::CustomEntity759Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity759")
    }

    type CustomEntity760Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_760_repository(&self) -> Result<Self::CustomEntity760Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity760")
    }

    type CustomEntity761Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_761_repository(&self) -> Result<Self::CustomEntity761Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity761")
    }

    type CustomEntity762Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_762_repository(&self) -> Result<Self::CustomEntity762Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity762")
    }

    type CustomEntity763Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_763_repository(&self) -> Result<Self::CustomEntity763Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity763")
    }

    type CustomEntity764Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_764_repository(&self) -> Result<Self::CustomEntity764Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity764")
    }

    type CustomEntity765Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_765_repository(&self) -> Result<Self::CustomEntity765Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity765")
    }

    type CustomEntity766Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_766_repository(&self) -> Result<Self::CustomEntity766Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity766")
    }

    type CustomEntity767Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_767_repository(&self) -> Result<Self::CustomEntity767Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity767")
    }

    type CustomEntity768Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_768_repository(&self) -> Result<Self::CustomEntity768Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity768")
    }

    type CustomEntity769Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_769_repository(&self) -> Result<Self::CustomEntity769Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity769")
    }

    type CustomEntity770Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_770_repository(&self) -> Result<Self::CustomEntity770Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity770")
    }

    type CustomEntity771Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_771_repository(&self) -> Result<Self::CustomEntity771Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity771")
    }

    type CustomEntity772Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_772_repository(&self) -> Result<Self::CustomEntity772Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity772")
    }

    type CustomEntity773Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_773_repository(&self) -> Result<Self::CustomEntity773Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity773")
    }

    type CustomEntity774Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_774_repository(&self) -> Result<Self::CustomEntity774Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity774")
    }

    type CustomEntity775Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_775_repository(&self) -> Result<Self::CustomEntity775Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity775")
    }

    type CustomEntity776Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_776_repository(&self) -> Result<Self::CustomEntity776Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity776")
    }

    type CustomEntity777Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_777_repository(&self) -> Result<Self::CustomEntity777Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity777")
    }

    type CustomEntity778Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_778_repository(&self) -> Result<Self::CustomEntity778Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity778")
    }

    type CustomEntity779Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_779_repository(&self) -> Result<Self::CustomEntity779Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity779")
    }

    type CustomEntity780Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_780_repository(&self) -> Result<Self::CustomEntity780Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity780")
    }

    type CustomEntity781Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_781_repository(&self) -> Result<Self::CustomEntity781Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity781")
    }

    type CustomEntity782Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_782_repository(&self) -> Result<Self::CustomEntity782Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity782")
    }

    type CustomEntity783Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_783_repository(&self) -> Result<Self::CustomEntity783Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity783")
    }

    type CustomEntity784Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_784_repository(&self) -> Result<Self::CustomEntity784Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity784")
    }

    type CustomEntity785Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_785_repository(&self) -> Result<Self::CustomEntity785Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity785")
    }

    type CustomEntity786Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_786_repository(&self) -> Result<Self::CustomEntity786Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity786")
    }

    type CustomEntity787Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_787_repository(&self) -> Result<Self::CustomEntity787Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity787")
    }

    type CustomEntity788Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_788_repository(&self) -> Result<Self::CustomEntity788Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity788")
    }

    type CustomEntity789Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_789_repository(&self) -> Result<Self::CustomEntity789Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity789")
    }

    type CustomEntity790Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_790_repository(&self) -> Result<Self::CustomEntity790Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity790")
    }

    type CustomEntity791Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_791_repository(&self) -> Result<Self::CustomEntity791Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity791")
    }

    type CustomEntity792Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_792_repository(&self) -> Result<Self::CustomEntity792Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity792")
    }

    type CustomEntity793Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_793_repository(&self) -> Result<Self::CustomEntity793Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity793")
    }

    type CustomEntity794Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_794_repository(&self) -> Result<Self::CustomEntity794Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity794")
    }

    type CustomEntity795Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_795_repository(&self) -> Result<Self::CustomEntity795Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity795")
    }

    type CustomEntity796Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_796_repository(&self) -> Result<Self::CustomEntity796Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity796")
    }

    type CustomEntity797Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_797_repository(&self) -> Result<Self::CustomEntity797Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity797")
    }

    type CustomEntity798Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_798_repository(&self) -> Result<Self::CustomEntity798Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity798")
    }

    type CustomEntity799Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_799_repository(&self) -> Result<Self::CustomEntity799Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity799")
    }

    type CustomEntity800Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_800_repository(&self) -> Result<Self::CustomEntity800Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity800")
    }

    type CustomEntity801Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_801_repository(&self) -> Result<Self::CustomEntity801Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity801")
    }

    type CustomEntity802Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_802_repository(&self) -> Result<Self::CustomEntity802Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity802")
    }

    type CustomEntity803Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_803_repository(&self) -> Result<Self::CustomEntity803Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity803")
    }

    type CustomEntity804Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_804_repository(&self) -> Result<Self::CustomEntity804Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity804")
    }

    type CustomEntity805Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_805_repository(&self) -> Result<Self::CustomEntity805Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity805")
    }

    type CustomEntity806Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_806_repository(&self) -> Result<Self::CustomEntity806Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity806")
    }

    type CustomEntity807Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_807_repository(&self) -> Result<Self::CustomEntity807Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity807")
    }

    type CustomEntity808Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_808_repository(&self) -> Result<Self::CustomEntity808Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity808")
    }

    type CustomEntity809Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_809_repository(&self) -> Result<Self::CustomEntity809Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity809")
    }

    type CustomEntity810Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_810_repository(&self) -> Result<Self::CustomEntity810Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity810")
    }

    type CustomEntity811Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_811_repository(&self) -> Result<Self::CustomEntity811Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity811")
    }

    type CustomEntity812Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_812_repository(&self) -> Result<Self::CustomEntity812Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity812")
    }

    type CustomEntity813Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_813_repository(&self) -> Result<Self::CustomEntity813Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity813")
    }

    type CustomEntity814Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_814_repository(&self) -> Result<Self::CustomEntity814Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity814")
    }

    type CustomEntity815Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_815_repository(&self) -> Result<Self::CustomEntity815Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity815")
    }

    type CustomEntity816Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_816_repository(&self) -> Result<Self::CustomEntity816Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity816")
    }

    type CustomEntity817Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_817_repository(&self) -> Result<Self::CustomEntity817Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity817")
    }

    type CustomEntity818Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_818_repository(&self) -> Result<Self::CustomEntity818Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity818")
    }

    type CustomEntity819Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_819_repository(&self) -> Result<Self::CustomEntity819Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity819")
    }

    type CustomEntity820Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_820_repository(&self) -> Result<Self::CustomEntity820Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity820")
    }

    type CustomEntity821Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_821_repository(&self) -> Result<Self::CustomEntity821Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity821")
    }

    type CustomEntity822Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_822_repository(&self) -> Result<Self::CustomEntity822Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity822")
    }

    type CustomEntity823Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_823_repository(&self) -> Result<Self::CustomEntity823Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity823")
    }

    type CustomEntity824Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_824_repository(&self) -> Result<Self::CustomEntity824Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity824")
    }

    type CustomEntity825Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_825_repository(&self) -> Result<Self::CustomEntity825Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity825")
    }

    type CustomEntity826Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_826_repository(&self) -> Result<Self::CustomEntity826Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity826")
    }

    type CustomEntity827Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_827_repository(&self) -> Result<Self::CustomEntity827Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity827")
    }

    type CustomEntity828Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_828_repository(&self) -> Result<Self::CustomEntity828Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity828")
    }

    type CustomEntity829Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_829_repository(&self) -> Result<Self::CustomEntity829Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity829")
    }

    type CustomEntity830Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_830_repository(&self) -> Result<Self::CustomEntity830Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity830")
    }

    type CustomEntity831Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_831_repository(&self) -> Result<Self::CustomEntity831Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity831")
    }

    type CustomEntity832Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_832_repository(&self) -> Result<Self::CustomEntity832Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity832")
    }

    type CustomEntity833Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_833_repository(&self) -> Result<Self::CustomEntity833Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity833")
    }

    type CustomEntity834Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_834_repository(&self) -> Result<Self::CustomEntity834Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity834")
    }

    type CustomEntity835Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_835_repository(&self) -> Result<Self::CustomEntity835Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity835")
    }

    type CustomEntity836Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_836_repository(&self) -> Result<Self::CustomEntity836Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity836")
    }

    type CustomEntity837Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_837_repository(&self) -> Result<Self::CustomEntity837Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity837")
    }

    type CustomEntity838Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_838_repository(&self) -> Result<Self::CustomEntity838Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity838")
    }

    type CustomEntity839Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_839_repository(&self) -> Result<Self::CustomEntity839Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity839")
    }

    type CustomEntity840Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_840_repository(&self) -> Result<Self::CustomEntity840Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity840")
    }

    type CustomEntity841Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_841_repository(&self) -> Result<Self::CustomEntity841Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity841")
    }

    type CustomEntity842Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_842_repository(&self) -> Result<Self::CustomEntity842Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity842")
    }

    type CustomEntity843Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_843_repository(&self) -> Result<Self::CustomEntity843Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity843")
    }

    type CustomEntity844Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_844_repository(&self) -> Result<Self::CustomEntity844Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity844")
    }

    type CustomEntity845Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_845_repository(&self) -> Result<Self::CustomEntity845Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity845")
    }

    type CustomEntity846Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_846_repository(&self) -> Result<Self::CustomEntity846Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity846")
    }

    type CustomEntity847Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_847_repository(&self) -> Result<Self::CustomEntity847Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity847")
    }

    type CustomEntity848Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_848_repository(&self) -> Result<Self::CustomEntity848Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity848")
    }

    type CustomEntity849Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_849_repository(&self) -> Result<Self::CustomEntity849Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity849")
    }

    type CustomEntity850Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_850_repository(&self) -> Result<Self::CustomEntity850Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity850")
    }

    type CustomEntity851Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_851_repository(&self) -> Result<Self::CustomEntity851Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity851")
    }

    type CustomEntity852Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_852_repository(&self) -> Result<Self::CustomEntity852Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity852")
    }

    type CustomEntity853Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_853_repository(&self) -> Result<Self::CustomEntity853Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity853")
    }

    type CustomEntity854Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_854_repository(&self) -> Result<Self::CustomEntity854Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity854")
    }

    type CustomEntity855Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_855_repository(&self) -> Result<Self::CustomEntity855Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity855")
    }

    type CustomEntity856Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_856_repository(&self) -> Result<Self::CustomEntity856Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity856")
    }

    type CustomEntity857Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_857_repository(&self) -> Result<Self::CustomEntity857Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity857")
    }

    type CustomEntity858Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_858_repository(&self) -> Result<Self::CustomEntity858Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity858")
    }

    type CustomEntity859Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_859_repository(&self) -> Result<Self::CustomEntity859Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity859")
    }

    type CustomEntity860Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_860_repository(&self) -> Result<Self::CustomEntity860Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity860")
    }

    type CustomEntity861Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_861_repository(&self) -> Result<Self::CustomEntity861Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity861")
    }

    type CustomEntity862Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_862_repository(&self) -> Result<Self::CustomEntity862Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity862")
    }

    type CustomEntity863Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_863_repository(&self) -> Result<Self::CustomEntity863Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity863")
    }

    type CustomEntity864Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_864_repository(&self) -> Result<Self::CustomEntity864Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity864")
    }

    type CustomEntity865Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_865_repository(&self) -> Result<Self::CustomEntity865Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity865")
    }

    type CustomEntity866Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_866_repository(&self) -> Result<Self::CustomEntity866Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity866")
    }

    type CustomEntity867Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_867_repository(&self) -> Result<Self::CustomEntity867Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity867")
    }

    type CustomEntity868Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_868_repository(&self) -> Result<Self::CustomEntity868Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity868")
    }

    type CustomEntity869Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_869_repository(&self) -> Result<Self::CustomEntity869Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity869")
    }

    type CustomEntity870Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_870_repository(&self) -> Result<Self::CustomEntity870Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity870")
    }

    type CustomEntity871Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_871_repository(&self) -> Result<Self::CustomEntity871Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity871")
    }

    type CustomEntity872Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_872_repository(&self) -> Result<Self::CustomEntity872Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity872")
    }

    type CustomEntity873Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_873_repository(&self) -> Result<Self::CustomEntity873Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity873")
    }

    type CustomEntity874Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_874_repository(&self) -> Result<Self::CustomEntity874Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity874")
    }

    type CustomEntity875Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_875_repository(&self) -> Result<Self::CustomEntity875Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity875")
    }

    type CustomEntity876Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_876_repository(&self) -> Result<Self::CustomEntity876Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity876")
    }

    type CustomEntity877Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_877_repository(&self) -> Result<Self::CustomEntity877Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity877")
    }

    type CustomEntity878Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_878_repository(&self) -> Result<Self::CustomEntity878Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity878")
    }

    type CustomEntity879Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_879_repository(&self) -> Result<Self::CustomEntity879Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity879")
    }

    type CustomEntity880Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_880_repository(&self) -> Result<Self::CustomEntity880Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity880")
    }

    type CustomEntity881Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_881_repository(&self) -> Result<Self::CustomEntity881Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity881")
    }

    type CustomEntity882Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_882_repository(&self) -> Result<Self::CustomEntity882Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity882")
    }

    type CustomEntity883Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_883_repository(&self) -> Result<Self::CustomEntity883Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity883")
    }

    type CustomEntity884Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_884_repository(&self) -> Result<Self::CustomEntity884Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity884")
    }

    type CustomEntity885Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_885_repository(&self) -> Result<Self::CustomEntity885Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity885")
    }

    type CustomEntity886Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_886_repository(&self) -> Result<Self::CustomEntity886Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity886")
    }

    type CustomEntity887Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_887_repository(&self) -> Result<Self::CustomEntity887Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity887")
    }

    type CustomEntity888Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_888_repository(&self) -> Result<Self::CustomEntity888Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity888")
    }

    type CustomEntity889Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_889_repository(&self) -> Result<Self::CustomEntity889Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity889")
    }

    type CustomEntity890Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_890_repository(&self) -> Result<Self::CustomEntity890Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity890")
    }

    type CustomEntity891Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_891_repository(&self) -> Result<Self::CustomEntity891Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity891")
    }

    type CustomEntity892Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_892_repository(&self) -> Result<Self::CustomEntity892Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity892")
    }

    type CustomEntity893Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_893_repository(&self) -> Result<Self::CustomEntity893Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity893")
    }

    type CustomEntity894Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_894_repository(&self) -> Result<Self::CustomEntity894Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity894")
    }

    type CustomEntity895Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_895_repository(&self) -> Result<Self::CustomEntity895Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity895")
    }

    type CustomEntity896Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_896_repository(&self) -> Result<Self::CustomEntity896Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity896")
    }

    type CustomEntity897Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_897_repository(&self) -> Result<Self::CustomEntity897Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity897")
    }

    type CustomEntity898Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_898_repository(&self) -> Result<Self::CustomEntity898Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity898")
    }

    type CustomEntity899Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_899_repository(&self) -> Result<Self::CustomEntity899Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity899")
    }

    type CustomEntity900Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_900_repository(&self) -> Result<Self::CustomEntity900Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity900")
    }

    type CustomEntity901Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_901_repository(&self) -> Result<Self::CustomEntity901Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity901")
    }

    type CustomEntity902Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_902_repository(&self) -> Result<Self::CustomEntity902Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity902")
    }

    type CustomEntity903Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_903_repository(&self) -> Result<Self::CustomEntity903Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity903")
    }

    type CustomEntity904Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_904_repository(&self) -> Result<Self::CustomEntity904Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity904")
    }

    type CustomEntity905Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_905_repository(&self) -> Result<Self::CustomEntity905Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity905")
    }

    type CustomEntity906Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_906_repository(&self) -> Result<Self::CustomEntity906Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity906")
    }

    type CustomEntity907Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_907_repository(&self) -> Result<Self::CustomEntity907Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity907")
    }

    type CustomEntity908Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_908_repository(&self) -> Result<Self::CustomEntity908Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity908")
    }

    type CustomEntity909Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_909_repository(&self) -> Result<Self::CustomEntity909Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity909")
    }

    type CustomEntity910Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_910_repository(&self) -> Result<Self::CustomEntity910Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity910")
    }

    type CustomEntity911Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_911_repository(&self) -> Result<Self::CustomEntity911Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity911")
    }

    type CustomEntity912Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_912_repository(&self) -> Result<Self::CustomEntity912Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity912")
    }

    type CustomEntity913Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_913_repository(&self) -> Result<Self::CustomEntity913Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity913")
    }

    type CustomEntity914Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_914_repository(&self) -> Result<Self::CustomEntity914Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity914")
    }

    type CustomEntity915Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_915_repository(&self) -> Result<Self::CustomEntity915Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity915")
    }

    type CustomEntity916Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_916_repository(&self) -> Result<Self::CustomEntity916Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity916")
    }

    type CustomEntity917Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_917_repository(&self) -> Result<Self::CustomEntity917Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity917")
    }

    type CustomEntity918Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_918_repository(&self) -> Result<Self::CustomEntity918Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity918")
    }

    type CustomEntity919Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_919_repository(&self) -> Result<Self::CustomEntity919Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity919")
    }

    type CustomEntity920Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_920_repository(&self) -> Result<Self::CustomEntity920Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity920")
    }

    type CustomEntity921Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_921_repository(&self) -> Result<Self::CustomEntity921Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity921")
    }

    type CustomEntity922Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_922_repository(&self) -> Result<Self::CustomEntity922Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity922")
    }

    type CustomEntity923Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_923_repository(&self) -> Result<Self::CustomEntity923Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity923")
    }

    type CustomEntity924Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_924_repository(&self) -> Result<Self::CustomEntity924Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity924")
    }

    type CustomEntity925Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_925_repository(&self) -> Result<Self::CustomEntity925Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity925")
    }

    type CustomEntity926Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_926_repository(&self) -> Result<Self::CustomEntity926Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity926")
    }

    type CustomEntity927Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_927_repository(&self) -> Result<Self::CustomEntity927Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity927")
    }

    type CustomEntity928Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_928_repository(&self) -> Result<Self::CustomEntity928Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity928")
    }

    type CustomEntity929Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_929_repository(&self) -> Result<Self::CustomEntity929Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity929")
    }

    type CustomEntity930Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_930_repository(&self) -> Result<Self::CustomEntity930Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity930")
    }

    type CustomEntity931Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_931_repository(&self) -> Result<Self::CustomEntity931Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity931")
    }

    type CustomEntity932Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_932_repository(&self) -> Result<Self::CustomEntity932Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity932")
    }

    type CustomEntity933Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_933_repository(&self) -> Result<Self::CustomEntity933Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity933")
    }

    type CustomEntity934Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_934_repository(&self) -> Result<Self::CustomEntity934Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity934")
    }

    type CustomEntity935Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_935_repository(&self) -> Result<Self::CustomEntity935Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity935")
    }

    type CustomEntity936Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_936_repository(&self) -> Result<Self::CustomEntity936Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity936")
    }

    type CustomEntity937Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_937_repository(&self) -> Result<Self::CustomEntity937Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity937")
    }

    type CustomEntity938Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_938_repository(&self) -> Result<Self::CustomEntity938Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity938")
    }

    type CustomEntity939Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_939_repository(&self) -> Result<Self::CustomEntity939Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity939")
    }

    type CustomEntity940Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_940_repository(&self) -> Result<Self::CustomEntity940Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity940")
    }

    type CustomEntity941Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_941_repository(&self) -> Result<Self::CustomEntity941Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity941")
    }

    type CustomEntity942Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_942_repository(&self) -> Result<Self::CustomEntity942Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity942")
    }

    type CustomEntity943Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_943_repository(&self) -> Result<Self::CustomEntity943Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity943")
    }

    type CustomEntity944Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_944_repository(&self) -> Result<Self::CustomEntity944Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity944")
    }

    type CustomEntity945Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_945_repository(&self) -> Result<Self::CustomEntity945Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity945")
    }

    type CustomEntity946Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_946_repository(&self) -> Result<Self::CustomEntity946Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity946")
    }

    type CustomEntity947Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_947_repository(&self) -> Result<Self::CustomEntity947Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity947")
    }

    type CustomEntity948Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_948_repository(&self) -> Result<Self::CustomEntity948Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity948")
    }

    type CustomEntity949Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_949_repository(&self) -> Result<Self::CustomEntity949Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity949")
    }

    type CustomEntity950Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_950_repository(&self) -> Result<Self::CustomEntity950Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity950")
    }

    type CustomEntity951Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_951_repository(&self) -> Result<Self::CustomEntity951Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity951")
    }

    type CustomEntity952Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_952_repository(&self) -> Result<Self::CustomEntity952Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity952")
    }

    type CustomEntity953Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_953_repository(&self) -> Result<Self::CustomEntity953Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity953")
    }

    type CustomEntity954Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_954_repository(&self) -> Result<Self::CustomEntity954Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity954")
    }

    type CustomEntity955Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_955_repository(&self) -> Result<Self::CustomEntity955Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity955")
    }

    type CustomEntity956Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_956_repository(&self) -> Result<Self::CustomEntity956Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity956")
    }

    type CustomEntity957Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_957_repository(&self) -> Result<Self::CustomEntity957Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity957")
    }

    type CustomEntity958Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_958_repository(&self) -> Result<Self::CustomEntity958Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity958")
    }

    type CustomEntity959Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_959_repository(&self) -> Result<Self::CustomEntity959Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity959")
    }

    type CustomEntity960Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_960_repository(&self) -> Result<Self::CustomEntity960Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity960")
    }

    type CustomEntity961Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_961_repository(&self) -> Result<Self::CustomEntity961Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity961")
    }

    type CustomEntity962Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_962_repository(&self) -> Result<Self::CustomEntity962Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity962")
    }

    type CustomEntity963Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_963_repository(&self) -> Result<Self::CustomEntity963Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity963")
    }

    type CustomEntity964Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_964_repository(&self) -> Result<Self::CustomEntity964Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity964")
    }

    type CustomEntity965Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_965_repository(&self) -> Result<Self::CustomEntity965Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity965")
    }

    type CustomEntity966Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_966_repository(&self) -> Result<Self::CustomEntity966Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity966")
    }

    type CustomEntity967Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_967_repository(&self) -> Result<Self::CustomEntity967Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity967")
    }

    type CustomEntity968Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_968_repository(&self) -> Result<Self::CustomEntity968Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity968")
    }

    type CustomEntity969Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_969_repository(&self) -> Result<Self::CustomEntity969Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity969")
    }

    type CustomEntity970Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_970_repository(&self) -> Result<Self::CustomEntity970Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity970")
    }

    type CustomEntity971Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_971_repository(&self) -> Result<Self::CustomEntity971Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity971")
    }

    type CustomEntity972Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_972_repository(&self) -> Result<Self::CustomEntity972Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity972")
    }

    type CustomEntity973Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_973_repository(&self) -> Result<Self::CustomEntity973Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity973")
    }

    type CustomEntity974Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_974_repository(&self) -> Result<Self::CustomEntity974Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity974")
    }

    type CustomEntity975Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_975_repository(&self) -> Result<Self::CustomEntity975Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity975")
    }

    type CustomEntity976Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_976_repository(&self) -> Result<Self::CustomEntity976Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity976")
    }

    type CustomEntity977Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_977_repository(&self) -> Result<Self::CustomEntity977Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity977")
    }

    type CustomEntity978Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_978_repository(&self) -> Result<Self::CustomEntity978Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity978")
    }

    type CustomEntity979Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_979_repository(&self) -> Result<Self::CustomEntity979Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity979")
    }

    type CustomEntity980Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_980_repository(&self) -> Result<Self::CustomEntity980Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity980")
    }

    type CustomEntity981Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_981_repository(&self) -> Result<Self::CustomEntity981Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity981")
    }

    type CustomEntity982Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_982_repository(&self) -> Result<Self::CustomEntity982Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity982")
    }

    type CustomEntity983Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_983_repository(&self) -> Result<Self::CustomEntity983Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity983")
    }

    type CustomEntity984Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_984_repository(&self) -> Result<Self::CustomEntity984Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity984")
    }

    type CustomEntity985Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_985_repository(&self) -> Result<Self::CustomEntity985Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity985")
    }

    type CustomEntity986Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_986_repository(&self) -> Result<Self::CustomEntity986Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity986")
    }

    type CustomEntity987Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_987_repository(&self) -> Result<Self::CustomEntity987Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity987")
    }

    type CustomEntity988Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_988_repository(&self) -> Result<Self::CustomEntity988Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity988")
    }

    type CustomEntity989Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_989_repository(&self) -> Result<Self::CustomEntity989Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity989")
    }

    type CustomEntity990Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_990_repository(&self) -> Result<Self::CustomEntity990Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity990")
    }

    type CustomEntity991Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_991_repository(&self) -> Result<Self::CustomEntity991Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity991")
    }

    type CustomEntity992Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_992_repository(&self) -> Result<Self::CustomEntity992Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity992")
    }

    type CustomEntity993Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_993_repository(&self) -> Result<Self::CustomEntity993Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity993")
    }

    type CustomEntity994Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_994_repository(&self) -> Result<Self::CustomEntity994Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity994")
    }

    type CustomEntity995Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_995_repository(&self) -> Result<Self::CustomEntity995Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity995")
    }

    type CustomEntity996Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_996_repository(&self) -> Result<Self::CustomEntity996Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity996")
    }

    type CustomEntity997Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_997_repository(&self) -> Result<Self::CustomEntity997Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity997")
    }

    type CustomEntity998Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_998_repository(&self) -> Result<Self::CustomEntity998Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity998")
    }

    type CustomEntity999Repository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn custom_entity_999_repository(&self) -> Result<Self::CustomEntity999Repository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomEntity999")
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
